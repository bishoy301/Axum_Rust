extern crate gl;
extern crate sdl2;

use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::video::GLProfile;

unsafe fn draw(r: f32, g: f32, b: f32, a: f32) {
    gl::ClearColor(r, g, b, a);
    gl::Clear(gl::COLOR_BUFFER_BIT);
}

fn main() {
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();

    let gl_attr = video_subsystem.gl_attr();
    gl_attr.set_context_profile(GLProfile::Core);
    gl_attr.set_context_version(3, 3);

    let window = video_subsystem
        .window("Axum", 800, 600)
        .opengl()
        .position_centered()
        .resizable()
        .maximized()
        .build()
        .unwrap();

    let _gl_context = window.gl_create_context().unwrap();
    gl::load_with(|name| video_subsystem.gl_get_proc_address(name) as *const _);

    let mut event_pump = sdl_context.event_pump().unwrap();

    'running: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => break 'running,
                Event::KeyDown {
                    keycode: Some(Keycode::R),
                    ..
                } => unsafe { draw(1.0, 0.0, 0.0, 1.0) },
                Event::KeyDown {
                    keycode: Some(Keycode::G),
                    ..
                } => unsafe { draw(0.0, 1.0, 0.0, 1.0) },
                Event::KeyDown {
                    keycode: Some(Keycode::B),
                    ..
                } => unsafe { draw(0.0, 0.0, 1.0, 1.0) },
                _ => {}
            }
            window.gl_swap_window();
        }
        ::std::thread::sleep(::std::time::Duration::new(0, 1_000_000_000u32 / 60));
    }
}
