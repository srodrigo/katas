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
	{Violets, Radishes, Clover, Grass},
	{Violets, Radishes, Clover, Clover},
}

func formatStudentPlants(plants [4]PlantName) string {
	plantNames := ""
	for index, plant := range plants {
		plantNames += plant.String()
		if index < len(plants)-1 {
			plantNames += ", "
		}
	}

	return strings.ToUpper(plantNames[:1]) + strings.ToLower(plantNames[1:])
}

func getStudentPlants(studentName StudentName) [4]PlantName {
	yStart := studentName * 2

	return [4]PlantName{
		garden[0][yStart],
		garden[0][yStart+1],
		garden[1][yStart],
		garden[1][yStart+1],
	}
}

func PrintStudentPlants(writer io.Writer, studentName StudentName) {
	fmt.Fprint(
		writer, formatStudentPlants(getStudentPlants(studentName)))
}
