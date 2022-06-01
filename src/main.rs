use std::path::Path;

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
}

impl Default for Config {
	fn default() -> Self {
		Self {
			glow_amount: 0.5,
			glow_color: Vector3::new(1.0, 0.0, 1.0),
			object_color: Vector3::new(0.0, 0.0, 0.9),
			steps: 32,
		}
	}
}

fn main() {
	//let app = Application::builder()
	//.application_id("com.theoparis.rayrus")
	//.build();

	//app.connect_activate(build_ui);

	//app.run();
	//}

	//fn build_ui(app: &Application) {
	let aspect_ratio = 16.0 / 9.0;
	let image_width = 1920;
	let image_height = image_width / aspect_ratio as i32;
	let cam_pos = Vector3::new(0.0, 0.0, -15.0);

	let config: Config = nu_json::from_str(
		&std::fs::read_to_string("config.hjson")
			.expect("Failed to read config.hjson"),
	)
	.expect("Failed to parse config");

	let render_data: Vec<u8> = (0..image_width)
		.into_par_iter()
		.map(|x| {
			(0..image_height)
				.into_par_iter()
				.map(|y| {
					let uv = Vector2::new(
						x as f32 - 0.5 * image_width as f32,
						y as f32 - 0.5 * image_height as f32,
					);
					let ray_dir = Vector3::new(
						uv.x as f32,
						uv.y as f32,
						image_height as f32,
					)
					.normalize();

					let mut scene = Scene::default();
					scene.add(Box::new(Sphere {
						center: Vector3::new(0.0, 0.0, 0.0),
						radius: 0.5,
					}));
					//scene.add(Box::new(Cube {
					//center: Vector3::new(0.0, 3.0, 0.0),
					//size: Vector3::new(0.6, 0.6, 0.5),
					//}));

					let scene = Box::new(scene);

					let march_settings = MarchSettings {
						origin: cam_pos,
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

						vec![color.x as u8, color.y as u8, color.z as u8, 255]
					} else {
						let glow = f32::max(
							config.glow_amount - ray.glow_distance * 20.0,
							0.0,
						);
						let color = Vector3::new(glow, glow, glow)
							.component_mul(&config.glow_color)
							* 255.0;

						vec![
							(color.x) as u8,
							(color.y) as u8,
							(color.z) as u8,
							255,
						]
					}
				})
				.flatten()
				.collect::<Vec<u8>>()
		})
		.flatten()
		.collect();

	image::save_buffer(
		&Path::new("image.png"),
		&render_data,
		image_width as u32,
		image_height as u32,
		image::ColorType::Rgba8,
	)
	.unwrap();

	//let pixbuf = Pixbuf::from_bytes(
	//&Bytes::from(&render_data),
	//gtk4::gdk_pixbuf::Colorspace::Rgb,
	//true,
	//8,
	//image_width,
	//image_height,
	//4 * image_width,
	//);
	//let picture = Picture::for_pixbuf(&pixbuf);
	//picture.set_keep_aspect_ratio(true);
	//picture.set_can_shrink(false);
	//picture.set_hexpand(false);
	//picture.set_vexpand(false);
	//picture.set_halign(gtk4::Align::Fill);
	//picture.set_valign(gtk4::Align::Fill);

	// Present window
	//let content = Box::new(gtk4::Orientation::Vertical, 0);
	//content.append(&picture);

	//let window = ApplicationWindow::builder()
	//.application(app)
	//.title("Rayrus")
	//.content(&content)
	//.build();

	//window.present();
}
