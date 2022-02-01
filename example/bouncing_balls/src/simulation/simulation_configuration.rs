use amethyst::{
    assets::{AssetStorage, Loader, Handle},
    core::transform::Transform,
    prelude::*,
    renderer::{Camera, ImageFormat, SpriteSheet, SpriteSheetFormat, Texture},
};

 
// Configure camera setup
pub const VIEW_HEIGHT: f32 = 400.0;
pub const VIEW_WIDTH: f32 = 400.0;

pub fn initialise_camera(world: &mut World) {
	// Setup camera in a way that screen covers whole view and (0, 0) is in the bottom left.
    let mut transform = Transform::default();
    transform.set_translation_xyz(VIEW_WIDTH * 0.5, VIEW_HEIGHT * 0.5, 1.0);

    world
        .create_entity()
        .with(Camera::standard_2d(VIEW_WIDTH, VIEW_HEIGHT))
        .with(transform)
        .build();
}

// Configure Sprite Sheet setup
pub fn load_sprite_sheet(world: &mut World) -> Handle<SpriteSheet> {
	let texture_handle = {
		let loader = world.read_resource::<Loader>();
		let texture_storage = world.read_resource::<AssetStorage<Texture>>();
		loader.load(
			"texture/ball.png",
			ImageFormat::default(),
			(),
			&texture_storage,
		)
	}; 
	let loader = world.read_resource::<Loader>();
	let sprite_sheet_store = world.read_resource::<AssetStorage<SpriteSheet>>();
	loader.load(
		"texture/ball.ron",
		SpriteSheetFormat(texture_handle),
		(),
		&sprite_sheet_store
	)
}