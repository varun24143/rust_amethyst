use amethyst::{
    core::timing::Time,
    core::transform::Transform,
    core::SystemDesc,
    derive::SystemDesc,
    ecs::prelude::{Join, Read, ReadExpect, ReadStorage, System, SystemData, World, WriteStorage}
};

use crate::catvolleyball::{Player, Ball, Side, ARENA_HEIGHT, ARENA_WIDTH};

#[derive(SystemDesc)]
pub struct BounceSystem;

impl<'s> System<'s> for BounceSystem {
    type SystemData = (
        WriteStorage<'s, Ball>,
        ReadStorage<'s, Player>,
        ReadStorage<'s, Transform>,
    );

    fn run(&mut self, (mut balls, players, tranforms):Self::SystemData) {
        /* check whether a ball collided, and bounce off accordingly. We
        also check for the velocity of the ball every time, to prevent
        multiple collisions from occuring
        */
        for (ball, transform) in (&mut balls, &tranforms).join() {
            let ball_x = transform.translation().x;
            let ball_y = transform.translation().y;

            // Bounce at the four sides of the arena
            if ball_y <= ball.radius && ball.velocity[1] < 0.0 {
                ball.velocity[1] = -ball.velocity[1];
            }else if ball_y >= (ARENA_HEIGHT - ball.radius) && ball.velocity[1] > 0.0 {
                ball.velocity[1] = -ball.velocity[1]
            }else if ball_x <= ball.radius && ball.velocity[0] < 0.0 {
                ball.velocity[0] = -ball.velocity[0];
            }else if ball_x >= (ARENA_HEIGHT - ball.radius) && ball.velocity[1] > 0.0 {
                ball.velocity[0] = -ball.velocity[0];
            }
        }
    }
}