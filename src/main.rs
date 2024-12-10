use day_09::{calculate_all_fit_checksum, calculate_checksum};

fn main() {

    let optimized_checksum = calculate_checksum("src/resources/puzzle.txt");
    
    println!("Optimized checksum: {optimized_checksum}");

    let optimized_checksum = calculate_all_fit_checksum("src/resources/puzzle.txt");
    
    println!("Optimized checksum all files fit: {optimized_checksum}");
    
    
}
