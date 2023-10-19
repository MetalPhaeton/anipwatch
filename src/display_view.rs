use crate::{error::Error, state::{State, WatchMode}, view::View};

use eframe::{
    egui::{
        Ui,
        TextureId,
        Rect,
        Pos2,
        Color32
    }
};

use chobitlibs::chobit_ani_value::ChobitAniValue;

pub struct DisplayNumber {
    region_d1: Rect,
    region_d2: Rect,
    ani_value_d1: ChobitAniValue,
    ani_value_d2: ChobitAniValue,
}

impl DisplayNumber {
    #[inline]
    pub fn new(
        region_d1: Rect,
        region_d2: Rect,
    ) -> Self {
        let ani_value = ChobitAniValue::new(10, &[10], 1.0).expect(
            "Error at DisplayNumber::new()"
        );

        Self {
            region_d1: region_d1,
            region_d2: region_d2,
            ani_value_d1: ani_value.clone(),
            ani_value_d2: ani_value
        }
    }

    fn show(
        &mut self,
        ui: &Ui,
        texture_id: TextureId,
        number: u32
    ) -> Result<(), Error> {
        let digit_1 = (number % 10) as usize;
        let digit_2 = ((number / 10) % 10) as usize;

        self.ani_value_d1.set_frame(digit_1);
        self.ani_value_d2.set_frame(digit_2);

        let (left_1, top_1, right_1, bottom_1) =
            self.ani_value_d1.uv_frame_left_top_right_bottom();

        let (left_2, top_2, right_2, bottom_2) =
            self.ani_value_d2.uv_frame_left_top_right_bottom();

        ui.painter().image(
            texture_id,
            self.region_d1,
            Rect::from_min_max(
                Pos2::new(*left_1, *top_1),
                Pos2::new(*right_1, *bottom_1)
            ),
            Color32::WHITE
        );

        ui.painter().image(
            texture_id,
            self.region_d2,
            Rect::from_min_max(
                Pos2::new(*left_2, *top_2),
                Pos2::new(*right_2, *bottom_2)
            ),
            Color32::WHITE
        );

        Ok(())
    }
}

pub struct DisplayView {
    depth: i32,

    texture_id: TextureId,

    cents: Option<DisplayNumber>,
    seconds: Option<DisplayNumber>,
    minutes: Option<DisplayNumber>,
    hours: Option<DisplayNumber>
}

impl DisplayView {
    pub fn new(
        depth: i32,
        texture_id: TextureId,
        cents: Option<DisplayNumber>,
        seconds: Option<DisplayNumber>,
        minutes: Option<DisplayNumber>,
        hours: Option<DisplayNumber>
    ) -> Self {
        Self {
            depth: depth,

            texture_id: texture_id,

            cents: cents,
            seconds: seconds,
            minutes: minutes,
            hours: hours
        }
    }
}

impl View for DisplayView {
    #[inline]
    fn depth(&self) -> i32 {self.depth}

    fn ready(&mut self, _ui: &Ui, _state: &mut State) -> Result<(), Error> {
        // nothing to do.
        Ok(())
    }

    fn show(&mut self, ui: &Ui, state: &mut State) -> Result<(), Error> {
        let current_time = match state.watch_mode {
            WatchMode::Stopwatch(..) => state.current_stopwatch_time.clone(),
            WatchMode::Clock => state.current_clock_time.clone()
        };

        if let Some(cents) = self.cents.as_mut() {
            cents.show(ui, self.texture_id, current_time.cents)?;
        }

        if let Some(seconds) = self.seconds.as_mut() {
            seconds.show(ui, self.texture_id, current_time.seconds)?;
        }

        if let Some(minutes) = self.minutes.as_mut() {
            minutes.show(ui, self.texture_id, current_time.minutes)?;
        }

        if let Some(hours) = self.hours.as_mut() {
            hours.show(ui, self.texture_id, current_time.hours)?;
        }

        Ok(())
    }

    fn init(&mut self, _state: &mut State) -> Result<(), Error> {
        // pass
        Ok(())
    }
}
