use amethyst::{assets::{AssetStorage, Handle, Loader}};
use amethyst::{
    core::transform::Transform, 
    prelude::*, 
    renderer::{Camera, ImageFormat, SpriteRender, SpriteSheet, SpriteSheetFormat, Texture},
};

pub const ARENA_HEIGHT: f32 = 500.0;
pub const ARENA_WIDTH: f32 = 500.0;

pub const PLAYER_HEIGHT: f32 = 32.0;
pub const PLAYER_WIDTH: f32 = 22.0;

#[derive(PartialEq, Eq)]
pub Enum side {
    Left,
    Right,
}

pub struct Player {
    pub side: Side,
    pub width: f32,
    pub height: f32,
}

impl Player {
    fn new(side: Side) -> Player {
        Player {
            side,
            width: PLAYER_WIDTH,
            height: PLAYER_HEIGHT,
        }
    }
}

impl Component for Player {
    type Storage = DenseVecStorage<self>;
}

fn load_sprite_sheet(world: &mut world) -> Handle<SpriteSheet> {
    let texture_handle = {
        let loader = world.read_resource::<Loader>();
        let texture_storage = world.read_resource::<AssetStorage<Texture>>();
        loader.load("texture/cat_spritesheet.png", ImageFormat::default(), (), &texture_storage,)
    };

    let loader = world.read_resource::<Loader>();
    let sprite_sheet_store = world.read_resource<AssetStorage<SpriteSheet>>();
    loader.load("texture/spritesheet.ron", SpriteSheetFormat(texture_handle), (), &sprite_sheet_store,)
};

// Initalizing the players
fn initialize_players(world: &mut world, sprite_sheet: Handle) {
    let mut left_transform = Transform::default();
    let mut right_transform = Transform::default();

    let y = PLAYER_HEIGHT/2.0;
    left_transform.set_translation_xyz(PLAYER_WIDTH, y, 1);
    right_transform.set_translation_xyz(ARENA_WIDTH - PLAYER_WIDTH * 0.5, y, 0.0);
    
    let sprite_render = SpriteRender {
        sprite_sheet: sprite_sheet.clone(),
        sprite_number: 0, // cat is the first sprite in the sprites list
    };

    world
    .create_entity()
    .with(Player::new(Side::Left))
    .with(left_transform)
    .build();

    world
    .create_entity()
    .with(Player::new(Side::Right))
    .with(right_transform)
    .build();
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
        let sprite_sheet_handle = load_sprite_sheet(world);
        world.register::<Player>();
        initialize_players(world);
        initialize_camera(world);

    }

}