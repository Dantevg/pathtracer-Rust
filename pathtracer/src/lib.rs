mod camera;
pub mod hittable;
mod ray;
mod util;

use camera::Camera;
use euclid::default::Point3D;
use hittable::{HittableObject, Scene};

#[derive(Debug)]
pub struct Pathtracer {
	canvas_width: u32,
	canvas_height: u32,
	max_bounces: u32,
	camera: Camera,
	scene: Scene,
	pixels: Box<[u32]>,
	n_iterations: u32,
}

impl Pathtracer {
	pub fn new(width: u32, height: u32, max_bounces: u32, scene: Vec<HittableObject>) -> Self {
		Self {
			canvas_width: width,
			canvas_height: height,
			max_bounces,
			camera: Camera::new(width as f32 / height as f32, Point3D::zero(), 1.0),
			scene: Scene(scene),
			pixels: vec![0; (width * height * 4) as usize].into_boxed_slice(),
			n_iterations: 0,
		}
	}

	pub fn render_single(&mut self) {
		for y in 0..self.canvas_height {
			for x in 0..self.canvas_width {
				let u =
					(x as f32 + util::random_in_range(-0.5, 0.5)) / (self.canvas_width - 1) as f32;
				let v = 1.0
					- (y as f32 + util::random_in_range(-0.5, 0.5))
						/ (self.canvas_height - 1) as f32;

				let ray = self.camera.get_ray(u, v);
				let colour = ray.cast(&self.scene, self.max_bounces);
				let pixel_idx = util::coords_to_idx(x, y, self.canvas_width);

				let colour_arr = util::colour_f32_to_u32(colour);
				self.pixels[pixel_idx + 0] += colour_arr[0];
				self.pixels[pixel_idx + 1] += colour_arr[1];
				self.pixels[pixel_idx + 2] += colour_arr[2];
			}
		}
		self.n_iterations += 1;
	}

	pub fn draw(&self, canvas: &mut [u8]) {
		debug_assert_eq!(
			canvas.len(),
			(self.canvas_width * self.canvas_height * 4) as usize
		);

		for y in 0..self.canvas_height {
			for x in 0..self.canvas_width {
				let idx = util::coords_to_idx(x, y, self.canvas_width);
				let output_colour =
					util::colour_scale_sqrt(&self.pixels[idx..idx + 3], self.n_iterations);
				canvas[idx + 0] = output_colour[0];
				canvas[idx + 1] = output_colour[1];
				canvas[idx + 2] = output_colour[2];
				canvas[idx + 3] = 255;
			}
		}
	}

	pub fn render(
		&mut self,
		canvas: &mut [u8],
		samples_per_pixel: u32,
		progress_cb: impl Fn() -> (),
	) {
		debug_assert_eq!(
			canvas.len(),
			(self.canvas_width * self.canvas_height * 4) as usize
		);

		self.n_iterations = 0;

		for _i in 0..samples_per_pixel {
			self.render_single();
			progress_cb();
		}

		self.draw(canvas);
	}
}