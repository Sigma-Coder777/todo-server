A todo backend server is considered a very good beginner project to learn about backend,databases etc .As i was learning about backend and rust i created this along with a frontend cli,webapp and will create a tauri app in future

This server manages a simple to-do list application, allowing you to create, read, update, and delete tasks.

# Tech Used
+ Axum 
+ Surrealdb
+ Rust
+ Serde
# Features
+ CRUD Operations: Create, Read, Update, and Delete to-do items.
+ JSON API: Communicate using a JSON-based API.
+ Lightweight and Fast: Built with Rust for performance and safety.
+ Easy to Extend: Modular design for easy customization and extension.
# Requirements
Before running the server, ensure you have the following installed:
+ Rust: The programming language used to build this project. 

+ Cargo: The Rust package manager, usually installed alongside Rust.

+ SurrealDB: The database engine.

# Running

```bash
git clone https://github.com/Sigma-Coder777/todo-server.git
cd todo-server
./run_surrealdb
cargo run
```
The server will start on http://localhost:8000. You can change the port by modifying `.env` file see `.env.example`

# API Endpoints
1. List All ToDos

    URL: /list
    Method: GET
    Response: JSON array of all todos

2. Toggle the done state

    URL: /toggle/
    Method: GET
    Body : String of ID
    Response: JSON object of the specific todo

3. Create a New ToDo

    URL: /add
    Method: POST
    Body: String containing the todo title
    Response: JSON object of the created todo

4. Delete a ToDo

    URL: /remove/
    Method: POST
    Body : String of ID
    Response: JSON object of deleted todo

# LICENSE
This project is licensed under the MIT License. See the LICENSE file for details.
