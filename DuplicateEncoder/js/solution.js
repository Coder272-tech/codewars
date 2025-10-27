function duplicateEncode(word){
	
    freqCount = [];
	
	for (var i = 0; i <= word.length - 1; i++) {
		console.log(word[i]);
		freqCount[word[i]] = (freqCount[word[i]] === undefined) ? 1 : freqCount[word[i]] + 1;
	}
	
	console.log(freqCount);
	
	var output = "";
	
	
	
	
    return output;
}

var input = "din";
duplicateEncode(input);
