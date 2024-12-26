fn main() {
    let max_num = 10;

    for n in (1..max_num + 1).rev() {
        let num_string = n.to_string();
        println!("{num_string}");
    }
    println!("Blast Off!");
}
