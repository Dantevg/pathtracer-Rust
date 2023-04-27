use crate::{
	camera::Camera,
	hittable::{HittableObject, Sphere},
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
	let material_centre = Material::diffuse(SolidColour::new(0.1, 0.2, 0.5).into());
	let material_left = Material::transparent(SolidColour::new(0.8, 0.8, 0.8).into(), 0.0, 1.5);
	let material_right = Material::metal(SolidColour::new(0.8, 0.6, 0.2).into(), 0.3);
	let material_top = Material::emissive(SolidColour::new(1.0, 1.0, 1.0).into());

	let objects = vec![
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
		HittableObject::Sphere(Sphere {
			centre: Point3D::new(0.0, 0.0, 5.0),
			radius: 3.0,
			material: material_top,
		}),
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
		background_colour: Vector3D::new(0.0, 0.0, 0.0),
	}
}
