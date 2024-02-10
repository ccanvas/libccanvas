use crate::features::common::Rect;

#[derive(Clone)]
pub struct LayoutInfo {
    pub rect: Rect,
}

impl LayoutInfo {
    pub fn new(rect: Rect) -> Self {
        LayoutInfo { rect }
    }
}
