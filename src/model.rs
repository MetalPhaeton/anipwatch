use crate::{
    error::Error,
    state::*
};

#[derive(Debug, Clone, PartialEq, Default)]
pub struct Model {
    save_data_buf: Vec<u8>
}

impl Model {
    pub fn new() -> Self {
        Self {
            save_data_buf: Vec::<u8>::new()
        }
    }

    fn update_stopwatch_skin_id(
        &self,
        state: &mut State
    ) {
        let skin_id = state.find_skin_id_from_stopwatch_events();

        if state.current_skin_id != skin_id {
            state.change_skin_request = true;
            state.current_skin_id = skin_id;
        }
    }

    fn update_clock_skin_id(
        &self,
        state: &mut State
    ) {
        let skin_id = state.find_skin_id_from_clock_events();

        if state.current_skin_id != skin_id {
            state.change_skin_request = true;
            state.current_skin_id = skin_id;
        }
    }

    #[inline]
    pub fn init(&mut self, state: &mut State) -> Result<(), Error> {
        state.clicked_btn = None;
        state.dt = 0.0;
        state.quit_request = false;
        state.change_skin_request = true;

        match state.watch_mode {
            WatchMode::Stopwatch(..) => {
                self.update_stopwatch_skin_id(state);
                Ok(())
            },

            WatchMode::Clock => {
                state.set_clock();

                self.update_clock_skin_id(state);

                Ok(())
            },
        }
    }

    #[inline]
    pub fn preproc(&mut self, state: &mut State) -> Result<(), Error> {
        state.clicked_btn = None;
        state.change_skin_request = false;

        Ok(())
    }

    #[inline]
    pub fn update(&mut self, state: &mut State) -> Result<(), Error> {
        state.change_skin_request = false;

        match state.watch_mode {
            WatchMode::Stopwatch(stopwatch_mode) =>
                self.update_stopwatch_mode(state, stopwatch_mode),

            WatchMode::Clock => self.update_clock_mode(state),
        }
    }

    #[inline]
    fn update_stopwatch_mode(
        &mut self,
        state: &mut State,
        stopwatch_mode: StopwatchMode
    ) -> Result<(), Error> {
        match stopwatch_mode {
            StopwatchMode::Running => 
                self.update_stopwatch_running_mode(state),

            StopwatchMode::Stopped => 
                self.update_stopwatch_stopped_mode(state)
        }
    }

    fn update_stopwatch_running_mode(
        &mut self,
        state: &mut State
    ) -> Result<(), Error> {
        match state.clicked_btn {
            Some(btn_mode) =>  match btn_mode {
                Button::Switch => {
                    state.set_clock();
                    self.update_clock_skin_id(state);

                    state.watch_mode = WatchMode::Clock;

                    Ok(())
                },

                Button::StartStop => {
                    self.update_stopwatch_skin_id(state);

                    state.watch_mode = WatchMode::Stopwatch(
                        StopwatchMode::Stopped
                    );

                    Ok(())
                },

                Button::Reset => {
                    state.current_stopwatch_time = Default::default();
                    state.saved_time = 0.0;

                    self.update_stopwatch_skin_id(state);

                    Ok(())
                },

                Button::Quit => {
                    self.quick_button_clicked(state)
                }
                
            },

            None => {
                state.add_stopwatch_time();
                self.update_stopwatch_skin_id(state);

                Ok(())
            }
        }
    }

    fn update_stopwatch_stopped_mode(
        &mut self,
        state: &mut State
    ) -> Result<(), Error> {
        match state.clicked_btn {
            Some(btn_mode) =>  match btn_mode {
                Button::Switch => {
                    state.set_clock();
                    self.update_clock_skin_id(state);

                    state.watch_mode = WatchMode::Clock;

                    Ok(())
                },

                Button::StartStop => {
                    self.update_stopwatch_skin_id(state);

                    state.watch_mode = WatchMode::Stopwatch(
                        StopwatchMode::Running
                    );

                    Ok(())
                },

                Button::Reset => {
                    state.current_stopwatch_time = Default::default();
                    state.saved_time = 0.0;

                    self.update_stopwatch_skin_id(state);

                    Ok(())
                },

                Button::Quit => self.quick_button_clicked(state)
            },

            None => {
                self.update_stopwatch_skin_id(state);

                Ok(())
            }
        }
    }

    fn update_clock_mode(&mut self, state: &mut State) -> Result<(), Error> {
        match state.clicked_btn {
            Some(btn_mode) =>  match btn_mode {
                Button::Switch => {
                    self.update_stopwatch_skin_id(state);

                    state.watch_mode = WatchMode::Stopwatch(
                        StopwatchMode::Stopped
                    );

                    Ok(())
                },

                Button::Quit => self.quick_button_clicked(state),

                _ => {
                    state.set_clock();

                    self.update_clock_skin_id(state);

                    Ok(())
                }
            }

            None => {
                state.set_clock();

                self.update_clock_skin_id(state);

                Ok(())
            }
        }
    }

