use euclid::Point2D;
//#[macro_use(body_view)]
//use rui::body_view;
use rui::*;
use vger::VGER;

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

const SLIDER_WIDTH: f32 = 20.0;
const SLIDER_THUMB_RADIUS: f32 = 10.0;

pub struct Scrollbar<B: Binding<Scroller>> {
    scroller: B,
}

impl<B> View for Scrollbar<B>
where
    B: Binding<Scroller>,
{
    body_view!();
}

impl<B> Scrollbar<B>
where
    B: Binding<Scroller>,
{
    fn body(&self) -> impl View {
        let scroller = self.scroller.clone();
        state(0.0, move |width| {
            let w = width.get();
            let x = scroller.get().value * w;
            let scope = scroller.get().width / scroller.get().content_width;
            let scroller = scroller.clone();

            canvas(move |sz, vger| {
                let c = sz.center();
                let paint = vger.color_paint(BUTTON_BACKGROUND_COLOR);
                vger.fill_rect(
                    euclid::rect(0.0, c.y - SLIDER_WIDTH / 2.0, sz.width(), SLIDER_WIDTH),
                    0.0,
                    paint,
                );
                let paint = vger.color_paint(AZURE_HIGHLIGHT);
                vger.fill_rect(
                    euclid::rect(x, c.y - SLIDER_WIDTH / 2.0, w * scope, SLIDER_WIDTH),
                    2.0,
                    paint,
                );
            })
            .geom(move |sz| {
                if sz.width != w {
                    width.set(sz.width)
                }
            })
            .drag(move |off, _state| {
                scroller
                    .with_mut(|v| (*v).value = ((*v).value + off.x / w).clamp(0.0, 1.0 - scope));
            })
        })
    }
}

pub fn scrollbar(scroller: impl Binding<Scroller>) -> Scrollbar<impl Binding<Scroller>> {
    Scrollbar { scroller }
}
