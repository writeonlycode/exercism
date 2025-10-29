// Package luhn provides a function to determine whether a given id is a valid
// according to Luhn's formula.
package luhn

import (
	"strconv"
	"strings"
	"unicode"
	"unicode/utf8"
)

// Valid returns true if the given id is a valid according to Luhn's formula.
func Valid(id string) bool {
	id = strings.ReplaceAll(id, " ", "")

	if utf8.RuneCountInString(id) <= 1 {
		return false
	}

	if strings.LastIndexFunc(id, func(s rune) bool { return !unicode.IsDigit(s) }) != -1 {
		return false
	}

	reversed := make([]int, 0)

	for _, v := range id {
		digit, _ := strconv.Atoi(string(v))
		reversed = append([]int{digit}, reversed...)
	}

	doubled := make([]int, 0)

	for i, v := range reversed {
		if i == 1 || i%2 != 0 {
			v *= 2

			if v > 9 {
				v -= 9
			}

			doubled = append(doubled, v)
		} else {
			doubled = append(doubled, v)
		}
	}

	sum := 0

	for _, v := range doubled {
		sum += v
	}

	return sum%10 == 0
}
