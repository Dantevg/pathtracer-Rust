use euclid::default::Vector3D;

use crate::{hittable::Hit, ray::Ray, util};

#[derive(Debug, Clone, Copy)]
pub struct Material {
	/// The base colour of this material.
	pub albedo: Vector3D<f32>,

	/// How metallic this material is. A value of `1.0` gives a fully specular
	/// reflection tinted with the base colour, without diffuse reflection or
	/// transmission. At `0.0` the material consists of a diffuse or
	/// transmissive base layer, with a specular reflection layer on top.
	///
	/// (description comes from the Blender docs: https://docs.blender.org/manual/en/latest/render/shader_nodes/shader/principled.html)
	pub metallic: f32,

	/// Amount of dielectric specular reflection. A value of `1.0` gives a fully
	/// specular non-tinted reflection or transmission. At `0.0` the material
	/// consists of a diffuse layer.
	pub specular: f32,

	/// How rough this material is. At `1.0`, specular and metallic reflections
	/// will be fully rough, while reflections at `0.0` are completely sharp.
	pub roughness: f32,

	/// How transparent the material is. At `1.0`, the material is fully
	/// transparent and will only reflect at grazing angles. At `0.0`, the
	/// material consists of a diffuse or specular reflection layer.
	pub transparency: f32,

	/// The index of refraction for transmission.
	pub ior: f32,
}

pub fn bounce(ray: &Ray, hit: &Hit) -> Option<(Ray, Vector3D<f32>)> {
	if rand::random::<f32>() < hit.material.metallic {
		metallic(ray, hit)
	} else if rand::random::<f32>() < hit.material.specular {
		if rand::random::<f32>() < (1.0 - schlick(ray, hit)) * hit.material.transparency {
			refract(ray, hit)
		} else {
			specular(ray, hit)
		}
	} else {
		diffuse(ray, hit)
	}
}

fn metallic(ray: &Ray, hit: &Hit) -> Option<(Ray, Vector3D<f32>)> {
	let reflected_dir = ray.dir.reflect(hit.normal);
	let new_ray = Ray::new(
		hit.point,
		reflected_dir + util::random_in_unit_sphere() * hit.material.roughness,
	);
	if new_ray.dir.dot(hit.normal) > 0.0 {
		Some((new_ray, hit.material.albedo))
	} else {
		None
	}
}

fn specular(ray: &Ray, hit: &Hit) -> Option<(Ray, Vector3D<f32>)> {
	let reflected_dir = ray.dir.reflect(hit.normal);
	let new_ray = Ray::new(
		hit.point,
		reflected_dir + util::random_in_unit_sphere() * hit.material.roughness,
	);
	if new_ray.dir.dot(hit.normal) > 0.0 {
		Some((new_ray, Vector3D::one()))
	} else {
		None
	}
}

fn diffuse(_ray: &Ray, hit: &Hit) -> Option<(Ray, Vector3D<f32>)> {
	let scattered_dir = hit.normal + util::random_unit_vector();
	Some((Ray::new(hit.point, scattered_dir), hit.material.albedo))
}

fn refract(ray: &Ray, hit: &Hit) -> Option<(Ray, Vector3D<f32>)> {
	let front_face = ray.dir.dot(hit.normal) < 0.0;
	let (outward_normal, ior) = if front_face {
		(hit.normal, 1.0 / hit.material.ior)
	} else {
		(-hit.normal, hit.material.ior)
	};

	let cos_theta = (-ray.dir).dot(outward_normal).min(1.0);
	let sin_theta = (1.0 - cos_theta * cos_theta).sqrt();

	if ior * sin_theta > 1.0 {
		return specular(ray, hit);
	}

	let ray_out_perp = (ray.dir + outward_normal * cos_theta) * ior;
	let ray_out_parallel = outward_normal * -(1.0 - ray_out_perp.square_length()).abs().sqrt();
	Some((
		Ray::new(hit.point, ray_out_perp + ray_out_parallel),
		Vector3D::one(),
	))
}

fn schlick(ray: &Ray, hit: &Hit) -> f32 {
	let front_face = ray.dir.dot(hit.normal) < 0.0;
	let (outward_normal, ior) = if front_face {
		(hit.normal, 1.0 / hit.material.ior)
	} else {
		(-hit.normal, hit.material.ior)
	};

	let cos_theta = (-ray.dir).dot(outward_normal).min(1.0);
	let r0 = ((1.0 - ior) / (1.0 + ior)).powi(2);
	r0 + (1.0 - r0) * (1.0 - cos_theta).powi(5)
}
