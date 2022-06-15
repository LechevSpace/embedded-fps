use embedded_time::{duration::Seconds, Clock, Instant};
use heapless::Deque;



/// Measures Frames Per Second (FPS).
///
/// `MAX_FPS` - Defines the maximum FPS that you expect to measure.
#[derive(Debug, Clone)]
pub struct FPS<const MAX_FPS: usize, C: Clock> {
    /// The last registered frames.
    last_second_frames: Deque<Instant<C>, MAX_FPS>,
    /// The embedded [`Clock`] that will be used to track the passed second.
    clock: C,
}

impl<const MAX_FPS: usize, C: Clock> FPS<MAX_FPS, C> {
    /// Creates a new Frames Per Second counter.
    pub fn new(clock: C) -> FPS<MAX_FPS, C> {
        FPS {
            last_second_frames: Deque::<_, MAX_FPS>::new(),
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
            .map_or(false, |tick| tick < a_second_ago)
        {
            self.last_second_frames.pop_front();
        }

        self.last_second_frames
            .push_back(now)
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

/// The errors that [`FPS`] can return.
///
/// Keep in mind that [`Error::MaxFPS`] will trigger panic on [`FPS::tick`]
/// or be returned as an error on [`FPS::try_tick`].
#[derive(Debug)]
pub enum Error {
    /// The clock returned an error when calling [`Clock::try_now`].
    Clock(embedded_time::clock::Error),
    /// The maximum reading of Frames per second was reached.
    /// The internal deque reached it's capacity.
    ///
    /// Increase the `MAX_FPS` to avoid this problem.
    MaxFPS(usize),
}