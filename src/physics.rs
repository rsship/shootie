use crate::components::*;
use specs::prelude::*;

pub struct Physics;

impl<'a> System<'a> for Physics {
    type SystemData = (WriteStorage<'a, Position>, ReadStorage<'a, Velocity>);
    fn run(&mut self, mut data: Self::SystemData) {
        for (pos, vel) in (&mut data.0, &data.1).join() {
            use self::Direction::*;
            match vel.direction {
                Left => {
                    pos.0 = pos.0.offset(-vel.speed, 0);
                }
                Right => {
                    pos.0 = pos.0.offset(vel.speed, 0);
                }
                Down => {
                    pos.0 = pos.0.offset(0, -vel.speed);
                }
                Up => {
                    pos.0 = pos.0.offset(0, vel.speed);
                }
                Nope => {}
            }
        }
    }
}
