package kindergarten

import (
	"bytes"
	"testing"
)

func TestPrintKindergarten(t *testing.T) {
	garden := [][]PlantName{
		{V, R, C, G, V, V, R, V, C, G, G, C, C, G, V, R, G, C, V, C, G, C, G, V},
		{V, R, C, C, C, G, C, R, R, G, V, C, G, C, R, V, V, C, V, G, C, G, C, V},
	}

	testCases := []struct {
		studentName StudentName
		expected    string
	}{
		{Alice, "Violets, radishes, violets, radishes"},
		{Bob, "Clover, grass, clover, clover"},
		{Charlie, "Violets, violets, clover, grass"},
		{David, "Radishes, violets, clover, radishes"},
		{Eve, "Clover, grass, radishes, grass"},
		{Fred, "Grass, clover, violets, clover"},
		{Ginny, "Clover, grass, grass, clover"},
		{Harriet, "Violets, radishes, radishes, violets"},
		{Ileana, "Grass, clover, violets, clover"},
		{Joseph, "Violets, clover, violets, grass"},
		{Kincaid, "Grass, clover, clover, grass"},
		{Larry, "Grass, violets, clover, violets"},
	}

	for _, testCase := range testCases {
		t.Run(testCase.studentName.String(), func(t *testing.T) {
			var output bytes.Buffer
			PrintStudentPlants(&output, garden, testCase.studentName)
			if testCase.expected != output.String() {
				t.Errorf("Expected: '%s', got '%s'", testCase.expected, output.String())
			}
		})
	}
}
