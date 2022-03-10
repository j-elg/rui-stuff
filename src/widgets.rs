use rui::*;

#[derive(Default, Clone)]
pub struct Scroller {
    pub value: f32,
    pub(crate) width: f32,
    pub(crate) content_width: f32,
}

impl Scroller {
    pub fn new() -> Self {
        Self {
            value: 0.0,
            width: 0.0, // of scrollable, i.e. the restricted view of the content
            content_width: 0.0,
        }
    }
}

/*
pub fn scroller(state: State<Scroller>, content: impl View + 'static) -> impl View {
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
*/

pub fn scrollable(state: State<Scroller>, content: impl View + 'static) -> impl View {
    let scr = state.get();
    // Note TODO the scroller can only move so much that it still fits, so never 0..1,
    // more like 0.. scrollable.width / content_width
    let scope = scr.width / scr.content_width;
    //let off = LocalOffset::new(( - scr.value) * scr.content_width + scr.width, 0.0);
    // move between 0 and scr.width - scr.content_width
    let v = (scr.value / (1.0 - scope)) * (scr.width - scr.content_width);
    let off = LocalOffset::new(v, 0.0); //scr.width, 0.0);
    let state2 = state.clone();

    zstack((content
        .geom(move |sz| {
            state.with_mut(|state| {
                if state.content_width != sz.width {
                    state.content_width = sz.width;
                }
            })
        })
        .offset(off),))
    .geom(move |sz| {
        state2.with_mut(|state| {
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
