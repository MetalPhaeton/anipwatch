use crate::{error::Error, state::{State, Button}, view::View};

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

pub struct ButtonView {
    depth: i32,

    texture_id: TextureId,

    region: Rect,
    ani_value: ChobitAniValue,

    button: Button
}

impl ButtonView {
    pub fn new(
        depth: i32,
        texture_id: TextureId,
        region: Rect,
        button: Button
    ) -> Self {
        Self {
            depth: depth,

            texture_id: texture_id,

            region: region,
            ani_value: ChobitAniValue::new(1, &[1], 1.0).expect(
                "Error at ButtonView::new()"
            ),

            button: button
        }
    }
}

impl View for ButtonView {
    #[inline]
    fn depth(&self) -> i32 {self.depth}

    fn ready(&mut self, ui: &Ui, state: &mut State) -> Result<(), Error> {
        ui.input(|i_state| {
            let pointer = &i_state.pointer;

            if pointer.has_pointer() {
                if let Some(pos) = pointer.interact_pos() {
                    if self.region.contains(pos) {
                        if pointer.primary_clicked() {
                            state.clicked_btn = Some(self.button);
                        }
                    }
                }
            }
        });

        Ok(())
    }

    fn show(&mut self, ui: &Ui, _state: &mut State) -> Result<(), Error> {
        self.ani_value.set_frame(0);
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
