extern crate rand;

use self::rand::distributions::uniform::{SampleBorrow, SampleUniform};
use self::rand::distributions::{Distribution, Standard};
use rand::Rng;
/// 产生随机数
pub fn gen_rand<T>() -> T
where
    Standard: Distribution<T>,
{
    let mut rng = rand::thread_rng();
    rng.gen()
}
/// 产生指定范围的随机数
pub fn gen_rand_rang<T: SampleUniform, B1, B2>(low: B1, high: B2) -> T
where
    B1: SampleBorrow<T> + Sized,
    B2: SampleBorrow<T> + Sized,
{
    let mut rng = rand::thread_rng();
    rng.gen_range(low, high)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rand() {
        let test: i32 = gen_rand();
        println!("{}", test)
    }
    #[test]
    fn test_rand_rang() {
        let test: u64 = gen_rand_rang(1, 10000000000000000000);
        println!("{}", test)
    }
}
