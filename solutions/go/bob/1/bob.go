// Package bob defines a funtion to determine what Bob will reply to someone
// when they say something to him or ask him a question.
package bob

import (
	"strings"
	"unicode"
)

func isQuestion(remark string) bool {
	return remark[len(remark)-1] == '?'
}

func hasLetters(remark string) bool {
	for _, v := range remark {
		if unicode.IsLetter(v) {
			return true
		}

	}
	return false
}

func isYelling(remark string) bool {
	for _, v := range remark {
		if unicode.IsLetter(v) && !unicode.IsUpper(v) {
			return false
		}
	}

	return hasLetters(remark)
}

func isSilence(remark string) bool {
	for _, v := range remark {
		if !unicode.IsSpace(v) {
			return false
		}
	}

	return true
}

// Hey takes a remark and returns what Bob will reply.
func Hey(remark string) string {
	remark = strings.TrimSpace(remark)
	switch {
	case isSilence(remark):
		return "Fine. Be that way!"
	case isQuestion(remark) && isYelling(remark):
		return "Calm down, I know what I'm doing!"
	case isQuestion(remark):
		return "Sure."
	case isYelling(remark):
		return "Whoa, chill out!"
	default:
		return "Whatever."
	}
}
