use std::{
    cmp::{max, min},
    path::PathBuf,
};

use clap::Parser;
use image::{io::Reader as ImageReader, DynamicImage, GenericImage, GenericImageView};
use std::fs;

mod grid;
mod pack;

const TARGET: &str = "new";
const ALPHA: u32 = 128;

#[derive(Parser, Debug)]
struct Args {
    #[clap(long, default_value = "false")]
    folders: bool,
    image: PathBuf,
    tmpk: PathBuf,
}

fn to_color(value: u8) -> image::Rgb<u8> {
    let rv = value as u64 + 42;
    let gv = value as u64 + 69;
    let bv = value as u64 + 20;

    let red = ((rv * rv) % 255) as u8;
    let green = ((gv * gv * gv) % 255) as u8;
    let blue = ((bv * bv * bv * bv) % 255) as u8;

    image::Rgb([red, green, blue])
}

fn get_grid_value(grid_s: &grid::Grid, x: u32, y: u32) -> u8 {
    let mut grid_part = grid_s.info.c
        [((y >> 7) * (grid_s.info.width) as u32) as usize + (x >> 7) as usize]
        as usize
        * 4;

    if (y & 64) != 0 {
        grid_part += 2;
    }

    if (x & 64) != 0 {
        grid_part += 1;
    }

    let mut grid_part_1 = grid_s.header1[grid_part as usize] as usize * 4;

    if (y & 32) != 0 {
        grid_part_1 += 2;
    }

    if (x & 32) == 0 {
        grid_part_1 = grid_part_1 << 1;
    } else {
        grid_part_1 = grid_part_1 * 2 + 2;
    }

    let mut grid_part_2 = u16::from_le_bytes([
        grid_s.header2[grid_part_1 as usize],
        grid_s.header2[(grid_part_1 + 1) as usize],
    ]) as usize
        * 4;

    if (y & 16) != 0 {
        grid_part_2 = grid_part_2 + 2;
    }

    if (x & 16) == 0 {
        grid_part_2 = grid_part_2 << 1;
    } else {
        grid_part_2 = grid_part_2 * 2 + 2;
    }

    let mut fpart = u16::from_le_bytes([
        grid_s.header3[grid_part_2 as usize],
        grid_s.header3[(grid_part_2 + 1) as usize],
    ]) as usize
        * 4;

    if (y & 8) != 0 {
        fpart = fpart + 2;
    }

    if (x & 8) == 0 {
        fpart = fpart << 1;
    } else {
        fpart = fpart * 2 + 2;
    }

    grid_s.blocks[grid_s.indices[fpart as usize] as usize][(y & 7) as usize][(x & 7) as usize]
}

fn display_grid(grids: Vec<grid::Grid>, og: &DynamicImage, filename: &str) {
    fs::create_dir_all(format!("{TARGET}/{filename}")).unwrap();

    for i in 0..grids.len() {
        let grid_s = &grids[i];

        let mut new_image = og.clone();

        let full_width = (grid_s.info.width as u32) * 128;
        let full_height = (grid_s.info.height as u32) * 128;

        for i in 0..full_width {
            for j in 0..full_height {
                let cv = get_grid_value(grid_s, i, j);

                if cv > 0 {
                    let current_pixel = new_image.get_pixel(i, j);

                    let color = to_color(cv);

                    let result_alpha = ALPHA + ((255 - ALPHA) * current_pixel[3] as u32) / 255;

                    let r_result = min(
                        max(
                            (color[0] as u32 * ALPHA
                                + ((current_pixel[3] as u32)
                                    * (current_pixel[0] as u32)
                                    * (255 - ALPHA))
                                    / 255)
                                / result_alpha,
                            0,
                        ),
                        255,
                    );

                    let g_result = min(
                        max(
                            (color[1] as u32 * ALPHA
                                + ((current_pixel[3] as u32)
                                    * (current_pixel[1] as u32)
                                    * (255 - ALPHA))
                                    / 255)
                                / result_alpha,
                            0,
                        ),
                        255,
                    );

                    let b_result = min(
                        max(
                            (color[2] as u32 * ALPHA
                                + ((current_pixel[3] as u32)
                                    * (current_pixel[2] as u32)
                                    * (255 - ALPHA))
                                    / 255)
                                / result_alpha,
                            0,
                        ),
                        255,
                    );

                    let new_pixel =
                        image::Rgba::<u8>([r_result as u8, g_result as u8, b_result as u8, 255]);
                    new_image.put_pixel(i, j, new_pixel);
                }
            }
        }

        new_image
            .save(format!("{TARGET}/{filename}/{i}.png"))
            .unwrap();
    }

    og.save(format!("{TARGET}/{filename}/original.png"))
        .unwrap();
}

