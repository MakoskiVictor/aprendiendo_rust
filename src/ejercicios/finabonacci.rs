pub fn finabonacci (n:u32) -> u32 {
    if n == 0 {
        0
    } else if n == 1 {
        1
    } else {
        finabonacci(n-1) + finabonacci(n-2)
    }
}