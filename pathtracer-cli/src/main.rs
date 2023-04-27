use std::{
	fs::File,
	io::BufWriter,
	path::PathBuf,
	sync::{Arc, Mutex},
	thread,
	time::Instant,
};

use clap::Parser;
use euclid::default::Point3D;
use indicatif::ProgressBar;
use pathtracer::{camera::Camera, default_scene, Pathtracer};

#[derive(Debug, Parser)]
#[command(author, version, about, long_about = None)]
struct Args {
	/// Path to output file
	#[arg(short, long, value_name = "FILE")]
	output: PathBuf,

	/// Width of the image
	#[arg(long, default_value_t = 512)]
	width: u32,

	/// Height of the image
	#[arg(long, default_value_t = 512)]
	height: u32,

	/// Number of samples per pixel
	#[arg(long = "spp", default_value_t = 10)]
	samples_per_pixel: u32,

	/// Maximum number of bounces for a single ray
	#[arg(long = "bounces", default_value_t = 10)]
	max_bounces: u32,

	/// Field-of-view of the camera
	#[arg(long, default_value_t = 70.0, alias = "field_of_view")]
	fov: f32,

	/// Aperture of the camera. 0.0 is fully sharp
	#[arg(long, default_value_t = 0.1)]
	aperture: f32,

	/// The number of threads to use for rendering
	#[arg(long = "threads", default_value_t = 1)]
	n_threads: u32,
}

fn main() {
	let args = Args::parse();

	let mut shared_pixels = vec![0; (args.width * args.height * 4) as usize];

	let progress_bar = Arc::new(Mutex::new(ProgressBar::new(args.samples_per_pixel as u64)));
	let start_time = Instant::now();

	if let Ok(max_threads) = thread::available_parallelism() {
		if args.n_threads as usize > max_threads.get() {
			println!(
				"Warning: using more threads ({}) than available ({})",
				args.n_threads, max_threads
			);
		}
	}

	if args.n_threads > args.samples_per_pixel {
		println!(
			"Warning: number of threads ({}) is larger than the number of samples per pixel ({})",
			args.n_threads, args.samples_per_pixel
		)
	}

	let mut samples_left = args.samples_per_pixel;
	let samples_per_thread = (args.samples_per_pixel as f32 / args.n_threads as f32).ceil() as u32;

	thread::scope(|scope| {
		let mut pathtracers = Vec::new();
		for _i in 0..args.n_threads {
			let n_samples = samples_per_thread.min(samples_left);
			samples_left -= n_samples;
			if n_samples <= 0 {
				continue;
			}
			pathtracers.push(scope.spawn(|| {
				let mut scene = default_scene::make_scene();
				let look_from = Point3D::new(-2.0, -2.0, 1.5);
				let look_at = Point3D::new(0.0, 0.0, 0.0);
				scene.camera = Camera::new(
					look_from,
					(look_at - look_from).normalize(),
					args.width as f32 / args.height as f32,
					args.fov,
					args.aperture,
					(look_at - look_from).length(),
				);
				let mut pathtracer =
					Pathtracer::new(args.width, args.height, args.max_bounces, scene);
				for _j in 0..samples_per_thread {
					pathtracer.render_single();
					progress_bar.lock().unwrap().inc(1);
				}
				pathtracer
			}));
		}
		for handle in pathtracers.into_iter() {
			for (idx, subpixel) in handle.join().unwrap().pixels.iter().enumerate() {
				shared_pixels[idx] += subpixel;
			}
		}
	});

	let render_time = Instant::now();

	let mut canvas: Vec<u8> = vec![0; (args.width * args.height * 4) as usize];
	pathtracer::draw_pixels_to_canvas(&shared_pixels, &mut canvas, args.samples_per_pixel);

	progress_bar.lock().unwrap().finish_with_message("done");
	println!("render time: {:?}", render_time.duration_since(start_time));

	let file = File::create(args.output).unwrap();
	let ref mut file_writer = BufWriter::new(file);

	let mut png_encoder = png::Encoder::new(file_writer, args.width, args.height);
	png_encoder.set_color(png::ColorType::Rgba);
	png_encoder.set_depth(png::BitDepth::Eight);
	let mut png_writer = png_encoder.write_header().unwrap();
	png_writer.write_image_data(&canvas).unwrap();
}
