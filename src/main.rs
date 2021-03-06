use glium::*;
use std::time::*;
use std::io::Cursor;
use std::file;

mod mesh;
mod input;
mod math;
mod camera;
mod voxel;


fn main() {
    let event_loop = glutin::event_loop::EventLoop::new();
    let wb = glutin::window::WindowBuilder::new(); //janela
    let cb = glutin::ContextBuilder::new().with_depth_buffer(24);
    let display = glium::Display::new(wb, cb, &event_loop).unwrap();
	
	let mut chunk = voxel::VoxelChunk::new(2usize,[1i16, 1, 1]);
	let mesh = chunk.generate_mesh(&display);

	let mut input = input::InputHandler::new();
	
	let mut cam = camera::Camera::new();
	
	let mut delta = 0f32;
	let mut last_frame: Instant = Instant::now();

	let mouse: bool = true;

    event_loop.run(move |event, sus, control_flow| {
        let next_frame_time = std::time::Instant::now() +
            std::time::Duration::from_nanos(16_666_667);
        *control_flow = glutin::event_loop::ControlFlow::WaitUntil(next_frame_time);
        
		
		input.is_key_pressed(1u32);
		
		let current_frame = Instant::now();
		delta = (current_frame - last_frame).as_secs_f32();
		last_frame = current_frame;
		
		cam.update(delta, &input);
		
		input.update_mouse(input.mouse_pos);
		
		println!("\x1B[2J\x1B[1;1Hdelta = {}\n{:#?}\n{:#?}", delta, input, cam);
        match event {
            glutin::event::Event::WindowEvent { event, .. } => match event {
                glutin::event::WindowEvent::CloseRequested => {
                    *control_flow = glutin::event_loop::ControlFlow::Exit;
                    return;
                },
                glutin::event::WindowEvent::CursorMoved {
					position: mouse_pos,
					..
				} => {
					let current_pos = [mouse_pos.x, mouse_pos.y];
				
					input.update_mouse(current_pos);
				},
				glutin::event::WindowEvent::KeyboardInput {
					input: glutin::event::KeyboardInput{ scancode: key, state: is_pressed, .. },
					..	
				} => {
					input.update_keyboard(key, is_pressed);
				},
                _ => return,
            },
            glutin::event::Event::NewEvents(cause) => match cause {
                glutin::event::StartCause::ResumeTimeReached { .. } => (),
                glutin::event::StartCause::Init => (),
                _ => return,
            },
            _ => return,
        }
//        if mouse{
//		    display.gl_window().window().set_cursor_grab(true);
//		    display.gl_window().window().set_cursor_visible(false);
//		    display.gl_window().window().set_cursor_position(glutin::dpi::Position::Physical(glutin::dpi::PhysicalPosition{ x: 0i32, y: 0i32}));
//		} else{
//		    display.gl_window().window().set_cursor_grab(false);
//		    display.gl_window().window().set_cursor_visible(true);
//		}

        let mut target = display.draw();
        target.clear_color_and_depth((0.0, 0.0, 1.0, 1.0), 1.0);

        let view = {
        	let mut direction = [0f32, 0f32, 0f32];
        	
        	direction[0] = cam.yaw.to_radians().cos() * cam.pitch.to_radians().cos();//x
        	direction[1] = cam.pitch.to_radians().sin();				   //y
        	direction[2] = cam.yaw.to_radians().sin() * cam.pitch.to_radians().cos();//z
        	
//        	println!("{:?}", direction);
        	math::view_matrix(&cam.pos, &direction, &[0.0, 1.0, 0.0])
        };
		
		cam.view_matrix = view;
		
        let perspective = {
            let (width, height) = target.get_dimensions();
            let aspect_ratio = height as f32 / width as f32;

            let fov: f32 = 3.141592 / 3.0;
            let zfar = 1024.0;
            let znear = 0.001;

            let f = 1.0 / (fov / 2.0).tan();

            [
                [f *   aspect_ratio   ,    0.0,              0.0              ,   0.0],
                [         0.0         ,     f ,              0.0              ,   0.0],
                [         0.0         ,    0.0,  (zfar+znear)/(zfar-znear)    ,   1.0],
                [         0.0         ,    0.0, -(2.0*zfar*znear)/(zfar-znear),   0.0],
            ]
        };

        let light = [-1.0, 0.4, 0.9f32];

        let params = glium::DrawParameters {
            depth: glium::Depth {
                test: glium::draw_parameters::DepthTest::IfLess,
                write: true,
                .. Default::default()
            },
//			backface_culling: glium::draw_parameters::BackfaceCullingMode::CullClockwise,
            .. Default::default()
        };
        
        target.draw(&mesh.v_buffer, &mesh.i_buffer, &mesh.material.shaders,
                    &uniform! { model: mesh.model, view: view, perspective: perspective, u_light: light, 
                    tex: glium::uniforms::Sampler(&mesh.material.texture, mesh.material.behavior) },
//                    tex: &texture },
                    &mesh.material.draw_parameters).unwrap();
        target.finish().unwrap();
    });
}
