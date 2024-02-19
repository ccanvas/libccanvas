use crate::{bindings::Discriminator, features::common::Direction};
use serde::{Deserialize, Serialize};

use super::{Border, Constraint};

#[derive(Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum Layout {
    #[serde(rename = "none")]
    None,
    #[serde(rename = "single")]
    Single {
        discrim: Option<Discriminator>,
        border: Option<Border>,
    },
    #[serde(rename = "split horizontal")]
    SplitHorizontal {
        left_constraint: Constraint,
        left: Box<Layout>,
        right_constraint: Constraint,
        right: Box<Layout>,
    },
    #[serde(rename = "split vertical")]
    SplitVertical {
        top_constraint: Constraint,
        top: Box<Layout>,
        bottom_constraint: Constraint,
        bottom: Box<Layout>,
    },
}

impl Default for Layout {
    fn default() -> Self {
        Self::None
    }
}

impl Layout {
    pub fn single(discrim: Option<Discriminator>, border: Option<Border>) -> Self {
        Self::Single { discrim, border }
    }

    pub fn horizontal(
        left: Layout,
        right: Layout,
        left_constraint: Constraint,
        right_constraint: Constraint,
    ) -> Self {
        Self::SplitHorizontal {
            left: left.into(),
            right: right.into(),
            left_constraint,
            right_constraint,
        }
    }

    pub fn vertical(
        top: Layout,
        bottom: Layout,
        top_constraint: Constraint,
        bottom_constraint: Constraint,
    ) -> Self {
        Self::SplitVertical {
            top: top.into(),
            bottom: bottom.into(),
            top_constraint,
            bottom_constraint,
        }
    }
}

impl Layout {
    /// add an item, returns whether layout is updated
    pub fn add(
        &mut self,
        at: &[Direction],
        split: Direction,
        constraint_1: Constraint,
        constraint_2: Constraint,
        component: Option<Discriminator>,
        border: Option<Border>,
    ) -> bool {
        if at.is_empty() {
            match split {
                Direction::Up => {
                    *self = Self::vertical(
                        Layout::single(component, border),
                        std::mem::take(self),
                        constraint_1,
                        constraint_2,
                    )
                }
                Direction::Down => {
                    *self = Self::vertical(
                        std::mem::take(self),
                        Layout::single(component, border),
                        constraint_1,
                        constraint_2,
                    )
                }
                Direction::Left => {
                    *self = Self::horizontal(
                        Layout::single(component, border),
                        std::mem::take(self),
                        constraint_1,
                        constraint_2,
                    )
                }
                Direction::Right => {
                    *self = Self::horizontal(
                        std::mem::take(self),
                        Layout::single(component, border),
                        constraint_1,
                        constraint_2,
                    )
                }
            }

            return true;
        }

        match self {
            Self::SplitHorizontal { left, right, .. }
                if matches!(at[0], Direction::Left | Direction::Right) =>
            {
                if at[0] == Direction::Left {
                    left.add(
                        &at[1..],
                        split,
                        constraint_1,
                        constraint_2,
                        component,
                        border,
                    )
                } else {
                    right.add(
                        &at[1..],
                        split,
                        constraint_1,
                        constraint_2,
                        component,
                        border,
                    )
                }
            }
            Self::SplitVertical { top, bottom, .. }
                if matches!(at[0], Direction::Up | Direction::Down) =>
            {
                if at[0] == Direction::Up {
                    top.add(
                        &at[1..],
                        split,
                        constraint_1,
                        constraint_2,
                        component,
                        border,
                    )
                } else {
                    bottom.add(
                        &at[1..],
                        split,
                        constraint_1,
                        constraint_2,
                        component,
                        border,
                    )
                }
            }
            _ => false,
        }
    }

    /// remove an item, returns whether layout is updated
    pub fn remove(&mut self, at: &[Direction]) -> bool {
        if at.is_empty() {
            *self = Self::None;
            return true;
        }

        if at.len() == 1 {
            match self {
                Self::SplitHorizontal { left, right, .. }
                    if matches!(at[0], Direction::Left | Direction::Right) =>
                {
                    if at[0] == Direction::Left {
                        *self = std::mem::take(right);
                    } else {
                        *self = std::mem::take(left);
                    }

                    return true;
                }
                Self::SplitVertical { top, bottom, .. }
                    if matches!(at[0], Direction::Up | Direction::Down) =>
                {
                    if at[0] == Direction::Up {
                        *self = std::mem::take(bottom);
                    } else {
                        *self = std::mem::take(top);
                    }

                    return true;
                }
                _ => {}
            }
        }

        match self {
            Self::SplitHorizontal { left, right, .. }
                if matches!(at[0], Direction::Left | Direction::Right) =>
            {
                if at[0] == Direction::Left {
                    left.remove(&at[1..])
                } else {
                    right.remove(&at[1..])
                }
            }
            Self::SplitVertical { top, bottom, .. }
                if matches!(at[0], Direction::Up | Direction::Down) =>
            {
                if at[0] == Direction::Up {
                    top.remove(&at[1..])
                } else {
                    bottom.remove(&at[1..])
                }
            }
            _ => false,
        }
    }

