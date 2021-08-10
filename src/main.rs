use amethyst::{
    core::transform::TransformBundle,
    prelude::*,
    renderer::{
        plugins::{RenderFlat2D, RenderToWindow},
        types::DefaultBackend,
        RenderingBundle,
    },
    ui::{RenderUi, UiBundle},
    utils::application_root_dir,
    input::{InputBundle, StringBindings},
};

mod catvolleyball;
mod systems;

use crate::catvolleyball::CatVolleyball;


fn main() -> amethyst::Result<()> {
    amethyst::start_logger(Default::default());

    let app_root = application_root_dir()?;

    let binding_dir = app_root.join("resources");
    let binding_path = binding_dir.join("bindings_config.ron");
    let input_bundle = InputBundle::<StringBindings>::new().with_bindings_from_file(binding_path)?;

    let assets_dir = app_root.join("assets");
    let config_dir = app_root.join("config");
    let display_config_path = config_dir.join("display.ron");

    let game_data = GameDataBuilder::default()
    .with_bundle(TransformBundle::new())?
    .with_bundle(UiBundle::<StringBindings>::new())?
    .with_bundle(input_bundle)?
    .with_bundle(RenderingBundle::<DefaultBackend>::new()
    .with_plugin(RenderToWindow::from_config_path(display_config_path)?
    .with_clear([0,0,0,1]),
    )
    .with(systems::MoveBallsSystem, "ball_system", &[])
    .with(systems::BounceSystem, "collision_system", &["player_system", "ball_sytem"])
    .with_plugin(RenderFlat2D::default())
    .with_plugin(RenderUi::default()),
    )?;


    let mut game = Application::new(assets_dir, CatVolleyball, game_data)?;
    game.run();

    Ok(())
}
