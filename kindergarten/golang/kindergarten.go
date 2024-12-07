package kindergarten

import (
	"fmt"
	"io"
	"strings"
)

type PlantName int

const (
	Violets PlantName = iota + 1
	Radishes
	Clover
	Grass
)

func (name PlantName) String() string {
	switch name {
	case Violets:
		return "violets"
	case Radishes:
		return "radishes"
	case Clover:
		return "clover"
	case Grass:
		return "grass"
	default:
		panic("Unsupported plant name")
	}
}

type StudentName int

const (
	Alice StudentName = iota
	Bob
)

func (name StudentName) String() string {
	switch name {
	case Alice:
		return "Alice"
	case Bob:
		return "Bob"
	default:
		panic("Unsupported student name")
	}
}

var garden = [...][]PlantName{
	{Violets, Radishes, Violets, Radishes},
	{Clover, Grass, Clover, Clover},
}

func formatStudentPlants(plants []PlantName) string {
	plantNames := ""
	for index, plant := range plants {
		plantNames += plant.String()
		if index < len(plants)-1 {
			plantNames += ", "
		}
	}

	return strings.ToUpper(plantNames[:1]) + strings.ToLower(plantNames[1:])
}

func getStudentPlants(studentName StudentName) []PlantName {
	return garden[studentName]
}

func PrintStudentPlants(writer io.Writer, studentName StudentName) {
	fmt.Fprint(
		writer, formatStudentPlants(getStudentPlants(studentName)))
}
