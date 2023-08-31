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
const PLAYER_SPEED: i32 = 5;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
    Nope,
}

#[derive()]
pub struct Player {
    position: Point,
    sprite: Rect,
    speed: i32,
    direction: Direction,
}

impl Player {
    pub fn new(
        position: Point,
        sprite: Rect,
        speed: Option<i32>,
        direction: Option<Direction>,
    ) -> Self {
        Self {
            position,
            sprite,
            speed: speed.unwrap_or(0),
            direction: direction.unwrap_or(Direction::Nope),
        }
    }
    pub fn spawn_position(&self, canvas: &WindowCanvas) -> Result<Rect, String> {
        let (w, h) = canvas.output_size()?;
        let point = self.position + Point::new((w / 2) as i32, (h / 2) as i32);
        let screen_position = Rect::from_center(point, self.sprite.width(), self.sprite.height());

        Ok(screen_position)
    }

    pub fn update_position(&mut self) {
        use self::Direction::*;

        match self.direction {
            Left => {
                self.speed = PLAYER_SPEED;
                self.position = self.position.offset(-self.speed, 0);
            }
            Right => {
                self.speed = PLAYER_SPEED;
                self.position = self.position.offset(self.speed, 0);
            }
            Up => {
                self.speed = PLAYER_SPEED;
                self.position = self.position.offset(0, -self.speed);
            }
            Down => {
                self.speed = PLAYER_SPEED;
                self.position = self.position.offset(0, self.speed);
            }

            Nope => {
                self.speed = 0;
                self.direction = Direction::Nope;
            }
        }
    }
}

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

    let position = Point::new(0, 0);
    let sprite = Rect::new(0, 0, 26, 36);

    let mut player = Player::new(position, sprite, None, None);

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
                Event::KeyDown {
                    keycode: Some(Keycode::Left),
                    repeat: false,
                    ..
                } => {
                    player.speed = PLAYER_SPEED;
                    player.direction = Direction::Left;
                }
                Event::KeyDown {
                    keycode: Some(Keycode::Right),
                    repeat: false,
                    ..
                } => {
                    player.speed = PLAYER_SPEED;
                    player.direction = Direction::Right;
                }
                Event::KeyDown {
                    keycode: Some(Keycode::Up),
                    repeat: false,
                    ..
                } => {
                    player.speed = PLAYER_SPEED;
                    player.direction = Direction::Up;
                }
                Event::KeyDown {
                    keycode: Some(Keycode::Down),
                    repeat: false,
                    ..
                } => {
                    player.speed = PLAYER_SPEED;
                    player.direction = Direction::Down;
                }
                Event::KeyUp {
                    keycode: Some(Keycode::Left),
                    repeat: false,
                    ..
                }
                | Event::KeyUp {
                    keycode: Some(Keycode::Right),
                    repeat: false,
                    ..
                }
                | Event::KeyUp {
                    keycode: Some(Keycode::Up),
                    repeat: false,
                    ..
                }
                | Event::KeyUp {
                    keycode: Some(Keycode::Down),
                    repeat: false,
                    ..
                } => {
                    player.speed = 0;
                    player.direction = Direction::Nope;
                }
                _ => {}
            }
        }

        canvas
            .set_logical_size(800, 800)
            .expect("Couldn't set the logical boundries");

        player.update_position();
        render(&mut canvas, Color::RGB(128, 255, 255), &texture, &player)?;

        std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }

    Ok(())
}

fn render(
    canvas: &mut WindowCanvas,
    color: Color,
    texture: &Texture,
    player: &Player,
) -> Result<(), String> {
    canvas.set_draw_color(color);
    canvas.clear();

    let player_screen_position = player.spawn_position(&canvas)?;
    canvas.copy(texture, player.sprite, player_screen_position)?;
    canvas.present();
    Ok(())
}
