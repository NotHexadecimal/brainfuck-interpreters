package main

import (
	"fmt"
	"io/ioutil"
	"os"
	"os/exec"
	"runtime"
	"strings"
)

func main() {
	var ip int

	data, _ := ioutil.ReadFile(os.Args[1])

	splitted := strings.Split(string(data), "")
	f, _ := os.Create(os.Args[1] + ".go")

	_, _ = f.WriteString("package main\nimport \"fmt\"\nfunc main() {\nvar dp int\nptr := [30000]byte{}\n")

	for ip < len(splitted) {

		switch splitted[ip] {
		case ">":
			_, _ = f.WriteString("dp++\n")
			ip++

		case "<":
			_, _ = f.WriteString("dp--\n")
			ip++

		case "+":
			_, _ = f.WriteString("ptr[dp] += 1\n")
			ip++

		case "-":
			_, _ = f.WriteString("ptr[dp] -= 1\n")
			ip++

		case ".":
			_, _ = f.WriteString("fmt.Printf(\"%c\", ptr[dp])\n")
			ip++

		case ",":
			_, _ = f.WriteString("_, _ = fmt.Scanf(\"%c\", &ptr[dp])\n")
			ip++

		case "[":
			_, _ = f.WriteString("for ptr[dp] > 0 {\n")
			ip++

		case "]":
			_, _ = f.WriteString("}\n")
			ip++

		default:
			ip++

		}
	}

	_, _ = f.WriteString("}")
	_ = f.Close()

	executableName := strings.TrimSuffix(os.Args[1], ".bf")

	switch runtime.GOOS {
	case "windows":
		_ = exec.Command("go", "build", "-o", executableName+".exe", os.Args[1]+".go").Run()
		fmt.Println("Generated " + executableName + ".exe")
	default:
		_ = exec.Command("go", "build", "-o", executableName, os.Args[1]+".go").Run()
		fmt.Println("Generated " + executableName)
	}
}
