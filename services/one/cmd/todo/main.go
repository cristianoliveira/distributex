package main

import (
	"html/template"
	"log"
	"net/http"

	"github.com/gorilla/mux"
)

func main() {
	todos, err := NewTodos()

	if err != nil {
		log.Println("Error creating todos reason " + err.Error())
		return
	}

	todos.Insert("Initial todo")

	r := mux.NewRouter()

	r.HandleFunc("/", mainHandler)

	http.Handle("/", r)

	log.Println("Starting server on port 4001")
	http.ListenAndServe("0.0.0.0:4001", nil)
}

func mainHandler(w http.ResponseWriter, r *http.Request) {
	todosTable, err := NewTodos()
	if err != nil {
		log.Println("Error instantiating todos table " + err.Error())
		w.WriteHeader(http.StatusInternalServerError)
		http.Error(w, "Internal Server Error", http.StatusInternalServerError)
		return
	}

	todosTable.Insert("Second todo")

	log.Println("GET /todos -- Request received for " + r.URL.Path)

	tmpl, err := template.ParseFiles("web/todo.html")
	if err != nil {
		log.Println("Error parsing template reason " + err.Error())
		w.WriteHeader(http.StatusInternalServerError)
		http.Error(w, "Internal Server Error", http.StatusInternalServerError)
		return
	}

	todos, err := todosTable.GetAll()
	if err != nil {
		log.Println("Error getting todos reason " + err.Error())
		w.WriteHeader(http.StatusInternalServerError)
		http.Error(w, "Internal Server Error", http.StatusInternalServerError)
		return
	}
	for _, todo := range todos {
		log.Println("Todo " + todo.Description)
	}

	tmpl.Execute(w, todos)
}
