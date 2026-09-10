//! titlebar.rs
//!
//! Define the title bar for the Doors application.

use bitflags::bitflags;
use gpui::{prelude::FluentBuilder, *};
use heck::ToTitleCase;

pub struct TitleBar {
    title: SharedString,
    flags: TitleBarFlags,
    states: TitleBarStates,
}

bitflags! {
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub struct TitleBarFlags: u32 {
        const CloseButton =     0b0001;
        const MaximizeButton =  0b0010;
        const MinimizeButton =  0b0100;
        const TitleName  =      0b1000;
        const WindowMove =      0b0001_0000;
        const DoubleClickMaximize = 0b0010_0000;
        const SettingButton =   0b0100_0000;
    }
}

bitflags! {
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    struct TitleBarStates: u32 {
        const Movable   = 0b0001;
        const SettingOpen = 0b0010;
    }
}

impl Default for TitleBarFlags {
    fn default() -> Self {
        Self::CloseButton |
        Self::MaximizeButton |
        Self::MinimizeButton |
        Self::TitleName |
        Self::WindowMove |
        Self::DoubleClickMaximize |
        Self::SettingButton |
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
            title = Self::setup_window_move(title, cx);
        }
        if self.flags.contains(TitleBarFlags::DoubleClickMaximize) {
            title = title.on_mouse_down(MouseButton::Left, cx.listener(|_, event: &MouseDownEvent, window, _| {
                if event.click_count == 2 {
                    window.zoom_window();
                }
            }));
        }
        content = content.child(title);

