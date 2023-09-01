mod components;
use components::*;
use std::time::Duration;

use sdl2::event::Event;
use sdl2::image::{self, InitFlag, LoadTexture};
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::rect::{Point, Rect};
use sdl2::render::{Texture, WindowCanvas};
use specs::prelude::*;

const WINDOW_TITLE: &str = "shooter game";
const WIDTH: u32 = 800;
const HEIGHT: u32 = 800;
const PLAYER_SPEED: i32 = 5;

fn character_animation_frames(
    spritesheet: usize,
    top_left_frame: Rect,
    direction: Direction,
) -> Vec<Sprite> {
    let (frame_width, frame_height) = top_left_frame.size();
    let mut frames = Vec::new();

    for i in 0..3 {
        frames.push(Sprite {
            spritesheet,
            region: Rect::new(
                top_left_frame.x() + frame_width as i32 * i,
                top_left_frame.y() + frame_height as i32 * direction_sprite_row(direction),
                frame_width,
                frame_height,
            ),
        })
    }

    frames
}

#[derive(Debug)]
pub struct Player {
    position: Point,
    sprite: Rect,
    speed: i32,
    direction: Direction,
    current_frame: i32,
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
            current_frame: 0,
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

        if self.speed != 0 {
            self.current_frame = (self.current_frame + 1) % 3;
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
    // let texture = texture_creator
    //     .load_texture("src/assets/bardo.png")
    //     .expect("couldn't load the texTure");
    let textures = [texture_creator.load_texture("src/assets/bardo.png")];

    let mut events = sdl_context.event_pump()?;

    let position = Point::new(0, 0);
    let player_spritesheet = 0;
    let player_top_left_frame = Rect::new(0, 0, 26, 36);

    let mut player = Player::new(position, player_top_left_frame, None, None);
    let player_animation = MovementAnimation {
        current_frame: 0,
        up_frames: character_animation_frames(
            player_spritesheet,
            player_top_left_frame,
            Direction::Up,
        ),
        down_frames: character_animation_frames(
            player_spritesheet,
            player_top_left_frame,
            Direction::Down,
        ),
        left_frames: character_animation_frames(
            player_spritesheet,
            player_top_left_frame,
            Direction::Left,
        ),
        right_frames: character_animation_frames(
            player_spritesheet,
            player_top_left_frame,
            Direction::Right,
        ),
    };

    let mut world = World::new();
    world.register::<Position>();
    world.register::<Velocity>();
    world.register::<Sprite>();
    world.register::<MovementAnimation>();

    world
        .create_entity()
        .with(Position(Point::new(0, 0)))
        .with(Velocity {
            speed: 0,
            direction: Direction::Nope,
        })
        .with(player_animation.right_frames[0].clone())
        .with(player_animation)
        .build();

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

        for texture in &textures {
            let texture = match texture {
                Err(err) => {
                    eprintln!("An error: {} skipped", err);
                    continue;
                }
                Ok(x) => x,
            };
            render(&mut canvas, Color::RGB(128, 255, 255), &texture, &player)?;
        }

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

    let (frame_width, frame_height) = player.sprite.size();
    let current_frame = Rect::new(
        player.sprite.x() + frame_width as i32 * player.current_frame,
        player.sprite.y() + frame_height as i32 * direction_sprite_row(player.direction),
        frame_width,
        frame_height,
    );

    let player_screen_position = player.spawn_position(&canvas)?;
    canvas.copy(texture, current_frame, player_screen_position)?;
    canvas.present();
    Ok(())
}

fn direction_sprite_row(direction: Direction) -> i32 {
    use self::Direction::*;
    match direction {
        Up => 3,
        Down => 0,
        Left => 1,
        Right => 2,
        Nope => 0,
    }
}
