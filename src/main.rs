extern crate sdl2;

use std::time::Duration;

use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;

use pellet::Pellet;
use snake::{Direction, Snake};

mod pellet;
mod snake;

const WINDOW_WIDTH: i32 = 800;
const WINDOW_HEIGHT: i32 = 600;
const TILE_WIDTH: u32 = 20;
const TILE_HEIGHT: u32 = 20;

fn main() -> Result<(), String> {
    let sdl_context = sdl2::init()?;
    let video_subsystem = sdl_context.video()?;

    let window = video_subsystem
        .window("Snake!", 800, 600)
        .position_centered()
        .build()
        .expect("Unexpected error when creating window");

    let mut renderer = window
        .into_canvas()
        .build()
        .expect("Unexpected error when creating renderer");
    let mut event_pump = sdl_context.event_pump()?;

    let mut snake = Snake::new(WINDOW_WIDTH, WINDOW_HEIGHT, TILE_WIDTH, TILE_HEIGHT);
    let mut pellet = Pellet::new(
        WINDOW_WIDTH,
        WINDOW_HEIGHT,
        TILE_WIDTH,
        TILE_HEIGHT,
        snake.occupied_tiles(),
    );

    'running: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => {
                    break 'running;
                }
                Event::KeyDown {
                    keycode: Some(Keycode::Left),
                    repeat: false,
                    ..
                } => {
                    snake.set_direction(Direction::Left);
                }
                Event::KeyDown {
                    keycode: Some(Keycode::Right),
                    repeat: false,
                    ..
                } => {
                    snake.set_direction(Direction::Right);
                }
                Event::KeyDown {
                    keycode: Some(Keycode::Up),
                    repeat: false,
                    ..
                } => {
                    snake.set_direction(Direction::Up);
                }
                Event::KeyDown {
                    keycode: Some(Keycode::Down),
                    repeat: false,
                    ..
                } => {
                    snake.set_direction(Direction::Down);
                }
                _ => {}
            }
        }

        renderer.set_draw_color(Color::RGBA(0, 0, 0, 0));
        renderer.clear();

        snake.draw(&mut renderer)?;
        pellet.draw(&mut renderer)?;

        renderer.present();

        snake.update();

        if snake.is_dead() {
            snake.respawn();
            pellet.respawn(snake.occupied_tiles());
            continue;
        }

        if snake.can_eat(pellet.current_location()) {
            snake.grow();
            pellet.respawn(snake.occupied_tiles());
        }
        std::thread::sleep(Duration::from_millis(100));
    }
    Ok(())
}
