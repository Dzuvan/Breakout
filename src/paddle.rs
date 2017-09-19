extern crate sdl2;

use sdl2::rect::Rect;
use sdl2::render::Canvas;
use sdl2::video::Window;
use sdl2::surface::Surface;

use constants::*;

pub struct Paddle {
    pub screen_location: Rect,
    pub bitmap_location: Rect,
    pub x_speed: i32,
}

pub trait PaddleHandler {
    fn new() -> Self;
    fn move_player_left(&mut self);
    fn move_player_right(&mut self);
    fn draw(&self, &mut Canvas<Window>);
}

impl PaddleHandler for Paddle {
    fn new() -> Self {
        let screen_rect = Rect::new((SCREEN_WIDTH / 2) - (PADDLE_WIDTH / 2), PLAYER_Y, PADDLE_WIDTH as u32, PADDLE_HEIGHT as u32);
        let bitmap_rect = Rect::new(PADDLE_BITMAP_X, PADDLE_BITMAP_Y, PADDLE_WIDTH as u32, PADDLE_HEIGHT as u32);

        Paddle {
           screen_location: screen_rect,
           bitmap_location: bitmap_rect,
           x_speed: PLAYER_SPEED,
        }
    }

    fn move_player_right (&mut self) {
             if self.screen_location.x < SCREEN_WIDTH - PADDLE_WIDTH {
                 self.screen_location.x += self.x_speed;
             }
    }
    fn move_player_left(&mut self){
             if self.screen_location.x > 0 {
             self.screen_location.x -= self.x_speed;
             }
    }

    fn draw(&self, canvas: &mut Canvas<Window>) {
        let surf = Surface::load_bmp("Data/BlockBreaker.bmp").unwrap();
        let texture_creator = canvas.texture_creator();
        let surface = texture_creator.create_texture_from_surface(surf).unwrap();
        canvas.copy(&surface, self.bitmap_location, self.screen_location).ok();
    }


}
