use crate::{
    bindings::{Discriminator, ResponseContent},
    client::Client,
    features::common::Direction,
};

use super::{Border, Constraint, Layout};

/// Convenient struct representing a ccanvas-layout component
#[derive(Clone)]
pub struct LayoutComponent {
    /// copy of client
    client: Client,
    /// discrim of ccanvas-layout
    discrim: Discriminator,
}

impl LayoutComponent {
    /// Construct a new LayoutComponent from client and discriminator for ccanvas-layout
    pub fn new(client: Client, discrim: Discriminator) -> Self {
        Self { client, discrim }
    }
}

impl LayoutComponent {
    /// Add a new layout section
    pub async fn add(
        &self,
        at: Vec<Direction>,
        split: Direction,
        constraint_1: Constraint,
        constraint_2: Constraint,
        component: Option<Discriminator>,
        border: Option<Border>,
    ) -> ResponseContent {
        self.client
            .layout_add(
                self.discrim.clone(),
                at,
                split,
                constraint_1,
                constraint_2,
                component,
                border,
            )
            .await
    }

    /// Add a new blank layout section
    pub async fn add_blank(
        &self,
        at: Vec<Direction>,
        split: Direction,
        constraint_1: Constraint,
        constraint_2: Constraint,
        border: Option<Border>,
    ) -> ResponseContent {
        self.client
            .layout_add_blank(
                self.discrim.clone(),
                at,
                split,
                constraint_1,
                constraint_2,
                border,
            )
            .await
    }

