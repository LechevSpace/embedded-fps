//! [![sponsor-us]](https://github.com/sponsors/LechevSpace)&ensp;[![github]](https://github.com/LechevSpace/embedded-fps)&ensp;[![crates-io]](https://crates.io/crates/embedded-fps)
//!
//! Frames Per Second counter for embedded devices.
//!
//! Create an [`FPS`] struct by passing the `MAX_FPS` (maximum frames per seconds)
//! that you expect to hit and a [`embedded_time::Clock`] implementation.
//!
//! # Examples
//!
//! ## Frames Per Second with a simple for-loop
//!
//! You can also run this example from the `examples` directory using:
//!
//! `cargo run --features=std --example fps_counter`
//!
//! ```
//! use embedded_fps::{FPS, StdClock};
//! use std::{thread::sleep, time::Duration};
//!
//! let std_clock = StdClock::default();
//! let mut fps_counter = FPS::<10, _>::new(std_clock);
//!
//! for i in 0..20 {
//!     // sleep for 125 milliseconds
//!     // this will give us 8 FPS
//!     sleep(Duration::from_millis(125));
//!
//!     let fps = fps_counter.tick();
//!     println!("Frames per second: {fps}")
//! }
//! ```
//!
//! ## Frames Per Second with `embedded-graphics`
//!
//! This crate is suitable for usage with the [`embedded-graphics`] crate
//! when you want to know, log or even show the frames per second of a
//! display with an embedded device.
//!
//! Note: This example requires [`embedded-graphics-simulator`] and `SDL2` installed
//! on your machine.
//!
//! Refer to the [`embedded-graphics-simulator` documentation][simulator-docs]
//! for detailed instructions.
//!
//! You can also run this example from the `examples` directory using:
//!
//! `cargo run --features=std --example fps_counter`
//!
//! ```no_run
//! use embedded_fps::{StdClock, FPS};
//! use embedded_graphics::{
//!     draw_target::DrawTarget,
//!     mono_font::{ascii::FONT_10X20, MonoTextStyle},
//!     pixelcolor::BinaryColor,
//!     prelude::{Point, Size},
//!     text::Text,
//!     Drawable,
//! };
//! use embedded_graphics_simulator::{
//!     OutputSettingsBuilder, SimulatorDisplay, SimulatorEvent, Window,
//! };
//!
//! pub const DISPLAY_360P: Size = Size::new(480, 360);
//!
//! fn main() -> Result<(), core::convert::Infallible> {
//!     let mut display = SimulatorDisplay::<BinaryColor>::new(DISPLAY_360P);
//!
//!     let output_settings = OutputSettingsBuilder::new().scale(1).build();
//!     let mut window = Window::new("FPS using embedded-graphics & simulator", &output_settings);
//!
//!     // starts the StdClock
//!     // `200` MAX_FPS is more than enough since `SimulatorDisplay`
//!     // doesn't reach more than `15` FPS when using `BinaryColor`.
//!     let mut fps_counter = FPS::<200, StdClock>::default();
//!     // create an initial value for FPS
//!     let mut fps = 0;
//!
//!     'running: loop {
//!         display.clear(BinaryColor::Off)?;
//!
//!         let character_style = MonoTextStyle::new(&FONT_10X20, BinaryColor::On);
//!         let fps_position = Point::new(20, 30);
//!
//!         Text::new(&format!("FPS: {fps}"), fps_position, character_style).draw(&mut display)?;
//!
//!         window.update(&display);
//!
//!         if window.events().any(|e| e == SimulatorEvent::Quit) {
//!             break 'running Ok(());
//!         }
//!
//!         // tick and update the FPS at the end of the loop
//!         fps = fps_counter.tick();
//!     }
//! }
//! ```
//!
//! # Crate features
//!
//! - `std` - enables [`StdClock`] - a [`Clock`] implementation using [`std`] for usage on a host machine.
//!
//! [`StdClock`]: crate::StdClock
//! [`embedded-graphics`]: https://crates.io/crates/embedded-graphics
//! [`embedded-graphics-simulator`]: https://crates.io/crates/embedded-graphics-simulator
//! [simulator-docs]: https://docs.rs/embedded-graphics-simulator/latest/embedded_graphics_simulator/#setup
//! [github]: https://img.shields.io/badge/github-3873AD?style=for-the-badge&labelColor=555555&logo=github
//! [crates-io]: https://img.shields.io/crates/v/embedded-fps?logo=rust&style=for-the-badge
//! [sponsor-us]: https://img.shields.io/github/sponsors/LechevSpace?color=bf3989&label=Sponsor%20us&style=for-the-badge&logoColor=bf3989&logo=data%3Aimage%2Fsvg%2Bxml%3Bbase64%2CPHN2ZyBoZWlnaHQ9IjE2IiB2aWV3Qm94PSIwIDAgMTYgMTYiIHZlcnNpb249IjEuMSIgd2lkdGg9IjE2IiB4bWxucz0iaHR0cDovL3d3dy53My5vcmcvMjAwMC9zdmciPgogICAgPHBhdGggZmlsbD0iI2JmMzk4OSIgZmlsbC1ydWxlPSJldmVub2RkIiBkPSJNNC4yNSAyLjVjLTEuMzM2IDAtMi43NSAxLjE2NC0yLjc1IDMgMCAyLjE1IDEuNTggNC4xNDQgMy4zNjUgNS42ODJBMjAuNTY1IDIwLjU2NSAwIDAwOCAxMy4zOTNhMjAuNTYxIDIwLjU2MSAwIDAwMy4xMzUtMi4yMTFDMTIuOTIgOS42NDQgMTQuNSA3LjY1IDE0LjUgNS41YzAtMS44MzYtMS40MTQtMy0yLjc1LTMtMS4zNzMgMC0yLjYwOS45ODYtMy4wMjkgMi40NTZhLjc1Ljc1IDAgMDEtMS40NDIgMEM2Ljg1OSAzLjQ4NiA1LjYyMyAyLjUgNC4yNSAyLjV6TTggMTQuMjVsLS4zNDUuNjY2LS4wMDItLjAwMS0uMDA2LS4wMDMtLjAxOC0uMDFhNy42NDMgNy42NDMgMCAwMS0uMzEtLjE3IDIyLjA3NSAyMi4wNzUgMCAwMS0zLjQzNC0yLjQxNEMyLjA0NSAxMC43MzEgMCA4LjM1IDAgNS41IDAgMi44MzYgMi4wODYgMSA0LjI1IDEgNS43OTcgMSA3LjE1MyAxLjgwMiA4IDMuMDIgOC44NDcgMS44MDIgMTAuMjAzIDEgMTEuNzUgMSAxMy45MTQgMSAxNiAyLjgzNiAxNiA1LjVjMCAyLjg1LTIuMDQ1IDUuMjMxLTMuODg1IDYuODE4YTIyLjA4IDIyLjA4IDAgMDEtMy43NDQgMi41ODRsLS4wMTguMDEtLjAwNi4wMDNoLS4wMDJMOCAxNC4yNXptMCAwbC4zNDUuNjY2YS43NTIuNzUyIDAgMDEtLjY5IDBMOCAxNC4yNXoiPjwvcGF0aD4KPC9zdmc%2BCg%3D%3D

