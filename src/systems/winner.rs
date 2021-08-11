use amethyst::{
    core::transform::Transform,
    core::SystemDesc,
    derive::SystemDesc,
    ecs::prelude::{Join, Read, ReadStorage, ReadExpect, WriteStorage, System, SystemData, World
    },
};

use crate::catvolleyball::{CatVolleyball, Ball, ARENA_HEIGHT, ARENA_WIDTH};

#[derive(SystemDesc)]
pub struct WinnerSystem;

impl<'s> System<'s> for WinnerSystem {
    type SystemData = (
        WriteStorage<'s, Ball>,
        WriteStorage<'s, Transform>,
    );

    fn run(&mut self, (mut balls, mut locals): Self::SystemData) {
        for (ball, transform) in (&mut balls, &mut locals).join() {
            let ball_x = transform.translation().x;
            let ball_y = transform.translation().y;

            if ball_y <= ball.radius {
                // touched the ground
                if ball_x <= (ARENA_WIDTH/2.0) {
                    println!("Right player scored");
                }else {
                    println!("Left player scored");
                }
            }

            // reset the ball to middle
            transform.set_translation_x(ARENA_WIDTH/2.0);
            transform.set_translation_y(ARENA_HEIGHT/2.0);
            // reverse the direction
            ball.velocity[0] = -ball.velocity[0];
            ball.velocity[1] = 0.0; // reset to free drop
            
        }
    }
}