package dna

import (
	"errors"
)

// Histogram is a mapping from nucleotide to its count in given DNA.
type Histogram map[rune]int

// DNA is a list of nucleotides. Choose a suitable data type.
type DNA string

// Counts generates a histogram of valid nucleotides in the given DNA.
// Returns an error if d contains an invalid nucleotide.
func (d DNA) Counts() (Histogram, error) {
	var h Histogram = map[rune]int{'A': 0, 'C': 0, 'G': 0, 'T': 0}

	for _, v := range d {
		if v != 'A' && v != 'C' && v != 'G' && v != 'T' {
			return nil, errors.New("Invalid DNA")
		}

		h[v]++
	}

	return h, nil
}
