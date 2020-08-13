struct Item {
    caption: String
}

struct Link {
    url: String
}
impl Item {
    fn new(caption: String) -> Self{
        Item{
            caption
        }
    }
}

static mut S: &i32 = &0;
// impl Item for Link {

// }

pub fn run() {
    unsafe {
        println!("S = {}, {:p}", S, S);
    }
    
    
}