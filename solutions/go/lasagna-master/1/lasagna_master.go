package lasagna

func PreparationTime(layers []string, time int) int {
	if time == 0 {
		time = 2
	}

	return len(layers) * time
}

func Quantities(layers []string) (noodles int, sauce float64) {
	for i := 0; i < len(layers); i++ {
		switch layers[i] {
		case "noodles":
			noodles += 50

		case "sauce":
			sauce += 0.2
		}
	}

	return
}

func AddSecretIngredient(friendList []string, myList []string) {
	myList = append(myList[:len(myList)-1], friendList[len(friendList)-1])
}

func ScaleRecipe(original []float64, portions int) []float64 {
	var scaled []float64

	for i := 0; i < len(original); i++ {
		scaled = append(scaled, (original[i]/2)*float64(portions))
	}

	return scaled
}
