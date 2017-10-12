extern crate sdl2;

use sdl2::rect::Rect;
use sdl2::render::Canvas;
use sdl2::video::Window;
use sdl2::surface::Surface;

use constants::*;
use ball::*;

#[derive(Debug, Copy, Clone)]
pub struct Block {
    pub screen_location: Rect,
    pub bitmap_location: Rect,
    pub num_hits: i32,
}

pub trait  Handler {
    fn new() -> Self;
    fn draw(&mut self, &mut Canvas<Window>);
    fn check_block_collisions(&mut self, &mut Ball);
    fn check_point_in_rect(&self, x: i32, y: i32, rect: Rect) -> bool {
        if x >= rect.x && x <= rect.x + rect.w  &&
            y >= rect.y && y <= rect.y + rect.h {
                return true;
            }
        false
    }
    fn handle_block_collision(&mut self);
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

    fn draw(&mut self, canvas: &mut Canvas<Window>) {
        let surf = Surface::load_bmp("Data/BlockBreaker.bmp").unwrap();
        let texture_creator = canvas.texture_creator();
        let surface = texture_creator.create_texture_from_surface(surf).unwrap();
        canvas.copy(&surface, self.bitmap_location, self.screen_location).ok();
    }

    fn check_block_collisions(&mut self, ball: &mut Ball) {
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

        if self.check_point_in_rect(top_x, top_y, self.screen_location) {
            top = true;
            self.handle_block_collision();
        }
        if self.check_point_in_rect(bottom_x, bottom_y, self.screen_location) {
            bottom = true;
            self.handle_block_collision();
        }
        if self.check_point_in_rect(left_x, left_y, self.screen_location) {
            left = true;
            self.handle_block_collision();
        }
        if self.check_point_in_rect(right_x, right_y, self.screen_location) {
            right = true;
            self.handle_block_collision();
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

    fn handle_block_collision(&mut self) {
        self.num_hits -= 1;
    }
}
