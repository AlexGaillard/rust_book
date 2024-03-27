fn main() {
    let mut s1 = String::from("Hello");

    let s2 = s1.clone();

    s1.push_str(", world!");

    println!("{}", s1);
    println!("{}", s2);
}
