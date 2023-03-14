use rpg_game::*;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;

fn run() -> SdlResult<()> {
    let sdl = sdl2::init()?;
    let video_subsystem = sdl.video()?;
    let window = video_subsystem
        .window(
            "rpg game",
            (Screen::WIDTH * Glyph::SIZE) as u32,
            (Screen::WIDTH * Glyph::SIZE) as u32,
        )
        .position_centered()
        .build()?;

    let canvas = window.into_canvas().build()?;

    let texture_creator = canvas.texture_creator();

    let mut screen = Screen::new(&texture_creator, canvas)?;

    let mut event_pump = sdl.event_pump()?;

    let mut world = World::new();

    let mut now = std::time::Instant::now();
    let frame_time = std::time::Duration::from_secs(1) / 60;

    'running: loop {
        world.update();
        world.draw(&mut screen)?;
        screen.draw()?;

        let elapsed = now.elapsed();
        if elapsed < frame_time {
            std::thread::sleep(frame_time - elapsed);
        }
        println!("{:?}", now.elapsed());
        now = std::time::Instant::now();

        for evt in event_pump.poll_iter() {
            match evt {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => break 'running,
                Event::KeyDown {
                    keycode: Some(key), ..
                } => {
                    world.update_key(key);
                }
                _ => {}
            }
        }
    }
    Ok(())
}

fn main() -> SdlResult<()> {
    run().map_err(|e| format!("ERROR: {:?}", e))?;

    eprintln!("Waiting for no reason...");

    std::thread::sleep(std::time::Duration::from_secs(1));

    eprintln!("Game closed succesfully.");

    Ok(())
}
