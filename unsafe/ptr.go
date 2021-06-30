hello
package main

import (
	"fmt"
	"unsafe"
)

// Key Rules
// unsafe.Pointer <=> uintptr
// ptr <=> unsafe.Pointer

func main() {
	arr := []int{1, 2, 3, 4, 5}

	for i := 0; i < len(arr); i++ {
		fmt.Println(*(*int)(unsafe.Pointer(uintptr(unsafe.Pointer(&arr[0])) + (uintptr(i) * unsafe.Sizeof(int(0))))))
	}
}
