#![feature(control_flow_enum, try_blocks)]

use nalgebra::Vector3;

pub mod camera;
pub mod ray;
pub mod shape;

pub fn max(v: Vector3<f32>, other: Vector3<f32>) -> Vector3<f32> {
	Vector3::new(
		*nalgebra::partial_max(&v.x, &other.x).unwrap_or(&v.x),
		*nalgebra::partial_max(&v.y, &other.y).unwrap_or(&v.y),
		*nalgebra::partial_max(&v.z, &other.z).unwrap_or(&v.z),
	)
}

pub fn min(v: Vector3<f32>, other: Vector3<f32>) -> Vector3<f32> {
	Vector3::new(
		*nalgebra::partial_min(&v.x, &other.x).unwrap_or(&v.x),
		*nalgebra::partial_min(&v.y, &other.y).unwrap_or(&v.y),
		*nalgebra::partial_min(&v.z, &other.z).unwrap_or(&v.z),
	)
}
