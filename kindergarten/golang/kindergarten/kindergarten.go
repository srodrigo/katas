package kindergarten

import (
	"fmt"
	"io"
	"strings"
)

type PlantName int

const (
	V PlantName = iota + 1
	R
	C
	G
)

func (name PlantName) String() string {
	switch name {
	case V:
		return "violets"
	case R:
		return "radishes"
	case C:
		return "clover"
	case G:
		return "grass"
	default:
		panic("Unsupported plant name")
	}
}

type StudentName int

const (
	Alice StudentName = iota
	Bob
	Charlie
	David
	Eve
	Fred
	Ginny
	Harriet
	Ileana
	Joseph
	Kincaid
	Larry
)

func (name StudentName) String() string {
	switch name {
	case Alice:
		return "Alice"
	case Bob:
		return "Bob"
	case Charlie:
		return "Charlie"
	case David:
		return "David"
	case Eve:
		return "Eve"
	case Fred:
		return "Fred"
	case Ginny:
		return "Ginny"
	case Harriet:
		return "Harriet"
	case Ileana:
		return "Ileana"
	case Joseph:
		return "Joseph"
	case Kincaid:
		return "Kincaid"
	case Larry:
		return "Larry"
	default:
		panic("Unsupported student name")
	}
}

func joinPlants(plants [4]PlantName) string {
	plantNames := ""
	for index, plant := range plants {
		plantNames += plant.String()
		if index < len(plants)-1 {
			plantNames += ", "
		}
	}

	return plantNames
}

func formatStudentPlants(plants [4]PlantName) string {
	plantNames := joinPlants(plants)

	return strings.ToUpper(plantNames[:1]) + strings.ToLower(plantNames[1:])
}

func getStudentPlants(garden [][]PlantName, studentName StudentName) [4]PlantName {
	yStart := studentName * 2

	return [4]PlantName{
		garden[0][yStart],
		garden[0][yStart+1],
		garden[1][yStart],
		garden[1][yStart+1],
	}
}

func PrintStudentPlants(writer io.Writer, garden [][]PlantName, studentName StudentName) {
	fmt.Fprint(
		writer, formatStudentPlants(getStudentPlants(garden, studentName)))
}
