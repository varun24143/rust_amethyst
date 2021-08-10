use amethyst::{
    core::timing::Time,
    core::transform::Transform,
    core::SystemDesc,
    derive::SystemDesc,
    ecs::prelude::{Join, Read, ReadExpect, ReadStorage, System, SystemData, World, WriteStorage}
};
extern crate rand;

use crate::catvolleyball::{Player, Ball, Side, ARENA_HEIGHT, ARENA_WIDTH};
use rand::Rng;

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
        

        // Bounce at players
        for (player, player_transform) in (&players, &tranforms).join() {
            let player_x = player_transform.translation().x - (player.width * 0.5);
            let player_y = player_transform.translation().y - (player.height * 0.5);

            if point_in_rect(
                ball_x, 
                ball_y, 
                player_x - ball.radius, 
                player_y - ball.radius, 
                player_x + player.width + ball.radius, 
                player_y + player.height + ball.radius,
            ){
                if ball.velocity[1] < 0.0 {
                    // only bounce when ball is falling
                    ball.velocity[1] = -ball.velocity[1];
                    let mut rng = rand::thread_rng();
                    match player.side {
                        Side::Left => {
                            ball.velocity[0] = ball.velocity[0].abs() * rng.gen_range(0.6, 1.4)
                        }
                        Side::Right => {
                            ball.velocity[0] = -ball.velocity[0].abs() * rng.gen_range(0.6, 1.4)
                        }
                    }
                }
            }
        }
    }
    }
}

fn point_in_rect(
    x: f32, // ball's x and y location
    y: f32,  
    left: f32, // the player box's boundary 
    bottom: f32, 
    right: f32, 
    top: f32
    ) -> bool {
    x >= left && x <= right && y >= bottom && y <= top
}