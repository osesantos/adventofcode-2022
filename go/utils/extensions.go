package utils

type StringExtensions interface {
	ForEach(func(...any))
}

type StringArray []string

func (s StringArray) ForEach(f func(index int, value string)) {
	for i, v := range s {
		f(i, v)
	}
}

type IntSlice []int

func (is IntSlice) Sum() int {
	var s int
	for _, v := range is {
		s += v
	}
	return s
}
