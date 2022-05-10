package main

import (
	"bufio"
	"fmt"
	"os"
	"strings"
)

const version = "1.0.0"

// Counting mode
type mode int
const (
	chars mode = iota
	lines
	words
	all
)

// file holds the counters for a single file
type file struct {
	chars int64
	lines int64
	words int64
}

func readFile(filePath string) (fileInformation file, err error) {
	file, err := os.Open(filePath)
	if err != nil {
		return
	}
	defer file.Close()

	scanner := bufio.NewScanner(file)
	for scanner.Scan() {
		fileInformation.lines++
		fileInformation.chars++ // Count newline character

		fileInformation.chars += int64(len(scanner.Text()))
		fileInformation.words += int64(len(strings.Fields(scanner.Text())))
	}

	err = scanner.Err()
	return
}

func printCounts(counts file, mode mode, filename string) {
	switch mode {
	case chars:
		fmt.Printf(" %5d total\n", counts.chars);
	case lines:
		fmt.Printf(" %5d total\n", counts.lines);
	case words:
		fmt.Printf(" %5d total\n", counts.words);
	case all:
		fmt.Printf(" %5d %5d %5d %s\n", counts.lines, counts.words, counts.chars, filename);
	}
}

func main() {
	mode := all
	var files []string

	// Normally you would use the go internal "flag" package to parse commandline arguments
	// but the long options (e.g. --chars) only are allowed to use one dash with the flag package (-chars).
	// For equal usage for all our wc implementations we parse the commandline arguments manually.
	// There are also libraries doing this (e.g. github.com/pborman/getopt) but we try to avoid external libraries
	// for our wc implementations (refer to CONTRIBUTING.md).
	for _, arg := range os.Args[1:] {
		switch arg {
		case "-v":
			fallthrough
		case "--version":
			fmt.Println("xc version", version)
			return

		case "-h":
			fallthrough
		case "--help":
			fmt.Println("Usage:" +
				"\n\txc [FILES] [OPTIONS]" +
				"\n\n-m  --chars" +
				"\n\tPrint characters in file" +
				"\n\n-l  --lines" +
				"\n\tPrint lines in file" +
				"\n\n-w  --words" +
				"\n\tPrint words in file")
			return

		case "-m":
			fallthrough
		case "--chars":
			mode = chars

		case "-l":
			fallthrough
		case "--lines":
			mode = lines

		case "-w":
			fallthrough
		case "--words":
			mode = words

		default:
			files = append(files, arg)
		}
	}

	if len(files) == 0 {
		fmt.Println("xc: Not enough arguments")
		os.Exit(1)
	}

	var totalCounts file

	for _, filename := range files {
		f, err := readFile(filename)
		if err != nil {
			fmt.Println(err)
			os.Exit(1)
		}
		
		printCounts(f, mode, filename)
		totalCounts.chars += f.chars
		totalCounts.lines += f.lines
		totalCounts.words += f.words
	}

	if len(files) > 1 {
		printCounts(totalCounts, mode, "total")
	}
}
