use crate::{error::Error, state::State};

use eframe::egui::Ui;

pub trait View {
    fn depth(&self) -> i32;

    fn init(&mut self, state: &mut State) -> Result<(), Error>;

    fn ready(&mut self, ui: &Ui, state: &mut State) -> Result<(), Error>;
    fn show(&mut self, ui: &Ui, state: &mut State) -> Result<(), Error>;
}
