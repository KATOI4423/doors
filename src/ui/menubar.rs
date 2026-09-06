//! menubar.rs
//!
//! Define the menu bar for the Doors application.

use gpui::*;

pub struct MenuBar;

impl MenuBar {
    pub fn into_element() -> impl IntoElement {
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
