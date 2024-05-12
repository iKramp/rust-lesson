fn main() {
    println!("Hello, world!");

    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();
    let window = video_subsystem
        .window("test_window", 1280, 720)
        .position_centered()
        .build()
        .unwrap();
    let mut canvas = window.into_canvas().build().unwrap();
    let mut event_pump = sdl_context.event_pump().unwrap(); //code above inits sdl2 somehow, idk what it does

    'main: loop {
        for event in event_pump.poll_iter() {
            if let sdl2::event::Event::Quit { .. } = event {
                break 'main;
            }
        }

        canvas.set_draw_color(sdl2::pixels::Color::RGB(0, 0, 0));
        canvas.clear();
        canvas.set_draw_color(sdl2::pixels::Color::RGB(100, 100, 100));
        let draw_rect = sdl2::rect::Rect::new(0, 0, 20, 20);
        canvas.fill_rect(draw_rect);

        canvas.present();
    }
}
