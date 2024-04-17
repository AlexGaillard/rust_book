fn main() {
    let mut v = vec!["Alex", "Rae", "Tala"];

    v.push("Andrew");

    v[0] = "Nancy";

    for i in &v {
        println!("Hello, my name is {}!", i);
    }
}
