use crate::{
    bindings::{Discriminator, ResponseContent},
    client::Client,
    features::common::Direction,
};

use super::{Border, Constraint, Layout, LayoutComponent, LayoutRequest};

impl Client {
    #![allow(clippy::too_many_arguments)]
    pub fn layout(&self, discrim: Discriminator) -> LayoutComponent {
        LayoutComponent::new(self.clone(), discrim)
    }

    pub async fn layout_add(
        &self,
        layout: Discriminator,
        at: Vec<Direction>,
        split: Direction,
        constraint_1: Constraint,
        constraint_2: Constraint,
        component: Option<Discriminator>,
        border: Option<Border>,
    ) -> ResponseContent {
        self.message(
            layout,
            serde_json::to_value(LayoutRequest::Add {
                at,
                split,
                constraint_1,
                constraint_2,
                component,
                border,
            })
            .unwrap(),
            "!layout-add".to_string(),
        )
        .await
    }

    pub async fn layout_add_blank(
        &self,
        layout: Discriminator,
        at: Vec<Direction>,
        split: Direction,
        constraint_1: Constraint,
        constraint_2: Constraint,
        border: Option<Border>,
    ) -> ResponseContent {
        self.layout_add(layout, at, split, constraint_1, constraint_2, None, border)
            .await
    }

    pub async fn layout_add_component(
        &self,
        layout: Discriminator,
        at: Vec<Direction>,
        split: Direction,
        constraint_1: Constraint,
        constraint_2: Constraint,
        component: Discriminator,
        border: Option<Border>,
    ) -> ResponseContent {
        self.layout_add(
            layout,
            at,
            split,
            constraint_1,
            constraint_2,
            Some(component),
            border,
        )
        .await
    }

    pub async fn layout_add_below(
        &self,
        layout: Discriminator,
        at: Vec<Direction>,
        constraint_top: Constraint,
        constraint_bottom: Constraint,
        component: Option<Discriminator>,
        border: Option<Border>,
    ) -> ResponseContent {
        self.layout_add(
            layout,
            at,
            Direction::Down,
            constraint_top,
            constraint_bottom,
            component,
            border,
        )
        .await
    }

    pub async fn layout_add_above(
        &self,
        layout: Discriminator,
        at: Vec<Direction>,
        constraint_top: Constraint,
        constraint_bottom: Constraint,
        component: Option<Discriminator>,
        border: Option<Border>,
    ) -> ResponseContent {
        self.layout_add(
            layout,
            at,
            Direction::Up,
            constraint_top,
            constraint_bottom,
            component,
            border,
        )
        .await
    }

    pub async fn layout_add_left(
        &self,
        layout: Discriminator,
        at: Vec<Direction>,
        constraint_left: Constraint,
        constraint_right: Constraint,
        component: Option<Discriminator>,
        border: Option<Border>,
    ) -> ResponseContent {
        self.layout_add(
            layout,
            at,
            Direction::Left,
            constraint_left,
            constraint_right,
            component,
            border,
        )
        .await
    }

    pub async fn layout_add_right(
        &self,
        layout: Discriminator,
        at: Vec<Direction>,
        constraint_left: Constraint,
        constraint_right: Constraint,
        component: Option<Discriminator>,
        border: Option<Border>,
    ) -> ResponseContent {
        self.layout_add(
            layout,
            at,
            Direction::Right,
            constraint_left,
            constraint_right,
            component,
            border,
        )
        .await
    }

    pub async fn layout_add_bordered(
        &self,
        layout: Discriminator,
        at: Vec<Direction>,
        split: Direction,
        constraint_1: Constraint,
        constraint_2: Constraint,
        component: Option<Discriminator>,
        border: Border,
    ) -> ResponseContent {
        self.layout_add(
            layout,
            at,
            split,
            constraint_1,
            constraint_2,
            component,
            Some(border),
        )
        .await
    }

