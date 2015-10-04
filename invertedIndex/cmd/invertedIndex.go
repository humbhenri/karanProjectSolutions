package main

import (
	"bufio"
	"fmt"
	Index "github.com/humbhenri/invertedIndex"
	"os"
	"strings"
)

func main() {
	index := Index.New()
	for {
		fmt.Println("enter a to add file")
		fmt.Println("enter s to search")
		fmt.Println("enter q to quit")
		line := readInput()
		if line == "a" {
			fmt.Print("Enter file name: ")
			filename := readInput()
			index.Parse(filename)
		}
		if line == "s" {
			fmt.Print("Enter word to search: ")
			text := readInput()
			fmt.Println("Results ...")
			for _, result := range index.Get(text) {
				fmt.Printf("File: %s at line %d and column %d\n", result.File, result.Line, result.Index)
			}
		}
		if line == "q" {
			os.Exit(0)
		}
	}
}

func readInput() string {
	reader := bufio.NewReader(os.Stdin)
	line, err := reader.ReadString('\n')
	if err != nil {
		panic(err)
	}
	line = strings.TrimRight(line, "\r\n")
	return line
}
