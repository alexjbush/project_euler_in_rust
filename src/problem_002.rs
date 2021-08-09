pub fn run() {
    let mut prev: u64 = 1;
    let mut this: u64 = 1;
    let mut agg: u64 = 0;

    while this < 4_000_000 {
        if this % 2 == 0 {
            agg += this;
        }
        let next_prev = this;
        this = prev + this;
        prev = next_prev;
    }
    println!("Answer: {}", agg);
}
