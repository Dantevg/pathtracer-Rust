use crate::{
	hittable::{HittableObject, Sphere},
	material::Material,
};
use euclid::default::{Point3D, Vector3D};

pub fn make_scene() -> Vec<HittableObject> {
	let material_ground = Material {
		albedo: Vector3D::new(0.2, 0.3, 0.1),
		metallic: 0.0,
		specular: 0.0,
		roughness: 0.0,
		transparency: 0.0,
		ior: 0.0,
	};
	let material_centre = Material {
		albedo: Vector3D::new(0.1, 0.2, 0.5),
		metallic: 0.0,
		specular: 0.0,
		roughness: 0.0,
		transparency: 0.0,
		ior: 0.0,
	};
	let material_left = Material {
		albedo: Vector3D::new(0.8, 0.8, 0.8),
		metallic: 0.0,
		specular: 1.0,
		roughness: 0.0,
		transparency: 1.0,
		ior: 1.5,
	};
	let material_right = Material {
		albedo: Vector3D::new(0.8, 0.6, 0.2),
		metallic: 1.0,
		specular: 0.0,
		roughness: 0.3,
		transparency: 0.0,
		ior: 0.0,
	};

	vec![
		HittableObject::Sphere(Sphere {
			centre: Point3D::new(0.0, 0.0, 0.0),
			radius: 0.5,
			material: material_centre,
		}),
		HittableObject::Sphere(Sphere {
			centre: Point3D::new(0.0, 0.0, -100.5),
			radius: 100.0,
			material: material_ground,
		}),
		HittableObject::Sphere(Sphere {
			centre: Point3D::new(-1.0, 0.0, 0.0),
			radius: 0.5,
			material: material_left,
		}),
		HittableObject::Sphere(Sphere {
			centre: Point3D::new(1.0, 0.0, 0.0),
			radius: 0.5,
			material: material_right,
		}),
	]
}
