use std::{
    cmp::{Ordering, PartialEq, Eq, PartialOrd, Ord},
    path::PathBuf
};

use chrono::{
    Local,
    Timelike
};

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum WatchMode {
    Stopwatch(StopwatchMode),
    Clock
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum StopwatchMode {
    Running,
    Stopped
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Button {
    Switch,
    StartStop,
    Reset,
    Quit
}

#[derive(Debug, Clone, PartialEq, Default)]
pub struct WatchTime {
    pub hours: u32,
    pub minutes: u32,
    pub seconds: u32,
    pub cents: u32
}

impl WatchTime {
    #[inline]
    pub fn to_u128(&self) -> u128 {
        const SHIFT_1: usize = std::mem::size_of::<u32>() * 8;
        const SHIFT_2: usize = SHIFT_1 * 2;
        const SHIFT_3: usize = SHIFT_1 * 3;

        ((self.hours as u128) << SHIFT_3)
            | ((self.minutes as u128) << SHIFT_2)
            | ((self.seconds as u128) << SHIFT_1)
            | (self.cents as u128)
    }

    #[inline]
    pub fn normalize(&mut self) {
        self.seconds += self.cents / 100;
        self.minutes += self.seconds / 60;
        self.hours += self.minutes / 60;

        self.cents %= 100;
        self.minutes %= 60;
        self.seconds %= 60;
        self.hours %= 100;
    }
}

impl Eq for WatchTime {}

impl PartialOrd for WatchTime {
    #[inline]
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.to_u128().partial_cmp(&other.to_u128())
    }
}

impl Ord for WatchTime {
    #[inline]
    fn cmp(&self, other: &Self) -> Ordering {
        self.to_u128().cmp(&other.to_u128())
    }
}

#[derive(Debug, Clone, PartialEq, Default)]
pub struct SkinSwitchEvent {
    pub skin_id: u64, 

    pub from_time: WatchTime,
}

#[derive(Debug, Clone, PartialEq)]
pub struct State {
    pub watch_mode: WatchMode,
    pub clicked_btn: Option<Button>,
    pub dt: f32,
    pub button_is_pressed: bool,

    pub quit_request: bool,

    pub current_stopwatch_time: WatchTime,
    pub current_clock_time: WatchTime,

    pub change_skin_request: bool,
    pub current_skin_id: u64,

    pub saved_time: f32,

    pub default_stopwatch_skin_id: u64,
    pub default_clock_skin_id: u64,

    pub stopwatch_events: Vec<SkinSwitchEvent>,
    pub clock_events: Vec<SkinSwitchEvent>,

    pub save_data_path: PathBuf
}

impl Default for State {
    #[inline]
    fn default() -> Self {
        Self {
            watch_mode: WatchMode::Clock,
            clicked_btn: None,
            dt: Default::default(),
            button_is_pressed: false,

            quit_request: false,

            current_stopwatch_time: Default::default(),
            current_clock_time: Default::default(),

            change_skin_request: false,
            current_skin_id: Default::default(),

            saved_time: Default::default(),

            default_stopwatch_skin_id: Default::default(),
            default_clock_skin_id: Default::default(),

            stopwatch_events: Vec::<SkinSwitchEvent>::new(),
            clock_events: Vec::<SkinSwitchEvent>::new(),

            save_data_path: PathBuf::new()
        }
    }
}

impl State {
    pub fn init(&mut self) {
        self.stopwatch_events.iter_mut().for_each(
            |event| event.from_time.normalize()
        );

        self.clock_events.iter_mut().for_each(
            |event| event.from_time.normalize()
        );

        self.stopwatch_events.sort_by(
            |a, b| b.from_time.partial_cmp(&a.from_time).unwrap()
        );

        self.clock_events.sort_by(
            |a, b| b.from_time.partial_cmp(&a.from_time).unwrap()
        );
    }

    #[inline]
    pub fn find_skin_id_from_stopwatch_events(&self) -> u64 {
        if let Some(event) = self.stopwatch_events.iter().find(
            |event| self.current_stopwatch_time >= event.from_time
        ) {
            event.skin_id
        } else {
            self.default_stopwatch_skin_id
        }
    }

    #[inline]
    pub fn find_skin_id_from_clock_events(&self) -> u64 {
        if let Some(event) = self.clock_events.iter().find(
            |event| self.current_clock_time >= event.from_time
        ) {
            event.skin_id
        } else {
            self.default_clock_skin_id
        }
    }

    pub fn add_stopwatch_time(&mut self) {
        self.saved_time += self.dt;

        self.current_stopwatch_time.cents = (self.saved_time * 100.0) as u32;
        self.current_stopwatch_time.normalize();

        self.saved_time = self.saved_time.fract();
    }

    #[inline]
    pub fn set_clock(&mut self) {
        let time = Local::now();
        self.current_clock_time.hours = time.hour();
        self.current_clock_time.minutes = time.minute();
        self.current_clock_time.seconds = time.second();
        self.current_clock_time.cents = time.nanosecond() / 10000000;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn watch_time_test_1() {
        let wt = WatchTime {
            hours: 11,
            minutes: 22,
            seconds: 33,
            cents: 44
        };

        assert_eq!(wt.hours, (wt.to_u128() >> (32 * 3)) as u32);
        assert_eq!(wt.minutes, (wt.to_u128() >> (32 * 2)) as u32);
        assert_eq!(wt.seconds, (wt.to_u128() >> 32) as u32);
        assert_eq!(wt.cents, wt.to_u128() as u32);
    }

    #[test]
    fn watch_time_test_2() {
        let mut wt_1 = WatchTime {
            hours: 11,
            minutes: 22,
            seconds: 33,
            cents: 44
        };

        let wt_2 = wt_1.clone();

        assert_eq!(wt_1, wt_2);

        wt_1.hours += 1;
        assert_ne!(wt_1, wt_2);
        assert!(wt_1 > wt_2);
        assert!(wt_2 < wt_1);
    }

    #[test]
    fn watch_time_test_3() {
        let mut wt = WatchTime {
            hours: 20,
            minutes: 119,
            seconds: 125,
            cents: 150
        };

        wt.normalize();

        assert_eq!(wt.cents, 50);
        assert_eq!(wt.seconds, 6);
        assert_eq!(wt.minutes, 1);
        assert_eq!(wt.hours, 22);
    }

    fn gen_state(watch_mode: WatchMode) -> State {
        let mut ret = State::default();

        ret.watch_mode = watch_mode;

        ret
    }

    #[test]
    fn clock_test() {
        let mut state = gen_state(WatchMode::Clock);

        state.set_clock();

        println!("{:?}", state.current_clock_time);
    }

    #[test]
    fn stopwatch_test() {
        let mut state =
            gen_state(WatchMode::Stopwatch(StopwatchMode::Running));

        state.dt = 0.01;
        for _ in 0..321 {
            state.add_stopwatch_time();
        }

        assert_eq!(state.current_stopwatch_time.seconds, 3);
        assert_eq!(state.current_stopwatch_time.cents, 20);
        // for error less than 0.1 second.
        assert!(
            (state.current_stopwatch_time.cents >= 20)
                && (state.current_stopwatch_time.cents <= 22)
        );

        state.dt = 1.0;
        for _ in 0..((60 * 3) + 21) {
            state.add_stopwatch_time();
        }

        assert_eq!(state.current_stopwatch_time.minutes, 3);
        assert_eq!(state.current_stopwatch_time.seconds, 21 + 3);

        state.dt = 60.0;
        for _ in 0..((60 * 3) + 21) {
            state.add_stopwatch_time();
        }

        assert_eq!(state.current_stopwatch_time.hours, 3);
        assert_eq!(state.current_stopwatch_time.minutes, 21 + 3);
    }

    fn gen_stopwatch_events() -> Vec<SkinSwitchEvent> {
        let mut ret = Vec::<SkinSwitchEvent>::with_capacity(10);

        for i in 1u64..=10 {
            ret.push(
                SkinSwitchEvent {
                    skin_id: i * 11,

                    from_time: WatchTime {
                        hours: 0,
                        minutes: i as u32,
                        seconds: 0,
                        cents: 0
                    }
                }
            );
        }

        ret
    }

    fn gen_clock_events() -> Vec<SkinSwitchEvent> {
        let mut ret = Vec::<SkinSwitchEvent>::with_capacity(10);

        for i in 1u64..=10 {
            ret.push(
                SkinSwitchEvent {
                    skin_id: i * 111,

                    from_time: WatchTime {
                        hours: 0,
                        minutes: i as u32,
                        seconds: 0,
                        cents: 0
                    }
                }
            );
        }

        ret
    }

    #[test]
    fn events_test() {
        let mut state =
            gen_state(WatchMode::Stopwatch(StopwatchMode::Stopped));

        state.stopwatch_events = gen_stopwatch_events();
        state.clock_events = gen_clock_events();
        state.init();

        state.default_stopwatch_skin_id = 3;
        state.default_clock_skin_id = 4;

        for i in 0u32..=11 {
            state.current_stopwatch_time.seconds = 30;
            state.current_stopwatch_time.minutes = i;

            state.current_clock_time.seconds = 30;
            state.current_clock_time.minutes = i;

            if i == 0 {
                assert_eq!(state.find_skin_id_from_stopwatch_events(), 3);
                assert_eq!(state.find_skin_id_from_clock_events(), 4);
            } else if i == 11 {
                assert_eq!(state.find_skin_id_from_stopwatch_events(), 10 * 11);
                assert_eq!(state.find_skin_id_from_clock_events(), 10 * 111);
            } else {
                assert_eq!(
                    state.find_skin_id_from_stopwatch_events(),
                    (i as u64) * 11
                );
                assert_eq!(
                    state.find_skin_id_from_clock_events(),
                    (i as u64) * 111
                );
            }
        }
    }
}
