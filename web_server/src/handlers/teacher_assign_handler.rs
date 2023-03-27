use actix_web::{web, post, get, put, delete, Responder, HttpResponse};
use serde_json::json;
use crate::models::{user::*, status::*, subjects::*, master::*};
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct teacher_assign_request {
    master: master,

    user_id: String,
    user_name: String,
    user_surname: String,
    
    subjects: subjects
}

#[derive(Serialize, Deserialize)]
pub struct teacher_assign_response {
    user_id: String,
    user_name: String,
    user_surname: String,
    
    subjects: subjects,

    status: status,
}

#[derive(Serialize, Deserialize)]
pub struct teacher_assign_response_fail {
    status: status,
}

// teacher assign [post]
// (/teacher/assign)
#[post("/teacher/assign")]
async fn post_teacher_assign(input_data: web::Json<teacher_assign_request>) -> impl Responder {
    let req = input_data.into_inner();
    
    let master_name = req.master.master_name;
    let master_surname = req.master.master_surname;
    let master_role = req.master.master_role;
    
    let user_id = req.user_id;
    let user_name = req.user_name;
    let user_surname = req.user_surname;

    let python = req.subjects.python;
    let java = req.subjects.java;
    let matlab = req.subjects.matlab;
    let rust = req.subjects.rust;

    if master_role != "teacher" {
        let combined_response = teacher_assign_response_fail {
            status: status { 
                status: "failed".to_string(), 
                message: "You are forbidden from this action.".to_string(), 
                sessiontime: 0
            }
        };
        let response_body = json!(combined_response);
        HttpResponse::Forbidden().json(response_body) // ถ้าตัวนี้จะเป็น Status Code 401
    }
    else{
        let combined_response = teacher_assign_response {
            user_id: user_id,
            user_name: user_name,
            user_surname: user_surname,

            subjects: subjects { 
                python: "A".to_string(),
                java: "A".to_string(),
                matlab: "A".to_string(),
                rust: "A".to_string() 
            },
            
            status: status { 
                status: "successful".to_string(), 
                message: "Grading Successful.".to_string(), 
                sessiontime: 3000
            }
        };
        let response_body = json!(combined_response);
        HttpResponse::Created().json(response_body) // ถ้าตัวนี้จะเป็น Status Code 401
    
    }
}

