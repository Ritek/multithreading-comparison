package main

import (
	"fmt"
	"math/rand"
	"sync"
)

type BallState int

const (
	PING BallState = iota
	PONG
	DONE
)

type GameState struct {
	BallState BallState
	Score int16
}

func isReceived() bool {
	val := rand.Intn(100)
	return val != 99
}

func p1(prodChan chan GameState, recChan chan GameState) {
	for {
		state := <- recChan

		switch state.BallState {
		case DONE:
			return
		case PONG:
			if !isReceived() {
				prodChan <- GameState{BallState: DONE, Score: state.Score}
				return
			}

			var currentScore = state.Score
			fmt.Println("[PING] | score: ", currentScore)
			prodChan <- GameState{BallState: PING, Score: currentScore + 1}
		case PING:
			panic("p1 received \"PING\" from p2")
		}
	}
}

func p2(prodChan chan GameState, recChan chan GameState) {
	for {
		state := <- recChan

		switch state.BallState {
		case DONE:
			return
		case PING:
			if !isReceived() {
				prodChan <- GameState{BallState: DONE, Score: state.Score}
				return
			}

			var currentScore = state.Score
			fmt.Println("[PONG] | score: ", currentScore)
			prodChan <- GameState{BallState: PONG, Score: currentScore + 1}
		case PONG:
			panic("p2 received \"PONG\" from p1")
		}
	}
}

func main() {
	var wg sync.WaitGroup

	fmt.Println("PING-PONG")
	channel1 := make(chan GameState)	
	channel2 := make(chan GameState)

	wg.Add(2)

	go func() {
		defer wg.Done()
		p1(channel1, channel2)
	}()

	go func() {
		defer wg.Done()
		p2(channel2, channel1)
	}()

	channel1 <- GameState{BallState: PING, Score: 0}
	wg.Wait()
}