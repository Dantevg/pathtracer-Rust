use core::time::Duration;
use std::{
	sync::{Arc, Mutex},
	thread,
};

use clap::Parser;
use euclid::default::Vector3D;
use pathtracer::{default_scene, Pathtracer};
use pixels::{Pixels, SurfaceTexture};
use winit::{
	dpi::PhysicalSize,
	event::{Event, VirtualKeyCode, WindowEvent},
	event_loop::{ControlFlow, EventLoop},
	window::WindowBuilder,
};

#[derive(Debug, Parser)]
#[command(author, version, about, long_about = None)]
struct Args {
	#[arg(long, default_value_t = 512)]
	width: u32,

	#[arg(long, default_value_t = 512)]
	height: u32,

	#[arg(long = "bounces", default_value_t = 10)]
	max_bounces: u32,
}

fn move_cam(pt: &mut Pathtracer, by: Vector3D<f32>) {
	let pos = pt.camera.pos();
	let dir = pt.camera.dir();
	pt.camera.set_pos(pos + dir.component_mul(by));
	pt.n_iterations = 0;
	pt.pixels.fill(0);
}

fn main() {
	let args = Args::parse();

	let event_loop = EventLoop::new();
	let window = WindowBuilder::new()
		.with_title("Path tracer test")
		.with_inner_size(PhysicalSize::new(args.width, args.height))
		.with_resizable(false)
		.build(&event_loop)
		.unwrap();

	let (scene, camera) = default_scene::make_scene(args.width as f32, args.height as f32);
	let pathtracer = Arc::new(Mutex::new(Pathtracer::new(
		args.width,
		args.height,
		args.max_bounces,
		camera,
		scene,
	)));
	let pixels = Arc::new(Mutex::new(
		Pixels::new(
			args.width,
			args.height,
			SurfaceTexture::new(args.width, args.height, &window),
		)
		.unwrap(),
	));

	{
		let pathtracer = Arc::clone(&pathtracer);
		let pixels = Arc::clone(&pixels);
		thread::spawn(move || loop {
			thread::sleep(Duration::from_millis(100));
			let pt = &mut pathtracer.lock().unwrap();
			pt.render_single();
			println!("{}", pt.n_iterations);
			pt.draw(pixels.lock().unwrap().frame_mut());
		});
	}

	event_loop.run(move |e, _, control_flow| match e {
		Event::WindowEvent { event, .. } => match event {
			WindowEvent::CloseRequested => {
				*control_flow = ControlFlow::Exit;
			}
			WindowEvent::KeyboardInput { input, .. } => match input.virtual_keycode {
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
				_ => (),
			},
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
