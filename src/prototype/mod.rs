use std::collections::HashMap;

// ↓ここから framework

trait Product {
    fn operate(&self, s: String);
    fn create_clone(&self) -> Box<dyn Product>;
}

struct Manager {
    showcase: HashMap<String, Box<dyn Product>>,
}

impl Clone for Box<dyn Product> {
    fn clone(&self) -> Box<dyn Product> {
        self.create_clone()
    }
}

impl Manager {
    fn new() -> Self {
        let showcase = HashMap::new();
        Manager { showcase }
    }

    fn register(&mut self, name: String, proto: Box<dyn Product>) {
        self.showcase.insert(name, proto);
    }

    fn create(&self, protoname: String) -> Option<Box<dyn Product>> {
        if let Some(p) = self.showcase.get(&protoname) {
            Some((*p).clone())
        } else {
            None
        }
    }
}

// ↑ここまで

#[derive(Clone, Debug)]
struct MessageBox {
    decochar: char,
}

impl MessageBox {
    fn new(decochar: char) -> Box<MessageBox> {
        Box::new(MessageBox { decochar })
    }
}

impl Product for MessageBox {
    fn operate(&self, s: String) {
        let length = s.bytes().len();
        for _ in 0..(length + 4) {
            print!("{}", self.decochar);
        }
        println!();
        println!("{} {} {}", self.decochar, s, self.decochar);
        for _ in 0..(length + 4) {
            print!("{}", self.decochar);
        }
        println!();
    }
    fn create_clone(&self) -> Box<dyn Product> {
        Box::new((*self).clone())
    }
}

#[derive(Clone, Debug)]
struct UnderlinePen {
    ulchar: char,
}

impl UnderlinePen {
    fn new(ulchar: char) -> Box<UnderlinePen> {
        Box::new(UnderlinePen { ulchar })
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
    fn create_clone(&self) -> Box<dyn Product> {
        Box::new((*self).clone())
    }
}

pub fn run() {
    let mut manager = Manager::new();

    let upen = UnderlinePen::new('~');
    let mbox = MessageBox::new('*');
    let sbox = MessageBox::new('/');
    manager.register("strong message".into(), upen);
    manager.register("warning box".into(), mbox);
    manager.register("slash box".into(), sbox);

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
