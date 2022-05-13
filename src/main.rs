extern crate rand;
extern crate fps_clock;
extern crate pixels;
extern crate winit;
extern crate winit_input_helper;
extern crate game_loop;

mod chip8;
use chip8::Chip;
use std::path::Path;
use std::fs::File;

use winit::dpi::LogicalSize;
use winit::event::{Event, VirtualKeyCode};
use winit::event_loop::{ControlFlow, EventLoop};
use winit::window::WindowBuilder;
use winit_input_helper::WinitInputHelper;
use log::error;
use pixels::{Pixels, SurfaceTexture};
use settings::{WIDTH, HEIGHT};
use constants::{NO_KEY_PRESSED, KEY_SET, GAME_NAME};
use game_loop::{game_loop, Time, TimeTrait};
use std::time::Duration;


mod fontset;
mod graphics;
mod settings;
mod constants;

fn main(){

    //Load file
    let path = Path::new(GAME_NAME);
    let file_result = File::open(path);
    assert_eq!(file_result.is_ok(), true);
    let file = file_result.unwrap();

    env_logger::init();
    let event_loop = EventLoop::new();
    let mut input = WinitInputHelper::new();
    let window = {
        let size = LogicalSize::new((WIDTH * 10) as f64, (HEIGHT * 10) as f64);
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
    pixels.resize_surface(640, 320);
    let mut requested_key = NO_KEY_PRESSED;
    let mut fps = fps_clock::FpsClock::new(60);

    let mut chip = Chip::initialize(pixels);
    chip.load_program(file);

    const FPS : u32 = 240;
    const TIME_STEP: Duration = Duration::from_nanos(1_000_000_000 / FPS as u64);

    game_loop(
        event_loop,
        window,
        chip,
        FPS,
        0.1,
        move |g| {
            if g.game.requested_key == NO_KEY_PRESSED {
                g.game.emulate_cycle();
            }
            else { println!{"reqested in emulate {}", g.game.requested_key}}
        },
        move |g| {
            // Drawing
            if g.game.draw_flag {
                g.game.draw();
            }

            // Sleep the main thread to limit drawing to the fixed time step.
            // let dt = TIME_STEP.as_secs_f64() - Time::now().sub(&g.current_instant());
            // if dt > 0.0 {
            //     std::thread::sleep(Duration::from_secs_f64(dt));
            // }
        },
        move |g, event| {
            if input.update(&event) {
                if input.key_pressed(VirtualKeyCode::Escape) || input.quit() {
                    g.exit();
                    return;
                }

                for i in 0..16 {
                    if input.key_pressed(KEY_SET[i]) {
                        g.game.set_key(i, 1);
                    }
                    else if input.key_released(KEY_SET[i]) {
                        //println!{"requested in input {}", g.game.requested_key}
                        //println!{"i in input {}", i}
                        g.game.set_key(i, 0);
                        // if g.game.requested_key == i as u8 {
                        //     g.game.requested_key = NO_KEY_PRESSED;
                        //     g.game.pc += 2;
                        // }
                    }
                }

                // Resize the window
                if let Some(size) = input.window_resized() {
                    g.game.pixels.resize_surface(size.width, size.height);
                }
            }
        }
    );


    // event_loop.run(move |event, _, control_flow| {
    //
    //     requested_key = chip.emulate_cycle();
    //
    //     // Draw the current frame
    //     if let Event::RedrawRequested(_) = event {
    //         chip.draw(pixels.get_frame());
    //         if pixels.render().map_err(|e| error!("pixels.render() failed: {}", e)).is_err()
    //         {
    //             *control_flow = ControlFlow::Exit;
    //             return;
    //         }
    //     }
    //
    //     // Handle input events
    //     if input.update(&event) {
    //         // Close events
    //         if input.key_pressed(VirtualKeyCode::Escape) || input.quit() {
    //             *control_flow = ControlFlow::Exit;
    //             return;
    //         }
    //
    //         for i in 0..16 {
    //             if input.key_pressed(KEY_SET[i]) {
    //                 chip.set_key(i, 1);
    //             }
    //             if input.key_released(KEY_SET[i]) {
    //                 chip.set_key(i, 0);
    //                 if requested_key == i as u8 {
    //                     requested_key = NO_KEY_PRESSED;
    //                 }
    //             }
    //         }
    //
    //         // Resize the window
    //         if let Some(size) = input.window_resized() {
    //             pixels.resize_surface(size.width, size.height);
    //         }
    //
    //
    //
    //     }
    //     // Request a redraw
    //     if chip.draw_flag {
    //         window.request_redraw();
    //     }
    //
    //     fps.tick();
    // });

}
