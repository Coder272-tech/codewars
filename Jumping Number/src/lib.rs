pub fn jumping_number(n: u64) -> String {
	let digits: Vec<u32> = n
	.to_string() // Converts the number n (a u64) into a string — for example, 123 → "123".
	.chars() // Turns that string into an iterator over its characters — "123" → ['1', '2', '3'].
	
	.map(|c| c.to_digit(10).unwrap())
	
	/*
	
	Maps each character to its numeric value.

		'1' → 1

		'2' → 2

		'3' → 3
		
	The 10 means we’re interpreting the characters as base-10 digits.
	
	After this step, we have an iterator that would produce [1, 2, 3].
	
	*/
	
	.collect();
	
	/*
	
	This consumes the iterator and collects all the produced items into a collection — 
	in this case, a Vec<u32> (because of the explicit type annotation).

	So you end up with:
	digits = vec![1, 2, 3];
	
	In short:
	
	👉 .collect() takes an iterator and builds a collection (like a Vec, HashSet, etc.) out of its elements.

	Without .collect(), you’d still have an iterator, not a concrete vector you can index or loop over by position.

	*/
	
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