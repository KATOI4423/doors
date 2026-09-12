//! # mainwindow.rs
//!
//!

use gpui::*;
use rust_embed::RustEmbed;

use crate::ui::titlebar::TitleBar;

pub struct MainWindow {
    titlebar: Entity<TitleBar>,
}

impl Render for MainWindow {
    fn render(
        &mut self,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> impl IntoElement {
        div()
            .size_full()
            .flex()
            .bg(rgb(0xffffff) /* White */)
            .items_center()
            .flex_col()
            .on_mouse_down(MouseButton::Left, cx.listener(|this, _, _, cx| {
                this.titlebar.update(cx, |titlebar, cx| {
                    titlebar.close_setting_pulldown(cx);
                });
            }))
            .child(self.titlebar.clone())
            .child(MainWindow::render_main_content())
            .child(Self::render_resize_handles())
    }
}

impl MainWindow {
    fn new(cx: &mut Context<Self>) -> Self {
        let titlebar = cx.new(|_| TitleBar::default());
        Self {
            titlebar,
        }
    }

    /// # Register actions to cx
    ///
    /// MainWindowに必要なActionと、その他のUIで必要なActionをまとめて登録する
    fn register_actions(cx: &mut App) {
        TitleBar::register_actions(cx);
    }

    pub fn start() {
        Application::new()
            .with_assets(Assets)
            .run(|cx| {
                MainWindow::register_actions(cx);

                if let Err(e) = cx.open_window(
                    // GNOME wayland や Ghostty など、 xdg-decoration を実装していない環境では、
                    // WindowDecorations::Server (=Default) を使用するとタイトルバーが描画されない
                    // そのため、 WindowDecorations::Client (=独自実装) を選択する
                    WindowOptions {
                        titlebar: None,
                        window_decorations: Some(WindowDecorations::Client),
                        ..Default::default()
                    },
                    |_, cx| cx.new(|cx| Self::new(cx)),
                ) {
                    eprintln!("Failed to open window: {}", e);
                }
            });
    }

    fn render_main_content() -> impl IntoElement {
        div()
            .flex()
            .child("Main Content")
    }

    /// # Render Resize Handlers
    ///
    /// 4辺と4角にウィンドウサイズ変更用ハンドラを設定する
    fn render_resize_handles() -> impl IntoElement {
        div()
            .absolute()
            .inset_0()
            .children([
                Self::render_resize_handle(ResizeEdge::Top),
                Self::render_resize_handle(ResizeEdge::Bottom),
                Self::render_resize_handle(ResizeEdge::Left),
                Self::render_resize_handle(ResizeEdge::Right),
                Self::render_resize_handle(ResizeEdge::TopLeft),
                Self::render_resize_handle(ResizeEdge::TopRight),
                Self::render_resize_handle(ResizeEdge::BottomLeft),
                Self::render_resize_handle(ResizeEdge::BottomRight),
            ])
    }

    fn render_resize_handle(edge: ResizeEdge) -> impl IntoElement {
        // 共通部分
        const HANDLER_PIXEL_SIZE: f32 = 12.0;
        let cursor_style = match edge {
            ResizeEdge::Top | ResizeEdge::Bottom => CursorStyle::ResizeUpDown,
            ResizeEdge::Left | ResizeEdge::Right => CursorStyle::ResizeLeftRight,
            ResizeEdge::TopLeft | ResizeEdge::BottomRight => CursorStyle::ResizeUpLeftDownRight,
            ResizeEdge::TopRight | ResizeEdge::BottomLeft => CursorStyle::ResizeUpRightDownLeft,
        };
        let div = div()
            .absolute()
            .cursor(cursor_style)
            .on_mouse_down(MouseButton::Left, move |_, window, cx| {
                cx.stop_propagation();
                window.start_window_resize(edge);
            });

        match edge {
            ResizeEdge::Top => div
                .top_0()
                .left(px(HANDLER_PIXEL_SIZE))
                .right(px(HANDLER_PIXEL_SIZE))
                .h(px(HANDLER_PIXEL_SIZE)),
            ResizeEdge::Bottom => div
                .bottom_0()
                .left(px(HANDLER_PIXEL_SIZE))
                .right(px(HANDLER_PIXEL_SIZE))
                .h(px(HANDLER_PIXEL_SIZE)),
            ResizeEdge::Left => div
                .left_0()
                .top(px(HANDLER_PIXEL_SIZE))
                .bottom(px(HANDLER_PIXEL_SIZE))
                .w(px(HANDLER_PIXEL_SIZE)),
            ResizeEdge::Right => div
                .right_0()
                .top(px(HANDLER_PIXEL_SIZE))
                .bottom(px(HANDLER_PIXEL_SIZE))
                .w(px(HANDLER_PIXEL_SIZE)),
            ResizeEdge::TopLeft => div
                .top_0()
                .left_0()
                .h(px(HANDLER_PIXEL_SIZE))
                .w(px(HANDLER_PIXEL_SIZE)),
            ResizeEdge::TopRight => div
                .top_0()
                .right_0()
                .h(px(HANDLER_PIXEL_SIZE))
                .w(px(HANDLER_PIXEL_SIZE)),
            ResizeEdge::BottomLeft => div
                .bottom_0()
                .left_0()
                .h(px(HANDLER_PIXEL_SIZE))
                .w(px(HANDLER_PIXEL_SIZE)),
            ResizeEdge::BottomRight => div
                .bottom_0()
                .right_0()
                .h(px(HANDLER_PIXEL_SIZE))
                .w(px(HANDLER_PIXEL_SIZE)),
        }
    }
}

#[derive(RustEmbed)]
#[folder = "assets"]
struct Assets;

impl AssetSource for Assets {
    fn load(&self, path: &str) -> Result<Option<std::borrow::Cow<'static, [u8]>>> {
        Ok(Self::get(path).map(|file| file.data))
    }

    fn list(&self, prefix: &str) -> Result<Vec<SharedString>> {
        Ok(Self::iter()
            .filter_map(|asset_path| {
                asset_path.strip_prefix(prefix)
                    .map(|path| SharedString::from(path.to_owned()))
            })
            .collect())
    }
}
