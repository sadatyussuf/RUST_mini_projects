/// Create an empty vector
pub fn create_empty() -> Vec<u8> {
    let mut v:Vec<u8> = Vec::new();
    return v
}
/// Create a buffer of `count` zeroes.
///
/// Applications often use buffers when serializing data to send over the network.
pub fn create_buffer(count: usize) -> Vec<u8> {
    let mut v:Vec<u8> = Vec::new();
    for _ in 0..count{
        v.push(0)
    }
    return v
        
}
/// Create a vector containing the first five elements of the Fibonacci sequence.
///
/// Fibonacci's sequence is the list of numbers where the next number is a sum of the previous two.
/// Its first five elements are `1, 1, 2, 3, 5`.
pub fn fibonacci() -> Vec<u8> {
    let mut fst = 1;
    let mut snd = 1;
    let mut v:Vec<u8> = vec![1,1];
    for _ in 0..3{
        let mut tmp = 0;
         tmp = fst + snd;
        v.push(tmp);
        fst = snd;
        snd = tmp;
    }
    return v
}