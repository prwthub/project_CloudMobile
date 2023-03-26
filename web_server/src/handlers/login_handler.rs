use actix_web::{web, post, get, put, delete, Responder, HttpResponse};

// login [post]
// (/login)
#[get("/login")]
async fn post_login() -> impl Responder {
    
    HttpResponse::Ok().json("login")
}

// use actix_web::{web, HttpResponse};
// use serde::Deserialize;

// #[derive(Deserialize)]
// struct LoginRequest {
//     username: String,
//     password: String,
// }

// pub async fn login(login_data: web::Json<LoginRequest>) -> HttpResponse {
//     // Check if the provided username and password match a user in the database
//     let is_valid = validate_user(&login_data.username, &login_data.password);

//     if is_valid {
//         HttpResponse::Ok().body("Login successful!")
//     } else {
//         HttpResponse::Unauthorized().body("Invalid username or password")
//     }
// }

// fn validate_user(username: &str, password: &str) -> bool {
//     // Perform validation logic here
//     true // Replace this with your actual validation code
// }