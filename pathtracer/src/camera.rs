use euclid::default::{Point3D, Vector3D};

use crate::{ray::Ray, util};

#[derive(Debug)]
pub struct Camera {
	pos: Point3D<f32>,
	dir: Vector3D<f32>,
	u: Vector3D<f32>,
	v: Vector3D<f32>,
	horizontal: Vector3D<f32>,
	vertical: Vector3D<f32>,
	lower_left_corner: Point3D<f32>,
	aperture: f32,
}

impl Camera {
	pub fn new(
		pos: Point3D<f32>,
		dir: Vector3D<f32>,
		aspect_ratio: f32,
		fov: f32,
		aperture: f32,
		focus_distance: f32,
	) -> Self {
		let h = (fov.to_radians() / 2.0).tan();
		let u = dir.cross(Vector3D::new(0.0, 0.0, 1.0));
		let v = u.cross(dir);

		let horizontal = u * (2.0 * h * focus_distance * aspect_ratio);
		let vertical = v * (2.0 * h * focus_distance);
		let lower_left_corner = pos - horizontal / 2.0 - vertical / 2.0 + dir * focus_distance;

		Self {
			pos,
			dir,
			u,
			v,
			horizontal,
			vertical,
			lower_left_corner,
			aperture,
		}
	}

	pub fn get_ray(&self, u: f32, v: f32) -> Ray {
		let rd = util::random_in_unit_disc() * (self.aperture / 2.0);
		let offset = self.u * rd.x + self.v * rd.y;
		Ray::new(
			self.pos + offset,
			self.lower_left_corner + self.horizontal * u + self.vertical * v - self.pos - offset,
		)
	}

	pub fn pos(&self) -> Point3D<f32> {
		self.pos
	}

	pub fn set_pos(&mut self, pos: Point3D<f32>) {
		self.pos = pos;
		self.lower_left_corner = self.pos - self.horizontal / 2.0 - self.vertical / 2.0 + self.dir;
	}

	pub fn dir(&self) -> Vector3D<f32> {
		self.dir
	}
}
