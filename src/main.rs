use sdl2::{self, event::Event, keyboard::Keycode, pixels::Color, sys::KeyCode};
use snake::{Game, Vector2};
fn main() -> Result<(), Box<dyn std::error::Error>> {
    // init sdl
    let sdl_ctx = sdl2::init()?;

    // enable video, input
    let subsys_video = sdl_ctx.video()?;
    let mut event_pump = sdl_ctx.event_pump()?;

    // create window, canvas
    let window = subsys_video
        .window("Snake", 600, 600)
        .position_centered()
        .build()?;

    let mut canvas = window.into_canvas().build()?;

    // create game struct
    let game = Game::new(Vector2(3, 3), 40);

    'game: loop {
        // handle input
        for evt in event_pump.poll_iter() {
            match evt {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => {
                    break 'game;
                }
                Event::KeyDown {
                    keycode: Some(Keycode::Up),
                    ..
                } => {
                    println!("Up is Pressed")
                }
                _ => {}
            }
        }

        // draw
        canvas.set_draw_color(Color::BLACK);
        canvas.clear();
        game.draw(&mut canvas)?;
        canvas.present();
    }

    // lock to 60fps
    std::thread::sleep(std::time::Duration::from_millis(16));
    Ok(())
}