fn handle_single(args: Args) {
    println!("{}", args.image.file_name().unwrap().to_str().unwrap());

    let tmpk = fs::read(&args.tmpk).unwrap();

    let raw_files = pack::Packed::from(tmpk);

    let mut grids: Vec<grid::Grid> = Vec::new();

    for file in &raw_files.files {
        let grid_raw = pack::Packed::from(file.clone());

        if grid_raw.files.len() != 6 {
            continue;
        }

        let offsets_raw = &grid_raw.files[0];

        if offsets_raw.len() < 24 {
            continue;
        }

        let grid_header = grid::GridOffsets {
            info_offset: u32::from_le_bytes([
                offsets_raw[0],
                offsets_raw[1],
                offsets_raw[2],
                offsets_raw[3],
            ]),
            header1: u32::from_le_bytes([
                offsets_raw[4],
                offsets_raw[5],
                offsets_raw[6],
                offsets_raw[7],
            ]),
            header2: u32::from_le_bytes([
                offsets_raw[8],
                offsets_raw[9],
                offsets_raw[10],
                offsets_raw[11],
            ]),
            header3: u32::from_le_bytes([
                offsets_raw[12],
                offsets_raw[13],
                offsets_raw[14],
                offsets_raw[15],
            ]),
            indices: u32::from_le_bytes([
                offsets_raw[16],
                offsets_raw[17],
                offsets_raw[18],
                offsets_raw[19],
            ]),
            blocks_offset: u32::from_le_bytes([
                offsets_raw[20],
                offsets_raw[21],
                offsets_raw[22],
                offsets_raw[23],
            ]),
        };

        let mut blocks = Vec::new();

        let grid_info = grid::GridInfo {
            width: grid_raw.files[0][0],
            height: grid_raw.files[0][1],
            c: grid_raw.files[0][2..].into(),
        };

        for c in 0..grid_raw.files[5].len() / 64 {
            let mut block: [[u8; 8]; 8] = [[0; 8]; 8];

            for i in 0..8 {
                for j in 0..8 {
                    block[i][j] = grid_raw.files[5][c * 64 + i * 8 + j];
                }
            }

            blocks.push(block);
        }

        let grid_s = grid::Grid {
            offsets: grid_header,
            info: grid_info,
            header1: grid_raw.files[1].clone(),
            header2: grid_raw.files[2].clone(),
            header3: grid_raw.files[3].clone(),
            indices: grid_raw.files[4].clone(),
            blocks,
        };

        grids.push(grid_s);
    }

    let img = ImageReader::open(&args.image).unwrap().decode().unwrap();

    display_grid(
        grids,
        &img,
        args.image.file_stem().clone().unwrap().try_into().unwrap(),
    )
}

fn is_digit(c: u8) -> bool {
    b'0' <= c && c <= b'9'
}

fn full_image_file(filename: &str) -> bool {
    let bytes = filename.as_bytes();

    bytes[0] == b'S'
        && is_digit(bytes[1])
        && is_digit(bytes[2])
        && is_digit(bytes[3])
        && filename.ends_with("PACK.png")
        && filename.len() == 12
}

fn handle_folders(args: Args) {
    for entry_res in fs::read_dir(args.image).unwrap() {
        if let Ok(entry) = entry_res {
            if entry.file_type().unwrap().is_file()
                && full_image_file(entry.file_name().to_str().unwrap())
            {
                let name = entry.file_name().clone();
                let name_str = name.to_str().unwrap();

                let mut path = args.tmpk.clone();

                path.push(format!("S{}TMPK.BIN", &name_str[1..4]));

                if path.exists() {
                    handle_single(Args {
                        image: entry.path(),
                        tmpk: path,
                        folders: false,
                    });
                }
            }
        }
    }
}

fn main() {
    let args = Args::parse();

    if !args.folders {
        handle_single(args);
    } else {
        handle_folders(args);
    }
}
