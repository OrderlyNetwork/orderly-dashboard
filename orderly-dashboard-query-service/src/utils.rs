pub fn gen_rand(min: u64, max: u64) -> u64 {
    let y: f64 = rand::random();
    let offset = (max - min) as f64 * y;
    min + offset as u64
}

#[cfg(test)]
mod tests {
    use super::gen_rand;

    #[test]
    fn test_gen_rand() {
        let r1 = gen_rand(10, 20);
        let r2 = gen_rand(10, 20);
        let r3 = gen_rand(10, 20);
        let r4 = gen_rand(10, 20);
        let r5 = gen_rand(10, 20);
        println!(
            "gen_rand r1: {}, r2: {}, r3: {}, r4: {}, r5: {}",
            r1, r2, r3, r4, r5
        );
    }
}
