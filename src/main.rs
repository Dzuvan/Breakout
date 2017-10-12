extern crate sdl2;

pub mod block;
pub mod constants;
pub mod ball;
pub mod paddle;

use std::fs::File;
use std::io::prelude::*;

use sdl2::pixels::Color;
use sdl2::keyboard::Keycode;
use sdl2::event:: Event;
use sdl2::video:: Window;
use sdl2::render::Canvas;

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
    let mut blocks = init_blocks();
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
        canvas.set_draw_color(Color::RGB(255, 255, 255));
        canvas.clear();
        for block in &mut blocks {
            block.check_block_collisions(&mut ball);
            if block.num_hits != 0 {
                block.draw(&mut canvas);
            }
        }
        paddle.draw(&mut canvas);
        ball.draw(&mut canvas);
        canvas.present();
    }
}

fn init_blocks() -> Vec<Block> {
    let mut blocks = Vec::new();
    let mut file_name = "Data/level".to_string();
    let level = 1.to_string();
    file_name.push_str(&level);
    file_name.push_str(".txt");

    let mut file = File::open(file_name).expect("Couldn't open file");
    let mut s = String::new();
    file.read_to_string(&mut s).ok();

    let numbers: Vec<i32> = s.split(' ')
                        .map(|s| s.trim())
                        .filter(|s| !s.is_empty())
                        .map(|s| s.parse().unwrap())
                        .collect();

   for row in 1..NUM_ROWS+1 {
        for col in 1..NUM_COLS+1 {
                let mut block = Block::new();
                block.screen_location.x = col*BLOCK_WIDTH - BLOCK_SCREEN_BUFFER;
                block.screen_location.y = row*BLOCK_HEIGHT + BLOCK_SCREEN_BUFFER;
                block.screen_location.w = BLOCK_WIDTH;
                block.screen_location.h = BLOCK_HEIGHT;
                block.bitmap_location.w = BLOCK_WIDTH;
                block.bitmap_location.h = BLOCK_WIDTH;
                blocks.push(block);
        }
    }
    let mut num_blocks  = 55;
    for (i, n) in numbers.into_iter().enumerate() {
        if n==1 {
            blocks[i].num_hits = 1;
            blocks[i].bitmap_location.x = YELLOW_X;
            blocks[i].bitmap_location.y = YELLOW_Y;
        } if n==2 {
            blocks[i].num_hits = 2;
            blocks[i].bitmap_location.x = RED_X;
            blocks[i].bitmap_location.y = RED_Y;
        } if n==3 {
            blocks[i].num_hits = 3;
            blocks[i].bitmap_location.x = GREEN_X;
            blocks[i].bitmap_location.y = GREEN_Y;
        } if n==4 {
            blocks[i].num_hits = 4;
            blocks[i].bitmap_location.x = BLUE_X;
            blocks[i].bitmap_location.y = BLUE_Y;
        }
        // if n==0 {
        //     blocks[i] = blocks[num_blocks-1];
        //     num_blocks -=1;
        //     if num_blocks == 0 {
        //         break;
        //     }
        // }
    }
    blocks
}