    /// Set a sublayout with a layout
    pub fn set(&mut self, at: &[Direction], state: Layout) -> bool {
        if at.is_empty() {
            *self = state;
            return true;
        }

        match self {
            Self::SplitHorizontal { left, right, .. }
                if matches!(at[0], Direction::Left | Direction::Right) =>
            {
                if at[0] == Direction::Left {
                    left.set(&at[1..], state)
                } else {
                    right.set(&at[1..], state)
                }
            }
            Self::SplitVertical { top, bottom, .. }
                if matches!(at[0], Direction::Up | Direction::Down) =>
            {
                if at[0] == Direction::Up {
                    top.set(&at[1..], state)
                } else {
                    bottom.set(&at[1..], state)
                }
            }
            _ => false,
        }
    }

    /// Get a sublayout
    pub fn get(&self, at: &[Direction]) -> Option<&Self> {
        if at.is_empty() {
            return Some(self);
        }

        match self {
            Self::SplitHorizontal { left, right, .. }
                if matches!(at[0], Direction::Left | Direction::Right) =>
            {
                if at[0] == Direction::Left {
                    left.get(&at[1..])
                } else {
                    right.get(&at[1..])
                }
            }
            Self::SplitVertical { top, bottom, .. }
                if matches!(at[0], Direction::Up | Direction::Down) =>
            {
                if at[0] == Direction::Up {
                    top.get(&at[1..])
                } else {
                    bottom.get(&at[1..])
                }
            }
            _ => None,
        }
    }
}

impl Layout {
    /// Add a new blank layout section
    pub fn add_blank(
        &mut self,
        at: &[Direction],
        split: Direction,
        constraint_1: Constraint,
        constraint_2: Constraint,
        border: Option<Border>,
    ) -> bool {
        self.add(at, split, constraint_1, constraint_2, None, border)
    }

    /// Add a new component layout section
    pub fn add_component(
        &mut self,
        at: &[Direction],
        split: Direction,
        constraint_1: Constraint,
        constraint_2: Constraint,
        component: Discriminator,
        border: Option<Border>,
    ) -> bool {
        self.add(
            at,
            split,
            constraint_1,
            constraint_2,
            Some(component),
            border,
        )
    }

    /// Add a new layout section below target
    pub fn add_below(
        &mut self,
        at: &[Direction],
        constraint_top: Constraint,
        onstraint_bottom: Constraint,
        component: Option<Discriminator>,
        border: Option<Border>,
    ) -> bool {
        self.add(
            at,
            Direction::Down,
            constraint_top,
            onstraint_bottom,
            component,
            border,
        )
    }

    /// Add a new layout section above target
    pub fn add_above(
        &mut self,
        at: &[Direction],
        constraint_top: Constraint,
        onstraint_bottom: Constraint,
        component: Option<Discriminator>,
        border: Option<Border>,
    ) -> bool {
        self.add(
            at,
            Direction::Up,
            constraint_top,
            onstraint_bottom,
            component,
            border,
        )
    }

    /// Add a new layout section to the left of target
    pub fn add_left(
        &mut self,
        at: &[Direction],
        constraint_left: Constraint,
        constraint_right: Constraint,
        component: Option<Discriminator>,
        border: Option<Border>,
    ) -> bool {
        self.add(
            at,
            Direction::Left,
            constraint_left,
            constraint_right,
            component,
            border,
        )
    }

    /// Add a new layout section to the right of target
    pub fn add_right(
        &mut self,
        at: &[Direction],
        constraint_left: Constraint,
        constraint_right: Constraint,
        component: Option<Discriminator>,
        border: Option<Border>,
    ) -> bool {
        self.add(
            at,
            Direction::Right,
            constraint_left,
            constraint_right,
            component,
            border,
        )
    }

    /// Add a new bordered layout section
    pub fn add_bordered(
        &mut self,
        at: &[Direction],
        split: Direction,
        constraint_1: Constraint,
        constraint_2: Constraint,
        component: Option<Discriminator>,
        border: Border,
    ) -> bool {
        self.add(
            at,
            split,
            constraint_1,
            constraint_2,
            component,
            Some(border),
        )
    }

