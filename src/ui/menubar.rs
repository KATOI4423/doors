//! menubar.rs
//!
//! Define the menu bar for the Doors application.

use gpui::*;
use gpui::prelude::FluentBuilder;
use webbrowser;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
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

    fn create_help_menu(&mut self, cx: &mut Context<Self>) -> impl IntoElement {
        div()
            .relative()
            .h_full()
            .flex()
            .items_start()
            .justify_start()
            .child(
                div()
                    .id("help")
                    .px_2()
                    .child("Help")
                    .on_click(cx.listener(|this, _, _, _| {
                        if this.state == MenuBarState::HelpOpen {
                            this.state = MenuBarState::AllClosed;
                        } else {
                            this.state = MenuBarState::HelpOpen;
                        }
                    })),
            ).when(self.state == MenuBarState::HelpOpen, |this| {
                this.child(
                    div()
                        .absolute()
                        .top(px(32.0))
                        .left_0()
                        .w(px(160.0))
                        .bg(rgb(0x303030))
                        .border_1()
                        .border_color(rgb(0x505050))
                        .flex()
                        .flex_col()
                        .child(
                            div()
                                .id("read-me")
                                .px_3()
                                .py_2()
                                .child("Read Me")
                                .on_click(cx.listener(|this, _, _, _| {
                                    this.state = MenuBarState::AllClosed;
                                    let _ = webbrowser::open(std::env!("CARGO_PKG_REPOSITORY"));
                                }))
                        )
                        .child(
                            div()
                                .id("version")
                                .px_3()
                                .py_2()
                                .child("Version"),
                        ),
                )
            })
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
            .child(self.create_help_menu(cx))
    }
}
