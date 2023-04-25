use std::{fs::File, io::BufWriter, path::PathBuf};

use clap::Parser;
use euclid::default::{Point3D, Vector3D};
use indicatif::ProgressBar;
use pathtracer::{
	hittable::{HittableObject, Sphere},
	material::Material,
	Pathtracer,
};

#[derive(Debug, Parser)]
#[command(author, version, about, long_about = None)]
struct Args {
	#[arg(short, long)]
	output: PathBuf,

	#[arg(long, default_value_t = 512)]
	width: u32,

	#[arg(long, default_value_t = 512)]
	height: u32,

	#[arg(long = "spp", default_value_t = 1)]
	samples_per_pixel: u32,

	#[arg(long = "bounces", default_value_t = 10)]
	max_bounces: u32,
}

fn main() {
	let args = Args::parse();

	let material_ground = Material {
		albedo: Vector3D::new(0.8, 0.8, 0.0),
		metallic: 0.0,
		specular: 0.0,
		roughness: 0.0,
		transparency: 0.0,
		ior: 0.0,
	};
	let material_centre = Material {
		albedo: Vector3D::new(0.7, 0.3, 0.3),
		metallic: 0.0,
		specular: 1.0,
		roughness: 0.0,
		transparency: 1.0,
		ior: 1.5,
	};
	let material_left = Material {
		albedo: Vector3D::new(0.8, 0.8, 0.8),
		metallic: 0.0,
		specular: 1.0,
		roughness: 0.2,
		transparency: 0.0,
		ior: 0.0,
	};
	let material_right = Material {
		albedo: Vector3D::new(0.8, 0.6, 0.2),
		metallic: 1.0,
		specular: 0.0,
		roughness: 1.0,
		transparency: 0.0,
		ior: 0.0,
	};

	let scene = vec![
		HittableObject::Sphere(Sphere {
			centre: Point3D::new(0.0, 2.0, 0.0),
			radius: 0.5,
			material: material_centre,
		}),
		HittableObject::Sphere(Sphere {
			centre: Point3D::new(0.0, 2.0, -100.5),
			radius: 100.0,
			material: material_ground,
		}),
		HittableObject::Sphere(Sphere {
			centre: Point3D::new(-1.0, 2.0, 0.0),
			radius: 0.5,
			material: material_left,
		}),
		HittableObject::Sphere(Sphere {
			centre: Point3D::new(1.0, 2.0, 0.0),
			radius: 0.5,
			material: material_right,
		}),
	];

	let mut pathtracer = Pathtracer::new(args.width, args.height, args.max_bounces, scene);
	let mut canvas: Vec<u8> = vec![0; (args.width * args.height * 4) as usize];

	let progress_bar = ProgressBar::new(args.samples_per_pixel as u64);
	pathtracer.render(&mut canvas, args.samples_per_pixel, || progress_bar.inc(1));
	progress_bar.finish_with_message("done");

	let file = File::create(args.output).unwrap();
	let ref mut file_writer = BufWriter::new(file);

	let mut png_encoder = png::Encoder::new(file_writer, args.width, args.height);
	png_encoder.set_color(png::ColorType::Rgba);
	png_encoder.set_depth(png::BitDepth::Eight);
	let mut png_writer = png_encoder.write_header().unwrap();
	png_writer.write_image_data(&canvas).unwrap();
}
