extern crate amethyst;

use amethyst::{
    prelude::*,
    renderer::{DisplayConfig, DrawFlat2D, Pipeline, PosNormTex, RenderBundle, Stage},
    utils::application_root_dir,
    core::transform::TransformBundle,
    input::InputBundle,
    ui::{DrawUi, UiBundle},
};

mod hexagame;
use crate::hexagame::Hexagame;
mod systems; // Import the module



fn main() -> amethyst::Result<()> {
    amethyst::start_logger(Default::default());
    let root_dir = application_root_dir().unwrap().to_str().unwrap().to_string();
    let display_path = format!( "{}/resources/display_config.ron", &root_dir);
    let binding_path = format!( "{}/resources/bindings.ron", &root_dir);

    let config = DisplayConfig::load(&display_path);

    let pipe = Pipeline::build().with_stage(
        Stage::with_backbuffer()
            .clear_target([0.1*0.00196, 0.1*0.13726, 0.1*0.21765, 1.0], 1.0)
            .with_pass(DrawFlat2D::new())
            .with_pass(DrawUi::new()),
    );


    let input_bundle = InputBundle::<String, String>::new()
    .with_bindings_from_file(binding_path)?;

    let game_data = GameDataBuilder::default()
        .with_bundle(RenderBundle::new(pipe, Some(config)).with_sprite_sheet_processor())?
        .with_bundle(TransformBundle::new())?
        .with_bundle(input_bundle)?
        .with_bundle(UiBundle::<String, String>::new())?
        .with(systems::PaddleSystem, "paddle_system", &["input_system"])
        .with(systems::MoveBallsSystem, "ball_system", &[])
        .with(systems::BounceSystem, "collision_system", &["paddle_system", "ball_system"])
        .with(systems::WinnerSystem, "winner_system", &["ball_system"]);

    let mut game = Application::new("./", Hexagame{}, game_data)?;

    game.run();

    Ok(())
}