// Rustdoc lints
#![deny(rustdoc::broken_intra_doc_links)]
// Rustc lints
#![deny(missing_docs, unused_imports)]
// adds `#[no_std]` attribute if the `std` feature is not enabled
#![cfg_attr(not(feature = "std"), no_std)]

use embedded_time::{duration::Seconds, Clock, Instant};
use holodeque::ArrayDeque;

#[cfg(feature = "std")]
#[doc(inline)]
pub use std_clock::StdClock;

/// Measures Frames Per Second (FPS).
///
/// `MAX_FPS` - Defines the maximum FPS that you expect to measure.
#[derive(Debug, Clone)]
pub struct FPS<const MAX_FPS: usize, C: Clock> {
    /// The last registered frames.
    last_second_frames: ArrayDeque<Option<Instant<C>>, MAX_FPS>,
    /// The embedded [`Clock`] that will be used to track the passed second.
    clock: C,
}

impl<const MAX_FPS: usize, C: Clock> FPS<MAX_FPS, C> {
    /// Creates a new Frames Per Second counter.
    pub fn new(clock: C) -> FPS<MAX_FPS, C> {
        FPS {
            last_second_frames: ArrayDeque::<_, MAX_FPS>::new(),
            clock,
        }
    }

    /// Adds another frame tick and returns the current Frames Pre Second.
    ///
    /// # Panics
    ///
    /// When [`Clock::try_now()`] returns an error or if the `MAX_FPS` is reached.
    pub fn tick(&mut self) -> usize {
        self.try_tick().unwrap()
    }

    /// Adds another frame tick and returns the current Frames Pre Second.
    ///
    /// This method will not panic if the `MAX_FPS` is reached,
    /// instead it will just return the `MAX_FPS` value (capping it in a nutshell).
    ///
    /// # Panics
    ///
    /// If [`Clock::try_now()`] returns an error.
    pub fn tick_max(&mut self) -> usize {
        self.try_tick_max().unwrap()
    }

