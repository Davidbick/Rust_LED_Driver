use crate::neopixel_matrix::{MATRIX_HEIGHT, MATRIX_PIXELS, MATRIX_WIDTH};

pub trait Animation {
    fn next(&mut self);
    fn to_list(&self) -> [u32; MATRIX_PIXELS];
}

const BLACK: u32        = 0x00000000;

const RED_DIM: u32      = ((40u32) << 8) | ((0u32) << 16) | 0u32;
const RED_MED: u32      = ((140u32) << 8) | ((0u32) << 16) | 0u32;
const RED_BRIGHT: u32   = ((180u32) << 8) | ((0u32) << 16) | 0u32;

const PINK_DIM: u32     = ((60u32) << 8) | ((0u32) << 16) | 30u32;
const PINK_MED: u32     = ((170u32) << 8) | ((0u32) << 16) | 80u32;
const PINK_BRIGHT: u32  = ((255u32) << 8) | ((0u32) << 16) | 130u32;

const WHITE_DIM: u32    = ((30u32) << 8) | ((30u32) << 16) | 30u32;
const WHITE_MED: u32    = ((60u32) << 8) | ((60u32) << 16) | 60u32;
const WHITE_BRIGHT: u32 = ((150u32) << 8) | ((150u32) << 16) | 150u32;

const BLUE_DIM: u32     = ((0u32) << 8) | ((0u32) << 16) | 20u32;
const BLUE_MED: u32     = ((0u32) << 8) | ((20u32) << 16) | 80u32;

const PURPLE: u32       = ((120u32) << 8) | ((0u32) << 16) | 120u32;
const FLAG_RED: u32     = ((180u32) << 8) | ((0u32) << 16) | 0u32;
const FLAG_WHITE: u32   = ((120u32) << 8) | ((150u32) << 16) | 150u32;
const FLAG_BLACK: u32   = 0x00000000;


fn set_xy(frame: &mut [u32; MATRIX_PIXELS], x: usize, y: usize, color: u32) {
    if x < MATRIX_WIDTH && y < MATRIX_HEIGHT {
        frame[y * MATRIX_WIDTH + x] = color;
    }
}

fn set_xy_rot_cw(frame: &mut [u32; MATRIX_PIXELS], x: usize, y: usize, color: u32) {
    let new_x = 7 - y;
    let new_y = x;
    set_xy(frame, new_x, new_y, color);
}

fn set_xy_rot_ccw(frame: &mut [u32; MATRIX_PIXELS], x: usize, y: usize, color: u32) {
    let new_x = y;
    let new_y = 7 - x;
    set_xy(frame, new_x, new_y, color);
}

fn fill_all(frame: &mut [u32; MATRIX_PIXELS], color: u32) {
    for pixel in frame.iter_mut() {
        *pixel = color;
    }
}

pub struct HeartPulse {
    frame: usize,
}

impl HeartPulse {
    pub fn new() -> Self {
        Self { frame: 0 }
    }

    fn draw_small_heart(frame: &mut [u32; MATRIX_PIXELS], color: u32) {
        let points = [
            (2, 2), (5, 2),
            (2, 3), (3, 3), (4, 3), (5, 3),
            (3, 4), (4, 4),
            (3, 5), (4, 5),
        ];

        for (x, y) in points {
            set_xy_rot_cw(frame, x, y, color);
        }
    }

    fn draw_medium_heart(frame: &mut [u32; MATRIX_PIXELS], color: u32) {
        let points = [
            (2, 1), (5, 1),
            (1, 2), (2, 2), (3, 2), (4, 2), (5, 2), (6, 2),
            (1, 3), (2, 3), (3, 3), (4, 3), (5, 3), (6, 3),
            (2, 4), (3, 4), (4, 4), (5, 4),
            (3, 5), (4, 5),
        ];

        for (x, y) in points {
            set_xy_rot_cw(frame, x, y, color);
        }
    }

    fn draw_large_heart(frame: &mut [u32; MATRIX_PIXELS], color: u32) {
        let points = [
            (1, 1), (2, 1), (5, 1), (6, 1),
            (0, 2), (1, 2), (2, 2), (3, 2), (4, 2), (5, 2), (6, 2), (7, 2),
            (1, 3), (2, 3), (3, 3), (4, 3), (5, 3), (6, 3),
            (2, 4), (3, 4), (4, 4), (5, 4),
            (3, 5), (4, 5),
            (3, 6), (4, 6),
        ];

        for (x, y) in points {
            set_xy_rot_cw(frame, x, y, color);
        }
    }
}

