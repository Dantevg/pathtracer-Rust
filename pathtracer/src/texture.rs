use std::{fs::File, path::PathBuf};

use euclid::default::{Point3D, Vector2D, Vector3D};
use serde::Deserialize;

use crate::util;

pub trait Texture {
	fn colour(&self, uv: Vector2D<f32>, point: Point3D<f32>) -> Vector3D<f32>;
}

#[derive(Debug, Clone, Deserialize)]
#[serde(tag = "type")]
pub enum AnyTexture {
	SolidColour(SolidColour),
	CheckerTexture(CheckerTexture),
	ImageTexture(ImageTexture),
	UVTexture(UVTexture),
}

impl Texture for AnyTexture {
	fn colour(&self, uv: Vector2D<f32>, point: Point3D<f32>) -> Vector3D<f32> {
		match self {
			AnyTexture::SolidColour(t) => t.colour(uv, point),
			AnyTexture::CheckerTexture(t) => t.colour(uv, point),
			AnyTexture::ImageTexture(t) => t.colour(uv, point),
			AnyTexture::UVTexture(t) => t.colour(uv, point),
		}
	}
}

#[derive(Debug, Clone, Copy, Deserialize)]
pub struct SolidColour {
	colour: Vector3D<f32>,
}

impl SolidColour {
	pub fn new(r: f32, g: f32, b: f32) -> Self {
		Self {
			colour: Vector3D::new(r, g, b),
		}
	}
}

impl From<SolidColour> for AnyTexture {
	fn from(value: SolidColour) -> Self {
		AnyTexture::SolidColour(value)
	}
}

impl Texture for SolidColour {
	fn colour(&self, _uv: Vector2D<f32>, _point: Point3D<f32>) -> Vector3D<f32> {
		self.colour
	}
}

#[derive(Debug, Clone, Deserialize)]
pub struct CheckerTexture {
	even: Box<AnyTexture>,
	odd: Box<AnyTexture>,
	scale: f32,
}

impl CheckerTexture {
	pub fn new(even: AnyTexture, odd: AnyTexture, scale: f32) -> Self {
		Self {
			even: Box::new(even),
			odd: Box::new(odd),
			scale,
		}
	}
}

impl From<CheckerTexture> for AnyTexture {
	fn from(value: CheckerTexture) -> Self {
		AnyTexture::CheckerTexture(value)
	}
}

impl Texture for CheckerTexture {
	fn colour(&self, uv: Vector2D<f32>, point: Point3D<f32>) -> Vector3D<f32> {
		let sines = (point.x * self.scale).sin()
			* (point.y * self.scale).sin()
			* (point.z * self.scale).sin();
		if sines < 0.0 {
			self.odd.colour(uv, point)
		} else {
			self.even.colour(uv, point)
		}
	}
}

#[derive(Debug, Clone, Deserialize)]
pub struct UVTexture;

impl From<UVTexture> for AnyTexture {
	fn from(value: UVTexture) -> Self {
		AnyTexture::UVTexture(value)
	}
}

impl Texture for UVTexture {
	fn colour(&self, uv: Vector2D<f32>, _point: Point3D<f32>) -> Vector3D<f32> {
		Vector3D::new(uv.x, uv.y, 0.0)
	}
}

#[derive(Debug, Clone, Deserialize)]
#[serde(from = "SerializedImageTexture")]
pub struct ImageTexture {
	image: Box<[u8]>,
	width: u32,
	height: u32,
}

impl ImageTexture {
	pub fn new(image: Box<[u8]>, width: u32, height: u32) -> Self {
		assert_eq!(image.len(), (width * height * 3) as usize);
		Self {
			image,
			width,
			height,
		}
	}

	pub fn from_path(path: PathBuf) -> Self {
		let decoder = png::Decoder::new(File::open(path).unwrap());
		let mut reader = decoder.read_info().unwrap();
		let mut buf = vec![0; reader.output_buffer_size()];
		let info = reader.next_frame(&mut buf).unwrap();

		assert_eq!(info.bit_depth, png::BitDepth::Eight);

		Self {
			image: buf.into_boxed_slice(),
			width: info.width,
			height: info.height,
		}
	}
}

impl From<ImageTexture> for AnyTexture {
	fn from(value: ImageTexture) -> Self {
		AnyTexture::ImageTexture(value)
	}
}

impl Texture for ImageTexture {
	fn colour(&self, uv: Vector2D<f32>, _point: Point3D<f32>) -> Vector3D<f32> {
		let x = (uv.x * (self.width - 1) as f32) as usize;
		let y = ((1.0 - uv.y) * (self.height - 1) as f32) as usize;
		let idx = (x + y * self.width as usize) * 3;

		util::colour_u8_to_f32([self.image[idx], self.image[idx + 1], self.image[idx + 2]])
	}
}

#[derive(Debug, Clone, Deserialize)]
struct SerializedImageTexture {
	image: PathBuf,
}

impl From<SerializedImageTexture> for ImageTexture {
	fn from(value: SerializedImageTexture) -> Self {
		ImageTexture::from_path(value.image)
	}
}
