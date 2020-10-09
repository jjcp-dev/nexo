use std::vec::Vec;

#[derive(Debug)]
pub struct Node {
    parent: i32,
    value: i32,
}

#[derive(Debug)]
pub struct Tree {
    nodes: Vec<Node>,
}

impl Tree {
    pub fn new(root: i32) -> Tree {
        Tree {
            nodes: vec![Node {
                parent: -1,
                value: root,
            }],
        }
    }

    pub fn root(&self) -> Element {
        Element { index: 0 }
    }

    // pub fn create(&mut self, value: i32) -> Element {
    //     self.nodes.push(Node {
    //         parent: -1,
    //         value: value,
    //     });

    //     let index = self.nodes.len() as i32 - 1;

    //     Element { index: index }
    // }

    pub fn create(&mut self, parent: Element, value: i32) -> Element {
        self.nodes.push(Node {
            parent: parent.index,
            value: value,
        });

        let index = self.nodes.len() as i32 - 1;

        Element { index: index }
    }

    pub fn append(&mut self, parent: Element, child: Element) {
        self.nodes[child.index as usize].parent = parent.index;
    }
}

#[derive(Debug, Copy, Clone)]
pub struct Element {
    index: i32,
}

fn main() {
    let mut tree = Tree::new(0);

    let ch0 = tree.create(tree.root(), 0);
    let _ch1 = tree.create(ch0, 1);
    tree.create(tree.root(), 10);

    println!("Hello, world!");
    println!("Tree: {:?}", tree);
}
