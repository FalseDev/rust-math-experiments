use std::collections::HashMap;

fn fib_with_memo(num: i32, memo: &mut HashMap<i32, i32>) -> i32 {
    match (*memo).get(&num) {
        Some(&value) => value,
        _ => {
            if num < 3 {
                return 1;
            }
            let value = fib_with_memo(num - 1, memo) + fib_with_memo(num - 2, memo);
            (*memo).insert(num, value);
            value
        }
    }
}

fn make_fib() -> impl FnMut(i32) -> i32 {
    let mut memo: HashMap<i32, i32> = HashMap::new();
    move |num| fib_with_memo(num, &mut memo)
}

fn factorial_with_memo(num: i64, memo: &mut HashMap<i64, i64>) -> i64 {
    match memo.get(&num) {
        Some(&value) => value,
        _ => {
            if num < 2 {
                return 1;
            };
            let value = factorial_with_memo(num - 1, memo) * num;
            memo.insert(num, value);
            value
        }
    }
}

fn make_factorial() -> impl FnMut(i64) -> i64 {
    let mut memo = HashMap::new();
    move |num| factorial_with_memo(num, &mut memo)
}

fn main() {
    // Prepare functions
    let mut factorial = make_factorial();
    let mut fib = make_fib();

    for i in 1..20 {
        println!("fib({}): {}", i, fib(i));
    }
    for i in 1..20 {
        println!("factorial({}): {}", i, factorial(i));
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_fib() {
        let mut fib = make_fib();
        let expected: Vec<(i32, i32)> = vec![(1, 1), (2, 1), (3, 2), (4, 3)];
        for (input, output) in expected.iter() {
            assert_eq!(fib(*input), *output);
        }
    }

    #[test]
    fn test_factorial() {
        let mut factorial = make_factorial();
        let expected: Vec<(i64, i64)> = vec![(0, 1), (1, 1), (2, 2), (3, 6), (4, 24)];
        for (input, output) in expected.iter() {
            assert_eq!(factorial(*input), *output);
        }
    }
}
