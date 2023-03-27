use actix_web::{web, post, get, put, delete, Responder, HttpResponse};
use serde_json::json;
use crate::models::{user::*, status::*, subjects::*, master::*};
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct grade_response {
    user: user,
    subjects: subjects,
    status: status
}

#[derive(Serialize, Deserialize)]
pub struct grade_response_fail {
    status: status
}

// grade [get]
// (/grade)
#[get("/grade")]
async fn get_grade() -> impl Responder {
    let sessiontime = 1000;
    if sessiontime == 0 {
        let combined_response = grade_response_fail {
            status: status { 
                status: "time out".to_string(), 
                message: "Session expired, please login.".to_string(), 
                sessiontime: 0
            }
        };
        let response_body = json!(combined_response);
        HttpResponse::Forbidden().json(response_body) // ถ้าตัวนี้จะเป็น Status Code 403
    }
    else{
        let combined_response = grade_response {
            user: user { 
                user_name: "perawit".to_string(), 
                user_surname: "anansukatham".to_string(), 
                user_id: "6303051613050".to_string(), 
                user_password: "perawit007".to_string(), 
                user_role: "student".to_string(),
                user_status: "enrolled".to_string(),
                user_year: 3, 
            },
            
            subjects: subjects { 
                python: "A".to_string(),
                java: "A".to_string(),
                matlab: "A".to_string(),
                rust: "A".to_string() 
            },
            
            status: status { 
                status: "successful".to_string(), 
                message: "already logged in.".to_string(), 
                sessiontime: 1000
            }
        };
        let response_body = json!(combined_response);
        HttpResponse::Created().json(response_body) // ถ้าตัวนี้จะเป็น Status Code 201
    
    }
}