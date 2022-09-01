package src

import (
	"fmt"
	"os"

	"github.com/gin-contrib/cors"
	"github.com/gin-contrib/static"
	"github.com/gin-gonic/gin"
)

var url string = "0.0.0.0"

// Setup du port dans env pour Heroku
var port string = getPort()

func LaunchAPI() {

	gin.SetMode(gin.ReleaseMode)

	router := gin.New()

	router.Use(cors.New(cors.Config{
		AllowOrigins:     []string{"*"},
		AllowMethods:     []string{"*"},
		AllowHeaders:     []string{"Origin"},
		ExposeHeaders:    []string{"Content-Length"},
		AllowCredentials: true,
	}))

	router.Use(static.Serve("/", static.LocalFile("static", false)))

	fmt.Println("Server listening on " + url + ":" + port)
	router.Run(url + ":" + port)
}

// Get either port of env else 3000
func getPort() (port string) {
	port = os.Getenv("PORT")
	if port == "" {
		port = "3000"
	}
	return
}
