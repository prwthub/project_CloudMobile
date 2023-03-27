use actix_web::{web, post, get, put, delete, Responder, HttpResponse};
use serde_json::json;
use crate::models::{user::*, status::*, subjects::*, master::*};
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct teacher_response {
    master: master
}

#[derive(Serialize, Deserialize)]
pub struct teacher_response_fail {
    status: status
}

// teacher [get]
// (/teacher)
#[get("/teacher")]
async fn get_teacher() -> impl Responder {
    let master_role = "teacher";
    if master_role != "teacher" {
        let combined_response = teacher_response_fail {
            status: status { 
                status: "failed".to_string(), 
                message: "You are forbidden from this action.".to_string(), 
                sessiontime: 0
            }
        };
        let response_body = json!(combined_response);
        HttpResponse::Forbidden().json(response_body) // ถ้าตัวนี้จะเป็น Status Code 403
    }
    else{
        let combined_response = teacher_response {
            master: master { 
                master_name: "prayut".to_string(), 
                master_surname: "rod o cha".to_string(), 
                master_role: "teacher".to_string() 
            }
            
        };
        let response_body = json!(combined_response);
        HttpResponse::Ok().json(response_body) // ถ้าตัวนี้จะเป็น Status Code 200
    
    }
}