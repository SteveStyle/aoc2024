type Fib = u128;

pub const fn fib2(n: usize) -> Fib {
    if n <= 1 {
        n as u128
    } else {
        fib2(n - 2) + fib2(n - 1)
    }
}

#[allow(long_running_const_eval)]
pub const FIBRESULT2: Fib = fib2(35);

pub fn main() {
    println!("{FIBRESULT2}");
}

// const FIBLIMIT1: usize = 150;

// pub const FIBARRAY: [Fib; FIBLIMIT1 + 1] = {
//     let mut a: [Fib; FIBLIMIT1 + 1] = [0; FIBLIMIT1 + 1];
//     a[0] = 0;
//     a[1] = 1;
//     let mut i: usize = 2;
//     while i <= FIBLIMIT1 {
//         a[i] = a[i - 1] + a[i - 2];
//         i += 1;
//     }
//     a
// };

// const fn fib1(n: usize) -> Fib {
//     FIBARRAY[n]
// }

// pub const FIBRESULT1: Fib = fib1(26);
