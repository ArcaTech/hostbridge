package main

// NOTE: There should be NO space between the comments and the `import "C"` line.
// The -ldl is necessary to fix the linker errors about `dlsym` that would otherwise appear.

/*
#cgo LDFLAGS: ./lib/libhostbridge.a -ldl -framework Carbon -framework Cocoa -framework CoreFoundation -framework CoreVideo -framework IOKit -framework WebKit
#include "../../lib/hostbridge.h"
*/
import "C"

type TestStruct struct {
	a int32
	b uint32
}

func main() {
	test := C.struct_TestStruct{
		a: -3,
		b: 4,
	}

	C.hello(C.CString("static"))
	C.test_pass_struct(test)
	C.gomain()
}