    /// Add a new unbordered layout section
    pub fn add_unbordered(
        &mut self,
        at: &[Direction],
        split: Direction,
        constraint_1: Constraint,
        constraint_2: Constraint,
        component: Option<Discriminator>,
    ) -> bool {
        self.add(at, split, constraint_1, constraint_2, component, None)
    }
}

impl Layout {
    /// Add a new blank layout section above target
    pub fn add_blank_above(
        &mut self,
        at: &[Direction],
        constraint_top: Constraint,
        onstraint_bottom: Constraint,
        border: Option<Border>,
    ) -> bool {
        self.add_blank(at, Direction::Up, constraint_top, onstraint_bottom, border)
    }

    /// Add a new blank layout section below target
    pub fn add_blank_below(
        &mut self,
        at: &[Direction],
        constraint_top: Constraint,
        onstraint_bottom: Constraint,
        border: Option<Border>,
    ) -> bool {
        self.add_blank(
            at,
            Direction::Down,
            constraint_top,
            onstraint_bottom,
            border,
        )
    }

    /// Add a new blank layout section to the left of target
    pub fn add_blank_left(
        &mut self,
        at: &[Direction],
        constraint_left: Constraint,
        constraint_right: Constraint,
        border: Option<Border>,
    ) -> bool {
        self.add_blank(
            at,
            Direction::Left,
            constraint_left,
            constraint_right,
            border,
        )
    }

    /// Add a new blank layout section to the right of target
    pub fn add_blank_right(
        &mut self,
        at: &[Direction],
        constraint_left: Constraint,
        constraint_right: Constraint,
        border: Option<Border>,
    ) -> bool {
        self.add_blank(
            at,
            Direction::Right,
            constraint_left,
            constraint_right,
            border,
        )
    }

    /// Add a new component layout section above target
    pub fn add_component_above(
        &mut self,
        at: &[Direction],
        constraint_top: Constraint,
        onstraint_bottom: Constraint,
        component: Discriminator,
        border: Option<Border>,
    ) -> bool {
        self.add_component(
            at,
            Direction::Up,
            constraint_top,
            onstraint_bottom,
            component,
            border,
        )
    }

    /// Add a new component layout section below target
    pub fn add_component_below(
        &mut self,
        at: &[Direction],
        constraint_top: Constraint,
        onstraint_bottom: Constraint,
        component: Discriminator,
        border: Option<Border>,
    ) -> bool {
        self.add_component(
            at,
            Direction::Down,
            constraint_top,
            onstraint_bottom,
            component,
            border,
        )
    }

    /// Add a new component layout section to the left of target
    pub fn add_component_left(
        &mut self,
        at: &[Direction],
        constraint_left: Constraint,
        constraint_right: Constraint,
        component: Discriminator,
        border: Option<Border>,
    ) -> bool {
        self.add_component(
            at,
            Direction::Left,
            constraint_left,
            constraint_right,
            component,
            border,
        )
    }

    /// Add a new component layout section to the right of target
    pub fn add_component_right(
        &mut self,
        at: &[Direction],
        constraint_left: Constraint,
        constraint_right: Constraint,
        component: Discriminator,
        border: Option<Border>,
    ) -> bool {
        self.add_component(
            at,
            Direction::Right,
            constraint_left,
            constraint_right,
            component,
            border,
        )
    }

    /// Add a new bordered layout section above target
    pub fn add_bordered_above(
        &mut self,
        at: &[Direction],
        constraint_top: Constraint,
        onstraint_bottom: Constraint,
        component: Option<Discriminator>,
        border: Border,
    ) -> bool {
        self.add_bordered(
            at,
            Direction::Up,
            constraint_top,
            onstraint_bottom,
            component,
            border,
        )
    }

    /// Add a new bordered layout section below target
    pub fn add_bordered_below(
        &mut self,
        at: &[Direction],
        constraint_top: Constraint,
        onstraint_bottom: Constraint,
        component: Option<Discriminator>,
        border: Border,
    ) -> bool {
        self.add_bordered(
            at,
            Direction::Down,
            constraint_top,
            onstraint_bottom,
            component,
            border,
        )
    }

    /// Add a new bordered layout section to the left of target
    pub fn add_bordered_left(
        &mut self,
        at: &[Direction],
        constraint_left: Constraint,
        constraint_right: Constraint,
        component: Option<Discriminator>,
        border: Border,
    ) -> bool {
        self.add_bordered(
            at,
            Direction::Left,
            constraint_left,
            constraint_right,
            component,
            border,
        )
    }

