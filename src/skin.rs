use crate::{view::View};

pub struct Skin {
    views: Vec<Box<dyn View>>
}

impl Skin {
    #[inline]
    pub fn new() -> Self {
        Self {
            views: Vec::<Box<dyn View>>::new()
        }
    }

    #[inline]
    pub fn as_mut_slice(&mut self) -> &mut [Box<dyn View>] {
        self.views.as_mut_slice()
    }

    #[inline]
    pub fn add(&mut self, view: Box<dyn View>) {
        let index = match self.views.binary_search_by(
            |elm| view.depth().cmp(&elm.depth())
        ) {
            Ok(index) => index,
            Err(index) => index
        };

        self.views.insert(index, view);
    }

}
