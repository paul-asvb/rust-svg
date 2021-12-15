// svg_clock example ported from:
// https://github.com/utkarshkukreti/draco/tree/master/examples/svg_clock

#![deny(warnings)]
#![deny(clippy::all)]
use std::f32::consts::PI;

use console_error_panic_hook;
use sauron::{html::attributes::style, prelude::*, wasm_bindgen::JsCast};

#[macro_use]
extern crate log;

pub enum Msg {
    Tick,
}

pub struct MySvg {
    radius: i8,
}

impl MySvg {
    pub fn new() -> Self {
        MySvg { radius: 1 }
    }
}

impl Application<Msg> for MySvg {
    // we wire the window set_interval api to trigger an Msg::Tick
    // by dispatching it from the program, through the Cmd interface
    fn init(&mut self) -> Cmd<Self, Msg> {
        Cmd::new(move |program| {
            let clock: Closure<dyn Fn()> = Closure::wrap(Box::new(move || {
                program.dispatch(Msg::Tick);
            }));

            web_sys::window()
                .expect("no global `window` exists")
                .set_interval_with_callback_and_timeout_and_arguments_0(
                    clock.as_ref().unchecked_ref(),
                    30,
                )
                .expect("Unable to start interval");
            clock.forget();
        })
    }

    fn update(&mut self, msg: Msg) -> Cmd<Self, Msg> {
        match msg {
            Msg::Tick => {
                self.radius = 2;
            }
        }
        Cmd::none()
    }

    fn view(&self) -> Node<Msg> {
        let hex =
            |_rotate: u8, _stroke_color: &'static str, _stroke_width_value: u8, _height: u8| {
                let mut p: String = "".to_string();
                let mut theta: f32 = 0.0;

                let _edges = 6;
                let circumradius: f32 = 200.0;
                let mid: (f32, f32) = (250.0, 250.0);

                for i in 1..3 {
                    theta = theta + PI as f32 / i as f32;
                    let coo = (
                        mid.0 + circumradius * theta.cos(),
                        mid.1 + circumradius * theta.sin(),
                    );
                    let s = format!("{},{}", coo.0, coo.1);
                    p = [p, s].join(" ");
                }

                // <polygon points="100,100 150,25 150,75 200,0" fill="none" stroke="black" />

                polygon([points(p), fill("red"), stroke("black")], [])
            };

        let _radius = &self.radius;

        article(
            [],
            [
                h2([], [text("logo")]),
                div(
                    [
                        style("display", "flex"),
                        style("align-items", "center"),
                        style("flex-direction", "column"),
                    ],
                    [svg(
                        [width(500), height(600), viewBox([0, 0, 500, 500])],
                        [hex(123, "red", 1, 2)],
                    )],
                ),
            ],
        )
    }
}

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen(start)]
pub fn main() {
    console_log::init_with_level(log::Level::Trace).unwrap();
    console_error_panic_hook::set_once();
    trace!("starting svg clock..");

    Program::mount_to_body(MySvg::new());
}