impl Animation for HeartPulse {
    fn next(&mut self) {
        self.frame = (self.frame + 1) % 4;
    }

    fn to_list(&self) -> [u32; MATRIX_PIXELS] {
        let mut frame = [BLACK; MATRIX_PIXELS];

        match self.frame {
            0 => Self::draw_small_heart(&mut frame, PINK_DIM),
            1 => Self::draw_medium_heart(&mut frame, RED_MED),
            2 => Self::draw_large_heart(&mut frame, PINK_BRIGHT),
            _ => Self::draw_medium_heart(&mut frame, RED_DIM),
        }

        frame
    }
}

pub struct ZanScroll {
    offset: isize,
}

impl ZanScroll {
    const TEXT_WIDTH: isize = 17;

    pub fn new() -> Self {
        Self { offset: MATRIX_WIDTH as isize }
    }

    fn pixel_on(x: isize, y: usize) -> bool {
        if y >= 7 || x < 0 || x >= Self::TEXT_WIDTH {
            return false;
        }

        let x = x as usize;

        match x {
            0..=4 => {
                const Z: [u8; 7] = [
                    0b11111,
                    0b00010,
                    0b00100,
                    0b01000,
                    0b10000,
                    0b00000,
                    0b11111,
                ];
                ((Z[y] >> (4 - x)) & 1) != 0
            }
            5 => false,
            6..=10 => {
                const A: [u8; 7] = [
                    0b01110,
                    0b10001,
                    0b10001,
                    0b11111,
                    0b10001,
                    0b10001,
                    0b10001,
                ];
                let local_x = x - 6;
                ((A[y] >> (4 - local_x)) & 1) != 0
            }
            11 => false,
            12..=16 => {
                const N: [u8; 7] = [
                    0b10001,
                    0b11001,
                    0b10101,
                    0b10011,
                    0b10001,
                    0b10001,
                    0b10001,
                ];
                let local_x = x - 12;
                ((N[y] >> (4 - local_x)) & 1) != 0
            }
            _ => false,
        }
    }
}

impl Animation for ZanScroll {
    fn next(&mut self) {
        self.offset -= 1;
        if self.offset < -Self::TEXT_WIDTH {
            self.offset = MATRIX_WIDTH as isize;
        }
    }

    fn to_list(&self) -> [u32; MATRIX_PIXELS] {
        let mut frame = [BLACK; MATRIX_PIXELS];

        for y in 0..7 {
            for display_x in 0..MATRIX_WIDTH {
                let text_x = display_x as isize - self.offset;
                if Self::pixel_on(text_x, y) {
                    set_xy_rot_ccw(&mut frame, display_x, y, PURPLE);
                }
            }
        }

        frame
    }
}

pub struct StarryTwinkle {
    frame: usize,
}

impl StarryTwinkle {
    pub fn new() -> Self {
        Self { frame: 0 }
    }
}

impl Animation for StarryTwinkle {
    fn next(&mut self) {
        self.frame = (self.frame + 1) % 5;
    }