    fn quick_button_clicked(
        &mut self,
        state: &mut State
    ) -> Result<(), Error> {
        state.quit_request = true;

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn clock_test_1() {
        let mut state = State::default();
        state.watch_mode = WatchMode::Clock;

        let mut model = Model::new();

        assert!(model.update(&mut state).is_ok());
        let mut stop_time = state.current_clock_time.clone();
        stop_time.seconds += 2;
        stop_time.normalize();

        assert!(state.current_clock_time < stop_time);

        loop {
            if state.current_clock_time >= stop_time {
                state.clicked_btn = Some(Button::Quit);
            }

            assert!(model.update(&mut state).is_ok());
            if state.quit_request {break;}
        }

        assert!(state.current_clock_time >= stop_time);
    }

    #[test]
    fn stopwatch_running_test_1() {
        let mut state = State::default();
        state.watch_mode = WatchMode::Stopwatch(StopwatchMode::Running);

        state.dt = 0.01;

        let mut model = Model::new();

        assert!(state.current_stopwatch_time.seconds < 1);

        loop {
            if state.current_stopwatch_time.seconds >= 1 {
                state.clicked_btn = Some(Button::Quit);
            }

            assert!(model.update(&mut state).is_ok());
            if state.quit_request {break;}
        }

        assert!(state.current_stopwatch_time.seconds >= 1);
    }

    #[test]
    fn stopwatch_stopped_test_1() {
        let mut state = State::default();
        state.watch_mode = WatchMode::Stopwatch(StopwatchMode::Stopped);

        state.dt = 0.01;
        let check = state.current_stopwatch_time.clone();

        let mut model = Model::new();

        let mut count: usize = 0;

        loop {
            if count >= 100 {
                state.clicked_btn = Some(Button::Quit);
            }

            assert!(model.update(&mut state).is_ok());
            if state.quit_request {break;}

            count += 1;
        }

        assert_eq!(state.current_stopwatch_time, check);
    }

    #[test]
    fn model_test_1() {
        let mut state = State::default();
        state.watch_mode = WatchMode::Clock;

        let mut model = Model::new();

        let check = WatchTime::default();
        assert_eq!(state.current_clock_time, check);

        assert!(model.update(&mut state).is_ok());
        assert_ne!(state.current_clock_time, check);

        state.clicked_btn = Some(Button::Switch);
        assert!(model.update(&mut state).is_ok());
        state.clicked_btn = None;

        assert_eq!(
            state.watch_mode,
            WatchMode::Stopwatch(StopwatchMode::Stopped)
        );
        assert_eq!(state.current_stopwatch_time, check);

        state.dt = 0.1;
        assert!(model.update(&mut state).is_ok());
        assert!(model.update(&mut state).is_ok());
        assert!(model.update(&mut state).is_ok());
        assert!(model.update(&mut state).is_ok());
        assert_eq!(state.current_stopwatch_time, check);

        state.clicked_btn = Some(Button::StartStop);
        assert!(model.update(&mut state).is_ok());
        state.clicked_btn = None;

        assert_eq!(
            state.watch_mode,
            WatchMode::Stopwatch(StopwatchMode::Running)
        );
        assert_eq!(state.current_stopwatch_time, check);

        assert!(model.update(&mut state).is_ok());
        assert!(model.update(&mut state).is_ok());
        assert!(model.update(&mut state).is_ok());
        assert!(model.update(&mut state).is_ok());
        assert!(state.current_stopwatch_time > check);
        let check_2 = state.current_stopwatch_time.clone();

        state.clicked_btn = Some(Button::Reset);
        assert!(model.update(&mut state).is_ok());
        state.clicked_btn = None;
        assert_eq!(state.current_stopwatch_time, check);

        assert!(model.update(&mut state).is_ok());
        assert!(model.update(&mut state).is_ok());
        assert!(model.update(&mut state).is_ok());
        assert!(model.update(&mut state).is_ok());
        assert!(state.current_stopwatch_time > check);
        assert_eq!(state.current_stopwatch_time, check_2);

        state.clicked_btn = Some(Button::StartStop);
        assert!(model.update(&mut state).is_ok());
        state.clicked_btn = None;

        assert!(model.update(&mut state).is_ok());
        assert!(model.update(&mut state).is_ok());
        assert!(model.update(&mut state).is_ok());
        assert!(model.update(&mut state).is_ok());
        assert_eq!(state.current_stopwatch_time, check_2);

        state.clicked_btn = Some(Button::Switch);
        assert!(model.update(&mut state).is_ok());
        state.clicked_btn = None;

        assert_eq!(
            state.watch_mode,
            WatchMode::Clock
        );

        assert!(model.update(&mut state).is_ok());
        assert_ne!(state.current_clock_time, check);
        assert_eq!(state.current_stopwatch_time, check_2);

        state.clicked_btn = Some(Button::StartStop);
        assert!(model.update(&mut state).is_ok());
        state.clicked_btn = None;

        assert_ne!(state.current_clock_time, check);

        state.clicked_btn = Some(Button::Reset);
        assert!(model.update(&mut state).is_ok());
        state.clicked_btn = None;

        assert_ne!(state.current_clock_time, check);
        assert_eq!(state.current_stopwatch_time, check_2);

        state.clicked_btn = Some(Button::Switch);
        assert!(model.update(&mut state).is_ok());
        state.clicked_btn = None;

        assert_eq!(
            state.watch_mode,
            WatchMode::Stopwatch(StopwatchMode::Stopped)
        );
        assert_eq!(state.current_stopwatch_time, check_2);

        assert!(!state.quit_request);

        state.clicked_btn = Some(Button::Quit);
        assert!(model.update(&mut state).is_ok());
        state.clicked_btn = None;

        assert!(state.quit_request);
    }
}
