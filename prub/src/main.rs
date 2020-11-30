use std::fmt;
use std::vec::Vec;

#[derive(Debug)]
enum Event {
    Click { x: i32, y: i32 },
}

struct EventListeners<'a> {
    click: &'a mut dyn FnMut(Event),
}

impl<'a> fmt::Debug for EventListeners<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("EventListeners").finish()
    }
}

#[derive(Debug)]
struct Tree<'a> {
    pub listeners: Vec<EventListeners<'a>>,
}

impl<'a> Tree<'a> {
    pub fn add(&'a mut self, el: EventListeners<'a>) {
        self.listeners.push(el)
    }
}

trait Component {
    type Message;

    fn update(&mut self, msg: Self::Message);

    fn render(&mut self, tree: &mut Tree);
}

trait EventReceiver {
    fn receive(&mut self, node: i32, event: Event);
}

struct Panel;

#[derive(Debug)]
enum PanelMsg {
    Click,
}

impl Component for Panel {
    type Message = PanelMsg;

    fn update(&mut self, msg: Self::Message) {
        println!("Hola {:?}", msg);
    }

    fn render(&mut self, tree: &mut Tree) {
        let mut m = |e: Event| {
            println!("click({:?})", e);
        };

        let listener = EventListeners { click: &mut m };

        tree.listeners.push(listener);
    }
}

fn main() {
    let mut tree = Tree {
        listeners: Vec::new(),
    };

    (*tree.listeners[0].click)(Event::Click { x: 1, y: 2 });

    // println!("Tree: {:?}", tree);

    Panel {}.update(PanelMsg::Click);
}
