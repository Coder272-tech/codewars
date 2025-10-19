fn main() {
	let num : u64 = 9119;
	let mut str : String = "".to_string(); 
	
	let digits: Vec<u64> = num
	.to_string()
	.chars()
	.map(|c| c.to_digit(10).unwrap() as u64)
	.collect();
	
	for i in 0..digits.len() {
		str.push_str( &(digits[i]*digits[i]).to_string()  );
	}
	println!("result: {}", str);
}
