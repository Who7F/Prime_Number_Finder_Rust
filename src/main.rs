use std::io::{stdin,stdout,Write};

fn get_prime() -> Vec<u32>{
	vec![2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37, 41, 43, 47, 53, 59, 61, 67, 71, 73, 79, 83, 89, 97]
	
}

fn get_number(msg: &str)-> u32{
	loop{
		let mut s = String::new();
	
		print!("{} of range: ", msg);
		let _= stdout().flush();
		stdin().read_line(&mut s).expect("Did not enter a correct string");
	
		if let Some('\n')=s.chars().next_back() {
			s.pop();
		}
		if let Some('\r')=s.chars().next_back() {
			s.pop();
		}
	
		match s.trim().parse::<u32>(){
			Ok(num) if num >= 1 && num <= 100 => return num,
			_ => println!("fubar")
		}
	}
}

fn main() {
	println!("Welcome to the prime number finder. Works with a range for 1 to a 100");
	
	// Gets input for user
    let start = get_number("Start");
	let end = get_number("End");
	
	// Some magic with sorting out user output
	let (start, end) = if start <= end {(start, end)} else {(end, start)};
	
    println!("Prime in range of {} and {}:",start, end);
	
	//Get prime vec
	let primes = get_prime();
	
	// Prints primes in range
	for i in start..=end{
		if primes.contains(&i){
			println!("{}", i);
		}
	}
}
