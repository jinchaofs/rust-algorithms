mod arithmetics;

use arithmetics::string::atoi::Atoi;
fn main() {
    let atoi = Atoi::new(" -22 asdb 33".to_string());
    println!("atoi res: {}", atoi.parse());
}
