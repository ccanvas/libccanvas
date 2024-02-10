use std::collections::BTreeMap;

use crate::{
    bindings::{Discriminator, ResponseContent},
    client::Client,
    features::common::Direction,
};

use super::{Border, Constraint, Layout, LayoutComponent, LayoutRequest, LAYOUT_ENV};

impl Client {
    /// Spawn a new process in the same parent space to use ccanvas-layout
    pub async fn spawn_layouted(
        &self,
        label: String,
        command: String,
        args: Vec<String>,
    ) -> ResponseContent {
        self.spawn_with_env(
            label,
            command,
            args,
            BTreeMap::from([(LAYOUT_ENV.to_string(), "1".to_string())]),
        )
        .await
    }

    /// Spawn a new process at a specific space to use ccanvas-layout
    pub async fn spawn_at_layouted(
        &self,
        label: String,
        command: String,
        args: Vec<String>,
        parent: Discriminator,
    ) -> ResponseContent {
        self.spawn_with_env_at(
            label,
            command,
            args,
            parent,
            BTreeMap::from([(LAYOUT_ENV.to_string(), "1".to_string())]),
        )
        .await
    }

    /// Spawn a new process at a specific space with additional env to use ccanvas-layout
    pub async fn spawn_with_env_at_layouted(
        &self,
        label: String,
        command: String,
        args: Vec<String>,
        parent: Discriminator,
        mut env: BTreeMap<String, String>,
    ) -> ResponseContent {
        env.insert(LAYOUT_ENV.to_string(), "1".to_string());
        self.spawn_with_env_at(label, command, args, parent, env)
            .await
    }

    /// Spawn a new process at a specific space with additional env to use ccanvas-layout
    pub async fn spawn_with_env_layouted(
        &self,
        label: String,
        command: String,
        args: Vec<String>,
        mut env: BTreeMap<String, String>,
    ) -> ResponseContent {
        env.insert(LAYOUT_ENV.to_string(), "1".to_string());
        self.spawn_with_env(label, command, args, env).await
    }
}

impl Client {
    #![allow(clippy::too_many_arguments)]

    /// Return the LayoutComponent struct from a discriminator
    pub fn layout(&self, discrim: Discriminator) -> LayoutComponent {
        LayoutComponent::new(self.clone(), discrim)
    }

    /// Add a new layout section
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

    /// Add a new blank layout section
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

    /// Add a new component layout section
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

    /// Add a new layout section below target
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

    /// Add a new layout section above target
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

    /// Add a new layout section to the left of target
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

    /// Add a new layout section to the right of target
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

    /// Add a new bordered layout section
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

    /// Add a new unbordered layout section
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
    /// Add a new blank layout section above target
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

    /// Add a new blank layout section below target
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

    /// Add a new blank layout section to the left of target
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

    /// Add a new blank layout section to the right of target
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

    /// Add a new component layout section above target
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

    /// Add a new component layout section below target
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

    /// Add a new component layout section to the left of target
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

    /// Add a new component layout section to the right of target
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

    /// Add a new bordered layout section above target
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

    /// Add a new bordered layout section below target
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

    /// Add a new bordered layout section to the left of target
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

    /// Add a new bordered layout section to the right of target
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

    /// Add a new unbordered layout section above target
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

    /// Add a new unbordered layout section below target
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

    /// Add a new unbordered layout section to the left of target
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

    /// Add a new unbordered layout section to the right of target
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
    /// Add a new bordered blank layout section
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

    /// Add a new unbordered blank layout section
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
    /// Add a new bordered component layout section
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

    /// Add a new unbordered component layout section
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
    /// Add a new bordered blank layout section above target
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

    /// Add a new bordered blank layout section below target
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

    /// Add a new bordered blank layout section to the left of target
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

    /// Add a new bordered blank layout section to the right of target
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

    /// Add a new bordered component layout section above target
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

    /// Add a new bordered component layout section below target
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

    /// Add a new bordered component layout section to the left of target
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

    /// Add a new bordered component layout section to the right of target
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

    /// Add a new unbordered blank layout section above target
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

    /// Add a new unbordered blank layout section below target
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

    /// Add a new unbordered blank layout section to the left of target
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

    /// Add a new unbordered blank layout section to the right of target
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

    /// Add a new unbordered component layout section above target
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

    /// Add a new unbordered component layout section below target
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

    /// Add a new unbordered component layout section to the left of target
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

    /// Add a new unbordered component layout section to the right of target
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
    /// Remove a layout section
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

    /// Overwrite a layout section
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

    /// Overwrite the root layout section
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
