package day4

import (
	"main/utils"
	"strconv"
	"strings"
)

type Pair struct {
	a string
	b string
}

func Part1(filename string) string {
	lines, err := utils.ReadLines(filename)
	if err != nil {
		return ""
	}

	counter := 0
	var setsRaw [][]string
	for _, l := range lines {
		setsRaw = append(setsRaw, parseLine(l))
	}

	var sets [][]string
	for _, s := range setsRaw {
		for _, v := range s {
			sets = append(sets, parseSet(v))
		}
	}

	for i, v := range sets {
		if i != 0 && i%2 == 1 && fullyContains(sets[i-1], v) {
			counter += 1
		}
	}

	return strconv.Itoa(counter)
}

func parseLine(line string) []string {
	return strings.Split(line, ",")
}

func parseSet(set string) []string {
	return strings.Split(set, "-")
}

func fullyContains(a []string, b []string) bool {
	if a[0] < b[0] && a[1] > b[1] {
		return true
	}
	if b[0] < a[0] && b[1] > a[1] {
		return true
	}
	return false
}
