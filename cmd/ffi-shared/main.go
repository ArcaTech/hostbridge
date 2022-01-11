package main

// NOTE: There should be NO space between the comments and the `import "C"` line.

/*
#cgo LDFLAGS: -L../../lib -lhostbridge
#include "../../lib/hostbridge.h"
*/
import "C"

type TestStruct struct {
	a int32
	b uint32
}

func main() {
	test := TestSTruct{
		a: -3,
		b: 4,
	}

	C.hello(C.CString("shared"))
	C.test_pass_struct(test)
	C.gomain()
}
