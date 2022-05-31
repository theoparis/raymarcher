use nalgebra::Vector3;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, PartialEq, PartialOrd, Debug, Clone)]
pub enum Shape {
	Cube {
		center: Vector3<f32>,
		size: Vector3<f32>,
	},
	Sphere {
		center: Vector3<f32>,
		radius: f32,
	},
}
