pub fn fib(num: u32) -> u32 {
    if(num <= 1){
        return num;
    }
    return fib(num - 1) + fib(num - 2);
}