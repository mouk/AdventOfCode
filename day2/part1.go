package main

// Importing packages
import (
	"fmt"
	"io/ioutil"
	"strconv"
	"strings"
)

// Main function
func main() {

	content, err := ioutil.ReadFile("input.txt") // the file is inside the local directory
	if err != nil {
		fmt.Println(err)
	}
	lines := strings.Split(string(content), "\r\n")
	horizontal := 0
	depth := 0
	for _, v := range lines {
		pair := strings.Split(string(v), " ")
		n, err := strconv.Atoi(pair[1])
		if err != nil {
			fmt.Println(err)
		}
		switch pair[0] {
		case "forward":
			horizontal += n
		case "down":
			depth += n
		case "up":
			depth -= n

		}
	}

	fmt.Println("part1 ", "horizontal ", horizontal, "depth ", depth, horizontal*depth)
	horizontal = 0
	depth = 0
	aim := 0
	for _, v := range lines {
		pair := strings.Split(string(v), " ")
		n, err := strconv.Atoi(pair[1])
		if err != nil {
			fmt.Println(err)
		}
		switch pair[0] {
		case "forward":
			horizontal += n
			depth += aim * n
		case "down":
			aim += n
		case "up":
			aim -= n

		}
	}

	fmt.Println("Part2 ", "horizontal ", horizontal, "depth ", depth, horizontal*depth)
}
