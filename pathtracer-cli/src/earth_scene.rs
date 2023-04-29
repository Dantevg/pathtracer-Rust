use std::fs::File;

use euclid::default::{Point3D, Vector3D};
use pathtracer::{
	camera::Camera,
	hittable::Sphere,
	material::Material,
	scene::Scene,
	texture::{ImageTexture, SolidColour},
};

pub fn earth_scene(width: f32, height: f32, fov: f32, aperture: f32) -> Scene {
	let decoder = png::Decoder::new(File::open("img/earthmap.png").unwrap());
	let mut reader = decoder.read_info().unwrap();
	let mut buf = vec![0; reader.output_buffer_size()];
	let info = reader.next_frame(&mut buf).unwrap();

	assert_eq!(info.bit_depth, png::BitDepth::Eight);

	let earth_material = Material::diffuse(
		ImageTexture::new(buf.into_boxed_slice(), info.width, info.height).into(),
	);
	let sun_material = Material::emissive(SolidColour::new(50.0, 35.0, 35.0).into());
	let earth = Sphere {
		centre: Point3D::zero(),
		radius: 1.0,
		material: earth_material,
	};
	let sun = Sphere {
		centre: Point3D::new(0.0, 200.0, 100.0),
		radius: 50.0,
		material: sun_material,
	};
	let look_from = Point3D::new(2.0, 0.0, 1.5);
	let look_at = Point3D::new(0.0, 0.0, 0.0);
	let camera = Camera::new(
		look_from,
		(look_at - look_from).normalize(),
		width / height,
		fov,
		aperture,
		(look_at - look_from).length(),
	);
	Scene {
		objects: vec![earth.into(), sun.into()],
		camera,
		background_colour: Vector3D::zero(),
	}
}
