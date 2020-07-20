//　ここから
trait Builder {
    fn make_title(&mut self, title: String);
    fn make_string(&mut self, str: String);
    fn make_items(&mut self, items: Vec<String>);
    fn close(&mut self);
}

struct Director<B: Builder> {
    builder: B,
}

impl<B: Builder> Director<B> {
    fn set_builder(&mut self, builder: B) {
        self.builder = builder
    }
    fn construct(&mut self) {
        self.builder.make_title("Greeting".into());
        self.builder.make_string("from morning to midday".into());
        self.builder
            .make_items(vec!["Good Morning".into(), "Hello".into()]);
        self.builder.make_string("after evening".into());
        self.builder.make_items(vec![
            "Good evening".into(),
            "Good night".into(),
            "See you tomorrow".into(),
        ]);
        self.builder.close();
    }
}

struct TextBuilder {
    buffer: Vec<String>,
}

impl Builder for TextBuilder {
    fn make_title(&mut self, title: String) {
        self.buffer.push("=============================\n".into());
        self.buffer.push(format!("[ {} ] \n", title));
        self.buffer.push("\n".into());
    }
    fn make_string(&mut self, str: String) {
        self.buffer.push(format!("■{}\n", str));
        self.buffer.push("\n".into());
    }
    fn make_items(&mut self, items: Vec<String>) {
        for item in items {
            self.buffer.push(format!("  ・{}\n", item));
        }
        self.buffer.push("\n".into());
    }
    fn close(&mut self) {
        self.buffer.push("=============================\n".into());
    }
}

// ここまでフレームワーク

pub fn run() {
    println!("Hello from Builder")
}
