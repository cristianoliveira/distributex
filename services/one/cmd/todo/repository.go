package main

import (
	"database/sql"
	"log"
	"sync"

	"github.com/google/uuid"
	_ "github.com/mattn/go-sqlite3"
)

const create string = `
  CREATE TABLE IF NOT EXISTS todos (
		id TEXT NOT NULL PRIMARY KEY,
		description TEXT,
		done BOOLEAN NOT NULL DEFAULT 0
  );`

const file string = "todos.db"

type TodosDB struct {
	mu sync.Mutex
	db *sql.DB
}

type Todo struct {
	ID          string
	Description string
	Done        bool
}

func NewTodos() (*TodosDB, error) {
	db, err := sql.Open("sqlite3", file)
	if err != nil {
		return nil, err
	}

	if _, err := db.Exec(create); err != nil {
		return nil, err
	}

	return &TodosDB{
		db: db,
	}, nil
}

func (c *TodosDB) Insert(description string) (string, error) {
	id := uuid.New()
	_, err := c.db.Exec("INSERT INTO todos (id, description) VALUES(?, ?);", id.String(), description)

	if err != nil {
		println("Error inserting todo reason " + err.Error())
		return "", err
	}

	return id.String(), nil
}

func (c *TodosDB) GetAll() ([]Todo, error) {
	todos := []Todo{}

	rows, err := c.db.Query("SELECT * FROM todos;")
	if err != nil {
		println("Error inserting todo reason " + err.Error())
		return todos, err
	}

	for rows.Next() {
		var id string
		var description string
		var done bool
		err = rows.Scan(&id, &description, &done)
		if err != nil {
			println("Error inserting todo reason " + err.Error())
			return todos, err
		}

		todos = append(todos, Todo{
			ID:          id,
			Description: description,
			Done:        done,
		})

		log.Println("id: " + todos[0].ID)
	}

	return todos, nil
}