    pub async fn layout_add_unbordered(
        &self,
        layout: Discriminator,
        at: Vec<Direction>,
        split: Direction,
        constraint_1: Constraint,
        constraint_2: Constraint,
        component: Option<Discriminator>,
    ) -> ResponseContent {
        self.layout_add(
            layout,
            at,
            split,
            constraint_1,
            constraint_2,
            component,
            None,
        )
        .await
    }
}

impl Client {
    pub async fn layout_add_blank_above(
        &self,
        layout: Discriminator,
        at: Vec<Direction>,
        constraint_top: Constraint,
        constraint_bottom: Constraint,
        border: Option<Border>,
    ) -> ResponseContent {
        self.layout_add_blank(
            layout,
            at,
            Direction::Up,
            constraint_top,
            constraint_bottom,
            border,
        )
        .await
    }

    pub async fn layout_add_blank_below(
        &self,
        layout: Discriminator,
        at: Vec<Direction>,
        constraint_top: Constraint,
        constraint_bottom: Constraint,
        border: Option<Border>,
    ) -> ResponseContent {
        self.layout_add_blank(
            layout,
            at,
            Direction::Down,
            constraint_top,
            constraint_bottom,
            border,
        )
        .await
    }

    pub async fn layout_add_blank_left(
        &self,
        layout: Discriminator,
        at: Vec<Direction>,
        constraint_left: Constraint,
        constraint_right: Constraint,
        border: Option<Border>,
    ) -> ResponseContent {
        self.layout_add_blank(
            layout,
            at,
            Direction::Left,
            constraint_left,
            constraint_right,
            border,
        )
        .await
    }

    pub async fn layout_add_blank_right(
        &self,
        layout: Discriminator,
        at: Vec<Direction>,
        constraint_left: Constraint,
        constraint_right: Constraint,
        border: Option<Border>,
    ) -> ResponseContent {
        self.layout_add_blank(
            layout,
            at,
            Direction::Right,
            constraint_left,
            constraint_right,
            border,
        )
        .await
    }

    pub async fn layout_add_component_above(
        &self,
        layout: Discriminator,
        at: Vec<Direction>,
        constraint_top: Constraint,
        constraint_bottom: Constraint,
        component: Discriminator,
        border: Option<Border>,
    ) -> ResponseContent {
        self.layout_add_component(
            layout,
            at,
            Direction::Up,
            constraint_top,
            constraint_bottom,
            component,
            border,
        )
        .await
    }

    pub async fn layout_add_component_below(
        &self,
        layout: Discriminator,
        at: Vec<Direction>,
        constraint_top: Constraint,
        constraint_bottom: Constraint,
        component: Discriminator,
        border: Option<Border>,
    ) -> ResponseContent {
        self.layout_add_component(
            layout,
            at,
            Direction::Down,
            constraint_top,
            constraint_bottom,
            component,
            border,
        )
        .await
    }

    pub async fn layout_add_component_left(
        &self,
        layout: Discriminator,
        at: Vec<Direction>,
        constraint_left: Constraint,
        constraint_right: Constraint,
        component: Discriminator,
        border: Option<Border>,
    ) -> ResponseContent {
        self.layout_add_component(
            layout,
            at,
            Direction::Left,
            constraint_left,
            constraint_right,
            component,
            border,
        )
        .await
    }

    pub async fn layout_add_component_right(
        &self,
        layout: Discriminator,
        at: Vec<Direction>,
        constraint_left: Constraint,
        constraint_right: Constraint,
        component: Discriminator,
        border: Option<Border>,
    ) -> ResponseContent {
        self.layout_add_component(
            layout,
            at,
            Direction::Right,
            constraint_left,
            constraint_right,
            component,
            border,
        )
        .await
    }

    pub async fn layout_add_bordered_above(
        &self,
        layout: Discriminator,
        at: Vec<Direction>,
        constraint_top: Constraint,
        constraint_bottom: Constraint,
        component: Option<Discriminator>,
        border: Border,
    ) -> ResponseContent {
        self.layout_add_bordered(
            layout,
            at,
            Direction::Up,
            constraint_top,
            constraint_bottom,
            component,
            border,
        )
        .await
    }

