package main

import (
	"fmt"
	"io/ioutil"
	"os"
	"strings"
)

func main() {
	var dp, ip int
	ptr := [30000]byte{}

	data, _ := ioutil.ReadFile(os.Args[1])
	
	splitted := strings.Split(string(data), "")

	for ip < len(splitted) {

		switch splitted[ip] {
		case ">":
			dp++
			ip++

		case "<":
			dp--
			ip++

		case "+":
			ptr[dp] += 1
			ip++

		case "-":
			ptr[dp] -= 1
			ip++

		case ".":
			fmt.Printf("%c", ptr[dp])
			ip++

		case ",":
			_, _ = fmt.Scanf("%c", &ptr[dp])
			ip++

		case "[":
			if ptr[dp] == 0 {
				ip = findBracket(splitted, ip, true) + 1
			} else {
				ip++
			}

		case "]":
			if ptr[dp] != 0 {
				ip = findBracket(splitted, ip, false) + 1
			} else {
				ip++
			}

		default:
			ip++

		}
	}
}

func findBracket(code []string, pointer int, dir bool) int {
	j := 1

	if dir {
		for i := pointer + 1; i < len(code); i++ {
			if code[i] == "]" {
				j--
				if j == 0 {
					return i
				}
			} else if code[i] == "[" {
				j++
			}
		}
	} else {
		for i := pointer - 1; i >= 0; i-- {
			if code[i] == "[" {
				j--
				if j == 0 {
					return i
				}
			} else if code[i] == "]" {
				j++
			}
		}
	}

	return -1
}
