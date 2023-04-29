use crate::{
	camera::Camera,
	hittable::Sphere,
	material::Material,
	scene::Scene,
	texture::{CheckerTexture, SolidColour},
};
use euclid::default::{Point3D, Vector3D};

pub fn make_scene() -> Scene {
	let material_ground = Material::diffuse(
		CheckerTexture::new(
			SolidColour::new(0.2, 0.3, 0.1).into(),
			SolidColour::new(0.9, 0.9, 0.9).into(),
			10.0,
		)
		.into(),
	);

	let objects = vec![
		Sphere {
			centre: Point3D::new(0.0, 0.0, -100.5),
			radius: 100.0,
			material: material_ground,
		}
		.into(),
		Sphere {
			centre: Point3D::new(0.0, 0.0, 0.0),
			radius: 0.5,
			material: Material::dielectric(SolidColour::new(0.1, 0.2, 0.5).into(), 0.0),
		}
		.into(),
		Sphere {
			centre: Point3D::new(-1.0, 0.0, 0.0),
			radius: 0.5,
			material: Material::transparent(SolidColour::new(0.8, 0.8, 0.8).into(), 0.0, 1.5),
		}
		.into(),
		Sphere {
			centre: Point3D::new(1.0, 0.0, 0.0),
			radius: 0.5,
			material: Material::metal(SolidColour::new(0.8, 0.6, 0.2).into(), 0.3),
		}
		.into(),
		Sphere {
			centre: Point3D::new(0.0, 1.0, 2.0),
			radius: 1.0,
			material: Material::emissive(SolidColour::new(2.0, 1.6, 1.5).into()),
		}
		.into(),
		// Triangle {
		// 	a: Point3D::new(0.5, 0.0, 0.0),
		// 	b: Point3D::new(0.5, -1.0, 0.0),
		// 	c: Point3D::new(0.5, 0.0, 1.0),
		// 	material: Material::transparent(SolidColour::new(0.8, 0.8, 0.8).into(), 0.0, 1.5),
		// 	// material: Material::diffuse(SolidColour::new(1.0, 1.0, 1.0).into()),
		// 	// material: Material::dielectric(SolidColour::new(1.0, 1.0, 1.0).into(), 0.2),
		// }
		// .into(),
	];

	let look_from = Point3D::new(-2.0, -2.0, 1.5);
	let look_at = Point3D::new(0.0, 0.0, 0.0);
	let camera = Camera::new(
		look_from,
		(look_at - look_from).normalize(),
		1.0,
		70.0,
		0.1,
		(look_at - look_from).length(),
	);

	Scene {
		objects,
		camera,
		// background_colour: Vector3D::zero(),
		background_colour: Vector3D::new(0.04, 0.06, 0.16),
		// background_colour: Vector3D::new(0.5, 0.6, 0.8),
	}
}
