use enigo::*;

fn main() {
    println!("Enter the paragraph:");
    let mut text = String::new();
    let mut e = Enigo::new();

    std::io::stdin().read_line(&mut text).unwrap();

    std::thread::sleep(std::time::Duration::from_millis(2000));

    e.key_sequence(&text);
}