    /// Add a new bordered layout section to the right of target
    pub fn add_bordered_right(
        &mut self,
        at: &[Direction],
        constraint_left: Constraint,
        constraint_right: Constraint,
        component: Option<Discriminator>,
        border: Border,
    ) -> bool {
        self.add_bordered(
            at,
            Direction::Right,
            constraint_left,
            constraint_right,
            component,
            border,
        )
    }

    /// Add a new unbordered layout section above target
    pub fn add_unbordered_above(
        &mut self,
        at: &[Direction],
        constraint_top: Constraint,
        onstraint_bottom: Constraint,
        component: Option<Discriminator>,
    ) -> bool {
        self.add_unbordered(
            at,
            Direction::Up,
            constraint_top,
            onstraint_bottom,
            component,
        )
    }

    /// Add a new unbordered layout section below target
    pub fn add_unbordered_below(
        &mut self,
        at: &[Direction],
        constraint_top: Constraint,
        onstraint_bottom: Constraint,
        component: Option<Discriminator>,
    ) -> bool {
        self.add_unbordered(
            at,
            Direction::Down,
            constraint_top,
            onstraint_bottom,
            component,
        )
    }

    /// Add a new unbordered layout section to the left of target
    pub fn add_unbordered_left(
        &mut self,
        at: &[Direction],
        constraint_left: Constraint,
        constraint_right: Constraint,
        component: Option<Discriminator>,
    ) -> bool {
        self.add_unbordered(
            at,
            Direction::Left,
            constraint_left,
            constraint_right,
            component,
        )
    }

    /// Add a new unbordered layout section to the right of target
    pub fn add_unbordered_right(
        &mut self,
        at: &[Direction],
        constraint_left: Constraint,
        constraint_right: Constraint,
        component: Option<Discriminator>,
    ) -> bool {
        self.add_unbordered(
            at,
            Direction::Right,
            constraint_left,
            constraint_right,
            component,
        )
    }
}

impl Layout {
    /// Add a new bordered blank layout section
    pub fn add_bordered_blank(
        &mut self,
        at: &[Direction],
        split: Direction,
        constraint_1: Constraint,
        constraint_2: Constraint,
        border: Border,
    ) -> bool {
        self.add_blank(at, split, constraint_1, constraint_2, Some(border))
    }

    /// Add a new unbordered blank layout section
    pub fn add_unbordered_blank(
        &mut self,
        at: &[Direction],
        split: Direction,
        constraint_1: Constraint,
        constraint_2: Constraint,
    ) -> bool {
        self.add_blank(at, split, constraint_1, constraint_2, None)
    }

    /// Add a new bordered component layout section
    pub fn add_bordered_component(
        &mut self,
        at: &[Direction],
        split: Direction,
        constraint_1: Constraint,
        constraint_2: Constraint,
        component: Discriminator,
        border: Border,
    ) -> bool {
        self.add_component(
            at,
            split,
            constraint_1,
            constraint_2,
            component,
            Some(border),
        )
    }

    /// Add a new unbordered component layout section
    pub fn add_unbordered_component(
        &mut self,
        at: &[Direction],
        split: Direction,
        constraint_1: Constraint,
        constraint_2: Constraint,
        component: Discriminator,
    ) -> bool {
        self.add_component(at, split, constraint_1, constraint_2, component, None)
    }
}

impl Layout {
    /// Add a new bordered blank layout section above target
    pub fn add_bordered_blank_above(
        &mut self,
        at: &[Direction],
        constraint_top: Constraint,
        onstraint_bottom: Constraint,
        border: Border,
    ) -> bool {
        self.add_bordered_blank(at, Direction::Up, constraint_top, onstraint_bottom, border)
    }

    /// Add a new bordered blank layout section below target
    pub fn add_bordered_blank_below(
        &mut self,
        at: &[Direction],
        constraint_top: Constraint,
        onstraint_bottom: Constraint,
        border: Border,
    ) -> bool {
        self.add_bordered_blank(
            at,
            Direction::Down,
            constraint_top,
            onstraint_bottom,
            border,
        )
    }

    /// Add a new bordered blank layout section to the left of target
    pub fn add_bordered_blank_left(
        &mut self,
        at: &[Direction],
        constraint_left: Constraint,
        constraint_right: Constraint,
        border: Border,
    ) -> bool {
        self.add_bordered_blank(
            at,
            Direction::Left,
            constraint_left,
            constraint_right,
            border,
        )
    }

    /// Add a new bordered blank layout section to the right of target
    pub fn add_bordered_blank_right(
        &mut self,
        at: &[Direction],
        constraint_left: Constraint,
        constraint_right: Constraint,
        border: Border,
    ) -> bool {
        self.add_bordered_blank(
            at,
            Direction::Right,
            constraint_left,
            constraint_right,
            border,
        )
    }

