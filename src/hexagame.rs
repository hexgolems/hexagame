use amethyst::assets::{AssetStorage, Loader};
use amethyst::core::transform::Transform;
use amethyst::ecs::prelude::{Entity};
use amethyst::prelude::*;
use amethyst::ui::{Anchor, TtfFormat, UiText, UiTransform};
use amethyst::renderer::{
Camera, PngFormat, Projection, SpriteSheet,
SpriteSheetFormat, SpriteSheetHandle, Texture, TextureMetadata,
};

use crate::components;

pub const ARENA_HEIGHT: f32 = 100.0;
pub const ARENA_WIDTH: f32 = 100.0;
pub const PADDLE_HEIGHT: f32 = 16.0;
pub const PADDLE_WIDTH: f32 = 4.0;
pub const BALL_VELOCITY_X: f32 = 10.0;
pub const BALL_VELOCITY_Y: f32 = 7.0;
pub const BALL_RADIUS: f32 = 2.0;

fn initialise_camera(world: &mut World) {
    let mut transform = Transform::default();
    transform.set_z(1.0);
    world
        .create_entity()
        .with(Camera::from(Projection::orthographic(
            0.0,
            ARENA_WIDTH,
            0.0,
            ARENA_HEIGHT,
        )))
        .with(transform)
        .build();
}

fn load_sprite_sheet(world: &mut World) -> SpriteSheetHandle {
    // Load the sprite sheet necessary to render the graphics.
    // The texture is the pixel data
    // `texture_handle` is a cloneable reference to the texture
    let texture_handle = {
        let loader = world.read_resource::<Loader>();
        let texture_storage = world.read_resource::<AssetStorage<Texture>>();
        loader.load(
            "resources/texture/pong_spritesheet.png",
            PngFormat,
            TextureMetadata::srgb_scale(),
            (),
            &texture_storage,
        )
    };
    let loader = world.read_resource::<Loader>();
    let sprite_sheet_store = world.read_resource::<AssetStorage<SpriteSheet>>();
    let handle = loader.load(
        "resources/texture/pong_spritesheet.ron", // Here we load the associated ron file
        SpriteSheetFormat,
        texture_handle, // We pass it the handle of the texture we want it to use
        (),
        &sprite_sheet_store,
    );
    return handle;
}

/// ScoreBoard contains the actual score data
#[derive(Default)]
pub struct ScoreBoard {
    pub score_left: i32,
    pub score_right: i32,
}

/// ScoreText contains the ui text components that display the score
pub struct ScoreText {
    pub p1_score: Entity,
    pub p2_score: Entity,
}

fn initialise_scoreboard(world: &mut World) {
    let font = world.read_resource::<Loader>().load(
        "resources/font/square.ttf",
        TtfFormat,
        Default::default(),
        (),
        &world.read_resource(),
    );
    let p1_transform = UiTransform::new(
        "P1".to_string(), Anchor::TopMiddle,
        -50., -50., 1., 200., 50., 0,
    );
    let p2_transform = UiTransform::new(
        "P2".to_string(), Anchor::TopMiddle,
        50., -50., 1., 200., 50., 0,
    );

    let p1_score = world
        .create_entity()
        .with(p1_transform)
        .with(UiText::new(
            font.clone(),
            "0".to_string(),
            [1., 1., 1., 1.],
            50.,
        )).build();

    let p2_score = world
        .create_entity()
        .with(p2_transform)
        .with(UiText::new(
            font.clone(),
            "0".to_string(),
            [1., 1., 1., 1.],
            50.,
        )).build();

    world.add_resource(ScoreText { p1_score, p2_score });
}

pub struct Hexagame{}

impl SimpleState for Hexagame {
    fn on_start(&mut self, data: StateData<'_, GameData<'_, '_>>) { 
        let world = data.world;
        let sprite_sheet_handle = load_sprite_sheet(world);
        world.register::<components::Ball>(); // <- add this line temporarily
        components::initialise_ball(world, sprite_sheet_handle.clone()); // <- add this line
        components::initialise_paddles(world, sprite_sheet_handle);
        initialise_camera(world);
        initialise_scoreboard(world);
    }

}

