use amethyst::core::transform::Transform;
use amethyst::ecs::prelude::{Component, DenseVecStorage};
use amethyst::prelude::*;
use amethyst::renderer::{ SpriteRender, SpriteSheetHandle };

use crate::hexagame::{ARENA_HEIGHT, ARENA_WIDTH, BALL_RADIUS, BALL_VELOCITY_X, BALL_VELOCITY_Y};

pub struct Ball {
    pub velocity: [f32; 2],
    pub radius: f32,
}

impl Component for Ball {
    type Storage = DenseVecStorage<Self>;
}
/// Initialises one ball in the middle-ish of the arena.
pub fn initialise_ball(world: &mut World, sprite_sheet_handle: SpriteSheetHandle) {
    // Create the translation.
    let mut local_transform = Transform::default();
    local_transform.set_xyz(ARENA_WIDTH / 2.0, ARENA_HEIGHT / 2.0, 0.0);

    // Assign the sprite for the ball
    let sprite_render = SpriteRender {
        sprite_sheet: sprite_sheet_handle,
        sprite_number: 1, // ball is the second sprite on the sprite sheet
    };

    world
        .create_entity()
        .with(sprite_render)
        .with(Ball {
            radius: BALL_RADIUS,
            velocity: [BALL_VELOCITY_X, BALL_VELOCITY_Y],
        })
        .with(local_transform)
        .build();
}
