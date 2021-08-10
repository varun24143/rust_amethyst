use amethyst::{
    core::transform::Transform,
    core::SystemDesc,
    derive::SystemDesc,
    ecs::prelude::{Join, Read, ReadStorage, ReadExpect, WriteStorage, System, SystemData, World
    },
};

use crate::catvolleyball::{CatVolleyball, Ball, ARENA_HEIGHT, ARENA_WIDTH};

#[derive(SystemDesc)]
