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
}

#[derive(Serialize, Deserialize, PartialEq, PartialOrd, Debug, Clone)]
pub struct MarchSettings {
	pub origin: Vector3<f32>,
	pub direction: Vector3<f32>,
	pub max_dist: f32,
	pub min_dist: f32,
	pub steps: i32,
	pub shape: Shape,
	pub light_position: Vector3<f32>,
}

pub fn sd(pos: Vector3<f32>, shape: &Shape) -> f32 {
	match shape {
		Shape::Sphere { center, radius } => (pos - center).magnitude() - radius,
		_ => unimplemented!(),
	}
}

pub fn calculate_normal(pos: Vector3<f32>, shape: &Shape) -> Vector3<f32> {
	let e = Vector2::new(1.0, -1.0) * 0.0005; // epsilon

	(e.xyy() * sd(pos + e.xyy(), shape)
		+ e.yyx() * sd(pos + e.yyx(), shape)
		+ e.yxy() * sd(pos + e.yxy(), shape)
		+ e.xxx() * sd(pos + e.xxx(), shape))
	.normalize()
}

impl Ray {
	pub fn march(settings: MarchSettings) -> Ray {
		let ray = Ray {
			start_position: settings.origin,
			direction: settings.direction,
			total_distance: settings.max_dist,
			min_distance: settings.max_dist,
			end_position: settings.origin + settings.direction,
			diffuse_reflection: 0.0,
			hit: false,
		};
		let mut ray_dist = 0.0;

		for _i in 0..settings.steps {
			let pos = settings.origin + ray_dist * settings.direction;
			let distance = sd(pos, &settings.shape);
			let normal = calculate_normal(pos, &settings.shape);
			let light_direction = (settings.light_position - pos).normalize();
			let diffuse_reflection = normal.dot(&light_direction).abs();

			let ray_min_dist = if distance < settings.max_dist {
				distance
			} else {
				settings.max_dist
			};

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
				};
			}

			if ray_dist > settings.max_dist {
				break;
			}

			ray_dist += distance;
		}

		ray
	}

	pub fn world_space(&self, t: f32) -> Vector3<f32> {
		self.start_position + self.direction * t
	}
}
