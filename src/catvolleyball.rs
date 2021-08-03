use amethyst::{core::transform::Transform, prelude::*, renderer::Camera};

pub const ARENA_HEIGHT: f32 = 500.0;
pub const ARENA_WIDTH: f32 = 500.0;

pub const PLAYER_HEIGHT: f32 = 32.0;
pub const PLAYER_WIDTH: f32 = 22.0;

#[derive(PartialEq, Eq)]
pub Enum side {
    Left,
    Right,
}


// Initialize the camera

pub fn initialize_camera(world: &mut world) {
    let mut transform = Transform::default();
    transform.set_translation_xyz(ARENA_WIDTH * 0.5, ARENA_HEIGHT*0.5, 1);
    world.create_entity()
    .with(Camera::standard_2d(ARENA_WIDTH, ARENA_HEIGHT))
    .with(transform)
    .build();
}

pub struct CatVolleyball;

impl SimpleState for CatVolleyball {

    fn on_start(&mut self, data: StateData<'_, GameData<'_, '_>>) {
        let world = data.world;
        initialize_camera(world);
    }

}