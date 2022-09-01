package main

import "Hannover/src"

func main() {
	// Converts md file to html file
	src.WriteHTMLFromMd()

	// Starts the gin server
	src.LaunchAPI()
}
