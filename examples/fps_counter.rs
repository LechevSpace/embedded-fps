use embedded_fps::{StdClock, FPS};
use std::{thread::sleep, time::Duration};

fn main() {
    let std_clock = StdClock::default();
    let mut fps_counter = FPS::<10, _>::new(std_clock);

    for _ in 0..30 {
        // sleep for 125 milliseconds
        // this will give us 8 FPS
        sleep(Duration::from_millis(125));

        let fps = fps_counter.tick();
        println!("Frames per second: {fps}")
    }
}