        if self.flags.intersects(
            TitleBarFlags::MinimizeButton | TitleBarFlags::MaximizeButton | TitleBarFlags::CloseButton
                | TitleBarFlags::SettingButton
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
            states: TitleBarStates::empty(),
        }
    }

    pub fn register_actions(cx: &mut App) {
        cx.on_action(AboutWindow::show);
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

    fn setup_window_move(title: Div, cx: & Context<Self>) -> Div {
        title.on_mouse_down(MouseButton::Left, cx.listener(|this, _, _, _| {
            this.states = this.states | TitleBarStates::Movable;
        }))
        .on_mouse_move(cx.listener(|this, _, window, _| {
            if this.states.contains(TitleBarStates::Movable) {
                window.start_window_move();
            }
        }))
        .on_mouse_up(MouseButton::Left, cx.listener(|this, _, _, _| {
            this.states = this.states - TitleBarStates::Movable;
        }))
        .on_mouse_up_out(MouseButton::Left, cx.listener(|this, _, _, _| {
            this.states = this.states - TitleBarStates::Movable;
        }))
    }

    fn render_button(
        id: impl Into<ElementId>,
        icon_path: impl Into<SharedString>,
        listener: impl Fn(&ClickEvent, &mut Window, &mut App) + 'static,
    ) -> Stateful<Div> {
        div()
            .id(id)
            .size(px(32.0))
            .flex()
            .items_center()
            .justify_center()
            .child(
                svg()
                    .path(icon_path)
                    .size(px(16.0))
                    .text_color(rgb(0xffffff))
            )
            .on_click(listener)
    }

    fn render_minimize_button(_window: &Window, _cx: &mut Context<Self>) -> impl IntoElement {
        Self::render_button("minimize", "icons/window-minimize.svg", |_, window, _cx| {
            window.minimize_window();
        })
    }

    /// # Render maimize button
    ///
    /// 最大化 / 通常サイズ の切り替えボタンを作成する
    fn render_maximize_button(window: &Window, _cx: &mut Context<Self>) -> impl IntoElement {
        let path = if window.is_maximized() {
            "icons/window-restore.svg"
        } else {
            "icons/window-maximize.svg"
        };

        Self::render_button("maximize", path, |_, window, _cx| {
            // 通常サイズのときは最大化, 最大サイズのときは通常サイズ化になる
            window.zoom_window();
        })
    }

    fn render_close_button(_window: &Window, _cx: &mut Context<Self>) -> impl IntoElement {
        Self::render_button("close", "icons/window-close.svg", |_, window, _cx| {
            window.remove_window();
        })
    }

    pub fn close_setting_pulldown(&mut self, cx: &mut Context<Self>) {
        if self.states.contains(TitleBarStates::SettingOpen) {
            self.states -= TitleBarStates::SettingOpen;
            cx.notify();
        }
    }

    pub fn open_setting_pulldown(&mut self, cx: &mut Context<Self>) {
        if !self.states.contains(TitleBarStates::SettingOpen) {
            self.states |= TitleBarStates::SettingOpen;
            cx.notify();
        }
    }

    fn render_setting_button(_window: &Window, cx: &mut Context<Self>) -> impl IntoElement {
        Self::render_button("setting", "icons/setting.svg", cx.listener(|this, _, _, cx| {
            if this.states.contains(TitleBarStates::SettingOpen) {
                this.close_setting_pulldown(cx);
            } else {
                this.open_setting_pulldown(cx);
            }
        }))
        .on_mouse_down(MouseButton::Left, |_, _, cx| {
            cx.stop_propagation(); // MainWindow の titlebar.close_setting_pulldown() を呼ばないようにするため、親へイベントを伝搬させるのを停止する
        })
    }

    fn render_controls(&self, window: &Window, cx: &mut Context<Self>) -> impl IntoElement {
        let mut controls = div()
            .flex()
            .flex_row()
            .gap_4();
        if self.flags.contains(TitleBarFlags::SettingButton) {
            controls = controls.child(Self::render_setting_button(window, cx))
                .when(self.states.contains(TitleBarStates::SettingOpen), |this| {
                    this.child(self.render_setting_pulldown(window, cx))
                });
        }

        if self.flags.contains(TitleBarFlags::MinimizeButton) {
            controls = controls.child(Self::render_minimize_button(window, cx));
        }
        if self.flags.contains(TitleBarFlags::MaximizeButton) {
            controls = controls.child(Self::render_maximize_button(window, cx));
        }
        if self.flags.contains(TitleBarFlags::CloseButton) {
            controls = controls.child(Self::render_close_button(window, cx));
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

    fn render_setting_pulldown(&self, _window: &Window, cx: &mut Context<Self>) -> gpui::Div {
        div()
            .absolute()
            .top_8()
            .left_0()
            .w_40()
            .bg(rgb(0x303030))
            .border_1()
            .border_color(rgb(0x505050))
            .flex()
            .flex_col()
            .child("Setting 1")
            .child(Self::render_horizontal_bar())
            .child(self.render_readme(cx))
            .child(self.render_version(cx))
    }

    fn render_horizontal_bar() -> impl IntoElement {
        div().h(px(2.0)).w_full().bg(rgb(0x444444))
    }

    fn render_readme(&self, cx: &mut Context<Self>) -> impl IntoElement {
        div().id("read-me")
            .px_3()
            .py_2()
            .child("Read Me")
            .on_click(cx.listener(|this, _, _, cx| {
                this.close_setting_pulldown(cx);
                let _ = webbrowser::open(std::env!("CARGO_PKG_REPOSITORY"));
            }))
    }

    fn render_version(&self, cx: &mut Context<Self>) -> impl IntoElement {
        div().id("version")
            .px_3()
            .py_2()
            .child("Version")
            .on_click(cx.listener(|this, _, window, cx| {
                this.close_setting_pulldown(cx);
                window.dispatch_action(Box::new(ShowAbout), cx);
            }))
    }
}

impl Default for TitleBar {
    fn default() -> Self {
        Self {
            title: std::env!("CARGO_PKG_NAME").to_title_case().into(),
            flags: TitleBarFlags::default(),
            states: TitleBarStates::empty(),
        }
    }
}

actions!(menubar, [
    ShowAbout,
]);

struct AboutWindow {
    titlebar: Entity<TitleBar>,
}

impl Render for AboutWindow {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        div()
            .size_full()
            .bg(rgb(0x202020))
            .flex()
            .flex_col()
            .child(self.titlebar.clone())
            .child(Self::render_content())
    }
}

impl AboutWindow {
    pub fn new(cx: &mut Context<Self>) -> Self {
        let titlebar = cx.new(|_| TitleBar::new(
            format!("About {}", std::env!("CARGO_PKG_NAME").to_title_case()),
            TitleBarFlags::CloseButton | TitleBarFlags::TitleName | TitleBarFlags::WindowMove,
        ));

        Self {
            titlebar,
        }
    }

    pub fn show(_: &ShowAbout, cx: &mut App) {
        let bounds = Bounds::centered(
            None,
            size(px(500.0), px(250.0)), // TODO: サイズを文字列・フォントに合わせて動的に変える
            cx,
        );

        if let Err(e) = cx.open_window(
            WindowOptions {
                window_bounds: Some(WindowBounds::Windowed(bounds)),
                titlebar: None,
                window_decorations: Some(WindowDecorations::Client),
                ..Default::default()
            },
            |_, cx| cx.new(Self::new),
        ) {
            eprintln!("Failed to show About Window: {e}")
        }
    }

    fn render_content() -> impl IntoElement {
        div()
            .flex_1()
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
