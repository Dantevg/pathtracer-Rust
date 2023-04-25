use euclid::default::{Point3D, Vector3D};

use crate::ray::Ray;

#[derive(Debug)]
pub struct Camera {
	pos: Point3D<f32>,
	horizontal: Vector3D<f32>,
	vertical: Vector3D<f32>,
	lower_left_corner: Point3D<f32>,
}

impl Camera {
	pub fn new(aspect_ratio: f32, pos: Point3D<f32>, focal_length: f32) -> Self {
		let horizontal = Vector3D::new(2.0 * aspect_ratio, 0.0, 0.0);
		let vertical = Vector3D::new(0.0, 0.0, 2.0);
		let lower_left_corner =
			pos - horizontal / 2.0 - vertical / 2.0 + Vector3D::new(0.0, focal_length, 0.0);
		Self {
			pos,
			horizontal,
			vertical,
			lower_left_corner,
		}
	}

	pub fn get_ray(&self, u: f32, v: f32) -> Ray {
		Ray::new(
			self.pos,
			self.lower_left_corner + self.horizontal * u + self.vertical * v - self.pos,
		)
	}
}
