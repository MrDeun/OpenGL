use beryllium::*;
use ogl33::*;

type Vertex = [f32; 3];

fn main() {
    let sdl = Sdl::init(init::InitFlags::EVERYTHING);
    sdl.set_gl_context_major_version(3).unwrap();
    sdl.set_gl_context_minor_version(3).unwrap();
    sdl.set_gl_profile(video::GlProfile::Core).unwrap();

    let win_args = video::CreateWinArgs {
        title: "Hello this cruel world!",
        width: 800,
        height: 600,
        allow_high_dpi: true,
        borderless: false,
        resizable: false,
    };

    let _win = sdl
        .create_gl_window(win_args)
        .expect("Failed to create window damnit!");

    unsafe {
        glClearColor(1.0f);
    }

    'mainloop: loop {
        while let Some(event) = sdl.poll_events() {
            match event {
                (events::Event::Quit, _) => break 'mainloop,
                _ => (),
            }
        }
    }

    println!("Hello, world!");
}
