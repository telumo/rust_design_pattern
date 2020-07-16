use std::collections::HashMap;

// ↓ここから framework

trait Product {
    fn operate(&self, s: String);
    fn create_clone(&self) -> Self;
}

struct Manager<V: Product> {
    showcase: HashMap<String, V>,
}

impl<V: Product + Copy> Manager<V> {
    fn new() -> Self {
        let mut showcase = HashMap::new();
        Manager { showcase }
    }

    fn register(&mut self, name: String, proto: V) {
        self.showcase.insert(name, proto);
    }

    fn create(&self, protoname: String) -> Option<V> {
        if let Some(p) = self.showcase.get(&protoname) {
            return Some((*p).create_clone());
        } else {
            return None;
        }
    }
}

// ↑ここまで

#[derive(Clone, Copy)]
struct MessageBox {
    decochar: char,
}

impl MessageBox {
    fn new(decochar: char) -> Self {
        MessageBox { decochar }
    }
}

impl Product for MessageBox {
    fn operate(&self, s: String) {
        let length = s.bytes().len();
        for _ in 0..(length + 4) {
            print!("{}", self.decochar);
        }
        println!("");
        println!("{} {} {}", self.decochar, s, self.decochar);
        for _ in 0..(length + 4) {
            print!("{}", self.decochar);
        }
        println!("");
    }
    fn create_clone(&self) -> Self {
        self.clone()
    }
}

#[derive(Clone, Copy)]
struct UnderlinePen {
    ulchar: char,
}

impl UnderlinePen {
    fn new(ulchar: char) -> Self {
        UnderlinePen { ulchar }
    }
}

impl Product for UnderlinePen {
    fn operate(&self, s: String) {
        let length = s.bytes().len();
        println!("\"{}\"", s);
        print!(" ");
        for _ in 0..length {
            print!("{}", self.ulchar);
        }
        println!(" ");
    }
    fn create_clone(&self) -> Self {
        self.clone()
    }
}

pub fn run() {
    let mut manager = Manager::new();
    let upen = UnderlinePen::new('~');
    let mbox = MessageBox::new('*');
    let sbox = MessageBox::new('/');
    manager.register("strong message".into(), upen);
    // manager.register("warning box".into(), mbox);
    // manager.register("slash box".into(), sbox);

    if let Some(p) = manager.create("strong message".into()) {
        p.operate("Hello, world".into());
    }

    if let Some(p) = manager.create("warning box".into()) {
        p.operate("Hello, world".into());
    }

    if let Some(p) = manager.create("slash box".into()) {
        p.operate("Hello, world".into());
    }
}
