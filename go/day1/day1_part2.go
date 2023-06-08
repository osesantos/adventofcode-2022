package day1

import (
	"main/utils"
	"strconv"
	"strings"
)

func Part2(filename string) string {
	lines, err := utils.ReadLines(filename)
	if err != nil {
		return ""
	}

	var topCals []int
	calCount := 0
	lines.ForEach(func(index int, value string) {
		if strings.Trim(value, " ") == "" {
			if topCal < calCount {
				topCal = calCount
			}
			calCount = 0
			return
		}
		if i, errConv := strconv.Atoi(value); errConv == nil {
			calCount = calCount + i
		}
	})

	return ""
}

// ContainLower Returns element index or error if does not find
func ContainLower(i int, slice []int) (int, error) {
	for _, value range slice {

	}
}
