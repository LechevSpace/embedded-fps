use embedded_fps::{StdClock, FPS};
use embedded_graphics::{
    draw_target::DrawTarget,
    mono_font::{ascii::FONT_10X20, MonoTextStyle},
    pixelcolor::BinaryColor,
    prelude::{Point, Size},
    text::Text,
    Drawable,
};
use embedded_graphics_simulator::{
    OutputSettingsBuilder, SimulatorDisplay, SimulatorEvent, Window, sdl2::Keycode,
};

pub const DISPLAY_360P: Size = Size::new(480, 360);

/// Used to record the window for the GIF in the docs.
///
/// - use the `C` key to start the Iterations & FPS counting
fn main() -> Result<(), core::convert::Infallible> {
    let mut display = SimulatorDisplay::<BinaryColor>::new(DISPLAY_360P);

    let output_settings = OutputSettingsBuilder::new().scale(1).build();
    let mut window = Window::new("FPS using embedded-graphics & simulator", &output_settings);

    // starts the StdClock
    // `200` MAX_FPS is more than enough since `SimulatorDisplay`
    // doesn't reach more than `15` FPS when using `BinaryColor`.
    let mut fps_counter = FPS::<200, StdClock>::default();
    // create an initial value for FPS
    let mut fps = 0;

    let mut c: bool = false;
    
    let mut iteration = 0;

    'running: loop {
        display.clear(BinaryColor::Off)?;

        let character_style = MonoTextStyle::new(&FONT_10X20, BinaryColor::On);
        let fps_position = Point::new(20, 30);

        Text::new(&format!("FPS: {fps}"), fps_position, character_style).draw(&mut display)?;
        
        Text::new(&format!("Iteration: {iteration}"), fps_position + Point::new(0, 30), character_style).draw(&mut display)?;
        
        window.update(&display);

        if window.events().any(|e| e == SimulatorEvent::Quit) {
            break 'running Ok(());
        }

        // Should we start the iterations?
        if !c {
            'c: loop {
                std::thread::sleep(std::time::Duration::from_secs(2));

                if window.events().any(|e| match e {
                     SimulatorEvent::KeyDown { keycode: Keycode::C, .. } => {
                        true
                    },
                    _ => false,
                }) {
                    c = true;
                    break 'c;
                }
            }
        }

        iteration += 1;
        // tick and update the FPS at the end of the loop
        fps = fps_counter.tick();
        std::thread::sleep(std::time::Duration::from_millis(25));
    }
}
