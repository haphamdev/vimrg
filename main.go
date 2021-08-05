package main

import (
	"fmt"
	"os"
	"os/exec"
	"strings"
)

func main() {
	if len(os.Args) < 2 {
		return
	}

	//Only take the 3rd argument and ignore the rest
	query := os.Args[1]

	elements := strings.SplitN(query, ">", 2)

	count := len(elements)

	commandArgs := []string{
		"--column",
		"--line-number",
		"--no-heading",
		"--color=always",
		"--smart-case",
	}

	if count == 1 {
		commandArgs = append(commandArgs, "--", elements[0])
	} else {
		globs := strings.Fields(elements[0])

		for _, glob := range globs {
			commandArgs = append(commandArgs, "--iglob", glob)
		}

		commandArgs = append(commandArgs, "--", strings.Trim(elements[1], " "))
	}

	output, err := exec.Command("rg", commandArgs...).Output()

	if err != nil {
		fmt.Printf("No result. Error: %s\n", err)
		return
	} else {
		fmt.Printf(string(output))
	}

}
