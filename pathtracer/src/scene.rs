use core::ops::Range;

use euclid::default::Vector3D;

use crate::{
	camera::Camera,
	hittable::{Hit, Hittable, HittableObject},
	ray::Ray,
};

#[derive(Debug)]
pub struct Scene {
	pub objects: Vec<HittableObject>,
	pub camera: Camera,
	pub background_colour: Vector3D<f32>,
}

impl Hittable for Scene {
	fn hit(&self, ray: &Ray, range: Range<f32>) -> Option<Hit> {
		let mut closest_hit = None;

		for object in &self.objects {
			let max_distance = closest_hit.as_ref().map_or(range.end, |h: &Hit| h.distance);
			if let Some(hit) = object.hit(ray, range.start..max_distance) {
				closest_hit = Some(hit);
			}
		}

		closest_hit
	}
}