    /// Adds another frame tick and returns the current Frames Pre Second.
    ///
    /// This method will not return an error if the `MAX_FPS` is reached,
    /// instead it will just return the `MAX_FPS` value (capping it in a nutshell).
    pub fn try_tick_max(&mut self) -> Result<usize, Error> {
        match self.try_tick() {
            Ok(fps) => Ok(fps),
            Err(Error::MaxFPS(_)) => Ok(MAX_FPS),
            Err(err) => Err(err),
        }
    }

    /// Adds another frame tick and returns the current Frames Pre Second.
    ///
    /// # Panics
    ///
    /// When [`Clock::try_now()`] returns an error or if the `MAX_FPS` is reached.
    pub fn try_tick(&mut self) -> Result<usize, Error> {
        let now = self.clock.try_now().map_err(Error::Clock)?;
        let a_second_ago = now - Seconds(1);

        while self
            .last_second_frames
            .front()
            .copied()
            .flatten()
            .map_or(false, |tick| tick < a_second_ago)
        {
            self.last_second_frames.pop_front();
        }

        self.last_second_frames
            .push_back(Some(now))
            .map_err(|_cap_err| Error::MaxFPS(MAX_FPS))?;

        // return the frames per second
        Ok(self.last_second_frames.len())
    }
}

impl<const MAX_FPS: usize, C> Default for FPS<MAX_FPS, C>
where
    C: Clock + Default,
{
    fn default() -> Self {
        Self::new(C::default())
    }
}

#[derive(Debug)]
/// The errors that [`FPS`] can return.
///
/// Keep in mind that [`Error::MaxFPS`] will trigger panic on [`FPS::tick`]
/// or be returned as an error on [`FPS::try_tick`].
pub enum Error {
    /// The clock returned an error when calling [`Clock::try_now`].
    Clock(embedded_time::clock::Error),
    /// The maximum reading of Frames per second was reached
    /// The internal deque reached it's capacity.
    ///
    /// Increase the `MAX_FPS` to avoid this problem.
    MaxFPS(usize),
}

#[cfg(feature = "std")]
mod std_clock {
    use std::time::Instant as StdInstant;

    use embedded_time::{clock::Error, rate::Fraction, Clock, Instant as EmbeddedInstant};

    /// A Standard clock based on [`std`].
    ///
    /// It takes the [`Instant::elapsed()`] time and uses nanoseconds converted to [`u64`].
    /// This still leaves us with ~594 years of representable time
    ///
    /// [`Instant::elapsed()`]: std::time::Instant::elapsed()
    #[derive(Debug, Clone, Copy)]
    pub struct StdClock(StdInstant);

    impl Default for StdClock {
        fn default() -> Self {
            Self::new()
        }
    }

    impl StdClock {
        /// Creates a new [`StdClock`].
        /// Internally it calls [`Instant::now()`].
        ///
        /// [`Instant::now()`]: std::time::Instant::now()
        pub fn new() -> Self {
            Self(StdInstant::now())
        }
    }

    impl Clock for StdClock {
        type T = u64;

        const SCALING_FACTOR: Fraction = Fraction::new(1, 1_000_000_000);

        fn try_now(&self) -> Result<EmbeddedInstant<Self>, Error> {
            // discarding the upper u64 still leaves us with ~594 years of representable time
            Ok(EmbeddedInstant::new(self.0.elapsed().as_nanos() as u64))
        }
    }

    #[cfg(test)]
    mod tests {
        use std::thread::sleep;

        use embedded_time::{
            duration::{Extensions, Milliseconds},
            Clock,
        };

        use super::StdClock;

        #[test]
        fn it_creates_std_instant_from_milliseconds_clock() {
            let clock = StdClock::new();

            sleep(std::time::Duration::from_millis(400));

            let start = clock.try_now().unwrap();
            // wait 1.5 seconds
            sleep(std::time::Duration::from_millis(1_600));
            let end = clock.try_now().unwrap();

            let elapsed = Milliseconds::<u64>::try_from(end - start).unwrap();

            let lower_bound = Milliseconds::<u64>::try_from(1_599_u32.milliseconds()).unwrap();
            assert!(elapsed > lower_bound);

            let upper_bound = Milliseconds::<u64>::try_from(2_000_u32.milliseconds()).unwrap();
            assert!(elapsed < upper_bound);
        }
    }
}
