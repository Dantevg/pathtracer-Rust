use euclid::default::{Point3D, Vector3D};

use crate::{hittable::Hittable, material, scene::Scene};

#[derive(Debug)]
pub struct Ray {
	pub origin: Point3D<f32>,
	pub dir: Vector3D<f32>,
}

impl Ray {
	pub fn new(origin: Point3D<f32>, dir: Vector3D<f32>) -> Self {
		Self {
			origin,
			dir: dir.normalize(),
		}
	}

	#[inline]
	pub fn at(&self, t: f32) -> Point3D<f32> {
		self.origin + self.dir * t
	}

	pub fn cast(&self, scene: &Scene, background: Vector3D<f32>, depth: u32) -> Vector3D<f32> {
		if depth == 0 {
			return Vector3D::zero();
		}

		let Some(hit) = scene.hit(self, 0.001..f32::MAX) else {
			return background
		};

		let (ray, colour) = material::bounce(self, &hit);
		if let Some(ray) = ray {
			ray.cast(scene, background, depth - 1).component_mul(colour)
		} else {
			colour
		}
	}
}
