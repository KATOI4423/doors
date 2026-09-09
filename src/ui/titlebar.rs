//! titlebar.rs
//!
//! Define the title bar for the Doors application.

use bitflags::bitflags;
use gpui::*;
use heck::ToTitleCase;

pub struct TitleBar {
    title: SharedString,
    flags: TitleBarFlags,
}

bitflags! {
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub struct TitleBarFlags: u32 {
        const CloseButton =     0b0001;
        const MaximizeButton =  0b0010;
        const MinimizeButton =  0b0100;
        const TitleName  =      0b1000;
        const WindowMove =      0b0001_0000;
    }
}

impl Default for TitleBarFlags {
    fn default() -> Self {
        Self::CloseButton |
        Self::MaximizeButton |
        Self::MinimizeButton |
        Self::TitleName |
        Self::WindowMove |
        Self::empty() // Keep the trailing `|` style for minimal diffs.
    }
}

impl Render for TitleBar {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let mut content = div()
            .flex()
            .flex_row()
            .bg(rgb(0x000000))
            .text_color(rgb(0xeeeeee))
            .h_8()
            .w_full();

        let mut title = if self.flags.contains(TitleBarFlags::TitleName) {
                self.render_title_name(window, cx)
                    .flex_1()
        } else {
            // コントロールボタンを右端へ押し出すために空白を挿入
            div().flex_1()
        };
        if self.flags.contains(TitleBarFlags::WindowMove) {
            title = title.on_mouse_down(MouseButton::Left, |_, window, _cx| {
                window.start_window_move();
            });
        }
        content = content.child(title);

        // TODO: ダブルクリックで最大化・通常サイズ化

        if self.flags.intersects(
            TitleBarFlags::MinimizeButton | TitleBarFlags::MaximizeButton | TitleBarFlags::CloseButton
        ) {
            content = content.child(self.render_controls(window, cx));
        }

        content
    }
}

impl TitleBar {
    pub fn new(title: impl Into<SharedString>, flags: TitleBarFlags) -> Self {
        Self {
            title: title.into(),
            flags,
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

    fn render_controls(&self, window: &Window, cx: &mut Context<Self>) -> impl IntoElement {
        let mut controls = div()
            .flex()
            .flex_row()
            .gap_4();

        if self.flags.contains(TitleBarFlags::MinimizeButton) {
            controls = controls.child(Self::render_minimize_button(window, cx))
        }
        if self.flags.contains(TitleBarFlags::MaximizeButton) {
            controls = controls.child(Self::render_maximize_button(window, cx))
        }
        if self.flags.contains(TitleBarFlags::CloseButton) {
            controls = controls.child(Self::render_close_button(window, cx))
        }

        controls
    }

    /// # Render Title Name
    ///
    /// タイトル表示エリアを作成
    fn render_title_name(&self, _window: &Window, _cx: &mut Context<Self>) -> gpui::Div {
        div()
            .h_full()
            .flex()
            .items_center()
            .pl_3()
            .child(self.title.clone())
    }
}

impl Default for TitleBar {
    fn default() -> Self {
        Self {
            title: std::env!("CARGO_PKG_NAME").to_title_case().into(),
            flags: TitleBarFlags::default(),
        }
    }
}
