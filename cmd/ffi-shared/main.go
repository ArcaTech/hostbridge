package main

// NOTE: There should be NO space between the comments and the `import "C"` line.

/*
#cgo LDFLAGS: -L../../lib -lhostbridge
#include <stdio.h>
#include "../../lib/hostbridge.h"
*/
import "C"

import (
	"fmt"
	"unsafe"
)

type Callback struct {
	Func func(int32)
}

func on_event(a int32) {
	fmt.Println("Go: event: ", a)
}

var OnEventFunc = on_event

func register_callback() {
	fmt.Println("Go: register_callback")
	//C.RegisterCallback((*[0]byte)(unsafe.Pointer(&OnEventFunc)))
	C.RegisterNewCallback(&Callback{
		Func: on_event,
	})
}

func main() {
	register_callback()
	C.start_loop()
}
