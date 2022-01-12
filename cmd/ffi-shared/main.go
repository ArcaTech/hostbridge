package main

// NOTE: There should be NO space between the comments and the `import "C"` line.

/*
#cgo LDFLAGS: -L../../lib -lhostbridge
#include "../../lib/hostbridge.h"
*/
import "C"

import "fmt"

type TestStruct struct {
	a int32
	b uint32
}

func main() {
	test := C.struct_TestStruct{
		a: -3,
		b: 4,
	}

	C.hello(C.CString("shared"))
	C.test_pass_struct(test)
	handle := C.test_handle(test)
	test2 := C.get_struct_from_handle(handle)
	fmt.Printf("test2.a = %d\n", test2.a)
	fmt.Printf("test2.b = %d\n", test2.b)
	C.gomain()
}
