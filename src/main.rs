#[macro_use]
extern crate glium;

use glium::glutin;

fn main(){
	let mut event_loop = glutin::event_loop::EventLoop::new();
	let wb = glutin::window::WindowBuilder::new();
	let cb = glutin::ContextBuilder::new();
	let display = glium::Display::new(wb, cb, &event_loop).unwrap();
	
	event_loop.run(move |ev, _, control_flow| {
		let next_frame_time
		
		
	});
}
