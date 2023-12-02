package main

import (
	"log"
	"net/http"
	"os"

	"github.com/gorilla/mux"
)

func main() {
	port := os.Getenv("PORT")
	if port == "" {
		port = "4001"
	}

	todosRepo, err := NewTodosRepository()
	if err != nil {
		log.Println("Error creating todos reason " + err.Error())
		return
	}

	log.Println("Starting server on port 4001")

	http.ListenAndServe("0.0.0.0:"+port, RuterWithContext(todosRepo))
}

func RedirectTo(url string) http.HandlerFunc {
	return func(w http.ResponseWriter, r *http.Request) {
		http.Redirect(w, r, url, http.StatusSeeOther)
	}
}

func RuterWithContext(repo *TodoRepository) http.Handler {
	router := mux.NewRouter().StrictSlash(true)

	router.HandleFunc("/", RedirectTo("/todos")).Methods("GET")
	router.HandleFunc("/todos", IndexHandler(repo)).Methods("GET")
	router.HandleFunc("/todos", PostTodo(repo)).Methods("POST")
	router.HandleFunc("/todos/{todoId}", PutTodo(repo)).Methods("PUT")
	router.HandleFunc("/todos/{todoId}", DeleteTodo(repo)).Methods("DELETE")

	return router
}
