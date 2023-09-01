package main

import (
	"html/template"
	"log"
	"net/http"

	"github.com/gorilla/mux"
)

func IndexHandler(todosTable *TodoRepository) http.HandlerFunc {
	return func(w http.ResponseWriter, r *http.Request) {
		log.Println("GET /todos -- Request received for " + r.URL.Path)

		tmpl, err := template.ParseFiles("web/index.html", "web/_todo.html")
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
}

func PostTodo(todosTable *TodoRepository) http.HandlerFunc {
	return func(w http.ResponseWriter, r *http.Request) {
		log.Println("POST /todos -- Request received for " + r.URL.Path)

		r.ParseForm()

		details := r.FormValue("details")

		if details == "" {
			w.WriteHeader(http.StatusBadRequest)
			http.Error(w, "Bad Request", http.StatusBadRequest)
			return
		}

		id, err := todosTable.Insert(details)
		if err != nil {
			log.Println("Error inserting todo reason " + err.Error())
			w.WriteHeader(http.StatusInternalServerError)
			http.Error(w, "Internal Server Error", http.StatusInternalServerError)
			return
		}

		newTodo, err := todosTable.GetById(id)
		if err != nil {
			log.Println("Error getting todo reason " + err.Error())
			w.WriteHeader(http.StatusInternalServerError)
			http.Error(w, "Internal Server Error", http.StatusInternalServerError)
			return
		}

		log.Println("Todo " + newTodo.Description)

		tmpl, err := template.ParseFiles("web/add.html", "web/_todo.html")
		if err != nil {
			log.Println("Error parsing template reason " + err.Error())
			w.WriteHeader(http.StatusInternalServerError)
			http.Error(w, "Internal Server Error", http.StatusInternalServerError)
			return
		}

		tmpl.Execute(w, newTodo)
	}
}

func DeleteTodo(todosTable *TodoRepository) http.HandlerFunc {
	return func(w http.ResponseWriter, r *http.Request) {
		log.Println("DELETE /todos/{ID} -- Request received for " + r.URL.Path)

		vars := mux.Vars(r)
		id := vars["todoId"]

		if id == "" {
			w.WriteHeader(http.StatusBadRequest)
			http.Error(w, "Bad Request", http.StatusBadRequest)
			return
		}

		err := todosTable.Delete(id)
		if err != nil {
			log.Println("Error deleting todo reason " + err.Error())
			w.WriteHeader(http.StatusInternalServerError)
			http.Error(w, "Internal Server Error", http.StatusInternalServerError)
			return
		}

		w.WriteHeader(http.StatusOK)
		w.Write([]byte(""))
	}
}
