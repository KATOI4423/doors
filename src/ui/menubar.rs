//! menubar.rs
//!
//! Define the menu bar for the Doors application.

use gpui::*;

enum MenuBarState {
    AllClosed,
    HelpOpen,
}

pub struct MenuBar {
    state: MenuBarState,
}

impl MenuBar {
    pub fn new() -> Self {
        Self {
            state: MenuBarState::AllClosed,
        }
    }
}

impl Render for MenuBar {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        div()
            .flex()
            .flex_row()
            .bg(rgb(0x202020)) // TODO: 色の定義をどこかへまとめる
            .text_color(rgb(0xeeeeee))
            .items_start()
            .justify_start()
            .h_8() // 2rem
            .w_full()
            .pl_2() // pading-left
            .gap_4()
            .child("Menu Bar 1")
            .child("Menu Bar 2")
    }
}
