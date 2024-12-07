package kindergarten

import (
	"bytes"
	"testing"
)

func TestPrintKindergarten(t *testing.T) {
	testCases := []struct {
		studentName StudentName
		expected    string
	}{
		{
			Alice,
			"Violets, radishes, violets, radishes",
		},
		{
			Bob,
			"Clover, grass, clover, clover",
		},
	}

	for _, testCase := range testCases {
		t.Run(testCase.studentName.String(), func(t *testing.T) {
			var output bytes.Buffer
			PrintStudentPlants(&output, testCase.studentName)
			if testCase.expected != output.String() {
				t.Errorf("Expected: '%s', got '%s'", testCase.expected, output.String())
			}
		})
	}
}
