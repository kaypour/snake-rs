use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::render::WindowCanvas;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

pub struct Snake {
    body: Vec<Rect>,
    window_width: i32,
    window_height: i32,
    w: u32,
    h: u32,
    dead: bool,
    current_direction: Direction,
    state_updated: bool,
}

impl Snake {
    pub fn new(window_width: i32, window_height: i32, w: u32, h: u32) -> Self {
        Snake {
            body: Snake::spawn_location(w, h),
            window_width,
            window_height,
            w,
            h,
            dead: false,
            current_direction: Direction::Right,
            state_updated: false,
        }
    }

    pub fn draw(&self, renderer: &mut WindowCanvas) -> Result<(), String> {
        renderer.set_draw_color(Color::RGBA(255, 255, 255, 0));
        renderer.fill_rects(&self.body)?;
        renderer.draw_rects(&self.body)
    }

    pub fn update(&mut self) {
        for i in (1..self.body.len()).rev() {
            let x = self.body[i - 1].x();
            let y = self.body[i - 1].y();
            self.body[i].set_x(x);
            self.body[i].set_y(y);
        }

        match self.current_direction {
            Direction::Left => {
                let head = &mut self.body[0];
                let mut x = head.x() - self.w as i32;
                if x < 0 {
                    x = self.window_width - self.w as i32;
                }
                head.set_x(x);
            }
            Direction::Right => {
                let head = &mut self.body[0];
                let mut x = head.x() + self.w as i32;
                if x > self.window_width - self.w as i32 {
                    x = 0;
                }
                head.set_x(x);
            }
            Direction::Up => {
                let head = &mut self.body[0];
                let mut y = head.y() - self.h as i32;
                if y < 0 {
                    y = self.window_height - self.h as i32;
                }
                head.set_y(y);
            }
            Direction::Down => {
                let head = &mut self.body[0];
                let mut y = head.y() + self.h as i32;
                if y > self.window_height - self.h as i32 {
                    y = 0;
                }
                head.set_y(y);
            }
        }

        let head = &self.body[0];
        for i in 1..self.body.len() {
            if head.has_intersection(self.body[i]) {
                self.dead = true;
                return;
            }
        }
        self.state_updated = true
    }

    pub fn respawn(&mut self) {
        self.current_direction = Direction::Right;
        self.body = Snake::spawn_location(self.w, self.h);
        self.dead = false;
        self.state_updated = false
    }

    pub fn occupied_tiles(&self) -> &Vec<Rect> {
        &self.body
    }

    pub fn can_eat(&self, pellet_location: Rect) -> bool {
        let head = &self.body[0];
        head.has_intersection(pellet_location)
    }

    pub fn grow(&mut self) {
        if let Some(&last_rect) = self.body.last() {
            self.body
                .push(Rect::new(last_rect.x(), last_rect.y(), self.w, self.h));
        }
    }

    pub fn is_dead(&self) -> bool {
        self.dead
    }

    pub fn set_direction(&mut self, direction: Direction) {
        if !self.state_updated {
            return;
        }

        if direction == Direction::Left && self.current_direction != Direction::Right {
            self.current_direction = direction
        }
        if direction == Direction::Right && self.current_direction != Direction::Left {
            self.current_direction = direction
        }
        if direction == Direction::Up && self.current_direction != Direction::Down {
            self.current_direction = direction
        }
        if direction == Direction::Down && self.current_direction != Direction::Up {
            self.current_direction = direction
        }
        self.state_updated = false;
    }

    fn spawn_location(w: u32, h: u32) -> Vec<Rect> {
        vec![
            Rect::new(200, 200, w, h),
            Rect::new(180, 200, w, h),
            Rect::new(160, 200, w, h),
            Rect::new(140, 200, w, h),
        ]
    }
}
