use nalgebra::Vector3;
use serde::{Deserialize, Serialize};

use crate::max;

pub trait Shape {
	fn get_signed_distance(&self, pos: Vector3<f32>) -> f32;
}

#[derive(Serialize, Deserialize, PartialEq, PartialOrd, Debug, Clone)]
pub struct Cube {
	pub center: Vector3<f32>,
	pub size: Vector3<f32>,
}

impl Shape for Cube {
	fn get_signed_distance(&self, pos: Vector3<f32>) -> f32 {
		max((pos - self.center).abs() - self.size, Vector3::zeros()).magnitude()
	}
}

#[derive(Serialize, Deserialize, PartialEq, PartialOrd, Debug, Clone)]
pub struct Sphere {
	pub center: Vector3<f32>,
	pub radius: f32,
}

impl Shape for Sphere {
	fn get_signed_distance(&self, pos: Vector3<f32>) -> f32 {
		(pos - self.center).magnitude() - self.radius
	}
}
