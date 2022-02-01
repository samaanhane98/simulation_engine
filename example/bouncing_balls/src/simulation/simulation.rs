// Engine requirements
use amethyst::{
    assets::{Handle},
    core::transform::Transform,
    ecs::{Component, DenseVecStorage},
    prelude::*,
    renderer::{SpriteRender, SpriteSheet},
};

// Simulation requirements
use crate::simulation::simulation_configuration::*;
use rand::Rng;

pub struct Simulation;
impl SimpleState for Simulation {
	fn on_start(&mut self, data: StateData<'_, GameData<'_, '_>>) {
		// Handle simulation data
		let world = data.world;
		let sprite_sheet_handle = load_sprite_sheet(world);

		// Initialise Simulation
		initialise_balls(world, sprite_sheet_handle);
		initialise_camera(world);


	}
}

#[derive(Debug)]
pub struct Ball {
	pub radius: f32,
	pub speed: [f32; 2]
}

impl Ball {
	fn new(radius: f32) -> Ball {
		let mut rng = rand::thread_rng();
		let x_speed: f32 = rng.gen::<f32>() * 200.0;
		let y_speed: f32 = rng.gen::<f32>() * 200.0;

		Ball {
			radius,
			speed: [x_speed, y_speed]
		}
	}
}

impl Component for Ball {
	type Storage = DenseVecStorage<Self>; 
}

fn initialise_balls(world: &mut World, sprite_sheet_handle: Handle<SpriteSheet>) {
	// Assign sprites
	let sprite_render = SpriteRender::new(sprite_sheet_handle, 0);

	for _i in 0..8 {
	// Create ball
		let mut rng = rand::thread_rng();
		let ball = Ball::new(15.0);
		let x = rng.gen::<f32>() * VIEW_WIDTH;
		let y = rng.gen::<f32>() * VIEW_HEIGHT;

		// Set ball position
		let mut ball_position = Transform::default();
		ball_position.set_translation_xyz(x, y, 0.0);

		world.create_entity()
			.with(ball)
			.with(sprite_render.clone())
			.with(ball_position)
			.build();
	}
	
}