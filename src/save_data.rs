use chobitlibs::{
    chobit_sexpr::{ChobitSexpr, ChobitSexprBuf, Completed, Empty},
    chobit_hash::fnv_1a_64
};

use crate::state::*;

use std::cmp::PartialEq;

/*
(
    (watch_mode <watch_mode: u64>)
    (stopwatch_time
        <hours: u32>
        <minutes: u32>
        <seconds: u32>
        <centiseconds: u32>
        <saved_time: f32>)
)
 */

const SYMBOL_WATCH_MODE: u64 = fnv_1a_64(b"watch_mode");
const SYMBOL_STOPWATCH: u64 = fnv_1a_64(b"stopwatch");
const SYMBOL_CLOCK: u64 = fnv_1a_64(b"clock");
const SYMBOL_STOPWATCH_TIME: u64 = fnv_1a_64(b"stopwatch_time");

#[derive(Debug, Clone)]
pub struct SaveData {
    buf_0: Option<ChobitSexprBuf<Empty>>,
    buf_1: Option<ChobitSexprBuf<Empty>>,
    buf_2: Option<ChobitSexprBuf<Empty>>,

    pub watch_mode: WatchMode,
    pub stopwatch_time: WatchTime,
    pub saved_time: f32
}

impl PartialEq for SaveData {
    #[inline]
    fn eq(&self, other: &Self) -> bool {
        self.watch_mode == other.watch_mode
            && self.stopwatch_time == other.stopwatch_time
    }
}

impl SaveData {
    pub fn from_variables(
        watch_mode: WatchMode,
        stopwatch_time: WatchTime,
        saved_time: f32
    ) -> Self {
        Self {
            buf_0: Some(ChobitSexprBuf::new()),
            buf_1: Some(ChobitSexprBuf::new()),
            buf_2: Some(ChobitSexprBuf::new()),

            watch_mode: watch_mode,
            stopwatch_time: stopwatch_time,
            saved_time: saved_time
        }
    }

    pub fn from_bytes(
        bytes: &[u8]
    ) -> Option<Self> {
        let sexpr = ChobitSexpr::new(bytes);

        let mut watch_mode: Option<WatchMode> = None;
        let mut stopwatch_time: Option<(WatchTime, f32)> = None;

        for elm in sexpr.iter() {
            let (car, cdr) = elm.car_cdr().ok()?;

            match car.read_u64().ok()? {
                SYMBOL_WATCH_MODE => {
                    watch_mode = Self::load_watch_mode(cdr);
                },

                SYMBOL_STOPWATCH_TIME => {
                    stopwatch_time = Self::load_stopwatch_time(cdr);
                },

                _ => {return None;}
            }
        }

        let (stopwatch_time, saved_time) = stopwatch_time?;

        Some(Self {
            buf_0: Some(ChobitSexprBuf::new()),
            buf_1: Some(ChobitSexprBuf::new()),
            buf_2: Some(ChobitSexprBuf::new()),

            watch_mode: watch_mode?,
            stopwatch_time: stopwatch_time,
            saved_time: saved_time
        })
    }

    fn load_stopwatch_time(cdr: &ChobitSexpr) -> Option<(WatchTime, f32)> {
        let (hours_sexpr, cdr) = cdr.car_cdr().ok()?;
        let hours = hours_sexpr.read_u32().ok()?;

        let (minutes_sexpr, cdr) = cdr.car_cdr().ok()?;
        let minutes = minutes_sexpr.read_u32().ok()?;

        let (seconds_sexpr, cdr) = cdr.car_cdr().ok()?;
        let seconds = seconds_sexpr.read_u32().ok()?;

        let (cents_sexpr, cdr) = cdr.car_cdr().ok()?;
        let cents = cents_sexpr.read_u32().ok()?;

        let saved_time_sexpr = cdr.car().ok()?;
        let saved_time = saved_time_sexpr.read_f32().ok()?;

        Some((
            WatchTime {
                hours: hours,
                minutes: minutes,
                seconds: seconds,
                cents: cents
            },
            saved_time
        ))
    }

    fn load_watch_mode(cdr: &ChobitSexpr) -> Option<WatchMode> {
        let mode_sexpr = cdr.car().ok()?;

        match mode_sexpr.read_u64().ok()? {
            SYMBOL_STOPWATCH =>
                Some(WatchMode::Stopwatch(StopwatchMode::Stopped)),

            SYMBOL_CLOCK => Some(WatchMode::Clock),

            _ => None
        }
    }

