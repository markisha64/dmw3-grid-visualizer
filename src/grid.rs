pub struct GridInfo {
    pub width: u8,
    pub height: u8,
    pub c: Vec<u8>,
}

pub struct GridOffsets {
    pub info_offset: u32,
    pub header1: u32,
    pub header2: u32,
    pub header3: u32,
    pub indices: u32,
    pub blocks_offset: u32,
}

pub struct Grid {
    pub info: GridInfo,
    pub segment1: Vec<u8>,
    pub segment2: Vec<u16>,
    pub segment3: Vec<u16>,
    pub indices: Vec<u16>,
    pub blocks: Vec<[[u8; 8]; 8]>,
}

pub fn get_grid_value(grid_s: &Grid, x: u32, y: u32) -> u8 {
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

    let mut grid_part_1 = grid_s.segment1[grid_part as usize] as usize * 2;

    if (y & 32) != 0 {
        grid_part_1 += 1;
    }

    if (x & 32) == 0 {
        grid_part_1 = grid_part_1 * 2;
    } else {
        grid_part_1 = grid_part_1 * 2 + 1;
    }

    let mut grid_part_2 = grid_s.segment2[grid_part_1 as usize] as usize * 2;

    if (y & 16) != 0 {
        grid_part_2 = grid_part_2 + 1;
    }

    if (x & 16) == 0 {
        grid_part_2 = grid_part_2 * 2;
    } else {
        grid_part_2 = grid_part_2 * 2 + 1;
    }

    let mut fpart = grid_s.segment3[grid_part_2 as usize] as usize * 2;

    if (y & 8) != 0 {
        fpart = fpart + 1;
    }

    if (x & 8) == 0 {
        fpart = fpart * 2;
    } else {
        fpart = fpart * 2 + 1;
    }

    grid_s.blocks[grid_s.indices[fpart as usize] as usize][(y & 7) as usize][(x & 7) as usize]
}
