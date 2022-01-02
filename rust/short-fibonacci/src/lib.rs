/// Create an empty vector
pub fn create_empty() -> Vec<u8> {
    create_buffer(0)
}

/// Create a buffer of `count` zeroes.
///
/// Applications often use buffers when serializing data to send over the network.
pub fn create_buffer(count: usize) -> Vec<u8> {
    vec![0; count]
}

/// Create a vector containing the first five elements of the Fibonacci sequence.
///
/// Fibonacci's sequence is the list of numbers where the next number is a sum of the previous two.
/// Its first five elements are `1, 1, 2, 3, 5`.
pub fn fibonacci() -> Vec<u8> {
    let mut fibonacci_vec = create_buffer(5);
    fibonacci_vec[0] = 1;
    fibonacci_vec[1] = 1;
    for i in 2..5 {
        fibonacci_vec[i] = fibonacci_vec[i-1] + fibonacci_vec[i-2]
    };
    fibonacci_vec
}