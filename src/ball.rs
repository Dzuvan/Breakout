extern crate sdl2;

use sdl2::rect::Rect;
use sdl2::render::Canvas;
use sdl2::video::Window;
use sdl2::surface::Surface;

use constants::*;
use paddle::*;

pub struct Ball {
  pub  screen_location: Rect,
  pub  bitmap_location: Rect,
  pub  x_speed: i32,
  pub  y_speed: i32,
}

pub trait BallHandler {
    fn new() -> Self;
    fn draw(&self, &mut Canvas<Window>);
    fn handle_move(&mut self);
    fn check_ball_collision(&self, &Paddle) -> bool;
    fn handle_ball(&mut self, paddle: &Paddle);
}


impl BallHandler for Ball {
 fn new() -> Self {
        let screen_rect = Rect::new((SCREEN_WIDTH / 2) - (BALL_DIAMETER / 2), (SCREEN_HEIGHT / 2) - (BALL_DIAMETER / 2), BALL_DIAMETER as u32, BALL_DIAMETER as u32);
        let bitmap_rect = Rect::new(BALL_BITMAP_X, BALL_BITMAP_Y, BALL_DIAMETER as u32, BALL_DIAMETER as u32);

        Ball {
            screen_location: screen_rect,
            bitmap_location: bitmap_rect,
            x_speed: 0,
            y_speed: 0,
        }
    }

    fn draw(&self, canvas: &mut Canvas<Window>){
        let surf = Surface::load_bmp("Data/BlockBreaker.bmp").unwrap();
        let texture_creator = canvas.texture_creator();
        let surface = texture_creator.create_texture_from_surface(surf).unwrap();
        canvas.copy(&surface, self.bitmap_location, self.screen_location).ok();
    }

    fn check_ball_collision(&self, paddle: &Paddle) -> bool {
        let ball_x = self.screen_location.x;
        let ball_y = self.screen_location.y;
        let ball_width = self.screen_location.w;
        let ball_height = self.screen_location.h;
        let ball_speed = self.y_speed;

        let paddle_x = paddle.screen_location.x;
        let paddle_y = paddle.screen_location.y;
        let paddle_width = paddle.screen_location.w;
        let paddle_height= paddle.screen_location.h;

        if ball_speed > 0 && ball_y + ball_height >= paddle_y && ball_y + ball_height <= paddle_y + paddle_height {
                if ball_x <=  paddle_x + paddle_width && ball_x + ball_width >= paddle_x {
                    return true;
                }
        }

        false
    }

    fn handle_move(&mut self) {
        self.screen_location.x  += self.x_speed;
        self.screen_location.y  += self.y_speed;

        if self.x_speed < 0 && self.screen_location.x <= 0 ||
            self.x_speed > 0 && self.screen_location.x >= SCREEN_WIDTH {
                self.x_speed  = - self.x_speed;
        }

        if self.y_speed < 0 && self.screen_location.y <= 0 {
            self.y_speed  = - self.y_speed;
        }

        if self.screen_location.y >= SCREEN_HEIGHT {
            self.x_speed = 0;
            self.y_speed = 0;

            self.screen_location.x = SCREEN_WIDTH / 2 - self.screen_location.w / 2;
            self.screen_location.y = SCREEN_HEIGHT / 2 - self.screen_location.h / 2;
        }

    }

    fn handle_ball(&mut self, paddle: &Paddle) {
        self.handle_move();
        println!("{}",self.check_ball_collision(paddle));
        if self.check_ball_collision(paddle) {
            let paddle_center = paddle.screen_location.x + paddle.screen_location.w / 2;
            let ball_center =  self.screen_location.x + self.screen_location.w / 2;
            let paddle_location  = ball_center - paddle_center;
            self.x_speed = paddle_location / BALL_SPEED_MODIFIER;
            self.y_speed = - self.y_speed;

        }
    }

}
