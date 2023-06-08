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
