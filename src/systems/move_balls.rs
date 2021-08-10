// Move ball system

use amethyst::{
    core::timing::Time,
    core::transform::Transform,
    core::SystemDesc,
    derive::SystemDesc,
    ecs::prelude::{Join, Read, System, SystemData, World, WriteStorage}
};

use crate::catvolleyball::Ball;

pub const GRAVITY_ACCELERATION: f32 = -40.0;

#[derive(SystemDesc)]
pub struct MoveBallsSystem;

impl<'s> System<'s> for MoveBallsSystem {
    type SystemData = (
        WriteStorage<'s, Ball>, // Get the ball for getting & updating its velocity 
        WriteStorage<'s, Transform>, // For moving the ball
        Read<'s, Time>, // To know the time difference b/w executions
    );
    
    fn run(&mut self, (mut balls, mut locals, time)): Self::SystemData) {
        // Move every ball according to its speed and time passed
        // implementing gravity
        for (ball, local) in (&mut balls, &mut locals).join() {
            local.prepend_translation_x(ball.velocity[0] * time.delta_seconds());
            // using velocity verlet integration for the ball simulation
            local.prepend_translation_y((ball.velocity[1] + time.delta_seconds() * GRAVITY_ACCELERATION/2.0)*time.delta_seconds());
            ball.velocity[1] = ball.velocity[1] + time.delta_seconds() * GRAVITY_ACCELERATION;
        }
    }
}