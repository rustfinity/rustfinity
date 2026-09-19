use std::thread;

/// Sums each chunk on its own thread, keeping the input order.
pub fn sum_chunks(chunks: Vec<Vec<u64>>) -> Vec<u64> {
    // TODO: spawn every thread first, collect the handles, then join
    unimplemented!()
}

/// Sums every chunk in parallel and adds the partial sums together.
pub fn sum_all(chunks: Vec<Vec<u64>>) -> u64 {
    // TODO: reuse sum_chunks
    unimplemented!()
}

// Example usage
pub fn main() {
    let chunks = vec![vec![1, 2], vec![10], vec![]];

    println!("{:?}", sum_chunks(chunks.clone()));
    println!("{}", sum_all(chunks));
}
