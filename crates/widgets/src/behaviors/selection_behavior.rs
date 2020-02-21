use std::cell::Cell;

use crate::prelude::*;

/// The `SelectionBehaviorState` handles the `SelectionBehavior` widget.
#[derive(Default)]
pub struct SelectionBehaviorState {
    selected: Cell<bool>,
}

impl SelectionBehaviorState {
    fn toggle_selection(&self) {
        self.selected.set(!self.selected.get());
    }
}

impl State for SelectionBehaviorState {
    fn update(&self, ctx: &mut Context<'_>) {
        if !ctx.widget().get::<bool>("enabled")
            || *ctx.widget().get::<bool>("selected") == self.selected.get()
        {
            return;
        }

        ctx.widget().set("selected", self.selected.get());

        let element = ctx.widget().clone::<Selector>("selector").element.unwrap();

        if let Some(parent) = ctx.parent_entity_by_element(&*element) {
            ctx.get_widget(parent).update_theme_by_state(false);
        }
    }
}

widget!(
    /// The `SelectionBehavior` widget is used to handle internal the pressed behavior of a widget.
    /// 
    /// **CSS element:** `check-box`
    SelectionBehavior<SelectionBehaviorState>: MouseHandler {
        /// Sets or shares the css selector property. 
        selector: Selector,

        /// Sets or shares the selected property. 
        selected: bool
    }
);

impl Template for SelectionBehavior {
    fn template(self, _: Entity, _: &mut BuildContext) -> Self {
        let state = self.clone_state();

        self.name("SelectionBehavior")
            .selector("")
            .selected(true)
            .on_click(move |_| {
                state.toggle_selection();
                false
            })
    }
}