    /// Add a new bordered component layout section above target
    pub fn add_bordered_component_above(
        &mut self,
        at: &[Direction],
        constraint_top: Constraint,
        onstraint_bottom: Constraint,
        component: Discriminator,
        border: Border,
    ) -> bool {
        self.add_bordered_component(
            at,
            Direction::Up,
            constraint_top,
            onstraint_bottom,
            component,
            border,
        )
    }

    /// Add a new bordered component layout section below target
    pub fn add_bordered_component_below(
        &mut self,
        at: &[Direction],
        constraint_top: Constraint,
        onstraint_bottom: Constraint,
        component: Discriminator,
        border: Border,
    ) -> bool {
        self.add_bordered_component(
            at,
            Direction::Down,
            constraint_top,
            onstraint_bottom,
            component,
            border,
        )
    }

    /// Add a new bordered component layout section to the left of target
    pub fn add_bordered_component_left(
        &mut self,
        at: &[Direction],
        constraint_left: Constraint,
        constraint_right: Constraint,
        component: Discriminator,
        border: Border,
    ) -> bool {
        self.add_bordered_component(
            at,
            Direction::Left,
            constraint_left,
            constraint_right,
            component,
            border,
        )
    }

    /// Add a new bordered component layout section to the right of target
    pub fn add_bordered_component_right(
        &mut self,
        at: &[Direction],
        constraint_left: Constraint,
        constraint_right: Constraint,
        component: Discriminator,
        border: Border,
    ) -> bool {
        self.add_bordered_component(
            at,
            Direction::Right,
            constraint_left,
            constraint_right,
            component,
            border,
        )
    }

    /// Add a new unbordered blank layout section above target
    pub fn add_unbordered_blank_above(
        &mut self,
        at: &[Direction],
        constraint_top: Constraint,
        onstraint_bottom: Constraint,
    ) -> bool {
        self.add_unbordered_blank(at, Direction::Up, constraint_top, onstraint_bottom)
    }

    /// Add a new unbordered blank layout section below target
    pub fn add_unbordered_blank_below(
        &mut self,
        at: &[Direction],
        constraint_top: Constraint,
        onstraint_bottom: Constraint,
    ) -> bool {
        self.add_unbordered_blank(at, Direction::Down, constraint_top, onstraint_bottom)
    }

    /// Add a new unbordered blank layout section to the left of target
    pub fn add_unbordered_blank_left(
        &mut self,
        at: &[Direction],
        constraint_left: Constraint,
        constraint_right: Constraint,
    ) -> bool {
        self.add_unbordered_blank(at, Direction::Left, constraint_left, constraint_right)
    }

    /// Add a new unbordered blank layout section to the right of target
    pub fn add_unbordered_blank_right(
        &mut self,
        at: &[Direction],
        constraint_left: Constraint,
        constraint_right: Constraint,
    ) -> bool {
        self.add_unbordered_blank(at, Direction::Right, constraint_left, constraint_right)
    }

    /// Add a new unbordered component layout section above target
    pub fn add_unbordered_component_above(
        &mut self,
        at: &[Direction],
        constraint_top: Constraint,
        onstraint_bottom: Constraint,
        component: Discriminator,
    ) -> bool {
        self.add_unbordered_component(
            at,
            Direction::Up,
            constraint_top,
            onstraint_bottom,
            component,
        )
    }

    /// Add a new unbordered component layout section below target
    pub fn add_unbordered_component_below(
        &mut self,
        at: &[Direction],
        constraint_top: Constraint,
        onstraint_bottom: Constraint,
        component: Discriminator,
    ) -> bool {
        self.add_unbordered_component(
            at,
            Direction::Down,
            constraint_top,
            onstraint_bottom,
            component,
        )
    }

    /// Add a new unbordered component layout section to the left of target
    pub fn add_unbordered_component_left(
        &mut self,
        at: &[Direction],
        constraint_left: Constraint,
        constraint_right: Constraint,
        component: Discriminator,
    ) -> bool {
        self.add_unbordered_component(
            at,
            Direction::Left,
            constraint_left,
            constraint_right,
            component,
        )
    }

    /// Add a new unbordered component layout section to the right of target
    pub fn add_unbordered_component_right(
        &mut self,
        at: &[Direction],
        constraint_left: Constraint,
        constraint_right: Constraint,
        component: Discriminator,
    ) -> bool {
        self.add_unbordered_component(
            at,
            Direction::Right,
            constraint_left,
            constraint_right,
            component,
        )
    }
}
