use std::path::Path;
use std::sync::Arc;
use std::time::{Duration, Instant};

//use adw::{Application, ApplicationWindow};
//use gtk4::gdk_pixbuf::Pixbuf;
//use gtk4::glib::Bytes;
//use gtk4::{prelude::*, Box, Picture};
//use libadwaita as adw;
use nalgebra::{Vector2, Vector3};
use raymarcher::ray::{MarchSettings, Ray};
use raymarcher::scene::Scene;
use raymarcher::shape::Sphere;
use rayon::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Config {
	pub glow_amount: f32,
	pub object_color: Vector3<f32>,
	pub glow_color: Vector3<f32>,
	pub steps: i32,
	pub image_width: i32,
    pub image_height: i32,
	pub camera_position: Vector3<f32>,
}

impl Default for Config {
	fn default() -> Self {
		Self {
			glow_amount: 0.5,
			glow_color: Vector3::new(1.0, 0.0, 1.0),
			object_color: Vector3::new(0.0, 0.0, 0.9),
			steps: 32,
			image_width: 1280,
            image_height: 720,
			camera_position: Vector3::new(0.0, 0.0, -15.0),
		}
	}
}

fn render(config: &Config, scene: Arc<Scene>) -> (Vec<u8>, Duration) {
	let start = Instant::now();
	let mut render_data =
		vec![0; (config.image_width * config.image_height) as usize * 4];

	render_data
		.par_chunks_mut(4)
		.enumerate()
		.zip(
			vec![0; (config.image_width * config.image_height) as usize * 4]
				.par_chunks_mut(4)
				.map(|_| scene.clone()),
		)
		.for_each(|((i, pixel), scene)| {
			let x = i as i32 % config.image_width;
			let y = i as i32 / config.image_width;
			let uv = Vector2::new(
				x as f32 - 0.5 * config.image_width as f32,
				y as f32 - 0.5 * config.image_height as f32,
			);
			let ray_dir =
				Vector3::new(uv.x as f32, uv.y as f32,config.image_height as f32)
					.normalize();

			let march_settings = MarchSettings {
				origin: config.camera_position,
				direction: ray_dir,
				max_dist: 1000.0,
				min_dist: 0.0001,
				steps: config.steps,
				shape: scene,
				light_position: Vector3::new(2.0, 2.0, 7.0),
			};
			let ray = Ray::march(march_settings);

			if ray.hit {
				let color = Vector3::new(
					ray.diffuse_reflection,
					ray.diffuse_reflection,
					ray.diffuse_reflection,
				)
				.component_mul(&config.object_color)
					* 255.0;

				pixel.copy_from_slice(&[
					color.x as u8,
					color.y as u8,
					color.z as u8,
					255,
				]);
			} else {
				let glow = f32::max(
					config.glow_amount - ray.glow_distance * 20.0,
					0.0,
				);
				let color = Vector3::new(glow, glow, glow)
					.component_mul(&config.glow_color)
					* 255.0;

				pixel.copy_from_slice(&[
					color.x as u8,
					color.y as u8,
					color.z as u8,
					255,
				]);
			}
		});

	let duration = start.elapsed();

	(render_data, duration)
}

fn main() {
	let config: Config = nu_json::from_str(
		&std::fs::read_to_string("config.hjson")
			.expect("Failed to read config.hjson"),
	)
	.expect("Failed to parse config");

	let mut scene = Scene::default();

	scene.add(Box::new(Sphere {
		center: Vector3::new(0.0, 0.0, 0.0),
		radius: 0.5,
	}));

	let (render_data, duration) =
		render(&config, Arc::new(scene));
	println!("Time elapsed: {:?}", duration);

	image::save_buffer(
		&Path::new("image.png"),
		&render_data,
		config.image_width as u32,
		config.image_height as u32,
		image::ColorType::Rgba8,
	)
	.expect("Failed to save image");
}
