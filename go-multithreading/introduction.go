package main

import (
	"fmt"
	"time"
)

func say(s string) {
	for i := 0; i < 5; i++ {
		time.Sleep(100 * time.Millisecond)
		fmt.Println(s)
	}
}

func main() {
	// "go" starts a new goroutine managed by Go runtime
	// evaluation of a function and parameters happen in current current goroutine
	// execution of a function happens in a new goroutine
	go say("world")
	say("hello")
}
