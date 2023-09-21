package main

func Day01PartOne(input string) (int, error) {
	numbers, err := mapLinesToNumbers(input)
	if err != nil {
		return 0, err
	}

	// look for numbers whose sum equals 2020
	for i := 0; i < len(numbers); i++ {
		for j := i + 1; j < len(numbers); j++ {
			if numbers[i] + numbers[j] == 2020 {
				return numbers[i] * numbers[j], nil
			}
		}
	}

	return 0, nil
}

func Day01PartTwo(input string) (int, error) {
	numbers, err := mapLinesToNumbers(input)
	if err != nil {
		return 0, err
	}

	// look for 3 numbers whose sum equals 2020
	for i := 0; i < len(numbers); i++ {
		for j := i + 1; j < len(numbers); j++ {
			for k := i + 2; k < len(numbers); k++ {
				sum := numbers[i] + numbers[j] + numbers[k]
				if sum == 2020 {
					return numbers[i] * numbers[j] * numbers[k], nil
				}
			}
		}
	}

	return 0, nil
}
