use euclid::default::Vector3D;
use rand::random;

pub fn coords_to_idx(x: u32, y: u32, width: u32) -> usize {
	4 * (x + y * width) as usize
}

pub fn colour_f32_to_u32(colour_f32: Vector3D<f32>) -> [u32; 3] {
	let colour_f32 = (colour_f32 * 255.0).to_array();
	[
		colour_f32[0] as u32,
		colour_f32[1] as u32,
		colour_f32[2] as u32,
	]
}

pub fn colour_u8_to_f32(colour_u8: [u8; 3]) -> Vector3D<f32> {
	Vector3D::from(colour_u8).cast::<f32>() / 255.0
}

pub fn colour_scale_sqrt(colour: &[u32], scale: u32) -> [u8; 3] {
	debug_assert_eq!(colour.len(), 3);
	[
		(((colour[0] / scale) as f32 / 255.0).sqrt() * 255.0) as u8,
		(((colour[1] / scale) as f32 / 255.0).sqrt() * 255.0) as u8,
		(((colour[2] / scale) as f32 / 255.0).sqrt() * 255.0) as u8,
	]
}

pub fn random_in_range(min: f32, max: f32) -> f32 {
	min + (max - min) * random::<f32>()
}

pub fn random_vec_in_range(min: f32, max: f32) -> Vector3D<f32> {
	Vector3D::new(
		random_in_range(min, max),
		random_in_range(min, max),
		random_in_range(min, max),
	)
}

pub fn random_in_unit_sphere() -> Vector3D<f32> {
	loop {
		let p = random_vec_in_range(-1.0, 1.0);
		if p.square_length() < 1.0 {
			return p;
		}
	}
}

pub fn random_unit_vector() -> Vector3D<f32> {
	random_in_unit_sphere().normalize()
}

pub fn random_in_unit_disc() -> Vector3D<f32> {
	loop {
		let p = Vector3D::new(random_in_range(-1.0, 1.0), random_in_range(-1.0, 1.0), 0.0);
		if p.square_length() < 1.0 {
			return p;
		}
	}
}
