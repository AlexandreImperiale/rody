/// Data structure for regular time line.
pub struct RegularTimeLine {
    /// Associated time line minimal time.
    pub max_time: f64,
    /// Associated time step.
    pub time_step: f64,
    /// Associated current time.
    pub current_time: f64,
}

impl RegularTimeLine {
    /// Creating new regular time line. If the provided minimal time is greater than the
    /// provided maximal time, the time step of the time line is set to zero.
    ///
    /// * `min` - minimal time value of the time line.
    /// * `max` - maximal time value reached by the time line.
    /// * `nstep` - number of time step in time line.
    ///
    /// # Examples
    /// ```
    /// use rody::timeline::*;
    ///
    /// let tl = RegularTimeLine::new(0.0, 1.0, 10);
    /// assert!((tl.current_time).abs() < 1e-10);
    /// assert!((tl.time_step - 0.1).abs() < 1e-10);
    /// assert!((tl.max_time - 1.0).abs() < 1e-10);
    /// ```
    ///
    /// ```
    /// use rody::timeline::*;
    ///
    /// let tl = RegularTimeLine::new(1.0, 0.0, 10);
    /// assert!((tl.time_step).abs() < 1e-10);
    /// ```
    pub fn new(min: f64, max: f64, nstep: usize) -> RegularTimeLine
    {
        let mut dt : f64 = 0.;
        if min < max {
            dt = (max - min) / (nstep as f64)
        }
        RegularTimeLine{ max_time: max, time_step: dt, current_time: min }
    }
}

impl Iterator for RegularTimeLine {
    /// Return type of iterator.
    type Item = f64;

    /// Function used for iterating over regular time line instances.
    ///
    /// # Examples
    /// ```
    /// use rody::timeline::*;
    ///
    /// for (i, time) in RegularTimeLine::new(0., 1.0, 10).enumerate() {
    ///    assert!((time - (i as f64) * 0.1).abs() < 1e-10);
    /// }
    /// ```
    ///
    /// ```
    /// use rody::timeline::*;
    ///
    /// for (i, time) in RegularTimeLine::new(1.0, 0.0, 10).enumerate() {
    ///     assert!(false)
    /// }
    /// ```
    fn next(&mut self) -> Option<f64> {
        if self.current_time < self.max_time {
            let time = self.current_time;
            self.current_time += self.time_step;
            return Some(time);
        }
        None
    }
}
