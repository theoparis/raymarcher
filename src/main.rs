use std::sync::Arc;
use std::time::Duration;

use eframe::egui;
use egui::{ColorImage, ImageButton, Style, Visuals};
use egui_extras::RetainedImage;
use nalgebra::Vector3;
use raymarcher::renderer::{render, Config};
use raymarcher::scene::Scene;
use raymarcher::shape::Sphere;

fn main() {
	let options = eframe::NativeOptions::default();
	eframe::run_native(
		"Ray Marcher",
		options,
		Box::new(|creation_context| {
			let style = Style {
				visuals: Visuals::dark(),
				..Default::default()
			};
			creation_context.egui_ctx.set_style(style);

			Box::new(MyApp::default())
		}),
	);
}

#[derive(Default)]
struct MyApp {
	image: Option<RetainedImage>,
	last_elapsed: Duration,
}

impl eframe::App for MyApp {
	fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
		let mut scene = Scene::default();
		scene.add(Box::new(Sphere {
			center: Vector3::zeros(),
			radius: 0.5,
		}));

		let config = Config::default();

		egui::SidePanel::left("Settings").show(ctx, |ui| {
			ui.heading("Ray Marcher");

			if ui.button("Render").clicked() {
				let render_data = render(&config, Arc::new(scene));

				self.last_elapsed = render_data.1;

				self.image = Some(RetainedImage::from_color_image(
					"Rendered Image",
					ColorImage::from_rgba_unmultiplied(
						[
							config.image_width as usize,
							config.image_height as usize,
						],
						&render_data.0,
					),
				));
			}
		});
		egui::CentralPanel::default().show(ctx, |ui| {
			if let Some(image) = &self.image {
				ui.add(ImageButton::new(
					image.texture_id(ctx),
					image.size_vec2(),
				));
			}
		});
	}
}

//let config: Config = nu_json::from_str(
//&std::fs::read_to_string("config.hjson")
//.expect("Failed to read config.hjson"),
//)
//.expect("Failed to parse config");

//let mut scene = Scene::default();

//scene.add(Box::new(Sphere {
//center: Vector3::new(0.0, 0.0, 0.0),
//radius: 0.5,
//}));

//let (render_data, duration) = render(&config, Arc::new(scene));
//println!("Time elapsed: {:?}", duration);

//image::save_buffer(
//&Path::new("image.png"),
//&render_data,
//config.image_width as u32,
//config.image_height as u32,
//image::ColorType::Rgba8,
//)
//.expect("Failed to save image");
//}
