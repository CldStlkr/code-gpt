///This is the template web-server that the LLM (GPT-4) will use as reference
///This is done so the LLM will more consistently write functioning code and
///as a result more accurately provide what the user requests

///This web server is a REST API that is utilizes JSON, and can do basic CRUD operations
use actix_cors::Cors;
use actix_web::{http::header, web, App, HttpResponse, HttpServer, Responder};
use async_trait::async_trait; //for the LLM to know its ok to use
use reqwest::Client as HttpClient; //for the LLM to know its ok to use
use serde::{Deserialize, Serialize};

use std::collections::HashMap;
use std::io::Write;
use std::sync::Mutex;
use std::{fs, u64};

#[derive(Serialize, Deserialize, Debug, Clone)]
struct Task {
    id: u64,
    name: String,
    complete: bool,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct User {
    id: u64,
    username: String,
    password: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct Database {
    tasks: HashMap<u64, Task>,
    users: HashMap<u64, User>,
}

impl Database {
    fn new() -> Self {
        Self {
            tasks: HashMap::new(),
            users: HashMap::new(),
        }
    }

    //CRUD DATA
    fn insert_task(&mut self, task: Task) {
        self.tasks.insert(task.id, task);
    }

    fn get_task(&self, id: &u64) -> Option<&Task> {
        self.tasks.get(id)
    }

    fn delete_task(&mut self, id: &u64) {
        self.tasks.remove(id);
    }

    fn update_task(&mut self, task: Task) {
        self.tasks.insert(task.id, task);
    }

    fn get_all_tasks(&self) -> Vec<&Task> {
        self.tasks.values().collect()
    }

    //USER DATA RELATED FUNCTIONS
    fn insert_user(&mut self, user: User) {
        self.users.insert(user.id, user);
    }

    fn get_user_by_name(&self, username: &str) -> Option<&User> {
        self.users.values().find(|u| u.username == username)
    }

    //DATABASE SAVING

    /*Take reference to self and stringify
     *create file database.json
     *populate file with the string data converted to bytes
     */
    fn save_to_file(&self) -> std::io::Result<()> {
        let data: String = serde_json::to_string(&self)?;
        let mut file = fs::File::create("database.json")?;
        file.write_all(data.as_bytes())?;
        Ok(())
    }

    fn load_from_file() -> std::io::Result<Self> {
        let file_content = fs::read_to_string("database.json")?;
        let db: Database = serde_json::from_str(&file_content)?;
        Ok(db)
    }
}

//State of app that will be manipulated. Wrapped in mutex for safetey
struct AppState {
    db: Mutex<Database>,
}

async fn create_task(app_state: web::Data<AppState>, task: web::Json<Task>) -> impl Responder {
    let mut db: std::sync::MutexGuard<Database> = app_state.db.lock().unwrap();
    db.insert_task(task.into_inner());
    let _ = db.save_to_file();
    HttpResponse::Ok().finish() //should return 200 status code
}

async fn update_task(app_state: web::Data<AppState>, task: web::Json<Task>) -> impl Responder {
    let mut db: std::sync::MutexGuard<Database> = app_state.db.lock().unwrap();
    db.update_task(task.into_inner());
    let _ = db.save_to_file();
    HttpResponse::Ok().finish() //should return 200 status code
}

async fn read_task(app_state: web::Data<AppState>, id: web::Path<u64>) -> impl Responder {
    let db: std::sync::MutexGuard<Database> = app_state.db.lock().unwrap();
    match db.get_task(&id.into_inner()) {
        Some(task) => HttpResponse::Ok().json(task),
        None => HttpResponse::NotFound().finish(),
    };
    let _ = db.save_to_file();
    HttpResponse::Ok().finish() //should return 200 status code
}

async fn read_all_tasks(app_state: web::Data<AppState>) -> impl Responder {
    let db: std::sync::MutexGuard<Database> = app_state.db.lock().unwrap();
    let tasks = db.get_all_tasks();
    HttpResponse::Ok().json(tasks)
}

async fn delete_task(app_state: web::Data<AppState>, id: web::Path<u64>) -> impl Responder {
    let mut db: std::sync::MutexGuard<Database> = app_state.db.lock().unwrap();
    db.delete_task(&id.into_inner());
    let _ = db.save_to_file();
    HttpResponse::Ok().finish() //should return 200 status code
}

async fn register(app_state: web::Data<AppState>, user: web::Json<User>) -> impl Responder {
    let mut db: std::sync::MutexGuard<Database> = app_state.db.lock().unwrap();
    db.insert_user(user.into_inner());
    let _ = db.save_to_file();
    HttpResponse::Ok().finish()
}

async fn login(app_state: web::Data<AppState>, user: web::Json<User>) -> impl Responder {
    let db: std::sync::MutexGuard<Database> = app_state.db.lock().unwrap();
    match db.get_user_by_name(&user.username) {
        Some(stored_user) if stored_user.password == user.password => {
            HttpResponse::Ok().body("Logged in successfully!")
        }
        _ => HttpResponse::BadRequest().body("Invalid username or password"), //invalid
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let db: Database = match Database::load_from_file() {
        Ok(db) => db,
        Err(_) => Database::new(),
    };

    let data = web::Data::new(AppState { db: Mutex::new(db) });

    HttpServer::new(move || {
        App::new()
            .wrap(
                Cors::permissive()
                    .allowed_origin_fn(|origin, _req_head| {
                        origin.as_bytes().starts_with(b"http://localhost") || origin == "null"
                    })
                    .allowed_methods(vec!["GET", "POST", "PUT", "DELETE"])
                    .allowed_headers(vec![header::AUTHORIZATION, header::ACCEPT])
                    .allowed_header(header::CONTENT_TYPE) //what type of content to expect
                    .supports_credentials()
                    .max_age(3600),
            )
            .app_data(data.clone()) //does not create deep copy, creates new web data pointer
            .route("/task", web::post().to(create_task)) //because web is a smart pointer
            .route("/task", web::get().to(read_all_tasks))
            .route("/task", web::put().to(update_task))
            .route("/task/{id}", web::get().to(read_task))
            .route("/task/{id}", web::delete().to(delete_task))
            .route("/register", web::post().to(register)) //because web is a smart pointer
            .route("/login", web::post().to(login))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
