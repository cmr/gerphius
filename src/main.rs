#![feature(phase, macro_rules, trace_macros)]
#![crate_id = "gerphius"]
#![crate_type = "bin"]

#[phase(syntax, link)]
extern crate log;
extern crate libc;
extern crate native;
extern crate collections;

extern crate gl;
extern crate hgl;
extern crate png;
extern crate glfw;
extern crate ears;
extern crate noise;
extern crate cgmath;

use game::Game;
use glfw::Context;

macro_rules! debug_assert (
    ($ex:tt) => (
        if cfg!(not(ndebug)) {
            assert!($ex)
        }
    )
)

mod game;
mod render;
mod physics;

fn main() {
    let glfw = glfw::init(glfw::FAIL_ON_ERRORS).unwrap();

    // don't want to handle resizing logic
    glfw.window_hint(glfw::Resizable(false));
    glfw.window_hint(glfw::ContextVersion(3, 2));
    glfw.window_hint(glfw::OpenglProfile(glfw::OpenGlCoreProfile));

    // opengl 3.2 core profile

    let (window, events) = glfw.create_window(400, 400, "Gerphius", glfw::Windowed)
                               .expect("Error: could not open a window!");

    // we want every event
    window.set_all_polling(true);

    window.make_current();

    gl::load_with(|s| glfw.get_proc_address(s));

    let mut game = Game::new(400, 400);

    while !window.should_close() {
        glfw.poll_events();

        for event in glfw::flush_messages(&events) {
            game.handle_event(&window, event);
        }

        game.tick();
        game.render();

        window.swap_buffers();
    }
}
