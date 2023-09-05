use crate::components::*;
// use rand::prelude::*;
use specs::prelude::*;

//const ENEMY_SPEED: i32 = 10;

pub struct AI;

impl<'a> System<'a> for AI {
    type SystemData = (ReadStorage<'a, Enemy>, WriteStorage<'a, Velocity>);

    fn run(&mut self, mut data: Self::SystemData) {
        for (en, vel) in (&data.0, &mut data.1).join() {
            // println!("Enemy: {:?}", en);
            // println!("Enemy Velocity: {:?}", vel);
        }
        // let mut rng = thread_rng();
        //
        // for (_, vel) in (&data.0, &mut data.1).join() {
        //     if rng.gen_range(0..10) == 0 {
        //         vel.speed = ENEMY_SPEED;
        //         vel.direction = match rng.gen_range(0..4) {
        //             0 => Direction::Up,
        //             1 => Direction::Down,
        //             2 => Direction::Right,
        //             3 => Direction::Left,
        //             _ => unreachable!("you're not supposed to here"),
        //         }
        //     }
        // }
    }
}
