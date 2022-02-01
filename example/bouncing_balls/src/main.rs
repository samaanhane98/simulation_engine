use amethyst::{
    prelude::*,
    renderer::{
        plugins::{RenderFlat2D, RenderToWindow},
        types::DefaultBackend,
        RenderingBundle,
    },
    utils::application_root_dir,
    core::TransformBundle
};

// Bring in simulation entities and physics
mod simulation;
use crate::simulation::Simulation;

mod systems;

fn main() -> amethyst::Result<()> {
    // Engine initialisation
    amethyst::start_logger(Default::default());
    let app_root = application_root_dir()?;
    let display_config_path = app_root.join("config").join("display.ron");
    let assets_dir = app_root.join("assets");
    
    
    // Set up simulation data and rendering
    let simulation_data = GameDataBuilder::default()
        .with_bundle(
            RenderingBundle::<DefaultBackend>::new()
            // This bundle opens window and allows drawing on it
            .with_plugin(
                RenderToWindow::from_config_path(display_config_path)?
                // Set color here
                    .with_clear([0.921875, 0.921875, 0.921875, 1.0]),
            )
            // Plugin used to render sprites
            .with_plugin(RenderFlat2D::default()),
        )?
        // Track entity positions
        .with_bundle(TransformBundle::new())?
        // Pysics
        .with(systems::MoveBallSystem, "ball_system", &[])
        .with(systems::CollisionSystem, "collission_system", &[]);

    
    // Run simulation
    let mut simulation = Application::new(assets_dir, Simulation, simulation_data)?;
    simulation.run();

    Ok(())
}