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
	nums := []int{}
	for _, v := range lines {
		n, err := strconv.Atoi(v)
		if err != nil {
			fmt.Println(err)
		}
		nums = append(nums, n)
	}

	fmt.Println("items found", len(nums))
	increased := 0
	for i := 0; i < len(nums)-1; i++ {
		if nums[i] < nums[i+1] {
			increased++
		}
	}
	fmt.Println("part1", increased)

	increased = 0
	for i := 0; i < len(nums)-3; i++ {
		if nums[i]+nums[i+1]+nums[i+2] < nums[i+1]+nums[i+2]+nums[i+3] {
			increased++
		}
	}
	fmt.Println("part2", increased)
}
