use rui::*;
use rui_stuff::*;

fn main() {
    rui(state(
        || Scroller::new(),
        move |state| {
            vstack((
                hstack((
                    vscrollbar(state.clone()),
                    scrollable(
                        state.clone(),
                        rectangle()
                            .color(GREEN_HIGHLIGHT)
                            .corner_radius(20.0)
                            .size([900.0, 200.0]),
                    )
                    .background(rectangle().color(AZURE_HIGHLIGHT_BACKGROUND)),
                )),
                hscrollbar(state),
            ))
        },
    ))
}
