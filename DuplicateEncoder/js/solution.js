function duplicateEncode(word){
	
	// TODO case insensisitve
	
    var freqCount = [];
	
	for (var i = 0; i <= word.length - 1; i++) {
		//console.log(word[i]);
		freqCount[word[i]] = (freqCount[word[i]] === undefined) ? 1 : freqCount[word[i]] + 1;
	}
	
	//console.log(freqCount);
	
	var output = "";
	
	for (var i = 0; i <= word.length - 1; i++) {
		
		if (freqCount[word[i]] > 1) {
			output = output + ")";
		} else if (freqCount[word[i]] === 1) {
			output = output + "(";
		}
		/*
		console.log(word[i]);
		freqCount[word[i]] = (freqCount[word[i]] === undefined) ? 1 : freqCount[word[i]] + 1;
		*/
	}
	
	
	
	
    return output;
}

//var input = "din";
var input = "recede";
var output = duplicateEncode(input);
console.log(output);
