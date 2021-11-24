use std::mem; //brings in the mem crate from the rust standard library

// the fuction takes two signed integer n and p of variable size and returns a signed integer.
fn modinv(n: isize, p: isize) -> isize { 
	// this checks if the other integer(p) is equal to one. The reason beign that we're using the euclidean algo to calculate the mod inverse. //Euclidean algo finds the greatest common denominator of two intgers, p and n, and two integers, say, x and y such that px + ny is the gcd of n and p.
	// p and n are also called bezout's coefficient. So, if the second integer is one, the gcd will be equal to one therefore, the modinverse will also be 1. 
	if p == 1 { return 1 } 

	// A tuple is created to hold the bezout's coefficients and the inverse(inv) is initialized as 1. A tuple is a compound type in Rust used to group multiple values with different type. A tuple cannot grow or shrink in size once declared, it stays constant. 
	let (mut x, mut y, mut q, mut inv) = (n, p, 0, 1);

	while x > 1 {
		inv -= (x / y) * q;
		x = x % y; 
		mem::swap(&mut x, &mut y); 
		mem::swap(&mut q, &mut inv);
		//mem::swap swaps the values at two mutable locations. That is, as long as x is greater than 1, we subtract the result of the product of the bezout coefficients divided by q, from the inverse. then, the first integer in the bezout coefficient becomes the remainder of the division of the bezout coefficients. Then, the values of the bezout and that of q and the inverse coefficients are swapped. 
		// Quick note: the ampersand & before the args in mem::swap denotes that the reference of the respective variables should be taken because we want to swap them without deinitializing either one of them.
	} 

	if inv < 0 { inv += p }
	inv
}

fn main() {
  println!("{}", modinv(42, 2017))
}
