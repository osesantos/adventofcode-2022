package day5

import (
	"main/utils"
	"strconv"
	"strings"
)

func Part1(filename string) string {
	lines, err := utils.ReadLines(filename)
	if err != nil {
		return ""
	}

	return strconv.Itoa(len(lines))
}

func numberOfStacks(lines []string) int {
	i := 0
	utils.ForEach(lines, func(index int, value string) {
		if isEmptyLine(value) {
			i = len(strings.ReplaceAll(lines[index-1], " ", ""))
		}
	})
	return i
}

func isEmptyLine(line string) bool {
	return strings.Trim(line, " ") == ""
}

func fillStack(lines []string) [][]rune {
	numStacks := numberOfStacks(lines)
	stacks := make([][]rune, numStacks)

	firstLines := utils.Filter(lines, func(s string) bool {
		return strings.ContainsRune(s, '[')
	})

	firstLines = utils.Map(firstLines, func(s string) string {
		cleanedLine := strings.ReplaceAll(s, "   ", "0")
		cleanedLine = strings.ReplaceAll(cleanedLine, "[", "")
		cleanedLine = strings.ReplaceAll(cleanedLine, "]", "")
		cleanedLine = strings.ReplaceAll(cleanedLine, " ", "")

		for i := len(cleanedLine); i < numStacks; i++ {
			cleanedLine = cleanedLine + "0"
		}

		return cleanedLine
	})

	return stacks
}

func invertMatrix(runeMatrix [][]rune) [][]rune {
	var inverted = make([][]rune, len(runeMatrix))
	utils.ForEach(runeMatrix, func(ia int, a []rune) {
		utils.ForEach(a, func(ib int, b rune) {

		})
	})
	return inverted
}
