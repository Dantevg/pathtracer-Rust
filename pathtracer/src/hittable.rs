use core::{f32::consts::PI, ops::Range};

use euclid::default::{Point3D, Vector2D, Vector3D};

use crate::{material::Material, ray::Ray};

#[derive(Debug)]
pub struct Hit<'a> {
	pub point: Point3D<f32>,
	pub normal: Vector3D<f32>,
	pub distance: f32,
	pub material: &'a Material,
	pub uv: Vector2D<f32>,
}

impl<'a> Hit<'a> {
	pub fn new(
		point: Point3D<f32>,
		normal: Vector3D<f32>,
		distance: f32,
		material: &'a Material,
		uv: Vector2D<f32>,
	) -> Self {
		Self {
			point,
			normal,
			distance,
			material,
			uv,
		}
	}
}

pub trait Hittable {
	/// Returns the distance at which the `ray` hits this [`Hittable`], or
	/// [`None`] if the `ray` does not hit this object within the given `range`.
	fn hit(&self, ray: &Ray, range: Range<f32>) -> Option<Hit>;
}

#[derive(Debug)]
pub enum HittableObject {
	Sphere(Sphere),
}

impl Hittable for HittableObject {
	fn hit(&self, ray: &Ray, range: Range<f32>) -> Option<Hit> {
		match self {
			HittableObject::Sphere(s) => s.hit(ray, range),
		}
	}
}

#[derive(Debug)]
pub struct Scene(pub Vec<HittableObject>);

impl Hittable for Scene {
	fn hit(&self, ray: &Ray, range: Range<f32>) -> Option<Hit> {
		let mut closest_hit = None;

		for object in &self.0 {
			let max_distance = closest_hit.as_ref().map_or(range.end, |h: &Hit| h.distance);
			if let Some(hit) = object.hit(ray, range.start..max_distance) {
				closest_hit = Some(hit);
			}
		}

		closest_hit
	}
}

#[derive(Debug, Clone)]
pub struct Sphere {
	pub centre: Point3D<f32>,
	pub radius: f32,
	pub material: Material,
}

impl Sphere {
	fn uv(&self, point: Vector3D<f32>) -> Vector2D<f32> {
		let theta = (-point.z).acos();
		let phi = (-point.y).atan2(point.x) + PI;
		Vector2D::new(phi / (2.0 * PI), theta / PI)
	}
}

impl From<Sphere> for HittableObject {
	fn from(value: Sphere) -> Self {
		HittableObject::Sphere(value)
	}
}

impl Hittable for Sphere {
	fn hit(&self, ray: &Ray, range: Range<f32>) -> Option<Hit> {
		let oc = ray.origin - self.centre;
		let a = ray.dir.square_length();
		let half_b = oc.dot(ray.dir);
		let c = oc.square_length() - self.radius * self.radius;
		let discriminant = half_b * half_b - a * c;

		if discriminant < 0.0 {
			return None;
		}
		let sqrt_discriminant = discriminant.sqrt();

		let mut distance = (-half_b - sqrt_discriminant) / a;
		if !range.contains(&distance) {
			distance = (-half_b + sqrt_discriminant) / a;
			if !range.contains(&distance) {
				return None;
			}
		}

		let point = ray.at(distance);
		let outward_normal = (point - self.centre) / self.radius;
		Some(Hit::new(
			point,
			outward_normal,
			distance,
			&self.material,
			self.uv(outward_normal),
		))
	}
}
