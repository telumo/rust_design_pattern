#[derive(Debug, Clone)]
struct Banner {
    string: String
}

impl Banner {
    fn new(string: String) -> Self {
        Banner {
            string
        }
    }

    fn show_with_paren(&self) {
        println!("({})", self.string)
    }

    fn show_with_aster(&self) {
        println!("*{}*", self.string)
    }
}

trait Print {
    fn print_weak(&self);
    fn print_strong(&self);
}

struct PrintBanner{
    banner: Banner
}

impl PrintBanner {
    fn new(string: String) -> Self {
        let banner = Banner::new(string);
        PrintBanner {
            banner
        }
    }
}

impl Print for PrintBanner {
    fn print_weak(&self) {
        self.banner.show_with_paren()
    }

    fn print_strong(&self) {
        self.banner.show_with_aster()
    }
}


pub fn run() {

    let sample: String = "ほげほげ".into();
    let sample_banner = Banner::new(sample);
    sample_banner.show_with_paren();
    sample_banner.show_with_aster();

    let sample: String = "ほげほげ".into();
    let p = PrintBanner::new(sample);
    print_all(p);
    // print_all(sample_banner); Error
}

fn print_all<T: Print>(p: T) {
    p.print_weak();
    p.print_strong();
}

#[cfg(test)]
mod test {
    
    
    
    
    
    
    
    
    
    
    
    
    
}