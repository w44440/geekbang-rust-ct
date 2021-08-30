fn fib_loop(n: u32) {
    let (mut a, mut b, mut i) = (1,1,2);
    loop {
        next_fib_value(&mut a, &mut b);
        i += 1;

        if i >= n {
            break;
        }
    }
}

fn fib_while(n: u32) {
    let (mut a, mut b, mut i) = (1,1,2);

    while i < n {
        next_fib_value(&mut a, &mut b);
        i += 1;
    }
}


fn fib_for(n: u32) {
    #[allow(unused_variables)]
    let (mut a, mut b) = (1,1);
    
    for _i in 2..n {
        next_fib_value(&mut a, &mut b);
    }
}

fn next_fib_value(a: &mut u32, b: &mut u32) {
    let c = *a + *b;
    *a = *b;
    *b = c;

    println!("next val is {}", b);
}
fn main() {
    let n = 10;
    fib_loop(n);
    fib_while(n);
    fib_for(n);
}