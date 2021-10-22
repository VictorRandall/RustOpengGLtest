extern crate sdl2;
extern crate gl;

fn main() {
    println!("Hello, world!");
    
    //creating a window
    let sdl = sdl2::init().unwrap();
    let video_subsystem = sdl.video().unwrap();
    
    let gl_attr = video_subsystem.gl_attr();
    
    gl_attr.set_context_profile(sdl2::video::GLProfile::Core);
    gl_attr.set_context_version(4,5);
    
    let window = video_subsystem
    	.window("RustLang Game", 900,700)
    	.opengl()
    	.resizable()
    	.build()
    	.unwrap();
    let gl_context = window.gl_create_context().unwrap();
    let gl = 
		gl::load_with(|s| video_subsystem.gl_get_proc_address(s) as *const std::os::raw::c_void);
	
	unsafe {
		gl::Viewport(0, 0, 900, 700);
		gl::ClearColor(0.5, 0.3, 0.5, 1.0);
	}
    
    //events
    let mut event_pump = sdl.event_pump().unwrap();
    
    'main: loop {
        for event in event_pump.poll_iter() {
            match event {
                sdl2::event::Event::Quit {..} => break 'main,
                _ => {},
            }
        }
    	
    	// rendering stuff to the window
    	unsafe {
    		gl::Clear(gl::COLOR_BUFFER_BIT);
		}
		
		window.gl_swap_window();
    }
}

fn shader_from_source(
	source: &str,
	kind: gl::types::GLuint
) -> Result<gl::types::GLuint, String> {
	let id = unsafe { gl::CreateShader(gl::VERTEX_SHADER) };
}