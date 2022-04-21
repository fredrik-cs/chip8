extern crate rand;
extern crate fps_clock;
extern crate pixels;
extern crate winit;
extern crate winit_input_helper;

mod chip8;
use chip8::Chip;
use std::path::Path;
use std::fs::File;
use std::env;
use rand::prelude::*;
use pixels::*;

use winit::dpi::LogicalSize;
use winit::event::{Event, VirtualKeyCode};
use winit::event_loop::{ControlFlow, EventLoop};
use winit::window::WindowBuilder;
use winit_input_helper::WinitInputHelper;
use log::error;
use pixels::{Error, Pixels, SurfaceTexture};
use settings::{WIDTH, HEIGHT};

mod fontset;
mod graphics;
mod settings;
mod constants;

fn main(){
    // Make the chip8 backend
    let mut chip = Chip::initialize();

    //Load file into chip8
    let path = Path::new("tetris.ch8");
    let file_result = File::open(path);
    assert_eq!(file_result.is_ok(), true);
    let mut file = file_result.unwrap();
    chip.load_program(file);

    //Make 60fps clock
    let mut fps = fps_clock::FpsClock::new(60);

    env_logger::init();
    let event_loop = EventLoop::new();
    let mut input = WinitInputHelper::new();
    let window = {
        let size = LogicalSize::new(WIDTH as f64, HEIGHT as f64);
        WindowBuilder::new()
            .with_title("Hello Pixels")
            .with_inner_size(size)
            .with_min_inner_size(size)
            .build(&event_loop)
            .unwrap()
    };

    let mut pixel_result = {
        let window_size = window.inner_size();
        let surface_texture = SurfaceTexture::new(window_size.width, window_size.height, &window);
        Pixels::new(WIDTH, HEIGHT, surface_texture)
    };

    let mut pixels = pixel_result.unwrap();

    event_loop.run(move |event, _, control_flow| {
        // Draw the current frame
        if let Event::RedrawRequested(_) = event {
            chip.emulate_cycle();
            chip.draw(pixels.get_frame());
            if pixels.render().map_err(|e| error!("pixels.render() failed: {}", e)).is_err()
            {
                *control_flow = ControlFlow::Exit;
                return;
            }
        }

        // Handle input events
        if input.update(&event) {
            // Close events
            if input.key_pressed(VirtualKeyCode::Escape) || input.quit() {
                *control_flow = ControlFlow::Exit;
                return;
            }

            // Resize the window
            if let Some(size) = input.window_resized() {
                pixels.resize_surface(size.width, size.height);
            }

            // Update internal state and request a redraw
            window.request_redraw();
        }
    });


    // loop{
    //     chip.emulate_cycle();
    //
    //     //if chip.draw_flag { window.render_graphics(); }
    //
    //     chip.set_keys();
    //     fps.tick();
    // }
}
