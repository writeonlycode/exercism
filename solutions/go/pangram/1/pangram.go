// Package pangram defines a function to check whethera sentence is a  pangram.
package pangram

import "unicode"

// IsPangram returns true if the sentence is a pangram.
func IsPangram(input string) bool {
	var alphabet = map[rune]bool{
		'a': true,
		'b': true,
		'c': true,
		'd': true,
		'e': true,
		'f': true,
		'g': true,
		'h': true,
		'i': true,
		'j': true,
		'k': true,
		'l': true,
		'm': true,
		'n': true,
		'o': true,
		'p': true,
		'q': true,
		'r': true,
		's': true,
		't': true,
		'u': true,
		'v': true,
		'w': true,
		'x': true,
		'y': true,
		'z': true,
	}

	for _, v := range input {
		delete(alphabet, unicode.ToLower(v))

		if len(alphabet) == 0 {
			return true
		}
	}

	return false
}
