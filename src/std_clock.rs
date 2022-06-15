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