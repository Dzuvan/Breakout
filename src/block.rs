extern crate sdl2;

use std::path::Path;
use std::fs::File;
use std::io::prelude::*;

use sdl2::rect::Rect;
use sdl2::render::Canvas;
use sdl2::video::Window;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::surface::Surface;

use constants::*;
use ball::*;

#[derive(Debug)]
pub struct Block {
    pub screen_location: Rect,
    pub bitmap_location: Rect,
    num_hits: u32,
}

pub trait  Handler {
    fn init_blocks(&self) -> Vec<Block>;
    fn new() -> Self;
    fn draw(&mut self, &mut Canvas<Window>);
    fn check_block_collisions(&self, &mut Ball);
    fn check_point_in_rect(&self, x: i32, y: i32, rect: Rect) -> bool {
        if x >= rect.x && x <= rect.x + rect.w  &&
           y >= rect.y && y <= rect.y + rect.h {
                return true;
        }
        false
    }
    fn handle_block_collision(&self);
}

impl Handler for Block {
    fn new() -> Self{
        let screen_rect = Rect::new(1, 1, BLOCK_WIDTH as u32, BLOCK_HEIGHT as u32);
        let bitmap_rect = Rect::new(1,1, BLOCK_WIDTH as u32, BLOCK_HEIGHT as u32);

        Block {
            num_hits: 0,
            screen_location: screen_rect,
            bitmap_location: bitmap_rect,
        }

    }
    fn init_blocks(&self) -> Vec<Block> {
        let mut blocks = Vec::new();
        let mut file_name = "Data/level".to_string();
        let level = 1.to_string();
        file_name.push_str(&level);
        file_name.push_str(".txt");

        let mut file = File::open(file_name).expect("Couldn't open file");
        let mut s = String::new();
        file.read_to_string(&mut s).ok();

        let numbers: Vec<usize> = s.split(' ')
                            .map(|s| s.trim())
                            .filter(|s| !s.is_empty())
                            .map(|s| s.parse().unwrap())
                            .collect();

       for row in 1..NUM_ROWS+1{
            for col in 1..NUM_COLS+1 {
                   let mut block = Block::new();
                    block.screen_location.x = col*BLOCK_WIDTH - BLOCK_SCREEN_BUFFER;
                    block.screen_location.y = row*BLOCK_HEIGHT + BLOCK_SCREEN_BUFFER;
                    block.screen_location.w = BLOCK_WIDTH;
                    block.screen_location.h = BLOCK_HEIGHT;
                    block.bitmap_location.w = BLOCK_WIDTH;
                    block.bitmap_location.h = BLOCK_WIDTH;
                 for n in &numbers {
                    match *n {
                        1 => {
                                block.num_hits = 1;

                        },
                        2 => {
                                block.num_hits = 2;

                        },
                        3 => {
                                block.num_hits = 3;

                        },
                        4 => {
                                block.num_hits = 4;
                        },
                        0 => {
                                block.num_hits = 0;
                        },
                        _ => {},
                    }
                }
                    blocks.push(block);
            }
        }
        blocks
    }

    fn draw(&mut self, canvas: &mut Canvas<Window>) {
        let surf = Surface::load_bmp("Data/BlockBreaker.bmp").unwrap();
        let texture_creator = canvas.texture_creator();
        let surface = texture_creator.create_texture_from_surface(surf).unwrap();
        for mut block in self.init_blocks() {
            match block.num_hits {
                1 => {
                        block.bitmap_location.x = YELLOW_X;
                        block.bitmap_location.y = YELLOW_Y;
                        canvas.copy(&surface, block.bitmap_location, block.screen_location).ok();
                },
                2 => {
                        block.bitmap_location.x = RED_X;
                        block.bitmap_location.y = RED_Y;
                        canvas.copy(&surface, block.bitmap_location, block.screen_location).ok();
                },
                3 => {
                        block.bitmap_location.x = GREEN_X;
                        block.bitmap_location.y = GREEN_Y;
                        canvas.copy(&surface, block.bitmap_location, block.screen_location).ok();
                },
                4 => {
                        block.bitmap_location.x = BLUE_X;
                        block.bitmap_location.y = BLUE_Y;
                        canvas.copy(&surface, block.bitmap_location, block.screen_location).ok();
                },
                _=>{ continue; },
            }
        }
    }
    fn check_block_collisions(&self, ball: &mut Ball) {
        let left_x = ball.screen_location.x;
        let left_y = ball.screen_location.y + ball.screen_location.h / 2;
        let right_x = ball.screen_location.x + ball.screen_location.w;
        let right_y = ball.screen_location.y + ball.screen_location.h / 2;
        let top_x = ball.screen_location.x + ball.screen_location.w / 2;
        let top_y = ball.screen_location.y;
        let bottom_x = ball.screen_location.x + ball.screen_location.w / 2;
        let bottom_y = ball.screen_location.y + ball.screen_location.h;

        let mut top = false;
        let mut bottom = false;
        let mut left = false;
        let mut right = false;

        for block in self.init_blocks() {
            if self.check_point_in_rect(top_x, top_y, block.screen_location) {
                top = true;
                self.handle_block_collision();
            }
            if self.check_point_in_rect(bottom_x, bottom_y, block.screen_location) {
                bottom = true;
                self.handle_block_collision();
            }
            if self.check_point_in_rect(left_x, left_y, block.screen_location) {
                left = true;
                self.handle_block_collision();
            }
            if self.check_point_in_rect(right_x, right_y, block.screen_location) {
                right = true;
                self.handle_block_collision();
            }
        }

        if top {
            ball.y_speed  = - ball.y_speed;
            ball.screen_location.y += BALL_DIAMETER;
        }

        if bottom {
            ball.y_speed = - ball.y_speed;
            ball.screen_location.y -= BALL_DIAMETER;
        }
        if left {
            ball.x_speed = - ball.x_speed;
            ball.screen_location.x += BALL_DIAMETER;
        }
        if right{
            ball.x_speed = - ball.x_speed;
            ball.screen_location.x -= BALL_DIAMETER;
        }
    }
    fn handle_block_collision(&self) {
        for mut block in self.init_blocks() {
            block.num_hits -= 1;
            if block.num_hits == 0 {
                return;
            }
        }
    }
}
