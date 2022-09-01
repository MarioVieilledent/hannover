package src

import (
	"fmt"
	"io/ioutil"
	"strings"
)

const postPath string = "./static/posts"
const postHTMLPath string = "./static/dist"

func WriteHTMLFromMd() {
	files, err := ioutil.ReadDir(postPath)
	if err != nil {
		fmt.Printf("List file in %s error: %s", postPath, err)
	}

	for _, file := range files {
		if !file.IsDir() {
			if strings.Contains(file.Name(), ".md") {
				htmlFile := strings.Replace(file.Name(), ".md", ".html", -1)
				writeFile(htmlFile, mdToHtml(readFile(file.Name())))
			}
		}
	}
}

func readFile(path string) string {
	content, err := ioutil.ReadFile(postPath + "/" + path)

	if err != nil {
		fmt.Printf("Read file error: %s", err)
		return ""
	}

	return string(content)
}

func writeFile(path string, content string) {
	err := ioutil.WriteFile(postHTMLPath+"/"+path, []byte(content), 0644)

	if err != nil {
		fmt.Printf("Write file error: %s", err)
	}
}

func mdToHtml(md string) string {
	// Escape < and > character for html format
	md = strings.ReplaceAll(md, "<", "&lt;")
	md = strings.ReplaceAll(md, ">", "&gt;")

	// Each line of md file
	lines := strings.Split(md, "\n")

	// Return of html file
	html := `<div class="block">`

	// State variables (are we in a code block ?)
	inCodeBlock := false

	for _, line := range lines {
		switch line[0] {
		case '#':
			{
				if line[0:2] == "# " {
					html += `<h1>` + line[2:] + `</h1>`
				} else if line[0:3] == "## " {
					html += `<h2>` + line[3:] + `</h2>`
				} else if line[0:4] == "### " {
					html += `<h3>` + line[4:] + `</h3>`
				} else if line[0:5] == "#### " {
					html += `<h4>` + line[5:] + `</h4>`
				} else if line[0:6] == "#### " {
					html += `<h5>` + line[6:] + `</h5>`
				} else if line[0:6] == "##### " {
					html += `<h6>` + line[6:] + `</h6>`
				} else {
					html += `<span>` + line + `</span>`
				}
			}
		case '-':
			{
				if line[0:2] == "- " {
					html += `<li>` + line[2:] + `</li>`
				} else if line[0:3] == "---" {
					html += `<div class="separator"></div>`
				} else {
					html += `<span>` + line + `</span>`
				}
			}
		case '`':
			{
				if line[0:3] == "```" {
					if inCodeBlock {
						html += `</div>`
					} else {
						html += `<div class="code">`
					}
					inCodeBlock = !inCodeBlock
				} else {
					html += `<span>` + line + `</span>`
				}
			}
		case '&':
			if line[0:5] == "&gt; " {
				html += `<div class="commentary"><span>` + line[5:] + `</span></div>`
			} else {
				html += `<span>` + line + `</span>`
			}
		default:
			html += `<span>` + line + `</span>`
		}
	}
	html += `</div>`
	return html
}
