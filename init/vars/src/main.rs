fn doble(n: i32) -> i32 {
    n * 2
}

fn green(s: String) -> String {
    format!("{} - ", s);
    s
}

fn greet_borrow(s: &String) -> &String {
    println!("Hello, {}!", s);
    s
}

fn gree_borrow_mut(s: &mut String) {
    *s = format!("Hello {s}");
}

fn read_line() -> String {
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    input.trim().to_string()
}

fn main() {
    let n = 5;
    println!("{n}");

    {
        println!("{n}");
    }

    let n = n + 1;

    println!("{n}");

    println!("{}", doble(10));

    let n = if n != 10 { doble(100) } else { 100 };

    println!("{}", n);

    // Borrow
    let mut name = String::from("Leo");
    gree_borrow_mut(&mut name);
    println!("{}", name);

    // read-line
    let input = read_line();
    println!("Read line: [{}]", input);
}