    pub async fn layout_add_bordered_below(
        &self,
        layout: Discriminator,
        at: Vec<Direction>,
        constraint_top: Constraint,
        constraint_bottom: Constraint,
        component: Option<Discriminator>,
        border: Border,
    ) -> ResponseContent {
        self.layout_add_bordered(
            layout,
            at,
            Direction::Down,
            constraint_top,
            constraint_bottom,
            component,
            border,
        )
        .await
    }

    pub async fn layout_add_bordered_left(
        &self,
        layout: Discriminator,
        at: Vec<Direction>,
        constraint_left: Constraint,
        constraint_right: Constraint,
        component: Option<Discriminator>,
        border: Border,
    ) -> ResponseContent {
        self.layout_add_bordered(
            layout,
            at,
            Direction::Left,
            constraint_left,
            constraint_right,
            component,
            border,
        )
        .await
    }

    pub async fn layout_add_bordered_right(
        &self,
        layout: Discriminator,
        at: Vec<Direction>,
        constraint_left: Constraint,
        constraint_right: Constraint,
        component: Option<Discriminator>,
        border: Border,
    ) -> ResponseContent {
        self.layout_add_bordered(
            layout,
            at,
            Direction::Right,
            constraint_left,
            constraint_right,
            component,
            border,
        )
        .await
    }

    pub async fn layout_add_unbordered_above(
        &self,
        layout: Discriminator,
        at: Vec<Direction>,
        constraint_top: Constraint,
        constraint_bottom: Constraint,
        component: Option<Discriminator>,
    ) -> ResponseContent {
        self.layout_add_unbordered(
            layout,
            at,
            Direction::Up,
            constraint_top,
            constraint_bottom,
            component,
        )
        .await
    }

    pub async fn layout_add_unbordered_below(
        &self,
        layout: Discriminator,
        at: Vec<Direction>,
        constraint_top: Constraint,
        constraint_bottom: Constraint,
        component: Option<Discriminator>,
    ) -> ResponseContent {
        self.layout_add_unbordered(
            layout,
            at,
            Direction::Down,
            constraint_top,
            constraint_bottom,
            component,
        )
        .await
    }

    pub async fn layout_add_unbordered_left(
        &self,
        layout: Discriminator,
        at: Vec<Direction>,
        constraint_left: Constraint,
        constraint_right: Constraint,
        component: Option<Discriminator>,
    ) -> ResponseContent {
        self.layout_add_unbordered(
            layout,
            at,
            Direction::Left,
            constraint_left,
            constraint_right,
            component,
        )
        .await
    }

    pub async fn layout_add_unbordered_right(
        &self,
        layout: Discriminator,
        at: Vec<Direction>,
        constraint_left: Constraint,
        constraint_right: Constraint,
        component: Option<Discriminator>,
    ) -> ResponseContent {
        self.layout_add_unbordered(
            layout,
            at,
            Direction::Right,
            constraint_left,
            constraint_right,
            component,
        )
        .await
    }
}

impl Client {
    pub async fn layout_add_bordered_blank(
        &self,
        layout: Discriminator,
        at: Vec<Direction>,
        split: Direction,
        constraint_1: Constraint,
        constraint_2: Constraint,
        border: Border,
    ) -> ResponseContent {
        self.layout_add_bordered(layout, at, split, constraint_1, constraint_2, None, border)
            .await
    }

    pub async fn layout_add_unbordered_blank(
        &self,
        layout: Discriminator,
        at: Vec<Direction>,
        split: Direction,
        constraint_1: Constraint,
        constraint_2: Constraint,
    ) -> ResponseContent {
        self.layout_add_unbordered(layout, at, split, constraint_1, constraint_2, None)
            .await
    }

    #[allow(clippy::too_many_arguments)]
    pub async fn layout_add_bordered_component(
        &self,
        layout: Discriminator,
        at: Vec<Direction>,
        split: Direction,
        constraint_1: Constraint,
        constraint_2: Constraint,
        component: Discriminator,
        border: Border,
    ) -> ResponseContent {
        self.layout_add_bordered(
            layout,
            at,
            split,
            constraint_1,
            constraint_2,
            Some(component),
            border,
        )
        .await
    }

