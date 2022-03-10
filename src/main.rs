use rui::*;

#[derive(Default, Clone)]
struct Scroller {
    value: f32,
    width: f32,
}

impl Scroller {
    pub fn new() -> Self {
        Self {
            value: 0.0,
            width: 0.0,
        }
    }
}

fn scroller(state: State<Scroller>, content: impl View + 'static) -> impl View {
    let scr = state.get();
    let off = LocalOffset::new((0.5 - scr.value) * scr.width * 2.0, 0.0);

    vstack((
        text(&format!("value: {:?}", state.get().value)),
        content.offset(off),
        hslider(bind!(state, value)),
    ))
    .geom(move |sz| {
        if state.get().width != sz.width {
            state.set(Scroller {
                value: state.get().value,
                width: sz.width,
            })
        }
    })
}

fn main1() {
    rui(state(Scroller::new(), move |state| {
        scroller(state, rectangle().corner_radius(5.0).size([600.0, 200.0]))
    }))
}

fn scrollable(state: State<Scroller>, content: impl View + 'static) -> impl View {
    let scr = state.get();
    let off = LocalOffset::new((0.5 - scr.value) * scr.width * 2.0, 0.0);

    zstack((content.offset(off),)).geom(move |sz| {
        state.with_mut(|state| {
            if state.width != sz.width {
                state.width = sz.width;
            }
        })
        /*
        if state.get().width != sz.width {
            state.set(Scroller {
                value: state.get().value,
                width: sz.width,
            })
        }*/
    })
}

fn main() {
    rui(state(Scroller::new(), move |state| {
        vstack((
            scrollable(
                state.clone(),
                rectangle().corner_radius(5.0).size([600.0, 200.0]),
            ),
            hslider(bind!(state, value)),
        ))
    }))
}
