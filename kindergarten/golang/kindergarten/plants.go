package kindergarten

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
