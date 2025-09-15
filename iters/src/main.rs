struct Counter {
    count: u32,
    max: u32,
}

impl Counter {
    fn new(max: u32) -> Self {
        Self { count: 0, max }
    }
}

impl Iterator for Counter {
    type Item = u32;

    fn next(&mut self) -> Option<Self::Item> {
        if self.count < self.max {
            self.count += 1;
            Some(self.count)
        } else {
            None
        }
    }
}

impl ExactSizeIterator for Counter {
    fn len(&self) -> usize {
        self.max as usize
    }
}

fn main() {
    let mut counter = Counter::new(10);

    while let Some(count) = counter.next() {
        println!("Count: {count}");
    }

    let counter2: Vec<u32> = Counter::new(10).collect();
    println!("Count: {counter2:#?}");
}
