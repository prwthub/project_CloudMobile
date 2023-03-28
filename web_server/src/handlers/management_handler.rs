use actix_web::{web, post, get, put, delete, Responder, HttpResponse};
use serde_json::json;
use crate::models::{user::*, status::*, subjects::*, master::*};
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct management_response {
    master: master,
    status: status
}

#[derive(Serialize, Deserialize)]
pub struct management_response_fail {
    status: status
}

// management [get]
// (/management)
#[get("/management")]
async fn get_management() -> impl Responder {
    let master_role = "admin";
    if master_role != "admin" {
        let combined_response = management_response_fail {
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
        let combined_response = management_response {
            master: master { 
                master_name: "prayut".to_string(), 
                master_surname: "rod o cha".to_string(), 
                master_role: "admin".to_string(),
            },
            
            status: status { 
                status: "successful".to_string(), 
                message: "already logged in.".to_string(), 
                sessiontime: 2000
            }
        };
        let response_body = json!(combined_response);
        HttpResponse::Ok().json(response_body) // ถ้าตัวนี้จะเป็น Status Code 200
    
    }
}