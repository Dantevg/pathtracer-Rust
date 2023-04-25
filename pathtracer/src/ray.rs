use euclid::default::{Point3D, Vector3D};

use crate::{
	hittable::{Hittable, Scene},
	util,
};

#[derive(Debug)]
pub struct Ray {
	pub origin: Point3D<f32>,
	pub dir: Vector3D<f32>,
}

impl Ray {
	pub fn new(origin: Point3D<f32>, dir: Vector3D<f32>) -> Self {
		Self { origin, dir }
	}

	#[inline]
	pub fn at(&self, t: f32) -> Point3D<f32> {
		self.origin + self.dir * t
	}

	pub fn cast(&self, scene: &Scene, depth: u32) -> Vector3D<f32> {
		if depth == 0 {
			return Vector3D::zero();
		}

		if let Some(hit) = scene.hit(self, 0.001..f32::MAX) {
			let target = hit.point + hit.normal + util::random_unit_vector();
			let ray = Ray::new(hit.point, target - hit.point);
			ray.cast(scene, depth - 1) * 0.5
		} else {
			Vector3D::lerp(
				Vector3D::new(0.5, 0.7, 1.0),
				Vector3D::new(1.0, 1.0, 1.0),
				0.5 * (-self.dir.normalize().y + 1.0),
			)
		}
	}
}
