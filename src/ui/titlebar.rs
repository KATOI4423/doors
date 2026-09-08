//! titlebar.rs
//!
//! Define the title bar for the Doors application.

use gpui::*;
use heck::ToTitleCase;

pub struct TitleBar {
    title: SharedString,
}

impl Render for TitleBar {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        div()
            .flex()
            .flex_row()
            .bg(rgb(0x000000))
            .text_color(rgb(0xeeeeee))
            .items_center()
            .justify_center()
            .h_8()
            .w_full()
            .pl_2()
            .gap_4()
            .child(self.render_drag_area(window, cx))
            .child(Self::render_minimize_button(window, cx))
            .child(Self::render_maximize_button(window, cx))
            .child(Self::render_close_button(window, cx))
    }
}

impl TitleBar {
    pub fn new(title: impl Into<SharedString>) -> Self {
        Self {
            title: title.into(),
        }
    }

    /// # Set Title
    ///
    /// usage:
    /// ``` rust
    /// titlebar.update(cx, |this, cx| {
    ///     this.set_title("New Title", cx);
    /// });
    /// ```
    pub fn set_title(&mut self, title: impl Into<SharedString>, cx: &mut Context<Self>) {
        self.title = title.into();
        cx.notify();
    }

    fn render_button(
        _cx: &mut Context<Self>,
        id: impl Into<ElementId>,
        icon: impl IntoElement,
        listener: impl Fn(&ClickEvent, &mut Window, &mut App) + 'static,
    ) -> impl IntoElement {
        div()
            .id(id)
            .size(px(32.0))
            .flex()
            .items_center()
            .justify_center()
            .child(icon)
            .on_click(listener)
    }

    fn render_minimize_button(_window: &Window, _cx: &mut Context<Self>) -> impl IntoElement {
        Self::render_button(_cx, "minimize", "−", |_, window, _cx| {
            window.minimize_window();
        })
    }

    /// # Render maimize button
    ///
    /// 最大化 / 通常サイズ の切り替えボタンを作成する
    fn render_maximize_button(window: &Window, _cx: &mut Context<Self>) -> impl IntoElement {
        // TODO: アイコンのSVG化
        let icon = if window.is_maximized() {
            "❐"
        } else {
            "▢"
        };

        Self::render_button(_cx, "maximize", icon, |_, window, _cx| {
            // 通常サイズのときは最大化, 最大サイズのときは通常サイズ化になる
            window.zoom_window();
        })
    }

    fn render_close_button(_window: &Window, _cx: &mut Context<Self>) -> impl IntoElement {
        // TODO: アイコンのSVG化
        Self::render_button(_cx, "close", "✕", |_, window, _cx| {
            window.remove_window();
        })
    }

    /// # Render dragable area
    ///
    /// マウス移動可能なタイトル表示エリアを作成
    fn render_drag_area(&self, _window: &Window, _cx: &mut Context<Self>) -> impl IntoElement {
        div()
            .flex_1()
            .h_full()
            .flex()
            .items_center()
            .pl_3()
            .child(self.title.clone())
            .on_mouse_down(MouseButton::Left, |_, window, _cx| {
                window.start_window_move();
            })
        // TODO: ダブルクリックで最大化・通常サイズ化
    }
}

impl Default for TitleBar {
    fn default() -> Self {
        Self {
            title: std::env!("CARGO_PKG_NAME").to_title_case().into(),
        }
    }
}
