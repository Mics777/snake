use sdl2::{self, event::Event, keyboard::Keycode, pixels::Color};
use snake::{Game, Vector2};
fn main() -> Result<(), Box<dyn std::error::Error>> {
    // init sdl
    let sdl_ctx = sdl2::init()?;

    let (tile_size, area): (u32, u32) = (20, 30);
    let win_size = tile_size * area;

    // enable video, input
    let subsys_video = sdl_ctx.video()?;
    let mut event_pump = sdl_ctx.event_pump()?;

    // create window, canvas
    let window = subsys_video
        .window("Snake", win_size, win_size)
        .position_centered()
        .build()?;

    let mut canvas = window.into_canvas().build()?;

    // create game struct
    let mut game = Game::new(Vector2(3, 3), area, 5);
    let mut game_active = false;

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
                Event::KeyDown { ..  } => { game_active = true; }
                _ => {}
            }
        }
        game.handle_input(&mut event_pump);

        // update game
        if game_active { game.update(); }

        // draw
        canvas.set_draw_color(Color::BLACK);
        canvas.clear();
        game.draw(&mut canvas, tile_size)?;
        canvas.present();

        // lock to 60fps
        std::thread::sleep(std::time::Duration::from_millis(16));
    }

    Ok(())
}
