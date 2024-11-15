use floem::{
    kurbo::Size,
    prelude::*,
    reactive::{create_rw_signal, SignalGet, SignalUpdate},
    text::Weight,
    unit::UnitExt,
    views::{container, label, stack, text_input},
    window::WindowConfig,
    Application, IntoView,
};

pub fn app_view(presence_interval: RwSignal<u64>) -> impl IntoView {
    let text = create_rw_signal("300".to_string());

    // create_effect(move |_| {
    //     if let Ok(seconds) = text.get().parse::<u64>() {
    //         presence_interval.set(seconds);
    //     }
    // });

    let view = stack((form({
        (
            form_item("Simple Input:".to_string(), 120.0, move || {
                text_input(text)
                    .placeholder("input interval in seconds")
                    .keyboard_navigable()
            }),
            button("Save").on_click_stop(move |_| {
                if let Ok(seconds) = text.get().parse::<u64>() {
                    presence_interval.set(seconds);
                }
            }),
        )
    }),))
    .style(|s| {
        s.size(100.pct(), 100.pct())
            .flex_col()
            .items_center()
            .justify_center()
    });

    view
}

pub fn form<VT: ViewTuple + 'static>(children: VT) -> impl IntoView {
    stack(children).style(|s| {
        s.flex_col()
            .items_start()
            .margin(10.0)
            .padding(10.0)
            .width(100.pct())
    })
}

pub fn form_item<V: IntoView + 'static>(
    item_label: String,
    label_width: f32,
    view_fn: impl Fn() -> V,
) -> impl IntoView {
    container(
        stack((
            container(label(move || item_label.clone()).style(|s| s.font_weight(Weight::BOLD)))
                .style(move |s| s.width(label_width).justify_end().margin_right(10.0)),
            view_fn(),
        ))
        .style(|s| s.flex_row().items_center()),
    )
    .style(|s| {
        s.flex_row()
            .items_center()
            .margin_bottom(10.0)
            .padding(10.0)
            .width(100.pct())
            .min_height(32.0)
    })
}

pub fn run(presence_interval: RwSignal<u64>) {
    let app = Application::new().window(
        move |_| app_view(presence_interval),
        Some(
            WindowConfig::default()
                .title("Keep Presence")
                .size(Size::new(300.0, 100.0)),
        ),
    );
    app.run();
}
