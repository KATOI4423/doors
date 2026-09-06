//! menubar.rs
//!
//! Define the menu bar for the Doors application.

use gpui::*;
use gpui::prelude::FluentBuilder;
use heck::ToTitleCase;
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

    pub fn register_actions(cx: &mut App) {
        cx.on_action(AboutWindow::show);
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
                                .child("Version")
                                .on_click(cx.listener(|this, _, window, cx| {
                                    this.state = MenuBarState::AllClosed;
                                    window.dispatch_action(Box::new(ShowAbout), cx);
                                }))
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

actions!(menubar, [
    ShowAbout,
]);

struct AboutWindow;

impl Render for AboutWindow {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        div()
            .size_full()
            .bg(rgb(0x202020))
            .flex()
            .flex_col()
            .items_center()
            .justify_center()
            .gap_2()
            .text_color(rgb(0xeeeeee))
            .child(
                div()
                    .text_xl()
                    .child(std::env!("CARGO_PKG_NAME").to_title_case()) // 先頭を大文字にする
            )
            .child(
                div()
                    .whitespace_nowrap()
                    .child(format!("Version: {}", std::env!("CARGO_PKG_VERSION")))
            )
            .child(
                div()
                    .whitespace_nowrap()
                    .child(format!("Target: {}", std::env!("VERGEN_CARGO_TARGET_TRIPLE")))
            )
            .child(
                div()
                    .whitespace_nowrap()
                    .child(format!("Built at {}", std::env!("VERGEN_BUILD_TIMESTAMP")))
            )
            .child(
                div()
                    .whitespace_nowrap()
                    .child(format!("Commit: {}", std::option_env!("VERGEN_GIT_SHA").unwrap_or("unknown")))
            )
            .child(
                div()
                    .whitespace_nowrap()
                    .child(format!("by Rust {}", std::env!("VERGEN_RUSTC_SEMVER")))
            )
    }
}

impl AboutWindow {
    pub fn show(_: &ShowAbout, cx: &mut App) {
        let bounds = Bounds::centered(
            None,
            size(px(500.0), px(250.0)), // TODO: サイズを文字列・フォントに合わせて動的に変える
            cx,
        );

        if let Err(e) = cx.open_window(
            WindowOptions {
                window_bounds: Some(WindowBounds::Windowed(bounds)),
                titlebar: Some(TitlebarOptions {
                    title: Some(format!("About {}", std::env!("CARGO_PKG_NAME").to_title_case()).into()),
                    ..Default::default()
                }),
                ..Default::default()
            },
            |_, cx| cx.new(|_| AboutWindow),
        ) {
            eprintln!("Failed to show About Window: {e}")
        }
    }
}
