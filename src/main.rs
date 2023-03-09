use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use rpg_game::*;

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

    let canvas = window.into_canvas().present_vsync().build()?;

    let texture_creator = canvas.texture_creator();

    let mut screen = Screen::new(&texture_creator, canvas)?;

    let mut event_pump = sdl.event_pump()?;

    let mut world = World::new();

    let mut now = std::time::Instant::now();

    'running: loop {
        world.draw(&mut screen)?;
        screen.draw()?;

        for evt in event_pump.poll_iter() {
            match evt {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => break 'running,
                Event::KeyDown {
                    keycode: Some(key),
                    //repeat: false,
                    ..
                } => {
                    world.update(key);
                }
                _ => {}
            }
        }

        print!("\x1b[2J\x1b[1;1H");
        println!(
            "fps = {fps:?}",
            fps = std::time::Duration::from_secs(1).as_nanos() as f32
                / now.elapsed().as_nanos() as f32
        );
        now = std::time::Instant::now();
    }
    Ok(())
}

fn eclear() {
    eprint!("\x1b[2J\x1b[1;1H");
}

fn main() {
    run().unwrap_or_else(|e| {
        eprintln!("ERROR: {e:?}");
        std::process::exit(1);
    });

    eclear();
    eprintln!("Waiting for no reason...");

    std::thread::sleep(std::time::Duration::from_millis(2000));
    for i in 0..=100 {
        eclear();
        eprintln!("Printing percentages: {i}%");

        std::thread::sleep(std::time::Duration::from_millis(20));
    }

    eprintln!("Game closed succesfully.");
}
