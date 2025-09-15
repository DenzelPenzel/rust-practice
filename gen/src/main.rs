use std::collections::HashMap;
use std::fmt::Debug;

fn just_print<T, U>(x: T, y: U)
where
    T: ToString + Debug,
    U: ToString + Debug,
{
    println!("{}", x.to_string());
    println!("{}", y.to_string());
}

#[derive(Debug, Clone, Copy)]
struct Degrees(f32);

#[derive(Debug, Clone, Copy)]
struct Radians(f32);

impl From<Radians> for Degrees {
    fn from(radians: Radians) -> Self {
        Degrees(radians.0 * 180.0 / std::f32::consts::PI)
    }
}

impl From<Degrees> for Radians {
    fn from(degrees: Degrees) -> Self {
        Radians(degrees.0 * std::f32::consts::PI / 180.0)
    }
}

fn sin(angle: impl Into<Radians>) -> f32 {
    let angle: Radians = angle.into();
    angle.0.sin()
}

#[derive(Debug)]
struct HashMapBucket<K, V> {
    map: HashMap<K, Vec<V>>,
}

impl<K, V> HashMapBucket<K, V> 
where K: Eq + std::hash::Hash,
{
    fn new() -> Self {
        Self { map: HashMap::new() }
    }

    fn insert(&mut self, key: K, value: V) {
        let values = self.map.entry(key).or_insert(Vec::new());
        values.push(value);
    }
}

fn main() {
    just_print("Hello", "World");
    just_print(10, 20);

    // ================================================
    let degrees = Degrees(90.0);
    let radians = Radians::from(degrees);
    let radians2: Radians = Degrees(90.0).into();

    println!("Sin of 90 degrees is {}", sin(degrees));
    println!("Sin of 90 radians is {}", sin(radians));
    println!("Sin of 90 radians2 is {}", sin(radians2));

    // ================================================
    let mut bucket = HashMapBucket::new();
    bucket.insert("apple", 1);
    bucket.insert("banana", 2);
    bucket.insert("apple", 3);
    println!("{bucket:#?}");
}
