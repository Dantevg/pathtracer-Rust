use core::time::Duration;
use std::{
	sync::{Arc, Mutex},
	thread,
};

use clap::Parser;
use euclid::default::{Point3D, Vector3D};
use pathtracer::{camera::Camera, default_scene, Pathtracer};
use pixels::{Pixels, SurfaceTexture};
use winit::{
	dpi::PhysicalSize,
	event::{ElementState, Event, VirtualKeyCode, WindowEvent},
	event_loop::{ControlFlow, EventLoop},
	window::{Fullscreen, Window, WindowBuilder},
};

#[derive(Debug, Parser)]
#[command(author, version, about, long_about = None)]
struct Args {
	/// Initial width of the window
	#[arg(long, default_value_t = 512)]
	width: u32,

	/// Initial height of the window
	#[arg(long, default_value_t = 512)]
	height: u32,

	/// Maximum number of bounces for a single ray
	#[arg(long = "bounces", default_value_t = 10)]
	max_bounces: u32,

	/// Field-of-view of the camera
	#[arg(long, default_value_t = 70.0, alias = "field_of_view")]
	fov: f32,

	/// Aperture of the camera. 0.0 is fully sharp
	#[arg(long, default_value_t = 0.1)]
	aperture: f32,
}

fn move_cam(pt: &mut Pathtracer, by: Vector3D<f32>) {
	let pos = pt.scene.camera.pos();
	let dir = pt.scene.camera.dir();
	pt.scene.camera.set_pos(pos + dir.component_mul(by));
	pt.n_iterations = 0;
	pt.pixels.fill(0);
}

fn create_pt(args: &Args) -> Pathtracer {
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

	Pathtracer::new(args.width, args.height, args.max_bounces, scene)
}

fn create_pixels(args: &Args, window: &Window) -> Pixels {
	Pixels::new(
		args.width,
		args.height,
		SurfaceTexture::new(args.width, args.height, window),
	)
	.unwrap()
}

fn main() {
	let mut args = Args::parse();

	let event_loop = EventLoop::new();
	let window = WindowBuilder::new()
		.with_title("Path tracer test")
		.with_inner_size(PhysicalSize::new(args.width, args.height))
		.build(&event_loop)
		.unwrap();

	let pathtracer = Arc::new(Mutex::new(create_pt(&args)));
	let pixels = Arc::new(Mutex::new(create_pixels(&args, &window)));

	{
		let pathtracer = Arc::clone(&pathtracer);
		let pixels = Arc::clone(&pixels);
		thread::spawn(move || loop {
			thread::sleep(Duration::from_millis(100));
			let pt = &mut pathtracer.lock().unwrap();
			pt.render_single();
			pt.draw(pixels.lock().unwrap().frame_mut());
			println!("{}", pt.n_iterations);
		});
	}

	event_loop.run(move |e, _, control_flow| match e {
		Event::WindowEvent { event, .. } => match event {
			WindowEvent::CloseRequested => {
				*control_flow = ControlFlow::Exit;
			}
			WindowEvent::Resized(size) => {
				args.width = size.width;
				args.height = size.height;
				// Pathtracer and pixels need to be locked in this order, to
				// prevent deadlocks.
				let mut pt = pathtracer.lock().unwrap();
				let mut pixels = pixels.lock().unwrap();
				*pt = create_pt(&args);
				*pixels = create_pixels(&args, &window);
			}
			WindowEvent::KeyboardInput { input, .. } => {
				if input.state == ElementState::Released {
					return;
				}
				match input.virtual_keycode {
					Some(VirtualKeyCode::W) => {
						let pt = &mut pathtracer.lock().unwrap();
						move_cam(pt, Vector3D::new(0.0, 0.2, 0.0))
					}
					Some(VirtualKeyCode::A) => {
						let pt = &mut pathtracer.lock().unwrap();
						move_cam(pt, Vector3D::new(-0.2, 0.0, 0.0))
					}
					Some(VirtualKeyCode::S) => {
						let pt = &mut pathtracer.lock().unwrap();
						move_cam(pt, Vector3D::new(0.0, -0.2, 0.0))
					}
					Some(VirtualKeyCode::D) => {
						let pt = &mut pathtracer.lock().unwrap();
						move_cam(pt, Vector3D::new(0.2, 0.0, 0.0))
					}
					Some(VirtualKeyCode::Space) => {
						let pt = &mut pathtracer.lock().unwrap();
						move_cam(pt, Vector3D::new(0.0, 0.0, 0.2))
					}
					Some(VirtualKeyCode::R) => {
						let pt = &mut pathtracer.lock().unwrap();
						pt.n_iterations = 0;
						pt.pixels.fill(0);
					}
					Some(VirtualKeyCode::F11) => {
						if window.fullscreen() != None {
							window.set_fullscreen(None);
						} else {
							window.set_fullscreen(Some(Fullscreen::Borderless(None)));
						}
					}
					Some(VirtualKeyCode::Escape) => {
						window.set_fullscreen(None);
					}
					_ => (),
				}
			}
			WindowEvent::ModifiersChanged(state) => {
				if state.shift() {
					let pt = &mut pathtracer.lock().unwrap();
					move_cam(pt, Vector3D::new(0.0, 0.0, -0.2))
				}
			}
			_ => (),
		},

		Event::MainEventsCleared => {
			window.request_redraw();
		}

		Event::RedrawRequested(_) => {
			if let Err(err) = pixels.lock().unwrap().render() {
				println!("Error while calling pixels.render: {err}");
				*control_flow = ControlFlow::Exit;
			}
		}

		_ => (),
	});
}
