#[macro_use]
extern crate rocket;

use rocket::{ response::content, http::Status };
use serde::{ Serialize, Deserialize };
use rocket::serde::json::{ Json, to_string, from_str };
use redis::Commands;
use nanoid::nanoid;

#[derive(Serialize, Deserialize, Clone, Debug)]
struct Todo {
    #[serde(default)]
    id: String,
    title: String,
    content: String,
    status: i32,
}

#[derive(Serialize, Deserialize, Clone)]
struct TodoCreatedResponse {
    message: String,
    id: String,
}

#[derive(Serialize, Deserialize, Clone)]
struct NewUserResponse {
    message: String,
    id: String,
}

#[get("/")]
fn index() -> content::RawJson<&'static str> {
    // Add redis health cehck here
    content::RawJson("{\"message\": \"\"}")
}

#[get("/health")]
fn health() -> content::RawJson<&'static str> {
    dotenv::dotenv().ok();
    let redis_url: String = dotenv::var("REDIS_URL").expect("REDIS_URL must be set");
    let client: redis::Client = redis::Client::open(redis_url).unwrap();
    let mut con: redis::Connection = client.get_connection().unwrap();

    let pong: redis::RedisResult<()> = redis::cmd("PING").query(&mut con);

    match pong {
        Ok(_) => content::RawJson("{\"message\": \"Redis is running.\"}"),
        Err(_) => content::RawJson("{\"message\": \"Redis is not running.\"}"),
    }
}

#[post("/user/new")]
fn new_user() -> Json<NewUserResponse> {
    let user: String = nanoid!();
    let response: NewUserResponse = NewUserResponse {
        message: "New user created.".to_string(),
        id: user.clone(),
    };

    Json(response)
}

#[get("/todos/<user>")]
fn get_todos(user: String) -> Json<Vec<Todo>> {
    let todos: Vec<Todo> = get_objects(user).expect("Failed to retrieve todos from Redis");

    Json(todos)
}

#[get("/todo/<id>")]
fn get_todo_by_id(id: String) -> Result<Json<Todo>, Status> {
    let response: Result<Todo, _> = get_object(id);

    match response {
        Ok(todo) => Ok(Json(todo)),
        Err(_) => Err(Status::NotFound),
    }
}

#[post("/todo/<user>", format = "json", data = "<todo>")]
fn add_todo(user: String, todo: Json<Todo>) -> Result<Json<TodoCreatedResponse>, Status> {
    let valid_status: bool = match todo.status {
        0 | 1 | 2 => true,
        _ => false,
    };

    if !valid_status {
        return Err(Status::BadRequest);
    }

    let id: String = format!("{}:{}", user, nanoid!());

    let new_todo: Todo = Todo {
        id,
        title: todo.title.clone(),
        content: todo.content.clone(),
        status: todo.status.clone(),
    };

    set_object(new_todo.clone()).expect("Failed to store TODO in Redis");

    let created_todo_response: TodoCreatedResponse = TodoCreatedResponse {
        message: "Todo added successfully.".to_string(),
        id: new_todo.id,
    };
    Ok(Json(created_todo_response))
}

#[put("/todo/<id>/<status>")]
fn update_todo(id: String, status: i32) -> Result<content::RawJson<&'static str>, Status> {
    let valid_status: bool = match status {
        0 | 1 | 2 => true,
        _ => false,
    };

    if !valid_status {
        return Err(Status::BadRequest);
    }

    let mut todo: Todo = get_object(id).expect("Failed to get object from Redis");

    if todo.status == status {
        return Err(Status::Conflict);
    }

    todo.status = status;
    set_object(todo).expect("Failed to update TODO in Redis");

    Ok(content::RawJson("{\"message\": \"Todo status updated.\"}"))
}

#[delete("/todo/<id>")]
fn delete_todo_by_id(id: String) -> Result<content::RawJson<&'static str>, Status> {
    let result: Result<(), redis::RedisError> = delete_object(id);

    match result {
        Ok(_) => Ok(content::RawJson("{\"message\": \"Todo deleted.\"}")),
        Err(_) => Err(Status::InternalServerError),
    }
}

#[catch(400)]
fn bad_request() -> content::RawJson<&'static str> {
    content::RawJson("{\"message\": \"Bad request.\"}")
}

#[catch(404)]
fn not_found() -> content::RawJson<&'static str> {
    content::RawJson("{\"message\": \"Resource not found.\"}")
}

#[catch(409)]
fn conflict() -> content::RawJson<&'static str> {
    content::RawJson("{\"message\": \"Todo already completed.\"}")
}

#[launch]
fn rocket() -> _ {
    rocket
        ::build()
        .mount(
            "/",
            routes![
                index,
                get_todos,
                get_todo_by_id,
                add_todo,
                update_todo,
                delete_todo_by_id,
                health,
                new_user
            ]
        )
        .register("/", catchers![not_found, conflict, bad_request])
}

fn set_object(todo: Todo) -> redis::RedisResult<()> {
    dotenv::dotenv().ok();
    let redis_url = dotenv::var("REDIS_URL").expect("REDIS_URL must be set");
    let client: redis::Client = redis::Client::open(redis_url)?;
    let mut con: redis::Connection = client.get_connection()?;

    let todo_json: String = to_string(&todo).unwrap();

    let _: () = con.set(todo.id, todo_json)?;

    Ok(())
}

fn get_objects(user: String) -> redis::RedisResult<Vec<Todo>> {
    dotenv::dotenv().ok();
    let redis_url: String = dotenv::var("REDIS_URL").expect("REDIS_URL must be set");
    let client: redis::Client = redis::Client::open(redis_url)?;
    let mut con: redis::Connection = client.get_connection()?;

    let query = format!("{}:*", user);
    let keys: Vec<String> = con.keys(query)?;

    let mut values: Vec<Todo> = Vec::new();
    for key in keys {
        let json: String = con.get(&key)?;
        let todo: Todo = from_str(&json).unwrap();
        values.push(todo);
    }

    Ok(values)
}

fn get_object(id: String) -> redis::RedisResult<Todo> {
    dotenv::dotenv().ok();
    let redis_url = dotenv::var("REDIS_URL").expect("REDIS_URL must be set");
    let client: redis::Client = redis::Client::open(redis_url)?;
    let mut con: redis::Connection = client.get_connection()?;
    let todo_json: Result<String, _> = con.get(id);

    match todo_json {
        Ok(json) => {
            let todo: Todo = from_str(&json).unwrap();
            println!("Todo: {:?}", todo);
            Ok(todo)
        }
        Err(_) => {
            // Handle the error here
            println!("Failed to get object from Redis: key may not exist or is not a string");
            Err(
                redis::RedisError::from((
                    redis::ErrorKind::TypeError,
                    "Operation against a key holding the wrong kind of value",
                ))
            )
        }
    }
}

fn delete_object(id: String) -> redis::RedisResult<()> {
    dotenv::dotenv().ok();
    let redis_url = dotenv::var("REDIS_URL").expect("REDIS_URL must be set");
    let client: redis::Client = redis::Client::open(redis_url)?;
    let mut con: redis::Connection = client.get_connection()?;

    let _: () = con.del(id)?;

    Ok(())
}
