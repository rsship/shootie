use std::time::Duration;

use sdl2::event::Event;
use sdl2::image::{self, InitFlag, LoadTexture};
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::rect::{Point, Rect};
use sdl2::render::{Texture, WindowCanvas};

const WINDOW_TITLE: &str = "shooter game";
const WIDTH: u32 = 800;
const HEIGHT: u32 = 800;

// Into<Option<Rect>>,

fn main() -> Result<(), String> {
    let sdl_context = sdl2::init()?;

    let video_subsystem = sdl_context.video()?;

    let _image_context = image::init(InitFlag::PNG | InitFlag::JPG)?;

    let window = video_subsystem
        .window(WINDOW_TITLE, WIDTH, HEIGHT)
        .position_centered()
        .build()
        .expect("couldn't init vidoe subsystem");

    let mut canvas = window
        .into_canvas()
        .build()
        .expect("couldn't build'nt the canvas");

    let texture_creator = canvas.texture_creator();
    let texture = texture_creator
        .load_texture("src/assets/bardo.png")
        .expect("couldn't load the texTure");

    let mut events = sdl_context.event_pump()?;
    let mut i = 0;

    let position = Point::new(0, 0);
    let sprite = Rect::new(0, 0, 26, 36);

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

        i = (i + 1) % 255;

        update(&mut canvas);
        render(
            &mut canvas,
            Color::RGB(i, 255, 255 - i),
            &texture,
            position,
            sprite,
        )?;

        std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }

    Ok(())
}

fn render(
    canvas: &mut WindowCanvas,
    color: Color,
    texture: &Texture,
    position: Point,
    sprite: Rect,
) -> Result<(), String> {
    canvas.set_draw_color(color);
    canvas.clear();

    let (ctx_w, ctx_h) = canvas.output_size()?;

    let screen_center = position + Point::new((ctx_w / 2) as i32, (ctx_h / 2) as i32);
    let screen_position = Rect::from_center(screen_center, sprite.width(), sprite.height());

    canvas.copy(texture, sprite, screen_position)?;
    canvas.present();

    Ok(())
}

fn update(canvas: &mut WindowCanvas) {}
