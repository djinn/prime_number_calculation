

use rand::prelude::*;
use rayon::prelude::*;



fn power(mut x: u64, mut y: u64, p: u64) -> u64 {
	let mut res: u64 = 1;
	x = x % p;
	while y > 0 {
		if y & 1 == 1 {
			res = (res * x) % p;
		}
		y = y >> 1;
		x = (x * x) % p;
	}
	return res;
}

fn miller_test(mut d: u64, n: u64) -> bool {

	let mut rng = thread_rng();
	let a: u64 = (2 + rng.gen_range(0, n - 4)).into();
	let mut x: u64 = power(a, d, n);

	if x == 1 || x == (n - 1) {
		return true
	}
	while  d != (n - 1) {
		x = (x * x) % n;
		d *= 2;
		if x == 1 {
			return false
		}
		if x == (n - 1) {
			return true
		}
	}
	return false
}


fn is_prime(n: u64, k: i16) -> bool {

	// Since n is always greater than or equal to zero
	// negative values is not possible
	if n == 4 {
		return false
	}
	if n <= 3 {
		return true
	}

	let mut d: u64 = n - 1;

	while d % 2 == 0 {
		d /= 2;
	}


	for _ in 1..k {
		if miller_test(d, n) == false {
			return false
		}
	}

	return true
}

fn main() {


	let n: u64 = 10000000;
	let k: i16 = 200;
	
	let is_prime_n: Vec<u64> = (0..n).collect();


	let is_prime_status: Vec<bool> = is_prime_n.par_iter()
								 .map(|&x| is_prime(x, k))
								 .filter(|&x| x == true)
								 .collect();
	



	
	//for i in 4..n {
	//	is_prime_status.push(is_prime(i, k));
	//}

	//is_prime_status.retain(|&p| p == true);

    println!("{}", is_prime_status.len());
}
