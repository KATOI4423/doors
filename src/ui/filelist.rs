//! filelist.rs
//!
//! Show the file list with those information

use std::collections::VecDeque;
use std::path::PathBuf;

use gpui_kit::*;

pub struct FileList {
    current: PathBuf,
    back: VecDeque<PathBuf>,
    forward: VecDeque<PathBuf>,
}

impl Render for FileList {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        div()
            .h_full()
            .w_full()
            .rounded_b_lg()
            .bg(rgb(0x202020))
            .child(self.render_toolbar(window, cx))
    }
}

impl FileList {
    pub fn new(path: impl Into<PathBuf>) -> Self {
        Self {
            current: path.into(),
            back: VecDeque::default(), // TODO: 履歴復元機能を追加
            forward: VecDeque::default(), // TODO: 履歴復元機能を追加
        }
    }

    fn render_toolbar(&self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        div()
            .flex()
            .flex_row()
            .h_8()
            .w_full()
            .justify_start()
            .items_start()
            .child(Self::render_back_button(window, cx))
            .child(Self::render_front_button(window, cx))
            .child(Self::render_parent_button(window, cx))
            .child(Self::render_renew_button(window, cx))
    }

    /// # Update Stack
    ///
    /// ## Args:
    ///  - stack: 更新対象の VecDeque
    ///  - element: stack に入れる要素
    ///
    /// ## Notes:
    ///  - stack の要素数が MAX_HISTORY を超えないように、stack の要素を削除して自動調整する
    fn update_stack(stack: &mut VecDeque<PathBuf>, element: PathBuf) {
        const MAX_HISTORY: usize = 10;
        if stack.len() >= MAX_HISTORY {
            stack.pop_front();
        }
        stack.push_back(element);
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

    fn render_back_button(_window: &Window, cx: &mut Context<Self>) -> impl IntoElement {
        Self::render_button("back", "icons/chevron-left.svg", cx.listener(|this, _, _, _| {
            let Some(back) = this.back.pop_back() else {
                eprintln!("back is empty");
                return;
            };

            dbg!(&back);
            Self::update_stack(&mut this.forward, this.current.clone());
            this.current = back;
        }))
    }

    fn render_front_button(_window: &Window, cx: &mut Context<Self>) -> impl IntoElement {
        Self::render_button("front", "icons/chevron-right.svg", cx.listener(|this, _, _, _| {
            let Some(forward) = this.forward.pop_back() else {
                eprintln!("forward is empty");
                return;
            };

            dbg!(&forward);
            Self::update_stack(&mut this.back, this.current.clone());
            this.current = forward;
        }))
    }

    /// # Render Parrent Button
    ///
    /// Note: forward の履歴は削除する
    fn render_parent_button(_window: &Window, cx: &mut Context<Self>) -> impl IntoElement {
        Self::render_button("parent", "icons/allow-up.svg", cx.listener(|this, _, _, _| {
            let Some(parent) = this.current.parent() else {
                eprintln!("there are no parent directory");
                return;
            };

            dbg!(&parent);
            this.forward.clear();
            Self::update_stack(&mut this.back, this.current.clone());
            this.current = parent.to_owned();
        }))
    }

    fn render_renew_button(_window: &Window, cx: &mut Context<Self>) -> impl IntoElement {
        Self::render_button("renew", "icons/rotate.svg", cx.listener(|this, _, _, _| {
            eprintln!("Renew!");
        }))
    }
}
