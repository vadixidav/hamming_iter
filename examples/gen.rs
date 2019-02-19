fn main() {
    println!(
        "all 8-bit numbers in hamming weight order with numerical sub-order:\n{:02.X?}",
        hamming_iter::hamming_iter(8).collect::<Vec<u64>>()
    );
}
