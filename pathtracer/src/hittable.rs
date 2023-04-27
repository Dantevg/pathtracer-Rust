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
	Triangle(Triangle),
}

impl Hittable for HittableObject {
	fn hit(&self, ray: &Ray, range: Range<f32>) -> Option<Hit> {
		match self {
			HittableObject::Sphere(s) => s.hit(ray, range),
			HittableObject::Triangle(t) => t.hit(ray, range),
		}
	}
}

#[derive(Debug, Clone)]
pub struct Sphere {
	pub centre: Point3D<f32>,
	pub radius: f32,
	pub material: Material,
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
		let theta = (-outward_normal.z).acos();
		let phi = (-outward_normal.y).atan2(outward_normal.x) + PI;
		let u = phi / (2.0 * PI);
		let v = theta / PI;

		Some(Hit::new(
			point,
			outward_normal,
			distance,
			&self.material,
			Vector2D::new(u, v),
		))
	}
}

#[derive(Debug, Clone)]
pub struct Triangle {
	pub a: Point3D<f32>,
	pub b: Point3D<f32>,
	pub c: Point3D<f32>,
	pub material: Material,
}

impl From<Triangle> for HittableObject {
	fn from(value: Triangle) -> Self {
		HittableObject::Triangle(value)
	}
}

impl Hittable for Triangle {
	// https://en.wikipedia.org/wiki/M%C3%B6ller%E2%80%93Trumbore_intersection_algorithm
	// https://stackoverflow.com/a/42752998
	fn hit(&self, ray: &Ray, range: Range<f32>) -> Option<Hit> {
		let edge1 = self.b - self.a;
		let edge2 = self.c - self.a;
		let normal = edge1.cross(edge2);
		let det = -ray.dir.dot(normal);
		if det > -f32::EPSILON && det < f32::EPSILON {
			return None; // ray is parallel
		}

		let ao = ray.origin - self.a;
		let dao = ao.cross(ray.dir);
		let u = edge2.dot(dao) / det;
		let v = -edge1.dot(dao) / det;
		let distance = ao.dot(normal) / det;
		if range.contains(&distance) && u >= 0.0 && v >= 0.0 && u + v <= 1.0 {
			Some(Hit::new(
				ray.at(distance),
				normal.normalize(),
				distance,
				&self.material,
				Vector2D::new(u, v),
			))
		} else {
			None
		}
	}
}
