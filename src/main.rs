extern crate sdl2;

pub mod block;
pub mod constants;
pub mod ball;
pub mod paddle;

use std::time::Duration;

use sdl2::pixels::Color;
use sdl2::keyboard::Keycode;
use sdl2::event:: Event;
use sdl2::video:: Window;
use sdl2::render::Canvas;
use sdl2::rect::Rect;

use constants::*;
use ball::*;
use block::*;
use paddle::*;

fn init_sdl() ->  (Canvas<Window>, sdl2::EventPump) {
    let sdl_context = sdl2::init ().ok ().expect ("Could not initialize SDL2");
    let video_subsystem  = sdl_context.video ().ok ().expect ("Could not init video_subsystem");

    let window = video_subsystem.window ("Breakout", SCREEN_WIDTH as u32, SCREEN_HEIGHT as u32)
        .position_centered ()
        .opengl ()
        .build ()
        .unwrap ();

    let canvas = window.into_canvas ()
        .present_vsync ()
        .build ()
        .unwrap ();

    let event_pump = sdl_context.event_pump ().unwrap ();

    (canvas, event_pump)
}


fn main() {
    let (mut canvas, mut event_pump) = init_sdl ();

    let mut paddle  = Paddle::new();
    let mut ball = Ball::new();
    let mut block = Block::new();
    'mainloop: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit{..} | Event::KeyDown {keycode: Option::Some(Keycode::Escape), ..} =>
                    break 'mainloop,
                Event::KeyDown {keycode: Option::Some(Keycode::D), ..} => paddle.move_player_right(),
                Event::KeyDown {keycode: Option::Some(Keycode::A), ..} => paddle.move_player_left(),
                Event::KeyDown {keycode: Option::Some(Keycode::Space), ..} => {
                    if ball.y_speed == 0{
                        ball.y_speed = BALL_SPEED_Y  / 3;
                    }
                },
                _=> continue,
            }
        }
        ball.handle_ball(&paddle);
        block.check_block_collisions(&mut ball);
        canvas.set_draw_color(Color::RGB(255, 255, 255));
        canvas.clear();
        block.draw(&mut canvas);
        paddle.draw(&mut canvas);
        ball.draw(&mut canvas);
        canvas.present();
    }
}


