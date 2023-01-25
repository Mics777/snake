use sdl2::{
    keyboard::Scancode, pixels::Color, rect::Rect, render::Canvas, video::Window, EventPump,
};

use rand::{thread_rng, Rng};

#[derive(Clone, Copy)]
pub struct Vector2(pub u32, pub u32);

pub struct Game {
    size: u32,
    fruit: Vector2,
    snake: Snake,
    points: u32,
    immunity: u32,
    turn_rate: u32,
    turn_timer: u32,
}

pub struct Snake {
    body: Vec<Vector2>,
    head: Vector2,
    length: u32,
    direction: Option<Movement>,
    alive: bool,
}

#[derive(PartialEq)]
pub enum Movement {
    Left,
    Right,
    Up,
    Down,
}

impl PartialEq for Vector2 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0 && self.1 == other.1
    }
}

impl Vector2 {
    fn gen(max: u32) -> Vector2 {
        let mut rng = thread_rng();
        Vector2(rng.gen_range(0..max), rng.gen_range(0..max))
    }
}

impl Game {
    pub fn new(startpos: Vector2, size: u32, speed: u32) -> Game {
        Game {
            size,
            fruit: Vector2::gen(size),
            points: 0,
            immunity: 5,
            snake: Snake {
                alive: true,
                length: 1,
                head: startpos,
                body: Vec::new(),
                direction: None,
            },
            turn_rate: speed,
            turn_timer: speed,
        }
    }

    pub fn draw(&self, canvas: &mut Canvas<Window>, render_size: u32) -> Result<(), String> {
        // draw bg
        canvas.set_draw_color(Color::GRAY);
        canvas.fill_rect(Rect::new(
            0,
            0,
            self.size * render_size,
            self.size * render_size,
        ))?;

        // draw specific point
        let mut draw_point = |color, vec2: Vector2| {
            canvas.set_draw_color(color);
            canvas
                .fill_rect(Rect::new(
                    (render_size * vec2.0) as i32,
                    (render_size * vec2.1) as i32,
                    render_size,
                    render_size,
                ))
                .unwrap(); // '?' leads to complications
        };

        // draw snake
        if self.snake.alive {
            draw_point(Color::GREEN, self.snake.head);

            for segment in self.snake.body.iter() {
                draw_point(Color::GREEN, *segment);
            }
        }

        // draw fruit
        draw_point(Color::RED, self.fruit);

        Ok(())
    }

    pub fn handle_input(&mut self, event_pump: &mut EventPump) {
        let check_pressed = |scancode| event_pump.keyboard_state().is_scancode_pressed(scancode);
        if check_pressed(Scancode::Up) && self.snake.direction != Some(Movement::Down) {
            self.snake.direction = Some(Movement::Up)
        }
        if check_pressed(Scancode::Down) && self.snake.direction != Some(Movement::Up) {
            self.snake.direction = Some(Movement::Down)
        }
        if check_pressed(Scancode::Left) && self.snake.direction != Some(Movement::Right) {
            self.snake.direction = Some(Movement::Left)
        }
        if check_pressed(Scancode::Right) && self.snake.direction != Some(Movement::Left) {
            self.snake.direction = Some(Movement::Right)
        }
    }

    pub fn update(&mut self) {
        // check if movement is legal
        let mut eval_movement = |position_coord: &mut u32, movement: i32| {
            let res: i32 = *position_coord as i32 + movement;
            if res < 0 || res > self.size as i32 {
                self.snake.alive = false;
            } else {
                *position_coord = res as u32
            }
        };

        // further slow down game
        // update when turn_ timer reaches 0
        if self.turn_timer == 0 {
            // determine snake length
            self.snake.length = 2 * (self.points + 1);

            let prev_loc = self.snake.head.clone();

            // move snake
            if let Some(dir) = &self.snake.direction {
                match dir {
                    Movement::Left => eval_movement(&mut self.snake.head.0, -1),
                    Movement::Right => eval_movement(&mut self.snake.head.0, 1),
                    Movement::Up => eval_movement(&mut self.snake.head.1, -1),
                    Movement::Down => eval_movement(&mut self.snake.head.1, 1),
                }
            }

            for segment in self.snake.body.iter() {
                if self.snake.head == *segment && self.immunity == 0 {
                    self.snake.alive = false;
                }
            }

            self.snake.body.push(prev_loc);
            if self.snake.body.len() as u32 > self.snake.length {
                self.snake.body.remove(0);
            }

            // eat fruit
            if self.snake.head == self.fruit {
                self.fruit = Vector2::gen(self.size);
                while self.snake.body.contains(&self.fruit) {
                    self.fruit = Vector2::gen(self.size);
                }
                self.points += 1;
            }

            if self.immunity != 0 { self.immunity -= 1; }
            self.turn_timer = self.turn_rate;
        } else {
            self.turn_timer -= 1;
        }
    }
}
