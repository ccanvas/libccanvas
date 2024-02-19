use crate::features::common::Rect;

#[derive(Clone)]
/// Struct storing ccanvas-layout allocated space info
pub struct LayoutInfo {
    /// Area allocated
    pub rect: Rect,
}

impl LayoutInfo {
    /// Create a new layout info of rect
    pub fn new(rect: Rect) -> Self {
        LayoutInfo { rect }
    }
}
