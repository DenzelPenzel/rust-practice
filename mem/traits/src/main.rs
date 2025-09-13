fn borrow<'a>(x: &'a i32, y: &'a i32) -> &'a i32 {
    x
}

struct Cat(String);

impl Cat {
    fn feed(&mut self) {
        self.0 = format!("{} is fed", self.0);
    }
}

struct CatFeeder<'a> {
    cat: &'a mut Cat,
}

impl<'a> CatFeeder<'a> {
    fn feed_cat(&mut self) {
        self.cat.feed();
    }
}

fn main() {
    let x = 10;
    borrow(&x, &x);

    let mut cats = vec![Cat(String::from("Whiskers")), Cat(String::from("Tom"))];

    let mut feeders = Vec::new();

    // get reference for the each cat
    for cat in cats.iter_mut() {
        feeders.push(CatFeeder { cat: cat });
    }

    feeders.iter_mut().for_each(|f| {
        f.feed_cat();
    });
}
