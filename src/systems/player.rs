use amethyst::{
    core::Transform,
    core::SystemDesc,
    derive::SystemDesc,
    ecs::{Join, Read, ReadStorage, System, SystemData, World, WriteStorage},
    input::{InputHandler, StringBindings},
};

use crate::catvolleyball::{Player, Side, ARENA_HEIGHT, ARENA_WIDTH};

#[derive(SystemDesc)]
pub struct PlayerSystem;

impl<'s> System<'s> for PlayerSystem {
    type SystemData = (
        WriteStorage<'s Transform>, // Get the mutable reference to the entire storage of the compnent type
        ReadStorage<'s Player>, // Get an immutable reference to the entire storage of the component Type
        Read<'s InputHandler::<StringBindings>> // Get an immutable reference to the resource
    );

    fn run(&mut self, (mut transforms, players, input): Self::SystemData) {
        for (player, transform) in (&mut players, &mut transforms).join() {
            let movement = match player.side {
                Side::Left => input.axis_value("left_player"),
                Side::Right => input.axis_value("right_player"),
            };

            if let Some(mv_amount) = movement {
                if mv_amount != 0.0 {
                    let side_name = match player.side {
                        Side::Left => "Left",
                        Side::Right => "Right",
                    };
                    println!("Side {:?} moving {:?}", side_name, mv_amount);
                }
            }
        }
    }
} 