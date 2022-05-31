use nalgebra::Vector3;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, PartialOrd)]
pub struct Camera {
	pub viewport_height: f32,
	pub viewport_width: f32,
	pub focal_length: f32,
	pub origin: Vector3<f32>,
}

impl Camera {
	pub fn new(
		viewport_height: f32,
		aspect_ratio: f32,
		focal_length: f32,
	) -> Self {
		Self {
			viewport_width: aspect_ratio * viewport_height,
			viewport_height,
			focal_length,
			origin: Vector3::new(0.0, 0.0, 0.0),
		}
	}

	pub fn horizontal(&self) -> Vector3<f32> {
		Vector3::new(self.viewport_width, 0.0, 0.0)
	}

	pub fn vertical(&self) -> Vector3<f32> {
		Vector3::new(0.0, self.viewport_height, 0.0)
	}

	pub fn lower_left(&self) -> Vector3<f32> {
		self.origin
			- self.horizontal() / 2.0
			- self.vertical() / 2.0
			- Vector3::new(0.0, 0.0, self.focal_length)
	}
}
