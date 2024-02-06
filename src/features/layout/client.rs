use std::sync::Mutex;

use tokio::sync::OnceCell;

use crate::{
    bindings::{Discriminator, ResponseContent},
    client::Client,
    features::common::Direction,
};

use super::{Border, Constraint, Layout, LayoutRequest};

static LAYOUT_DISCRIM: OnceCell<Mutex<Option<Discriminator>>> =
    OnceCell::const_new_with(Mutex::new(None));

impl Client {
    pub fn layout_set_discrim(&self, discrim: Discriminator) {
        *LAYOUT_DISCRIM.get().unwrap().lock().unwrap() = Some(discrim);
    }

    pub async fn layout_add_with_discrim_raw(
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

    pub async fn layout_add_with_discrim(
        &self,
        layout: Discriminator,
        at: Vec<Direction>,
        split: Direction,
        constraint_1: Constraint,
        constraint_2: Constraint,
        component: Discriminator,
        border: Option<Border>,
    ) -> ResponseContent {
        self.layout_add_with_discrim_raw(
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

    pub async fn layout_add_blank_with_discrim(
        &self,
        layout: Discriminator,
        at: Vec<Direction>,
        split: Direction,
        constraint_1: Constraint,
        constraint_2: Constraint,
        border: Option<Border>,
    ) -> ResponseContent {
        self.layout_add_with_discrim_raw(
            layout,
            at,
            split,
            constraint_1,
            constraint_2,
            None,
            border,
        )
        .await
    }

    pub async fn layout_add(
        &self,
        at: Vec<Direction>,
        split: Direction,
        constraint_1: Constraint,
        constraint_2: Constraint,
        component: Discriminator,
        border: Option<Border>,
    ) -> ResponseContent {
        let layout = LAYOUT_DISCRIM.get().unwrap().lock().unwrap();

        if layout.is_none() {
            return ResponseContent::Undelivered;
        }

        self.layout_add_with_discrim(
            layout.clone().unwrap(),
            at,
            split,
            constraint_1,
            constraint_2,
            component,
            border,
        )
        .await
    }

    pub async fn layout_add_blank(
        &self,
        at: Vec<Direction>,
        split: Direction,
        constraint_1: Constraint,
        constraint_2: Constraint,
        border: Option<Border>,
    ) -> ResponseContent {
        let layout = LAYOUT_DISCRIM.get().unwrap().lock().unwrap();

        if layout.is_none() {
            return ResponseContent::Undelivered;
        }

        self.layout_add_blank_with_discrim(
            layout.clone().unwrap(),
            at,
            split,
            constraint_1,
            constraint_2,
            border,
        )
        .await
    }

    pub async fn layout_set_at_with_discrim(
        &self,
        discrim: Discriminator,
        at: Vec<Direction>,
        layout: Layout,
    ) -> ResponseContent {
        self.message(
            discrim,
            serde_json::to_value(LayoutRequest::SetLayout { at, layout }).unwrap(),
            "!layout-set".to_string(),
        )
        .await
    }

    pub async fn layout_set_at(&self, at: Vec<Direction>, layout: Layout) -> ResponseContent {
        let layout_discrim = LAYOUT_DISCRIM.get().unwrap().lock().unwrap();

        if layout_discrim.is_none() {
            return ResponseContent::Undelivered;
        }

        self.layout_set_at_with_discrim(layout_discrim.as_ref().unwrap().clone(), at, layout)
            .await
    }

    pub async fn layout_set_with_discrim(
        &self,
        discrim: Discriminator,
        layout: Layout,
    ) -> ResponseContent {
        self.layout_set_at_with_discrim(discrim, Vec::new(), layout)
            .await
    }

    pub async fn layout_set(&self, layout: Layout) -> ResponseContent {
        let layout_discrim = LAYOUT_DISCRIM.get().unwrap().lock().unwrap();

        if layout_discrim.is_none() {
            return ResponseContent::Undelivered;
        }

        self.layout_set_at_with_discrim(
            layout_discrim.as_ref().unwrap().clone(),
            Vec::new(),
            layout,
        )
        .await
    }
}
