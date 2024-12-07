package kindergarten

import (
	"fmt"
	"io"
	"strings"
)

func PrintStudentPlants(writer io.Writer, garden [][]PlantName, studentName StudentName) {
	fmt.Fprint(
		writer, formatPlants(getStudentPlants(garden, studentName)))
}

func formatPlants(plants [4]PlantName) string {
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
