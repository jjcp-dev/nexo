use super::event::Event;

pub trait CoreController {
    fn on_event(&mut self, event: &Event);
}

pub struct CoreApplication<Controller: CoreController> {
    controller: Rc<CoreController>,
}

impl CoreApplication {
    pub fn run() {}
}
