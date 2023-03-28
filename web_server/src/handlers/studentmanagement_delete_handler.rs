use actix_web::{web, post, get, put, delete, Responder, HttpResponse};
use serde_json::json;
use crate::models::{user::*, status::*, master::*, subjects::*};
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct studentmanagement_delete_request {
    master_role: String,
    user_id: String,
}

#[derive(Serialize, Deserialize)]
pub struct studentmanagement_delete_response {
    status: status,
}

// student manage delete [delete]
// (/studentmanage/delete)
#[delete("/studentmanagement/delete")]
async fn del_studentmanagement_delete(input_data: web::Json<studentmanagement_delete_request>) -> impl Responder {
    let req = input_data.into_inner();

    let user_id = req.user_id;
    let master_role = req.master_role;

    if master_role != "admin"{
        let combined_response = studentmanagement_delete_response {
            status: status { 
                status: "failed".to_string(), 
                message: "access denied".to_string(), 
                sessiontime: 0
            }
        };
        let response_body = json!(combined_response);
        HttpResponse::Forbidden().json(response_body) // ถ้าตัวนี้จะเป็น Status Code 403
    }
    else{
        let combined_response = studentmanagement_delete_response {
            status: status { 
                status: "successful".to_string(), 
                message: "student delete successfully".to_string(), 
                sessiontime: 2000
            }
        };
        let response_body = json!(combined_response);
        HttpResponse::Ok().json(response_body) // ถ้าตัวนี้จะเป็น Status Code 200
    }
}