    /// Add a new component layout section
    pub async fn add_component(
        &self,
        at: Vec<Direction>,
        split: Direction,
        constraint_1: Constraint,
        constraint_2: Constraint,
        component: Discriminator,
        border: Option<Border>,
    ) -> ResponseContent {
        self.client
            .layout_add(
                self.discrim.clone(),
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
    pub async fn add_below(
        &self,
        at: Vec<Direction>,
        constraint_1: Constraint,
        constraint_2: Constraint,
        component: Option<Discriminator>,
        border: Option<Border>,
    ) -> ResponseContent {
        self.client
            .layout_add_below(
                self.discrim.clone(),
                at,
                constraint_1,
                constraint_2,
                component,
                border,
            )
            .await
    }

    /// Add a new layout section above target
    pub async fn add_above(
        &self,
        at: Vec<Direction>,
        constraint_1: Constraint,
        constraint_2: Constraint,
        component: Option<Discriminator>,
        border: Option<Border>,
    ) -> ResponseContent {
        self.client
            .layout_add_above(
                self.discrim.clone(),
                at,
                constraint_1,
                constraint_2,
                component,
                border,
            )
            .await
    }

    /// Add a new layout section to the left of target
    pub async fn add_left(
        &self,
        at: Vec<Direction>,
        constraint_1: Constraint,
        constraint_2: Constraint,
        component: Option<Discriminator>,
        border: Option<Border>,
    ) -> ResponseContent {
        self.client
            .layout_add_left(
                self.discrim.clone(),
                at,
                constraint_1,
                constraint_2,
                component,
                border,
            )
            .await
    }

    /// Add a new layout section to the right of target
    pub async fn add_right(
        &self,
        at: Vec<Direction>,
        constraint_1: Constraint,
        constraint_2: Constraint,
        component: Option<Discriminator>,
        border: Option<Border>,
    ) -> ResponseContent {
        self.client
            .layout_add_right(
                self.discrim.clone(),
                at,
                constraint_1,
                constraint_2,
                component,
                border,
            )
            .await
    }

    /// Add a new bordered layout section
    pub async fn add_bordered(
        &self,
        at: Vec<Direction>,
        split: Direction,
        constraint_1: Constraint,
        constraint_2: Constraint,
        component: Option<Discriminator>,
        border: Border,
    ) -> ResponseContent {
        self.client
            .layout_add_bordered(
                self.discrim.clone(),
                at,
                split,
                constraint_1,
                constraint_2,
                component,
                border,
            )
            .await
    }

    /// Add a new unbordered layout section
    pub async fn add_unbordered(
        &self,
        at: Vec<Direction>,
        split: Direction,
        constraint_1: Constraint,
        constraint_2: Constraint,
        component: Option<Discriminator>,
    ) -> ResponseContent {
        self.client
            .layout_add_unbordered(
                self.discrim.clone(),
                at,
                split,
                constraint_1,
                constraint_2,
                component,
            )
            .await
    }
}

impl LayoutComponent {
    /// Add a new blank layout section above target
    pub async fn add_blank_above(
        &self,
        at: Vec<Direction>,
        constraint_top: Constraint,
        constraint_bottom: Constraint,
        border: Option<Border>,
    ) -> ResponseContent {
        self.client
            .layout_add_blank_above(
                self.discrim.clone(),
                at,
                constraint_top,
                constraint_bottom,
                border,
            )
            .await
    }

    /// Add a new blank layout section below target
    pub async fn add_blank_below(
        &self,
        at: Vec<Direction>,
        constraint_top: Constraint,
        constraint_bottom: Constraint,
        border: Option<Border>,
    ) -> ResponseContent {
        self.client
            .layout_add_blank_below(
                self.discrim.clone(),
                at,
                constraint_top,
                constraint_bottom,
                border,
            )
            .await
    }

    /// Add a new blank layout section to the left of target
    pub async fn add_blank_left(
        &self,
        at: Vec<Direction>,
        constraint_left: Constraint,
        constraint_right: Constraint,
        border: Option<Border>,
    ) -> ResponseContent {
        self.client
            .layout_add_blank_left(
                self.discrim.clone(),
                at,
                constraint_left,
                constraint_right,
                border,
            )
            .await
    }

    /// Add a new blank layout section to the right of target
    pub async fn add_blank_right(
        &self,
        at: Vec<Direction>,
        constraint_left: Constraint,
        constraint_right: Constraint,
        border: Option<Border>,
    ) -> ResponseContent {
        self.client
            .layout_add_blank_right(
                self.discrim.clone(),
                at,
                constraint_left,
                constraint_right,
                border,
            )
            .await
    }

    /// Add a new component layout section above target
    pub async fn add_component_above(
        &self,
        at: Vec<Direction>,
        constraint_top: Constraint,
        constraint_bottom: Constraint,
        component: Discriminator,
        border: Option<Border>,
    ) -> ResponseContent {
        self.client
            .layout_add_component_above(
                self.discrim.clone(),
                at,
                constraint_top,
                constraint_bottom,
                component,
                border,
            )
            .await
    }

    /// Add a new component layout section below target
    pub async fn add_component_below(
        &self,
        at: Vec<Direction>,
        constraint_top: Constraint,
        constraint_bottom: Constraint,
        component: Discriminator,
        border: Option<Border>,
    ) -> ResponseContent {
        self.client
            .layout_add_component_below(
                self.discrim.clone(),
                at,
                constraint_top,
                constraint_bottom,
                component,
                border,
            )
            .await
    }

    /// Add a new component layout section to the left of target
    pub async fn add_component_left(
        &self,
        at: Vec<Direction>,
        constraint_left: Constraint,
        constraint_right: Constraint,
        component: Discriminator,
        border: Option<Border>,
    ) -> ResponseContent {
        self.client
            .layout_add_component_left(
                self.discrim.clone(),
                at,
                constraint_left,
                constraint_right,
                component,
                border,
            )
            .await
    }

    /// Add a new component layout section to the right of target
    pub async fn add_component_right(
        &self,
        at: Vec<Direction>,
        constraint_left: Constraint,
        constraint_right: Constraint,
        component: Discriminator,
        border: Option<Border>,
    ) -> ResponseContent {
        self.client
            .layout_add_component_right(
                self.discrim.clone(),
                at,
                constraint_left,
                constraint_right,
                component,
                border,
            )
            .await
    }

    /// Add a new bordered layout section above target
    pub async fn add_bordered_above(
        &self,
        at: Vec<Direction>,
        constraint_top: Constraint,
        constraint_bottom: Constraint,
        component: Option<Discriminator>,
        border: Border,
    ) -> ResponseContent {
        self.client
            .layout_add_bordered_above(
                self.discrim.clone(),
                at,
                constraint_top,
                constraint_bottom,
                component,
                border,
            )
            .await
    }

    /// Add a new bordered layout section below target
    pub async fn add_bordered_below(
        &self,
        at: Vec<Direction>,
        constraint_top: Constraint,
        constraint_bottom: Constraint,
        component: Option<Discriminator>,
        border: Border,
    ) -> ResponseContent {
        self.client
            .layout_add_bordered_below(
                self.discrim.clone(),
                at,
                constraint_top,
                constraint_bottom,
                component,
                border,
            )
            .await
    }

    /// Add a new bordered layout section to the left of target
    pub async fn add_bordered_left(
        &self,
        at: Vec<Direction>,
        constraint_left: Constraint,
        constraint_right: Constraint,
        component: Option<Discriminator>,
        border: Border,
    ) -> ResponseContent {
        self.client
            .layout_add_bordered_left(
                self.discrim.clone(),
                at,
                constraint_left,
                constraint_right,
                component,
                border,
            )
            .await
    }

    /// Add a new bordered layout section to the right of target
    pub async fn add_bordered_right(
        &self,
        at: Vec<Direction>,
        constraint_left: Constraint,
        constraint_right: Constraint,
        component: Option<Discriminator>,
        border: Border,
    ) -> ResponseContent {
        self.client
            .layout_add_bordered_right(
                self.discrim.clone(),
                at,
                constraint_left,
                constraint_right,
                component,
                border,
            )
            .await
    }

    /// Add a new unbordered layout section above target
    pub async fn add_unbordered_above(
        &self,
        at: Vec<Direction>,
        constraint_top: Constraint,
        constraint_bottom: Constraint,
        component: Option<Discriminator>,
    ) -> ResponseContent {
        self.client
            .layout_add_unbordered_above(
                self.discrim.clone(),
                at,
                constraint_top,
                constraint_bottom,
                component,
            )
            .await
    }

    /// Add a new unbordered layout section below target
    pub async fn add_unbordered_below(
        &self,
        at: Vec<Direction>,
        constraint_top: Constraint,
        constraint_bottom: Constraint,
        component: Option<Discriminator>,
    ) -> ResponseContent {
        self.client
            .layout_add_unbordered_below(
                self.discrim.clone(),
                at,
                constraint_top,
                constraint_bottom,
                component,
            )
            .await
    }

    /// Add a new unbordered layout section to the left of target
    pub async fn add_unbordered_left(
        &self,
        at: Vec<Direction>,
        constraint_left: Constraint,
        constraint_right: Constraint,
        component: Option<Discriminator>,
    ) -> ResponseContent {
        self.client
            .layout_add_unbordered_left(
                self.discrim.clone(),
                at,
                constraint_left,
                constraint_right,
                component,
            )
            .await
    }

    /// Add a new unbordered layout section to the right of target
    pub async fn add_unbordered_right(
        &self,
        at: Vec<Direction>,
        constraint_left: Constraint,
        constraint_right: Constraint,
        component: Option<Discriminator>,
    ) -> ResponseContent {
        self.client
            .layout_add_unbordered_right(
                self.discrim.clone(),
                at,
                constraint_left,
                constraint_right,
                component,
            )
            .await
    }
}

impl LayoutComponent {
    /// Add a new bordered blank layout section
    pub async fn add_bordered_blank(
        &self,
        at: Vec<Direction>,
        split: Direction,
        constraint_1: Constraint,
        constraint_2: Constraint,
        border: Border,
    ) -> ResponseContent {
        self.client
            .layout_add_bordered_blank(
                self.discrim.clone(),
                at,
                split,
                constraint_1,
                constraint_2,
                border,
            )
            .await
    }

    /// Add a new unbordered blank layout section
    pub async fn add_unbordered_blank(
        &self,
        at: Vec<Direction>,
        split: Direction,
        constraint_1: Constraint,
        constraint_2: Constraint,
    ) -> ResponseContent {
        self.client
            .layout_add_unbordered_blank(
                self.discrim.clone(),
                at,
                split,
                constraint_1,
                constraint_2,
            )
            .await
    }

    /// Add a new bordered component layout section
    pub async fn add_bordered_component(
        &self,
        at: Vec<Direction>,
        split: Direction,
        constraint_1: Constraint,
        constraint_2: Constraint,
        component: Discriminator,
        border: Border,
    ) -> ResponseContent {
        self.client
            .layout_add_bordered_component(
                self.discrim.clone(),
                at,
                split,
                constraint_1,
                constraint_2,
                component,
                border,
            )
            .await
    }

    /// Add a new unbordered component layout section
    pub async fn add_unbordered_component(
        &self,
        at: Vec<Direction>,
        split: Direction,
        constraint_1: Constraint,
        constraint_2: Constraint,
        component: Discriminator,
    ) -> ResponseContent {
        self.client
            .layout_add_unbordered_component(
                self.discrim.clone(),
                at,
                split,
                constraint_1,
                constraint_2,
                component,
            )
            .await
    }
}

impl LayoutComponent {
    /// Add a new bordered blank layout section above target
    pub async fn add_bordered_blank_above(
        &self,
        at: Vec<Direction>,
        constraint_top: Constraint,
        constraint_bottom: Constraint,
        border: Border,
    ) -> ResponseContent {
        self.client
            .layout_add_bordered_blank_above(
                self.discrim.clone(),
                at,
                constraint_top,
                constraint_bottom,
                border,
            )
            .await
    }

    /// Add a new bordered blank layout section below target
    pub async fn add_bordered_blank_below(
        &self,
        at: Vec<Direction>,
        constraint_top: Constraint,
        constraint_bottom: Constraint,
        border: Border,
    ) -> ResponseContent {
        self.client
            .layout_add_bordered_blank_below(
                self.discrim.clone(),
                at,
                constraint_top,
                constraint_bottom,
                border,
            )
            .await
    }

    /// Add a new bordered blank layout section to the left of target
    pub async fn add_bordered_blank_left(
        &self,
        at: Vec<Direction>,
        constraint_left: Constraint,
        constraint_right: Constraint,
        border: Border,
    ) -> ResponseContent {
        self.client
            .layout_add_bordered_blank_left(
                self.discrim.clone(),
                at,
                constraint_left,
                constraint_right,
                border,
            )
            .await
    }

    /// Add a new bordered blank layout section to the right of target
    pub async fn add_bordered_blank_right(
        &self,
        at: Vec<Direction>,
        constraint_left: Constraint,
        constraint_right: Constraint,
        border: Border,
    ) -> ResponseContent {
        self.client
            .layout_add_bordered_blank_right(
                self.discrim.clone(),
                at,
                constraint_left,
                constraint_right,
                border,
            )
            .await
    }

    /// Add a new bordered component layout section above target
    pub async fn add_bordered_component_above(
        &self,
        at: Vec<Direction>,
        constraint_top: Constraint,
        constraint_bottom: Constraint,
        component: Discriminator,
        border: Border,
    ) -> ResponseContent {
        self.client
            .layout_add_bordered_component_above(
                self.discrim.clone(),
                at,
                constraint_top,
                constraint_bottom,
                component,
                border,
            )
            .await
    }

    /// Add a new bordered component layout section below target
    pub async fn add_bordered_component_below(
        &self,
        at: Vec<Direction>,
        constraint_top: Constraint,
        constraint_bottom: Constraint,
        component: Discriminator,
        border: Border,
    ) -> ResponseContent {
        self.client
            .layout_add_bordered_component_below(
                self.discrim.clone(),
                at,
                constraint_top,
                constraint_bottom,
                component,
                border,
            )
            .await
    }

    /// Add a new bordered component layout section to the left of target
    pub async fn add_bordered_component_left(
        &self,
        at: Vec<Direction>,
        constraint_left: Constraint,
        constraint_right: Constraint,
        component: Discriminator,
        border: Border,
    ) -> ResponseContent {
        self.client
            .layout_add_bordered_component_left(
                self.discrim.clone(),
                at,
                constraint_left,
                constraint_right,
                component,
                border,
            )
            .await
    }

    /// Add a new bordered component layout section to the right of target
    pub async fn add_bordered_component_right(
        &self,
        at: Vec<Direction>,
        constraint_left: Constraint,
        constraint_right: Constraint,
        component: Discriminator,
        border: Border,
    ) -> ResponseContent {
        self.client
            .layout_add_bordered_component_right(
                self.discrim.clone(),
                at,
                constraint_left,
                constraint_right,
                component,
                border,
            )
            .await
    }

    /// Add a new unbordered blank layout section above target
    pub async fn add_unbordered_blank_above(
        &self,
        at: Vec<Direction>,
        constraint_top: Constraint,
        constraint_bottom: Constraint,
    ) -> ResponseContent {
        self.client
            .layout_add_unbordered_blank_above(
                self.discrim.clone(),
                at,
                constraint_top,
                constraint_bottom,
            )
            .await
    }

    /// Add a new unbordered blank layout section below target
    pub async fn add_unbordered_blank_below(
        &self,
        at: Vec<Direction>,
        constraint_top: Constraint,
        constraint_bottom: Constraint,
    ) -> ResponseContent {
        self.client
            .layout_add_unbordered_blank_below(
                self.discrim.clone(),
                at,
                constraint_top,
                constraint_bottom,
            )
            .await
    }

    /// Add a new unbordered blank layout section to the left of target
    pub async fn add_unbordered_blank_left(
        &self,
        at: Vec<Direction>,
        constraint_left: Constraint,
        constraint_right: Constraint,
    ) -> ResponseContent {
        self.client
            .layout_add_unbordered_blank_left(
                self.discrim.clone(),
                at,
                constraint_left,
                constraint_right,
            )
            .await
    }

    /// Add a new unbordered blank layout section to the right of target
    pub async fn add_unbordered_blank_right(
        &self,
        at: Vec<Direction>,
        constraint_left: Constraint,
        constraint_right: Constraint,
    ) -> ResponseContent {
        self.client
            .layout_add_unbordered_blank_right(
                self.discrim.clone(),
                at,
                constraint_left,
                constraint_right,
            )
            .await
    }

    /// Add a new unbordered component layout section above target
    pub async fn add_unbordered_component_above(
        &self,
        at: Vec<Direction>,
        constraint_top: Constraint,
        constraint_bottom: Constraint,
        component: Discriminator,
    ) -> ResponseContent {
        self.client
            .layout_add_unbordered_component_above(
                self.discrim.clone(),
                at,
                constraint_top,
                constraint_bottom,
                component,
            )
            .await
    }

    /// Add a new unbordered component layout section below target
    pub async fn add_unbordered_component_below(
        &self,
        at: Vec<Direction>,
        constraint_top: Constraint,
        constraint_bottom: Constraint,
        component: Discriminator,
    ) -> ResponseContent {
        self.client
            .layout_add_unbordered_component_below(
                self.discrim.clone(),
                at,
                constraint_top,
                constraint_bottom,
                component,
            )
            .await
    }

    /// Add a new unbordered component layout section to the left of target
    pub async fn add_unbordered_component_left(
        &self,
        at: Vec<Direction>,
        constraint_left: Constraint,
        constraint_right: Constraint,
        component: Discriminator,
    ) -> ResponseContent {
        self.client
            .layout_add_unbordered_component_left(
                self.discrim.clone(),
                at,
                constraint_left,
                constraint_right,
                component,
            )
            .await
    }

    /// Add a new unbordered component layout section to the right of target
    pub async fn add_unbordered_component_right(
        &self,
        at: Vec<Direction>,
        constraint_left: Constraint,
        constraint_right: Constraint,
        component: Discriminator,
    ) -> ResponseContent {
        self.client
            .layout_add_unbordered_component_right(
                self.discrim.clone(),
                at,
                constraint_left,
                constraint_right,
                component,
            )
            .await
    }
}

impl LayoutComponent {
    /// Remove a layout section
    pub async fn remove(&self, at: Vec<Direction>) -> ResponseContent {
        self.client.layout_remove(self.discrim.clone(), at).await
    }

    /// Overwrite a layout section
    pub async fn set(&self, at: Vec<Direction>, layout: Layout) -> ResponseContent {
        self.client
            .layout_set(self.discrim.clone(), at, layout)
            .await
    }

    /// Overwrite the root layout section
    pub async fn set_root(&self, layout: Layout) -> ResponseContent {
        self.client
            .layout_set_root(self.discrim.clone(), layout)
            .await
    }
}
