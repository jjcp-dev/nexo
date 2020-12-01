use std::rc::Rc;

use wasm_bindgen::prelude::*;

pub struct Closures {
    pub on_click_handler: Closure<dyn Fn(web_sys::MouseEvent)>,
}

impl Closures {
    pub fn new(coso: Rc<dyn Fn(crate::core::event::Event)>) -> Closures {
        Closures {
            on_click_handler: Closure::wrap(Box::new(move |event: web_sys::MouseEvent| {
                coso(crate::core::event::Event::Click {
                    x: event.x() as i16,
                    y: event.y() as i16,
                });
            })),
        }
    }
}
