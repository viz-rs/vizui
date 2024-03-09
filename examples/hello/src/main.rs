use gpui::*;
use vizui::StyledExt;

struct HelloWorld {
    text: SharedString,
}

impl Render for HelloWorld {
    fn render(&mut self, _cx: &mut ViewContext<Self>) -> impl IntoElement {
        div()
            // .flex()
            .bg(rgb(0x2e7d32))
            .size_full()
            .h_flex()
            .gap_2()
            .overflow_x_hidden()
            // .flex_row()
            // .gap_2()
            .justify_center()
            // .items_center()
            .text_xl()
            .text_color(rgb(0xffffff))
            // .child(format!("Hello, {}!", &self.text))
            .children([
                format!("Hello, {}!", &self.text),
                format!("{}", "Fangdun Tsai"),
            ])
    }
}

fn main() {
    App::new().run(|cx: &mut AppContext| {
        cx.open_window(WindowOptions::default(), |cx| {
            cx.new_view(|_cx| HelloWorld {
                text: "World".into(),
            })
        });
    });
}
