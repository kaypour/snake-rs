use std::collections::HashSet;

use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::render::WindowCanvas;

pub struct Pellet {
    rect: Box<Rect>,
    window_width: i32,
    window_height: i32,
    w: u32,
    h: u32,
    eaten: bool,
}

impl Pellet {
    pub fn new(
        window_width: i32,
        window_height: i32,
        w: u32,
        h: u32,
        occupied_tiles: &[Rect],
    ) -> Self {
        Pellet {
            rect: Pellet::random_spawn_location(window_width, window_height, w, h, occupied_tiles),
            window_width,
            window_height,
            w,
            h,
            eaten: false,
        }
    }

    pub fn draw(&self, renderer: &mut WindowCanvas) -> Result<(), String> {
        renderer.set_draw_color(Color::RGBA(255, 255, 255, 0));
        renderer.fill_rect(*self.rect)?;
        renderer.draw_rect(*self.rect)
    }

    pub fn current_location(&self) -> Rect {
        *self.rect
    }

    pub fn respawn(&mut self, occupied_tiles: &[Rect]) {
        self.rect = Pellet::random_spawn_location(
            self.window_width,
            self.window_height,
            self.w,
            self.h,
            occupied_tiles,
        );
        self.eaten = false;
    }

    fn random_spawn_location(
        window_width: i32,
        window_height: i32,
        w: u32,
        h: u32,
        occupied_tiles: &[Rect],
    ) -> Box<Rect> {
        // The reason we do the following below is, so that we do not spawn the pellet inside the snake tiles.
        let length = ((window_width / w as i32) * (window_height / h as i32)) as usize;

        let mut set = HashSet::with_capacity(length);
        for tile in occupied_tiles {
            set.insert(*tile);
        }

        let mut available_tiles = Vec::with_capacity(length);

        let mut x = 0;
        let mut y = 0;
        for _ in 0..length {
            let rect = Rect::new(x, y, w, h);
            if !set.contains(&rect) {
                available_tiles.push(rect);
            }

            x = x + w as i32;
            if x > window_width - w as i32 {
                y = y + h as i32;
                x = 0;
            }
        }

        let random_index: usize = rand::random();
        Box::new(available_tiles[random_index % available_tiles.len()])
    }
}
