mod pong;

mod systems;

use crate::pong::Pong;

extern crate amethyst;

use amethyst::{
    ui::{RenderUi, UiBundle},
    prelude::*,
    renderer::{
        plugins::{RenderFlat2D, RenderToWindow},
        types::DefaultBackend,
        RenderingBundle,
    },
    core::transform::TransformBundle,
    utils::application_root_dir,
    Error,
    input::{InputBundle, StringBindings}
};

use std::env;

fn main() -> amethyst::Result<()> {

    //env::set_var("RUST_BACKTRACE", "1");

    amethyst::start_logger(Default::default());

    let app_root = application_root_dir()?;
    let display_config_path = app_root.join("config").join("display.ron");
    let binding_path = app_root.join("config").join("bindings.ron");
    
    let game_data = GameDataBuilder::default()
    .with_bundle(
        RenderingBundle::<DefaultBackend>::new()
            // The RenderToWindow plugin provides all the scaffolding for opening a window and drawing on it
            .with_plugin(
                RenderToWindow::from_config_path(display_config_path)?
                    .with_clear([0.2, 0.2, 0.4, 1.0]),
            )
            // RenderFlat2D plugin is used to render entities with a `SpriteRender` component.
            .with_plugin(RenderFlat2D::default())
            .with_plugin(RenderUi::default()),
    )?
    .with_bundle(TransformBundle::new())?
    .with_bundle(
        InputBundle::<StringBindings>::new()
            .with_bindings_from_file(binding_path)?
    )?
    .with_bundle(UiBundle::<StringBindings>::new())?
    .with(systems::PaddleSystem, "paddle_system", &["input_system"])
    .with(systems::MoveBallsSystem, "ball_system", &[])
    .with(systems::BounceSystem, "bounce_system", &["paddle_system", "ball_system"])
    .with(systems::WinnerSystem, "winner_system", &["ball_system"]);

    let assets_dir = app_root.join("assets");
    let mut game = Application::new(assets_dir, Pong::default(), game_data)?;
    game.run();

    Ok(())
}
