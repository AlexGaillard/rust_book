fn main() {
    print_labeled_measurements(5, 'h');

    let x = five();

    println!("x is equal to: {x}");
}

fn print_labeled_measurements(value: i32, unit_label: char) {
    println!("The measurement is: {value}{unit_label}");
}

fn five() -> i32 {
    5
}
