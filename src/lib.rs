use sdl2::{
    keyboard::Scancode, pixels::Color, rect::Rect, render::Canvas, video::Window, EventPump,
};

use rand::Rng;

#[derive(Clone, Copy)]
pub struct Vector2(pub u32, pub u32);

pub struct Game {
    size: u32,
    fruit: Vector2,
    snake: Snake,
    points: u32,
    turnRate: u32,
    turnTimer: u32,
}

pub struct Snake {
    body: Vec<Vector2>,
    head: Vector2,
    length: u32,
    direction: Option<Movement>,
    alive: bool,
}

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

impl Game {
    pub fn new(startpos: Vector2, size: u32, speed: u32) -> Game {
        Game {
            size,
            fruit: Vector2(0, 0),
            points: 0,
            snake: Snake {
                alive: true,
                length: 1,
                head: startpos,
                body: Vec::new(),
                direction: None,
            },
            turnRate: speed,
            turnTimer: speed,
        }
    }

    pub fn draw(&self, canvas: &mut Canvas<Window>) -> Result<(), String> {
        // draw specific point
        let mut draw_point = |color, vec2: Vector2| {
            canvas.set_draw_color(color);
            canvas
                .fill_rect(Rect::new(20 * vec2.0 as i32, 20 * vec2.1 as i32, 20, 20))
                .unwrap(); // '?' leads to complications
        };
        // draw snake
        if self.snake.alive {
            draw_point(Color::GREEN, self.snake.head);
        }

        // draw fruit
        draw_point(Color::RED, self.fruit);

        Ok(())
    }

    pub fn handle_input(&mut self, event_pump: &mut EventPump) {
        let check_pressed = |scancode| event_pump.keyboard_state().is_scancode_pressed(scancode);
        if check_pressed(Scancode::Up) {
            self.snake.direction = Some(Movement::Up)
        }
        if check_pressed(Scancode::Down) {
            self.snake.direction = Some(Movement::Down)
        }
        if check_pressed(Scancode::Left) {
            self.snake.direction = Some(Movement::Left)
        }
        if check_pressed(Scancode::Right) {
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
        // update when turn timer reaches 0
        if self.turnTimer == 0 {

            // move snake
            if let Some(dir) = &self.snake.direction {
                match dir {
                    Movement::Left => eval_movement(&mut self.snake.head.0, -1),
                    Movement::Right => eval_movement(&mut self.snake.head.0, 1),
                    Movement::Up => eval_movement(&mut self.snake.head.1, -1),
                    Movement::Down => eval_movement(&mut self.snake.head.1, 1),
                }
            }

            // eat fruit
            if self.snake.head == self.fruit {
                let mut rng = rand::thread_rng();
                let x = rng.gen_range(0..self.size);
                let y = rng.gen_range(0..self.size);
                self.fruit = Vector2(x, y);
            }

            self.turnTimer = self.turnRate;
        } else {
            self.turnTimer -= 1;
        }
    }
}
