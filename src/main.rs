use rui::*;
//use rui_extra::scroller::*;
use rui_extra::*;
/*
fn main1() {
    rui(state(Scroller::new(), move |state| {
        scroller(state, rectangle().corner_radius(5.0).size([600.0, 200.0]))
    }))
}
*/

fn main() {
    rui(state(Scroller::new(), move |state| {
        vstack(
            (
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
                hscrollbar(state), //bind!(state, value)),
            ), //.padding(Auto)
        )
    }))
}
