use gpui::*;
use uuid::{uuid, Uuid};
use vizui::ActiveTheme;
use vizui::StyledExt;

enum Event {}

struct Workspace {}

impl Workspace {
    pub fn new(cx: &mut ViewContext<Self>) -> Self {
        Self {}
    }
}

impl EventEmitter<Event> for Workspace {}

impl Render for Workspace {
    fn render(&mut self, cx: &mut ViewContext<Self>) -> impl IntoElement {
        let mut context = KeyContext::default();
        context.add("Workspace");

        let theme = cx.theme().clone();
        let colors = theme.colors();

        div()
            .key_context(context)
            .bg(colors.background)
            .text_color(colors.text)
            .size_full()
            .overflow_x_hidden()
            .h_flex()
            .gap_2()
            .justify_center()
            .child("hello")
    }
}

pub fn build_window_options(
    bounds: Option<WindowBounds>,
    display_uuid: Option<Uuid>,
    cx: &mut AppContext,
) -> WindowOptions {
    let bounds = bounds.unwrap_or(WindowBounds::Maximized);
    let display = display_uuid.and_then(|uuid| {
        cx.displays()
            .into_iter()
            .find(|display| display.uuid().ok() == Some(uuid))
    });

    WindowOptions {
        bounds,
        titlebar: Some(TitlebarOptions {
            title: None,
            appears_transparent: true,
            traffic_light_position: Some(point(px(9.5), px(9.5))),
        }),
        center: false,
        focus: false,
        show: true,
        kind: WindowKind::Normal,
        is_movable: true,
        display_id: display.map(|display| display.id()),
    }
}

actions!(Workspace, [Quit]);

fn main() {
    App::new().run(|cx: &mut AppContext| {
        cx.activate(true);
        cx.on_action(|_: &Quit, cx| cx.quit());
        cx.bind_keys([KeyBinding::new("cmd-q", Quit, None)]);

        let options = build_window_options(
            Some(WindowBounds::Fixed(Bounds {
                size: size(px(600.0), px(600.0)).into(),
                origin: Default::default(),
            })),
            Some(Uuid::new_v4()),
            cx,
        );
        cx.open_window(options, |cx| cx.new_view(|cx| Workspace::new(cx)));
    });
}
