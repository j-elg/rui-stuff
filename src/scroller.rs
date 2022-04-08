use euclid::{Point2D, Size2D};
//#[macro_use(body_view)]
//use rui::body_view;
use rui::*;
use vger::{defs::LocalVector, VGER};

#[derive(Default, Clone)]
pub struct Scroller {
    pub value: [f32; 2],
    scrollable_size: LocalSize,
    content_size: LocalSize,
}

impl Scroller {
    pub fn new() -> Self {
        Self {
            value: [0.0, 0.0],
            scrollable_size: LocalSize::default(),
            content_size: LocalSize::default(),
        }
    }

    fn x_scope(&self) -> f32 {
        self.scrollable_size.width / self.content_size.width
    }
    fn y_scope(&self) -> f32 {
        self.scrollable_size.height / self.content_size.height
    }
}

pub fn scrollable(scroller: State<Scroller>, content: impl View + 'static) -> impl View {
    let scr = scroller.get();
    let x = (scr.value[0] / (1.0 - scr.x_scope()))
        * (scr.scrollable_size.width - scr.content_size.width);
    let y = (scr.value[1] / (1.0 - scr.y_scope()))
        * (scr.scrollable_size.height - scr.content_size.height);
    let off = LocalOffset::new(x, y);
    let scroller2 = scroller.clone();

    zstack((content
        .geom(move |sz| {
            scroller.with_mut(|state| {
                if state.content_size != sz {
                    println!("content_size: {:?}", sz);
                    state.content_size = sz;
                }
            })
        })
        .offset(off),))
    .geom(move |sz| {
        scroller2.with_mut(|state| {
            if state.scrollable_size != sz {
                println!("scrollable_size: {:?}", sz);
                state.scrollable_size = sz;
            }
        })
    })
}

const SLIDER_THICKNESS: f32 = 20.0;
const SLIDER_THUMB_RADIUS: f32 = 10.0;

pub fn hscrollbar(scroller: State<Scroller>) -> impl View {
    //let scroller = scroller.clone();
    state(0.0, move |width| {
        let w = width.get();
        let x = scroller.get().value[0] * w;
        let scope = scroller.get().x_scope();
        let scroller = scroller.clone();
        let scroller2 = scroller.clone();

        // println!("h scope: {}\tw: {}\tx: {}", scope, w, x);
        cond(
            scope > 1.0,
            EmptyView {},
            canvas(move |sz, vger| {
                let c = sz.center();
                let paint = vger.color_paint(BUTTON_BACKGROUND_COLOR);
                vger.fill_rect(
                    euclid::rect(
                        0.0,
                        c.y - SLIDER_THICKNESS / 2.0,
                        sz.width(),
                        SLIDER_THICKNESS,
                    ),
                    0.0,
                    paint,
                );
                let paint = vger.color_paint(AZURE_HIGHLIGHT);
                vger.fill_rect(
                    euclid::rect(x, c.y - SLIDER_THICKNESS / 2.0, w * scope, SLIDER_THICKNESS),
                    2.0,
                    paint,
                );
            })
            .geom(move |sz| {
                if sz.width != w {
                    width.set(sz.width);
                    scroller2.with_mut(|v| {
                        if (*v).value[0] > 1.0 - scope - 0.05 {
                            (*v).value[0] = ((*v).value[0] + 1.0).clamp(0.0, 1.0 - scope)
                        }
                    });
                }
            })
            .drag(move |off, _state| {
                scroller.with_mut(|v| {
                    (*v).value[0] = ((*v).value[0] + off.x / w).clamp(0.0, 1.0 - scope)
                });
            }),
        )
    })
}

pub fn vscrollbar(scroller: State<Scroller>) -> impl View {
    //let scroller = scroller.clone();
    state(0.0, move |height| {
        let h = height.get();
        let y = scroller.get().value[1] * h;
        let scope = scroller.get().y_scope();
        let scroller = scroller.clone();
        let scroller2 = scroller.clone();

        //println!("v scope: {}", scope);
        cond(
            scope > 1.0,
            EmptyView {},
            canvas(move |sz, vger| {
                let c = sz.center();
                let paint = vger.color_paint(BUTTON_BACKGROUND_COLOR);
                vger.fill_rect(
                    euclid::rect(
                        c.x - SLIDER_THICKNESS / 2.0,
                        0.0,
                        SLIDER_THICKNESS,
                        sz.height(),
                    ),
                    0.0,
                    paint,
                );
                let paint = vger.color_paint(AZURE_HIGHLIGHT);
                vger.fill_rect(
                    euclid::rect(c.x - SLIDER_THICKNESS / 2.0, y, SLIDER_THICKNESS, h * scope),
                    2.0,
                    paint,
                );
            })
            .geom(move |sz| {
                if sz.height != h {
                    height.set(sz.height);
                    scroller2.with_mut(|v| {
                        if (*v).value[1] > 1.0 - scope - 0.05 {
                            (*v).value[1] = ((*v).value[1] + 1.0).clamp(0.0, 1.0 - scope)
                        }
                    });
                }
            })
            .drag(move |off, _state| {
                scroller.with_mut(|v| {
                    (*v).value[1] = ((*v).value[1] + off.y / h).clamp(0.0, 1.0 - scope)
                });
            }),
        )
    })
}
