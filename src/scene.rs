use nalgebra::Vector3;
use rayon::iter::{IntoParallelRefIterator, ParallelIterator};

use crate::shape::Shape;

pub type SceneObject = Box<dyn Shape + Send + Sync>;

pub struct Scene {
	shapes: Vec<SceneObject>,
	combinator: Box<dyn Fn(f32, f32) -> f32 + Send + Sync>,
}

impl Default for Scene {
	fn default() -> Self {
		Self {
			shapes: vec![],
			combinator: Box::new(f32::min),
		}
	}
}

impl Scene {
	pub fn add(&mut self, object: SceneObject) {
		self.shapes.push(object);
	}
}

impl Shape for Scene {
	fn get_signed_distance(&self, pos: Vector3<f32>) -> f32 {
		self.shapes
			.par_iter()
			.map(|obj| obj.get_signed_distance(pos))
			.reduce_with(|a, b| (self.combinator)(a, b))
			.unwrap_or_default()
	}
}
