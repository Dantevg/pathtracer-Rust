use std::{
	sync::{Arc, Mutex},
	thread,
};

use euclid::default::Point3D;
use pathtracer::{
	hittable::{HittableObject, Sphere},
	Pathtracer,
};
use pixels::{Pixels, SurfaceTexture};
use winit::{
	dpi::PhysicalSize,
	event::{Event, WindowEvent},
	event_loop::{ControlFlow, EventLoop},
	window::WindowBuilder,
};

const WIDTH: u32 = 512;
const HEIGHT: u32 = 512;
const BOUNCES: u32 = 10;

fn main() {
	let event_loop = EventLoop::new();
	let window = WindowBuilder::new()
		.with_title("Path tracer test")
		.with_inner_size(PhysicalSize::new(WIDTH, HEIGHT))
		.with_resizable(false)
		.build(&event_loop)
		.unwrap();

	let scene = vec![
		HittableObject::Sphere(Sphere {
			centre: Point3D::new(0.0, 0.0, -1.0),
			radius: 0.5,
		}),
		HittableObject::Sphere(Sphere {
			centre: Point3D::new(0.0, -100.5, -1.0),
			radius: 100.0,
		}),
	];

	let mut pathtracer = Pathtracer::new(WIDTH, HEIGHT, BOUNCES, scene);

	let pixels = Arc::new(Mutex::new(
		Pixels::new(WIDTH, HEIGHT, SurfaceTexture::new(WIDTH, HEIGHT, &window)).unwrap(),
	));

	{
		let pixels = Arc::clone(&pixels);
		thread::spawn(move || {
			for i in 0..u32::MAX {
				pathtracer.render_single();
				println!("{i}");
				pathtracer.draw(pixels.lock().unwrap().frame_mut());
			}
		});
	}

	event_loop.run(move |e, _, control_flow| match e {
		Event::WindowEvent {
			event: WindowEvent::CloseRequested,
			..
		} => {
			*control_flow = ControlFlow::Exit;
		}

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
