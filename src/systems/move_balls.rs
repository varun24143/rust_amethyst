// Move ball system

use amethyst::{
    core::timing::Time,
    core::transform::Transform,
    core::SystemDesc,
    derive::SystemDesc,
    ecs::prelude::{Join, Read, System, SystemData, World, WriteStorage}
};

use crate::catvolleyball::Ball;

