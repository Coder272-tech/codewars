pub fn jumping_number(n: u64) -> String {
	let digits: Vec<u32> = n
	.to_string()
	.chars()
	.map(|c| c.to_digit(10).unwrap())
	.collect();
	
	if digits.len() == 1 {
		return "Jumping!!".to_string();
	}
	
	for i in 0..digits.len() - 1 {
		if (digits[i] as i32 - digits[i + 1] as i32).abs() != 1 {
			return "Not!!".to_string();
		}
	}
	
	"Jumping!!".to_string()
}