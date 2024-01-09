fn main() {
    print_items_ab_nested(10, 8);
}

// this function is O(n^2) even if
// there is seccond iteration for k that run with O(n)
fn print_items(n: i32) {
    // this is O(n^2)
    for i in 0..n {
        for j in 0..n {
            print!("{}{}\n", i, j);
        }
    }

    // this is O(n)
    for k in 0..n {
        println!("{}", k);
    }
}

fn add_items(n: i32) -> i32 {
    // this is O(2) because we use 2 operation, but we can say it O(1)
    return n + n + n;
}

// different term of input
// this function is O(n + b)
fn print_items_ab(a: i32, b: i32) {
    for i in 0..a {
        println!("{}", i);
    }

    for j in 0..b {
        println!("{}", j)
    }
}

// this is O(a*b)
fn print_items_ab_nested(a: i32, b: i32) {
    for i in 0..a {
        for j in 0..b {
            println!("{}{}", i, j);
        }
    }
}
