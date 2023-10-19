use crate::{error::Error, state::State, view::View};

use eframe::{
    egui::{
        Ui,
        TextureId,
        Rect,
        Pos2,
        Color32
    }
};

use chobitlibs::{
    chobit_ani_value::ChobitAniValue,
    chobit_rand::ChobitRand
};

use std::{
    rc::Rc,
    cell::RefCell
};

pub struct AnimationView {
    depth: i32,

    texture_id: TextureId,

    region: Rect,
    ani_value: ChobitAniValue,

    do_animation: bool,

    probability: Option<f64>,
    rng: Rc<RefCell<ChobitRand>>,
    acc_time: f32
}

impl AnimationView {
    pub fn new(
        depth: i32,
        texture_id: TextureId,
        region: Rect,
        mut ani_value: ChobitAniValue,
        probability: Option<f32>,
        rng: Rc<RefCell<ChobitRand>>
    ) -> Self {
        ani_value.set_frame(0);

        Self {
            depth: depth,

            texture_id: texture_id,

            region: region,
            ani_value: ani_value,

            do_animation: false,

            probability: probability.map(|value| value as f64),
            rng: rng,
            acc_time: 0.0
        }
    }
}

impl View for AnimationView {
    #[inline]
    fn depth(&self) -> i32 {self.depth}

    fn ready(&mut self, _ui: &Ui, state: &mut State) -> Result<(), Error> {
        match self.probability {  // running animation.
            Some(probability) => if self.do_animation {
                let current_frame = self.ani_value.current_frame();
                self.ani_value.elapse(state.dt);
                let next_frame = self.ani_value.current_frame();

                // if frame has rewound, stop animation.
                if (next_frame == 0) && (next_frame != current_frame) {
                    self.do_animation = false;
                    self.acc_time = 0.0;
                }

                Ok(())
            } else {  // stopped animation
                self.acc_time += state.dt;

                if self.acc_time >= 1.0 {  // draw lots per 1 second.
                    let mut rng = self.rng.try_borrow_mut().expect(
                        "Error at AnimationView::show()"
                    );

                    if rng.next_f64() <= probability {
                        self.do_animation = true;
                    }
         
                    self.acc_time = 0.0;
                }

                Ok(())
            },

            None => {
                self.ani_value.elapse(state.dt);
                Ok(())
            }
        }
    }

    fn show(&mut self, ui: &Ui, _state: &mut State) -> Result<(), Error> {
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
        self.ani_value.set_frame(0);

        Ok(())
    }
}
