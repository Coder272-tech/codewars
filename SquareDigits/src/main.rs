fn square_digits(num: u64) -> u64 {
	//let num : u64 = 9119;
	let mut str : String = "".to_string(); 
	
	let digits: Vec<u64> = num
	.to_string()
	.chars()
	.map(|c| c.to_digit(10).unwrap() as u64)
	.collect();
	
	for i in 0..digits.len() {
		str.push_str( &(digits[i]*digits[i]).to_string()  );
	}
	return str.parse::<u64>().unwrap();
}


fn main() {

	println!("result: {}", square_digits(9119));
}
