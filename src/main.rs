trait AbstractDisplay {
    fn open(&self);
    fn print(&self);
    fn close(&self);
    fn display(&self) {
        self.open();
        for _ in 0..5{
            self.print();
        }
        self.close();
    }
}

impl AbstractDisplay for char {
    fn open(&self){
        print!("<<")
    }
    fn print(&self){
        print!("{}", &self)
    }
    fn close(&self){
        print!(">>");
        println!();
    }
}

trait PrintLine {
    fn print_line(&self);
}

impl PrintLine for String {
    fn print_line(&self){
        print!("+");
        // TODO:日本語対応したい。
        for _ in 0..self.bytes().len(){
            print!("-");
        }
        println!("+")
    }
}

impl AbstractDisplay for String {
    fn open(&self){
        self.print_line()
    }
    fn print(&self){
        println!("|{}|", &self)
    }
    fn close(&self){
        self.print_line()
    }
}

fn main() {
    'a'.display();
    
    String::from("Hello, World").display();
}