    pub fn write_bytes(&mut self, buf: &mut Vec<u8>) {
        let sexpr_buf_0 = self.buf_0.take().expect(
            "Error at SaveData::write_bytes() #1"
        );
        let sexpr_buf_1 = self.buf_1.take().expect(
            "Error at SaveData::write_bytes() #2"
        );
        let sexpr_buf_2 = self.buf_2.take().expect(
            "Error at SaveData::write_bytes() #3"
        );

        let sexpr_buf_0 = sexpr_buf_0.build_list();

        let (sexpr_buf_1, sexpr_buf_2) = Self::build_watch_mode_sexpr(
            &self.watch_mode,
            sexpr_buf_1,
            sexpr_buf_2
        );
        let sexpr_buf_0 = sexpr_buf_0.push_item(&sexpr_buf_1);

        let sexpr_buf_1 = sexpr_buf_1.clear();
        let sexpr_buf_2 = sexpr_buf_2.clear();

        let (sexpr_buf_1, sexpr_buf_2) = Self::build_stopwatch_time_sexpr(
            &self.stopwatch_time,
            self.saved_time,
            sexpr_buf_1,
            sexpr_buf_2
        );
        let sexpr_buf_0 = sexpr_buf_0.push_item(&sexpr_buf_1);

        let sexpr_buf_0 = sexpr_buf_0.finish();

        buf.clear();
        buf.extend_from_slice(sexpr_buf_0.as_sexpr().as_bytes());

        self.buf_0 = Some(sexpr_buf_0.clear());
        self.buf_1 = Some(sexpr_buf_1.clear());
        self.buf_2 = Some(sexpr_buf_2.clear());
    }

    fn build_watch_mode_sexpr(
        watch_mode: &WatchMode,
        buf_1: ChobitSexprBuf<Empty>,
        buf_2: ChobitSexprBuf<Empty>
    ) -> (ChobitSexprBuf<Completed>, ChobitSexprBuf<Completed>) {
        let buf_1 = buf_1.build_list();

        let buf_2 = buf_2.push_u64(SYMBOL_WATCH_MODE);
        let buf_1 = buf_1.push_item(&buf_2);

        let buf_2 = buf_2.clear().push_u64(match watch_mode {
            WatchMode::Stopwatch(..) => SYMBOL_STOPWATCH,
            WatchMode::Clock => SYMBOL_CLOCK,
        });
        let buf_1 = buf_1.push_item(&buf_2);

        let buf_1 = buf_1.finish();

        (buf_1, buf_2)
    }

    fn build_stopwatch_time_sexpr(
        stopwatch_time: &WatchTime,
        saved_time: f32,
        buf_1: ChobitSexprBuf<Empty>,
        buf_2: ChobitSexprBuf<Empty>
    ) -> (ChobitSexprBuf<Completed>, ChobitSexprBuf<Completed>) {
        let buf_1 = buf_1.build_list();

        let buf_2 = buf_2.push_u64(SYMBOL_STOPWATCH_TIME);
        let buf_1 = buf_1.push_item(&buf_2);

        let buf_2 = buf_2.clear().push_u32(stopwatch_time.hours);
        let buf_1 = buf_1.push_item(&buf_2);

        let buf_2 = buf_2.clear().push_u32(stopwatch_time.minutes);
        let buf_1 = buf_1.push_item(&buf_2);

        let buf_2 = buf_2.clear().push_u32(stopwatch_time.seconds);
        let buf_1 = buf_1.push_item(&buf_2);

        let buf_2 = buf_2.clear().push_u32(stopwatch_time.cents);
        let buf_1 = buf_1.push_item(&buf_2);

        let buf_2 = buf_2.clear().push_f32(saved_time);
        let buf_1 = buf_1.push_item(&buf_2);

        let buf_1 = buf_1.finish();

        (buf_1, buf_2)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn save_data_test() {
        let stopwatch_time = WatchTime {
            hours: 10,
            minutes: 20,
            seconds: 30,
            cents: 40
        };
        let saved_time: f32 = 0.003;

        let mut save_data_1 = SaveData::from_variables(
            WatchMode::Clock,
            stopwatch_time,
            saved_time
        );

        let mut buf = Vec::<u8>::new();

        save_data_1.write_bytes(&mut buf);

        let save_data_2 = SaveData::from_bytes(&buf).unwrap();

        assert_eq!(save_data_2, save_data_1);
    }
}
