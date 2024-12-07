package kindergarten

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
