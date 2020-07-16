// フレームワークパッケージ
// ↓ここから

trait Product {
    fn operate(&self);
}

trait Factory<Item: Product> {
    fn create(&mut self, owner: String) -> Item {
        let p = self.create_product(owner);
        self.register_product(&p);
        p
    }

    fn create_product(&mut self, owner: String) -> Item;
    fn register_product(&mut self, product: &Item);
}
// ↑ここまで

// idcard パッケージ
// ↓ここから

struct IDCard {
    owner: String,
}
impl IDCard {
    fn new(owner: String) -> Self {
        println!("Create {}'s ID card", owner);
        IDCard { owner }
    }
    fn get_owner(&self) -> &String {
        &(self.owner)
    }
}
impl Product for IDCard {
    fn operate(&self) {
        println!("Use {}'s ID card", self.owner);
    }
}

struct IDCardFactory {
    owners: Vec<String>,
}

impl IDCardFactory {
    fn new() -> Self {
        IDCardFactory { owners: vec![] }
    }
    fn get_owners(&mut self) -> &Vec<String> {
        &(self.owners)
    }
}

impl Factory<IDCard> for IDCardFactory {
    fn create_product(&mut self, owner: String) -> IDCard {
        IDCard::new(owner)
    }
    fn register_product(&mut self, product: &IDCard) {
        self.owners.push((**(product.get_owner())).into())
    }
}

// ↑ここまで

pub fn run() {
    let mut factory = IDCardFactory::new();
    let card1 = factory.create("Hikaru".into());
    let card2 = factory.create("Hiroki".into());
    card1.operate();
    card2.operate();
    println!("Owner's List: {:?}", factory.get_owners());
}
