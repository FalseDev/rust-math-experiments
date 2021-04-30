use std::collections::HashMap;
use std::io::Write;

fn fib(num: i32, memo: &mut HashMap<i32, i32>) -> i32 {
    match (*memo).get(&num) {
        Some(&value) => value,
        _ => {
            if num < 3 {
                return 1;
            }
            let value = fib(num - 1, memo) + fib(num - 2, memo);
            (*memo).insert(num, value);
            value
        }
    }
}

fn make_fib() -> impl FnMut(i32) -> i32 {
    let mut memo: HashMap<i32, i32> = HashMap::new();
    move |num| fib(num, &mut memo)
}

fn factorial(num: i64, memo: &mut HashMap<i64, i64>) -> i64 {
    match memo.get(&num) {
        Some(&value) => value,
        _ => {
            if num < 2 {
                return 1;
            };
            let value = factorial(num - 1, memo) * num;
            memo.insert(num, value);
            value
        }
    }
}

fn make_factorial() -> impl FnMut(i64) -> i64 {
    let mut memo = HashMap::new();
    move |num| factorial(num, &mut memo)
}

fn print(message: &str, num: impl ToString, stdout: &mut std::io::Stdout) {
    match stdout.write((String::from(message) + num.to_string().as_str() + "\n").as_bytes()) {
        Ok(_) => (),
        Err(_) => std::process::exit(1),
    }
}

fn main() {
    // Basic prepare
    let mut stdout = std::io::stdout();

    // Prepare functions
    let mut factorial = make_factorial();
    let mut fib = make_fib();
    for i in 1..20 {
        print(
            (String::from("Fib ") + i.to_string().as_str() + ": ").as_str(),
            fib(i),
            &mut stdout,
        );
    }
    for i in 1..20 {
        print(
            (String::from("Factorial ") + i.to_string().as_str() + ": ").as_str(),
            factorial(i.into()),
            &mut stdout,
        );
    }
}
