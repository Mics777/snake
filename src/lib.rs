use sdl2::{pixels::Color, rect::Rect, render::Canvas, video::Window};

pub struct Vector2(pub u32, pub u32);
pub struct Game {
    size: u32,
    fruit: Vector2,
    snake: Snake,
}

pub struct Snake {
    body: Vec<Vector2>,
    head: Vector2,
    length: u32,
}

pub enum Movement {
    Left,
    Right,
    Up,
    Down,
}

impl Game {
    pub fn new(startpos: Vector2, size: u32) -> Game {
        Game {
            size,
            fruit: Vector2(0, 0),
            snake: Snake {
                length: 1,
                head: startpos,
                body: Vec::new(),
            },
        }
    }

    pub fn draw(&self, canvas: &mut Canvas<Window>) -> Result<(), String> {
        canvas.set_draw_color(Color::GREEN);
        canvas.fill_rect(Rect::new(
            20 * self.snake.head.0 as i32,
            20 * self.snake.head.1 as i32,
            20,
            20,
        ))?;
        Ok(())
    }
}