    fn to_list(&self) -> [u32; MATRIX_PIXELS] {
        let mut frame = [BLACK; MATRIX_PIXELS];

        match self.frame {
            0 => {
                set_xy(&mut frame, 0, 0, WHITE_DIM);
                set_xy(&mut frame, 2, 1, BLUE_DIM);
                set_xy(&mut frame, 7, 1, WHITE_MED);
                set_xy(&mut frame, 1, 3, WHITE_DIM);
                set_xy(&mut frame, 5, 2, WHITE_BRIGHT);
                set_xy(&mut frame, 6, 4, BLUE_MED);
                set_xy(&mut frame, 3, 6, WHITE_DIM);
                set_xy(&mut frame, 7, 7, WHITE_MED);
            }
            1 => {
                set_xy(&mut frame, 0, 0, WHITE_MED);
                set_xy(&mut frame, 2, 1, WHITE_BRIGHT);
                set_xy(&mut frame, 4, 0, BLUE_DIM);
                set_xy(&mut frame, 7, 1, WHITE_DIM);
                set_xy(&mut frame, 1, 3, WHITE_MED);
                set_xy(&mut frame, 5, 2, WHITE_DIM);
                set_xy(&mut frame, 6, 4, WHITE_BRIGHT);
                set_xy(&mut frame, 4, 5, BLUE_DIM);
                set_xy(&mut frame, 3, 6, WHITE_DIM);
            }
            2 => {
                set_xy(&mut frame, 1, 0, BLUE_DIM);
                set_xy(&mut frame, 3, 1, WHITE_MED);
                set_xy(&mut frame, 6, 1, WHITE_BRIGHT);
                set_xy(&mut frame, 0, 4, WHITE_DIM);
                set_xy(&mut frame, 2, 3, WHITE_BRIGHT);
                set_xy(&mut frame, 5, 5, WHITE_MED);
                set_xy(&mut frame, 7, 6, BLUE_MED);
                set_xy(&mut frame, 3, 7, WHITE_DIM);
            }
            3 => {
                set_xy(&mut frame, 0, 1, WHITE_DIM);
                set_xy(&mut frame, 2, 2, WHITE_MED);
                set_xy(&mut frame, 4, 1, WHITE_DIM);
                set_xy(&mut frame, 7, 2, WHITE_BRIGHT);
                set_xy(&mut frame, 1, 5, BLUE_MED);
                set_xy(&mut frame, 5, 4, WHITE_BRIGHT);
                set_xy(&mut frame, 6, 6, WHITE_DIM);
                set_xy(&mut frame, 7, 7, WHITE_MED);
            }
            _ => {
                set_xy(&mut frame, 0, 0, WHITE_DIM);
                set_xy(&mut frame, 2, 1, WHITE_DIM);
                set_xy(&mut frame, 7, 0, WHITE_MED);
                set_xy(&mut frame, 1, 2, WHITE_BRIGHT);
                set_xy(&mut frame, 4, 4, BLUE_MED);
                set_xy(&mut frame, 6, 3, WHITE_MED);
                set_xy(&mut frame, 2, 6, WHITE_DIM);
                set_xy(&mut frame, 5, 7, WHITE_BRIGHT);
            }
        }

        frame
    }
}

pub struct TrinidadFlag {
    frame: usize,
}

impl TrinidadFlag {
    pub fn new() -> Self {
        Self { frame: 0 }
    }

    fn draw_red_background_partial(frame: &mut [u32; MATRIX_PIXELS]) {
        for y in 0..MATRIX_HEIGHT {
            for x in 0..MATRIX_WIDTH {
                if x + y >= 4 {
                    set_xy(frame, x, y, FLAG_RED);
                }
            }
        }
    }

    fn draw_red_background_full(frame: &mut [u32; MATRIX_PIXELS]) {
        fill_all(frame, FLAG_RED);
    }

    fn draw_white_borders(frame: &mut [u32; MATRIX_PIXELS]) {
        let white_points = [
            (0, 1), (1, 2), (2, 3), (3, 4), (4, 5), (5, 6),
            (1, 0), (2, 1), (3, 2), (4, 3), (5, 4), (6, 5), (7, 6),
            (0, 2), (1, 3), (2, 4), (3, 5), (4, 6), (5, 7),
            (2, 0), (3, 1), (4, 2), (5, 3), (6, 4), (7, 5),
        ];

        for (x, y) in white_points {
            set_xy(frame, x, y, FLAG_WHITE);
        }
    }

    fn draw_black_stripe(frame: &mut [u32; MATRIX_PIXELS]) {
        let black_points = [
            (0, 0), (1, 1), (2, 2), (3, 3),
            (4, 4), (5, 5), (6, 6), (7, 7),
            (1, 2), (2, 3), (3, 4), (4, 5), (5, 6), (6, 7),
        ];

        for (x, y) in black_points {
            set_xy(frame, x, y, FLAG_BLACK);
        }
    }
}

impl Animation for TrinidadFlag {
    fn next(&mut self) {
        self.frame = (self.frame + 1) % 6;
    }

    fn to_list(&self) -> [u32; MATRIX_PIXELS] {
        let mut frame = [BLACK; MATRIX_PIXELS];

        match self.frame {
            0 => {}
            1 => {
                Self::draw_red_background_partial(&mut frame);
            }
            2 => {
                Self::draw_red_background_full(&mut frame);
            }
            3 => {
                Self::draw_red_background_full(&mut frame);
                Self::draw_white_borders(&mut frame);
            }
            4 => {
                Self::draw_red_background_full(&mut frame);
                Self::draw_white_borders(&mut frame);
                Self::draw_black_stripe(&mut frame);
            }
            _ => {
                Self::draw_red_background_full(&mut frame);
                Self::draw_white_borders(&mut frame);
                Self::draw_black_stripe(&mut frame);
            }
        }

        frame
    }
}