    pub async fn layout_add_unbordered_component(
        &self,
        layout: Discriminator,
        at: Vec<Direction>,
        split: Direction,
        constraint_1: Constraint,
        constraint_2: Constraint,
        component: Discriminator,
    ) -> ResponseContent {
        self.layout_add_unbordered(
            layout,
            at,
            split,
            constraint_1,
            constraint_2,
            Some(component),
        )
        .await
    }
}

impl Client {
    pub async fn layout_add_bordered_blank_above(
        &self,
        layout: Discriminator,
        at: Vec<Direction>,
        constraint_top: Constraint,
        constraint_bottom: Constraint,
        border: Border,
    ) -> ResponseContent {
        self.layout_add_bordered_blank(
            layout,
            at,
            Direction::Up,
            constraint_top,
            constraint_bottom,
            border,
        )
        .await
    }

    pub async fn layout_add_bordered_blank_below(
        &self,
        layout: Discriminator,
        at: Vec<Direction>,
        constraint_top: Constraint,
        constraint_bottom: Constraint,
        border: Border,
    ) -> ResponseContent {
        self.layout_add_bordered_blank(
            layout,
            at,
            Direction::Down,
            constraint_top,
            constraint_bottom,
            border,
        )
        .await
    }

    pub async fn layout_add_bordered_blank_left(
        &self,
        layout: Discriminator,
        at: Vec<Direction>,
        constraint_left: Constraint,
        constraint_right: Constraint,
        border: Border,
    ) -> ResponseContent {
        self.layout_add_bordered_blank(
            layout,
            at,
            Direction::Left,
            constraint_left,
            constraint_right,
            border,
        )
        .await
    }

    pub async fn layout_add_bordered_blank_right(
        &self,
        layout: Discriminator,
        at: Vec<Direction>,
        constraint_left: Constraint,
        constraint_right: Constraint,
        border: Border,
    ) -> ResponseContent {
        self.layout_add_bordered_blank(
            layout,
            at,
            Direction::Right,
            constraint_left,
            constraint_right,
            border,
        )
        .await
    }

    pub async fn layout_add_bordered_component_above(
        &self,
        layout: Discriminator,
        at: Vec<Direction>,
        constraint_top: Constraint,
        constraint_bottom: Constraint,
        component: Discriminator,
        border: Border,
    ) -> ResponseContent {
        self.layout_add_bordered_component(
            layout,
            at,
            Direction::Up,
            constraint_top,
            constraint_bottom,
            component,
            border,
        )
        .await
    }

    pub async fn layout_add_bordered_component_below(
        &self,
        layout: Discriminator,
        at: Vec<Direction>,
        constraint_top: Constraint,
        constraint_bottom: Constraint,
        component: Discriminator,
        border: Border,
    ) -> ResponseContent {
        self.layout_add_bordered_component(
            layout,
            at,
            Direction::Down,
            constraint_top,
            constraint_bottom,
            component,
            border,
        )
        .await
    }

    pub async fn layout_add_bordered_component_left(
        &self,
        layout: Discriminator,
        at: Vec<Direction>,
        constraint_left: Constraint,
        constraint_right: Constraint,
        component: Discriminator,
        border: Border,
    ) -> ResponseContent {
        self.layout_add_bordered_component(
            layout,
            at,
            Direction::Left,
            constraint_left,
            constraint_right,
            component,
            border,
        )
        .await
    }

    pub async fn layout_add_bordered_component_right(
        &self,
        layout: Discriminator,
        at: Vec<Direction>,
        constraint_left: Constraint,
        constraint_right: Constraint,
        component: Discriminator,
        border: Border,
    ) -> ResponseContent {
        self.layout_add_bordered_component(
            layout,
            at,
            Direction::Right,
            constraint_left,
            constraint_right,
            component,
            border,
        )
        .await
    }

    pub async fn layout_add_unbordered_blank_above(
        &self,
        layout: Discriminator,
        at: Vec<Direction>,
        constraint_top: Constraint,
        constraint_bottom: Constraint,
    ) -> ResponseContent {
        self.layout_add_unbordered_blank(
            layout,
            at,
            Direction::Up,
            constraint_top,
            constraint_bottom,
        )
        .await
    }

