use std::sync::Arc;

use crate::shape::Shape;
use nalgebra::{Vector2, Vector3};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, PartialEq, PartialOrd, Debug, Clone)]
pub struct Ray {
	pub total_distance: f32,
	pub min_distance: f32,
	pub start_position: Vector3<f32>,
	pub end_position: Vector3<f32>,
	pub direction: Vector3<f32>,
	pub hit: bool,
	pub diffuse_reflection: f32,
	pub glow_distance: f32,
}

pub struct MarchSettings {
	pub origin: Vector3<f32>,
	pub direction: Vector3<f32>,
	pub max_dist: f32,
	pub min_dist: f32,
	pub steps: i32,
	pub shape: Arc<dyn Shape + Send + Sync>,
	pub light_position: Vector3<f32>,
}

#[allow(clippy::borrowed_box)]
pub fn calculate_normal(
	pos: Vector3<f32>,
	shape: &Arc<dyn Shape + Send + Sync>,
) -> Vector3<f32> {
	let e = Vector2::new(1.0, -1.0) * 0.0005; // epsilon

	(e.xyy() * shape.get_signed_distance(pos + e.xyy())
		+ e.yyx() * shape.get_signed_distance(pos + e.yyx())
		+ e.yxy() * shape.get_signed_distance(pos + e.yxy())
		+ e.xxx() * shape.get_signed_distance(pos + e.xxx()))
	.normalize()
}

impl Ray {
	pub fn march(settings: MarchSettings) -> Ray {
		let mut ray_dist = 0.0;
		let mut glow_dist = 1e9;

		for _i in 0..settings.steps {
			let pos = settings.origin + ray_dist * settings.direction;
			let distance = settings.shape.get_signed_distance(pos);
			let normal = calculate_normal(pos, &settings.shape);
			let light_direction = (settings.light_position - pos).normalize();
			let diffuse_reflection = normal.dot(&light_direction).abs();

			let ray_min_dist = if distance < settings.max_dist {
				distance
			} else {
				settings.max_dist
			};

			glow_dist = f32::min(glow_dist, distance);

			if distance < settings.min_dist {
				// ray hit an object

				return Ray {
					start_position: settings.origin,
					direction: settings.direction,
					total_distance: ray_dist,
					min_distance: ray_min_dist,
					end_position: pos,
					hit: true,
					diffuse_reflection,
					glow_distance: glow_dist,
				};
			}

			if ray_dist > settings.max_dist {
				break;
			}

			ray_dist += distance;
		}

		Ray {
			start_position: settings.origin,
			direction: settings.direction,
			total_distance: settings.max_dist,
			min_distance: settings.max_dist,
			end_position: settings.origin + settings.direction,
			diffuse_reflection: 0.0,
			glow_distance: glow_dist,
			hit: false,
		}
	}

	pub fn world_space(&self, t: f32) -> Vector3<f32> {
		self.start_position + self.direction * t
	}
}
