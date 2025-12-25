package main

import "fmt"

type Result struct {
	Message string
	Error error
}


func main() {
	
	ch := make(chan int, 2)
	
	ch <- 1
	ch <- 2
	// If no receiver exist goroutine will block and throw a fatal deadlock error
	// A third fmt.Println() will not solve the issue because the buffer is full 
	// ch <- 3
	
	// Receiving both messages will free the buffer
	fmt.Println(<- ch)
	fmt.Println(<- ch)

	// Sending another message now will work as long as there is a receiver
	ch <- 3
	v, ok := <- ch
	fmt.Println(v, ok)

	// Moving message send to a separate channel will not cause a deadlock
	go func() {
		ch <- 4
	}()

	// Sending two messages will fill up the channel
	ch <- 4
	ch <- 4
	// Example of a non blocking send with select
	select {
	case ch <- 5:
		fmt.Println(<- ch)
	default:
		fmt.Println("channel full")
	}
	
	fmt.Println("OK")
}

