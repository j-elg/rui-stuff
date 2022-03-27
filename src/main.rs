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
        hstack((
            //hstack((
            scrollable(
                state.clone(),
                rectangle().corner_radius(20.0).size([900.0, 200.0]),
            ),
            // vscrollbar(state.clone()),
            //)),
            vscrollbar(state), //bind!(state, value)),
        ))
    }))
}
