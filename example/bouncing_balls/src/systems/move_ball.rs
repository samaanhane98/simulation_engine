use amethyst::{
    core::timing::Time,
    core::transform::Transform,
    derive::SystemDesc,
    ecs::{Join, Read, ReadStorage, System, SystemData, WriteStorage},
};

use crate::simulation::Ball;

#[derive(SystemDesc)]
pub struct MoveBallSystem;

impl<'s> System<'s> for MoveBallSystem {
	type SystemData = (
		ReadStorage<'s, Ball>,
		WriteStorage<'s, Transform>,
		Read<'s, Time>,
	);

	fn run(&mut self, (balls, mut locals, time): Self::SystemData) {
		for (ball, local) in (&balls, &mut locals).join() {
			local.prepend_translation_x(ball.speed[0] * time.delta_seconds());
			local.prepend_translation_y(ball.speed[1] * time.delta_seconds());
		}
	}
}