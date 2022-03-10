use euclid::Point2D;
//#[macro_use(body_view)]
//use rui::body_view;
use rui::*;
use vger::VGER;

use crate::Scroller;

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
            let scroller = scroller.clone();
            let scroller2 = scroller.clone();

            println!(
                "scoller value: {}\tw: {}\twidth: {}\tcontent w: {}",
                scroller.get().value,
                w,
                scroller2.get().width,
                scroller2.get().content_width
            );

            zstack((
                rectangle().color(CLEAR_COLOR).drag(move |off, _state| {
                    scroller.with_mut(|v| (*v).value = ((*v).value + off.x / w).clamp(0.0, 1.0));
                }),
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
                        euclid::rect(
                            x,
                            c.y - SLIDER_WIDTH / 2.0,
                            w * scroller2.get().width / scroller2.get().content_width,
                            SLIDER_WIDTH,
                        ),
                        2.0,
                        paint,
                    );

                    //vger.fill_circle([x, c.y], 10.0, paint);
                }),
            ))
            .geom(move |sz| {
                if sz.width != w {
                    width.set(sz.width)
                }
            })
        })
    }
}

pub fn scrollbar(scroller: impl Binding<Scroller>) -> Scrollbar<impl Binding<Scroller>> {
    Scrollbar { scroller }
}
