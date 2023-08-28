use sdl2::event::Event;
use sdl2::image::{self, InitFlag, LoadTexture};
use sdl2::keyboard::Keycode;

const WINDOW_TITLE: &str = "shooter game";

fn main() -> Result<(), String> {
    let sdl_context = sdl2::init()?;

    let video_subsystem = sdl_context.video()?;

    let _image_context = image::init(InitFlag::PNG | InitFlag::JPG)?;

    let window = video_subsystem
        .window(WINDOW_TITLE, 800, 800)
        .position_centered()
        .build()
        .expect("couldn't init vidoe subsystem");

    let canvas = window
        .into_canvas()
        .build()
        .expect("couldn't build'nt the canvas");

    let texture_creator = canvas.texture_creator();
    let texture = texture_creator
        .load_texture("./assets/bardo.png")
        .expect("couldn't load the texture");

    let mut events = sdl_context.event_pump()?;

    'running: loop {
        for event in events.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => {
                    break 'running;
                }
                _ => {}
            }
        }
    }

    Ok(())
}
