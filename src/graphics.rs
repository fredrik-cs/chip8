// use pixel_canvas::{Canvas, Color};
// use pixel_canvas::input::MouseState;
// use pixel_canvas::input::glutin::event::WindowEvent::MouseInput;
//
// pub struct Window{
//    pub canvas: Canvas<MouseState>,
//    pub buffer: [u8; 64 * 32]
// }
//
// impl Window {
//     pub fn setup_graphics(gfx: [u8; 64*32]) -> Window {
//         let window = Window {
//             canvas:  Canvas::new(64, 32)
//                 .title("Tetris")
//                 .state(MouseState::new())
//                 .render_on_change(true),
//             buffer: gfx
//         };
//         window
//     }
//
//     pub fn render_graphics(self) {
//         self.canvas.state(MouseState::new());
//     }
//
//     pub fn start_render(mut self){
//         // self.canvas.render(|mouse, image| {
//         //     let buffer = self.buffer.clone();
//         //     let width = image.width() as usize;
//         //     for (y, row) in image.chunks_mut(width).enumerate() {
//         //         for (x, pixel) in row.iter_mut().enumerate() {
//         //             *pixel = Color {
//         //                 r: buffer[x + y * width] * 255,
//         //                 g: buffer[x + y * width] * 255,
//         //                 b: buffer[x + y * width] * 255,
//         //             }
//         //         }
//         //     }
//         // });
//        ()
//     }
// }
//
//
