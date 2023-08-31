package main

import (
	"html/template"
	"log"
	"net/http"

	"github.com/gorilla/mux"
)

func main() {
	r := mux.NewRouter()

	r.HandleFunc("/", mainHandler)

	http.Handle("/", r)

	log.Println("Starting server on port 4001")
	http.ListenAndServe("0.0.0.0:4001", nil)
}

func mainHandler(w http.ResponseWriter, r *http.Request) {

	log.Println("GET /todos -- Request received for " + r.URL.Path)

	tmpl, err := template.ParseFiles("web/main.html")
	if err != nil {
		log.Println("Error parsing template reason " + err.Error())
		w.WriteHeader(http.StatusInternalServerError)
		http.Error(w, "Internal Server Error", http.StatusInternalServerError)
		return
	}

	tmpl.Execute(w, nil)
}
