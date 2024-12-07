package main

import (
	"fmt"
	. "main/kindergarten"
	"os"
)

func main() {
	garden := [][]PlantName{
		{V, R, C, G, V, V, R, V, C, G, G, C, C, G, V, R, G, C, V, C, G, C, G, V},
		{V, R, C, C, C, G, C, R, R, G, V, C, G, C, R, V, V, C, V, G, C, G, C, V},
	}
	PrintStudentPlants(os.Stdout, garden, Alice)
	fmt.Println()
	PrintStudentPlants(os.Stdout, garden, Bob)
	fmt.Println()
}
