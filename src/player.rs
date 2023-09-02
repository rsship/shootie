use sdl2::{
    rect::{Point, Rect},
    render::WindowCanvas,
};

use crate::components::Direction;

pub const PLAYER_SPEED: i32 = 5;

#[derive(Debug)]
pub enum MovementCommand {
    Stop,
    Move(Direction),
}

#[derive(Debug)]
pub struct Player {
    pub position: Point,
    pub sprite: Rect,
    pub speed: i32,
    pub direction: Direction,
    pub current_frame: i32,
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
