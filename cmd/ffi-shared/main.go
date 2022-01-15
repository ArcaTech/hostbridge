package main

// NOTE: There should be NO space between the comments and the `import "C"` line.

/*
#cgo LDFLAGS: -L../../lib -lhostbridge
#include "../../lib/hostbridge.h"
*/
import "C"

import (
	"fmt"
	"runtime"
)

type TestStruct struct {
	a int32
	b uint32
}

func RunCMain(done chan bool) {
	runtime.LockOSThread()
	C.gomain()
	done <- true
}

func main() {
	done := make(chan bool)

	C.hello(C.CString("shared"))
	go RunCMain(done)

	s := C.struct_TestStruct{
		a: -3,
		b: 4,
	}

	handle := C.get_handle_from_struct(s)

	s2 := C.get_struct_from_handle(handle)
	fmt.Printf("s2.a = %d; s2.b = %d\n", s2.a, s2.b)

	<-done
}
