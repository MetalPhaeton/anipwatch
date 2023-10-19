use crate::{error::Error, state::{State, Button, WatchMode}, view::View};

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

pub struct SwitchBtnView {
    depth: i32,

    texture_id: TextureId,

    region: Rect,
    ani_value: ChobitAniValue
}

const STOPWATCH_MODE_FRAME: usize = 0;
const CLOCK_MODE_FRAME: usize = 1;

impl SwitchBtnView {
    pub fn new(
        depth: i32,
        texture_id: TextureId,
        region: Rect,
    ) -> Self {
        Self {
            depth: depth,

            texture_id: texture_id,

            region: region,
            ani_value: ChobitAniValue::new(2, &[2], 1.0).expect(
                "Error at SwitchBtnView::new()"
            )
        }
    }
}

impl View for SwitchBtnView {
    #[inline]
    fn depth(&self) -> i32 {self.depth}

    fn ready(&mut self, ui: &Ui, state: &mut State) -> Result<(), Error> {
        ui.input(|i_state| {
            let pointer = &i_state.pointer;

            if let Some(pos) = pointer.interact_pos() {
                if self.region.contains(pos) {
                    if pointer.primary_clicked() {
                        state.clicked_btn = Some(Button::Switch);
                    }
                }
            }
        });

        Ok(())
    }

    fn show(&mut self, ui: &Ui, state: &mut State) -> Result<(), Error> {
        let frame = match state.watch_mode {
            WatchMode::Stopwatch(..) => STOPWATCH_MODE_FRAME,
            WatchMode::Clock => CLOCK_MODE_FRAME
        };

        self.ani_value.set_frame(frame);
        let (left, top, right, bottom) =
            self.ani_value.uv_frame_left_top_right_bottom();

        ui.painter().image(
            self.texture_id,
            self.region,
            Rect::from_min_max(
                Pos2::new(*left, *top),
                Pos2::new(*right, *bottom)
            ),
            Color32::WHITE
        );

        Ok(())
    }

    fn init(&mut self, _state: &mut State) -> Result<(), Error> {
        // pass
        Ok(())
    }
}
