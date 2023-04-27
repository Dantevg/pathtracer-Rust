use crate::{
	hittable::{HittableObject, Sphere},
	material::Material,
	texture::{CheckerTexture, SolidColour},
};
use euclid::default::Point3D;

pub fn make_scene() -> Vec<HittableObject> {
	let material_ground = Material {
		texture: CheckerTexture::new(
			SolidColour::new(0.2, 0.3, 0.1).into(),
			SolidColour::new(0.9, 0.9, 0.9).into(),
			10.0,
		)
		.into(),
		metallic: 0.0,
		specular: 0.0,
		roughness: 0.0,
		transparency: 0.0,
		ior: 0.0,
	};
	let material_centre = Material {
		texture: SolidColour::new(0.1, 0.2, 0.5).into(),
		metallic: 0.0,
		specular: 0.0,
		roughness: 0.0,
		transparency: 0.0,
		ior: 0.0,
	};
	let material_left = Material {
		texture: SolidColour::new(0.8, 0.8, 0.8).into(),
		metallic: 0.0,
		specular: 1.0,
		roughness: 0.0,
		transparency: 1.0,
		ior: 1.5,
	};
	let material_right = Material {
		texture: SolidColour::new(0.8, 0.6, 0.2).into(),
		metallic: 1.0,
		specular: 0.0,
		roughness: 0.3,
		transparency: 0.0,
		ior: 0.0,
	};

	vec![
		HittableObject::Sphere(Sphere {
			centre: Point3D::new(0.0, 0.0, -100.5),
			radius: 100.0,
			material: material_ground,
		}),
		HittableObject::Sphere(Sphere {
			centre: Point3D::new(0.0, 0.0, 0.0),
			radius: 0.5,
			material: material_centre,
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
