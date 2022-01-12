package main

// NOTE: There should be NO space between the comments and the `import "C"` line.
// The -ldl is necessary to fix the linker errors about `dlsym` that would otherwise appear.

/*
#cgo LDFLAGS: ./lib/libhostbridge.a -ldl -framework Carbon -framework Cocoa -framework CoreFoundation -framework CoreVideo -framework IOKit -framework WebKit
#include "../../lib/hostbridge.h"
*/
import "C"

import "fmt"

type TestStruct struct {
	a int32
	b uint32
}

func main() {
	C.hello(C.CString("shared"))

	s := C.struct_TestStruct{
		a: -3,
		b: 4,
	}

	handle := C.get_handle_from_struct(s)

	s2 := C.get_struct_from_handle(handle)
	fmt.Printf("s2.a = %d; s2.b = %d\n", s2.a, s2.b)

	C.gomain()
}
