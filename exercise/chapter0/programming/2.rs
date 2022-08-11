// * 在Linux环境下编写一个会产生异常的应用程序，并简要解释操作系统的处理结果。

fn main() {
    let starting_number: i64 = 600851475143;
    let mut primes = vec![true; 600851475143];

    primes[0] = false;
    primes[1] = false;

    for i in 2..((starting_number as f64).ln() as usize) {
        if primes[i] {
            let mut j = i + i;
            while j < primes.len() {
                primes[j] = false;
                j += i;
            }
        }
    }
}