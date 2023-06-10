package main

import (
	"fmt"
	"main/day1"
	"main/day2"
	"main/day3"
)

func main() {
	// Day 1
	fmt.Printf("Day 1.1 sample: %s\n", day1.Part1("day1_sample"))
	fmt.Printf("Day 1.1: %s\n", day1.Part1("day1"))

	fmt.Printf("Day 1.2 sample: %s\n", day1.Part2("day1_sample"))
	fmt.Printf("Day 1.2: %s\n", day1.Part2("day1"))

	fmt.Println()

	// Day 2
	fmt.Printf("Day 2.1 sample: %s\n", day2.Part1("day2_sample"))
	fmt.Printf("Day 2.1: %s\n", day2.Part1("day2"))

	fmt.Printf("Day 2.2 sample: %s\n", day2.Part2("day2_sample"))
	fmt.Printf("Day 2.2: %s\n", day2.Part2("day2"))

	fmt.Println()

	// Day 3
	fmt.Printf("Day 3.1 sample: %s\n", day3.Part1("day3_sample"))
	fmt.Printf("Day 3.1: %s\n", day3.Part1("day3"))

	fmt.Printf("Day 3.2 sample: %s\n", day3.Part2("day3_sample"))
	fmt.Printf("Day 3.2: %s\n", day3.Part2("day3"))

	fmt.Println()

}
