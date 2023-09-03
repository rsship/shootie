mod ai;
mod animator;
mod components;
mod keyboard;
mod physics;
mod player;
mod renderer;

use components::*;
use player::*;
use sdl2::render::{Texture, TextureCreator};
use sdl2::video::WindowContext;
use std::time::Duration;

use sdl2::event::Event;
use sdl2::image::{self, InitFlag, LoadTexture};
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::rect::{Point, Rect};
use specs::prelude::*;

const WINDOW_TITLE: &str = "shootie";
const WIDTH: u32 = 800;
const HEIGHT: u32 = 800;

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

fn load_texture<'a>(
    texture_creator: &'a TextureCreator<WindowContext>,
    textures: &mut Vec<Texture<'a>>,
    file_path: String,
) {
    match texture_creator.load_texture(file_path) {
        Ok(texture) => textures.push(texture),
        Err(err) => {
            eprintln!("{}", err);
            return;
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

    let mut dispatcher = DispatcherBuilder::new()
        .with(keyboard::Keyboard, "Keyboard", &[])
        .with(ai::AI, "AI", &[])
        .with(physics::Physics, "Physics", &["Keyboard", "AI"])
        .with(animator::Animator, "Animator", &["Keyboard", "AI"])
        .build();

    let mut world = World::new();
    dispatcher.setup(&mut world);
    renderer::SystemData::setup(&mut world);

    let movement_command: Option<MovementCommand> = None;
    world.insert(movement_command);

    let texture_creator = canvas.texture_creator();
    let mut textures: Vec<Texture> = Vec::new();
    let file_path = String::from("src/assets/bardo.png");
    load_texture(&texture_creator, &mut textures, file_path);

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

    world.register::<Position>();
    world.register::<Velocity>();
    world.register::<Sprite>();
    world.register::<MovementAnimation>();
    world.register::<KeyboardControlled>();

    world
        .create_entity()
        .with(Position(Point::new(0, 0)))
        .with(Velocity {
            speed: 0,
            direction: Direction::Nope,
        })
        .with(player_animation.right_frames[0].clone())
        .with(player_animation)
        .with(KeyboardControlled)
        .build();

    'running: loop {
        let mut movement_command = None;
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
                    movement_command = Some(MovementCommand::Move(Direction::Left));
                }
                Event::KeyDown {
                    keycode: Some(Keycode::Right),
                    repeat: false,
                    ..
                } => {
                    movement_command = Some(MovementCommand::Move(Direction::Right));
                }
                Event::KeyDown {
                    keycode: Some(Keycode::Up),
                    repeat: false,
                    ..
                } => {
                    movement_command = Some(MovementCommand::Move(Direction::Up));
                }
                Event::KeyDown {
                    keycode: Some(Keycode::Down),
                    repeat: false,
                    ..
                } => {
                    movement_command = Some(MovementCommand::Move(Direction::Down));
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
                    movement_command = Some(MovementCommand::Stop);
                }
                _ => {}
            }
        }

        *world.write_resource() = movement_command;

        canvas
            .set_logical_size(800, 800)
            .expect("Couldn't set the logical boundries");

        dispatcher.dispatch(&mut world);
        world.maintain();

        player.update_position();
        let _ = renderer::render(
            &mut canvas,
            Color::RGB(1, 17, 20),
            &textures,
            world.system_data(),
        );

        std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }

    Ok(())
}
