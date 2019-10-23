use std::time::Instant;
use std::collections::HashMap;

macro_rules! bench {
    ( $x:expr ) => {
        {
            let start = Instant::now();
            let result = $x;
            let end = Instant::now();
            println!("elapsed: {:?}", end - start);
            result
        }
    };
}

fn main() {
    let num = 50;
    bench!({
        println!("fibonacchi -- IN:{} ONT:{}", num, fibonacci(num));
    });
    bench!({
        let mut memo: HashMap<u64, u64> = HashMap::new();
        memo.insert(0, 0);
        memo.insert(1, 1);
        println!("fibonacchi_memo -- IN:{} ONT:{}", num, fibonacci_memo(num, &mut memo));
    });
}

fn fibonacci(n: u64) -> u64 {
    return match n {
        0 => 0,
        1 => 1,
        _ => fibonacci(n - 2) + fibonacci(n - 1)
    }
}

#[test]
fn test_fibonacchi() {
    assert_eq!(fibonacci(0), 0);
    assert_eq!(fibonacci(1), 1);
    assert_eq!(fibonacci(2), 1);
    assert_eq!(fibonacci(3), 2);
    assert_eq!(fibonacci(4), 3);
    assert_eq!(fibonacci(10), 55);
}

fn fibonacci_memo(n: u64, m: &mut HashMap<u64, u64>) -> u64 {
    return match m.get(&n) {
        None => {
            let new_num = fibonacci_memo(n - 2, m) + fibonacci_memo(n - 1, m);
            m.insert(n, new_num);
            new_num
        },
        _ => m[&n]
    }
}

#[test]
fn test_fibonacchi_memo() {
    let mut memo: HashMap<u64, u64> = HashMap::new();
    memo.insert(0, 0);
    memo.insert(1, 1);
    assert_eq!(fibonacci_memo(0, &mut memo), 0);
    assert_eq!(fibonacci_memo(1, &mut memo), 1);
    assert_eq!(fibonacci_memo(2, &mut memo), 1);
    assert_eq!(fibonacci_memo(3, &mut memo), 2);
    assert_eq!(fibonacci_memo(4, &mut memo), 3);
    assert_eq!(fibonacci_memo(10, &mut memo), 55);
}
