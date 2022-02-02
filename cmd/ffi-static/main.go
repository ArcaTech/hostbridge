package main

// NOTE: There should be NO space between the comments and the `import "C"` line.
// The -ldl is necessary to fix the linker errors about `dlsym` that would otherwise appear.

/*
#cgo LDFLAGS: ./lib/libhostbridge.a -ldl -framework Carbon -framework Cocoa -framework CoreFoundation -framework CoreVideo -framework IOKit -framework WebKit
#include <stdio.h>
#include "../../lib/hostbridge.h"
*/
import "C"

import (
	"fmt"
	"unsafe"
)

func on_event(a int32) {
	fmt.Println("Go: event: ", a)
}

var OnEventFunc = on_event

func register_callback() {
	fmt.Println("Go: register_callback")
	C.RegisterCallback((*[0]byte)(unsafe.Pointer(&OnEventFunc)))
}

func main() {
	register_callback()
	C.start_loop()
}
