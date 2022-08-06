fn main() {
    println!("Lucas numbers:");
    for term in lucas::lucas().take(12) {
        println!("{}", term)
    }
}