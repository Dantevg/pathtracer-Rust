use std::{
	fs::{self, File},
	io::BufWriter,
	path::PathBuf,
	thread,
	time::Instant,
};

use clap::Parser;
use indicatif::{ProgressBar, ProgressStyle};
use pathtracer::{scene::Scene, Pathtracer};

#[derive(Debug, Parser)]
#[command(author, version, about)]
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

	/// Number of threads to use for rendering
	#[arg(long = "threads", default_value_t = 1)]
	n_threads: u32,

	/// Path to scene.toml
	#[arg(short = 'i', long, value_name = "FILE")]
	scene: PathBuf,
}

fn main() -> Result<(), ()> {
	let args = Args::parse();

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

	let scene = get_scene(&args).ok_or(())?;
	let canvas = render(&args, scene);
	save(&args, canvas);

	Ok(())
}

fn get_scene(args: &Args) -> Option<Scene> {
	let scene_str = fs::read_to_string(args.scene.clone()).unwrap();
	match toml::from_str(&scene_str) {
		Ok(scene) => Some(scene),
		Err(err) => {
			match err.span() {
				Some(span) => {
					let line = scene_str[..span.start]
						.chars()
						.filter(|&c| c == '\n')
						.count() + 1;
					eprintln!("Error in scene file at line {line}: {}", err.message())
				}
				None => eprintln!("Error in scene file: {}", err.message()),
			};
			None
		}
	}
}

fn render(args: &Args, scene: Scene) -> Vec<u8> {
	let mut shared_pixels = vec![0; (args.width * args.height * 4) as usize];

	let progress_bar = ProgressBar::new(args.samples_per_pixel as u64).with_style(
		ProgressStyle::with_template("▕{wide_bar}▏{pos:>4}/{len:4} ETA {eta} ")
			.unwrap()
			.progress_chars("█▉▊▋▌▍▎▏ "),
	);
	let start_time = Instant::now();

	let mut samples_left = args.samples_per_pixel;
	let samples_per_thread = (args.samples_per_pixel as f32 / args.n_threads as f32).ceil() as u32;

	thread::scope(|scope| {
		let mut pathtracers = Vec::new();
		for _i in 0..args.n_threads {
			let n_samples = samples_per_thread.min(samples_left);
			samples_left -= n_samples;
			if n_samples <= 0 {
				break;
			}
			let scene = scene.clone();
			let progress_bar = progress_bar.clone();
			pathtracers.push(scope.spawn(move || {
				let mut pathtracer =
					Pathtracer::new(args.width, args.height, args.max_bounces, scene);
				for _j in 0..n_samples {
					pathtracer.render_single();
					progress_bar.inc(1);
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

	progress_bar.finish_with_message("done");
	println!("render time: {:?}", render_time.duration_since(start_time));

	canvas
}

fn save(args: &Args, canvas: Vec<u8>) {
	let file = File::create(args.output.clone()).unwrap();
	let ref mut file_writer = BufWriter::new(file);

	let mut png_encoder = png::Encoder::new(file_writer, args.width, args.height);
	png_encoder.set_color(png::ColorType::Rgba);
	png_encoder.set_depth(png::BitDepth::Eight);
	png_encoder
		.add_text_chunk("spp".to_string(), args.samples_per_pixel.to_string())
		.unwrap();
	let mut png_writer = png_encoder.write_header().unwrap();
	png_writer.write_image_data(&canvas).unwrap();
}
