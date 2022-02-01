use amethyst::{
    core::{Transform},
    ecs::{Join, ReadStorage, System, WriteStorage},
};

use crate::simulation::{Ball, VIEW_HEIGHT, VIEW_WIDTH};

pub struct CollisionSystem;

impl<'s> System<'s> for CollisionSystem {
	type SystemData = (
		WriteStorage<'s, Ball>,
		ReadStorage<'s, Transform>
	);

	fn run(&mut self, (mut balls, transforms): Self::SystemData) {
		for (ball, transform) in (&mut balls, &transforms).join() {
			let ball_x = transform.translation().x;
			let ball_y = transform.translation().y;

			// Top and bottom border
			if (ball_y >= VIEW_HEIGHT - ball.radius && ball.speed[1] > 0.0 ) || (ball_y <=  ball.radius && ball.speed[1] < 0.0 ) {
				ball.speed[1] = -ball.speed[1];
			}

			// Left and right side
			if (ball_x <= ball.radius && ball.speed[0] < 0.0 ) || (ball_x >= VIEW_WIDTH - ball.radius && ball.speed[0] > 0.0 ) {
				ball.speed[0] = -ball.speed[0];
			}

			// Other balls
			for (transform_check) in (&transforms).join() {
				let ball_x_check = transform_check.translation().x;
				let ball_y_check = transform_check.translation().y;

				if ball_x == ball_x_check && ball_y == ball_y_check {
					continue
				} 

				if (ball_x_check - ball_x).abs() <= ball.radius && (ball_y_check - ball_y).abs() <= ball.radius {
					ball.speed[0] = -ball.speed[0];
					ball.speed[1] = -ball.speed[1];
				}

			}
		}
	}
}