    pub async fn layout_add_unbordered_blank_below(
        &self,
        layout: Discriminator,
        at: Vec<Direction>,
        constraint_top: Constraint,
        constraint_bottom: Constraint,
    ) -> ResponseContent {
        self.layout_add_unbordered_blank(
            layout,
            at,
            Direction::Down,
            constraint_top,
            constraint_bottom,
        )
        .await
    }

    pub async fn layout_add_unbordered_blank_left(
        &self,
        layout: Discriminator,
        at: Vec<Direction>,
        constraint_left: Constraint,
        constraint_right: Constraint,
    ) -> ResponseContent {
        self.layout_add_unbordered_blank(
            layout,
            at,
            Direction::Left,
            constraint_left,
            constraint_right,
        )
        .await
    }

    pub async fn layout_add_unbordered_blank_right(
        &self,
        layout: Discriminator,
        at: Vec<Direction>,
        constraint_left: Constraint,
        constraint_right: Constraint,
    ) -> ResponseContent {
        self.layout_add_unbordered_blank(
            layout,
            at,
            Direction::Right,
            constraint_left,
            constraint_right,
        )
        .await
    }

    pub async fn layout_add_unbordered_component_above(
        &self,
        layout: Discriminator,
        at: Vec<Direction>,
        constraint_top: Constraint,
        constraint_bottom: Constraint,
        component: Discriminator,
    ) -> ResponseContent {
        self.layout_add_unbordered_component(
            layout,
            at,
            Direction::Up,
            constraint_top,
            constraint_bottom,
            component,
        )
        .await
    }

    pub async fn layout_add_unbordered_component_below(
        &self,
        layout: Discriminator,
        at: Vec<Direction>,
        constraint_top: Constraint,
        constraint_bottom: Constraint,
        component: Discriminator,
    ) -> ResponseContent {
        self.layout_add_unbordered_component(
            layout,
            at,
            Direction::Down,
            constraint_top,
            constraint_bottom,
            component,
        )
        .await
    }

    pub async fn layout_add_unbordered_component_left(
        &self,
        layout: Discriminator,
        at: Vec<Direction>,
        constraint_left: Constraint,
        constraint_right: Constraint,
        component: Discriminator,
    ) -> ResponseContent {
        self.layout_add_unbordered_component(
            layout,
            at,
            Direction::Left,
            constraint_left,
            constraint_right,
            component,
        )
        .await
    }

    pub async fn layout_add_unbordered_component_right(
        &self,
        layout: Discriminator,
        at: Vec<Direction>,
        constraint_left: Constraint,
        constraint_right: Constraint,
        component: Discriminator,
    ) -> ResponseContent {
        self.layout_add_unbordered_component(
            layout,
            at,
            Direction::Right,
            constraint_left,
            constraint_right,
            component,
        )
        .await
    }
}

impl Client {
    pub async fn layout_remove(
        &self,
        layout: Discriminator,
        at: Vec<Direction>,
    ) -> ResponseContent {
        self.message(
            layout,
            serde_json::to_value(LayoutRequest::Remove { at }).unwrap(),
            "!layout-remove".to_string(),
        )
        .await
    }

    pub async fn layout_set(
        &self,
        layout: Discriminator,
        at: Vec<Direction>,
        overwrite: Layout,
    ) -> ResponseContent {
        self.message(
            layout,
            serde_json::to_value(LayoutRequest::SetLayout {
                at,
                layout: overwrite,
            })
            .unwrap(),
            "!layout-set".to_string(),
        )
        .await
    }

    pub async fn layout_set_root(
        &self,
        layout: Discriminator,
        overwrite: Layout,
    ) -> ResponseContent {
        self.message(
            layout,
            serde_json::to_value(LayoutRequest::SetLayout {
                at: Vec::new(),
                layout: overwrite,
            })
            .unwrap(),
            "!layout-set".to_string(),
        )
        .await
    }
}
