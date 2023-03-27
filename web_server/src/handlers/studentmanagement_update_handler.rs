use std::string;

use actix_web::{web, post, get, put, delete, Responder, HttpResponse};
use serde_json::json;
use crate::models::{user::*, status::*, master::*, subjects::*};
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct studentmanagement_update_request {
    master_role: String,

    user_name: String,
    user_surname: String,
    user_id: String,
    user_status: String,
    user_year: String
}

#[derive(Serialize, Deserialize)]
pub struct studentmanagement_update_response {
    user_name: String,
    user_surname: String,
    user_id: String,
    user_status: String,
    user_year: String,
    status: status,
}

#[derive(Serialize, Deserialize)]
pub struct studentmanagement_update_response_fail {
    status: status,
}


// student manage update [put]
// (/studentmanage/update)
#[put("/studentmanagement/update")]
async fn put_studentmanagement_update(input_data: web::Json<studentmanagement_update_request>) -> impl Responder {
    let req = input_data.into_inner();

    let master_role = req.master_role;
    let user_name = req.user_name;
    let user_surname = req.user_surname;
    let user_id = req.user_id;
    let user_status = req.user_status;
    let user_year = req.user_year;

    if master_role != "teacher"{
        let combined_response = studentmanagement_update_response_fail {
            status: status { 
                status: "failed".to_string(), 
                message: "access denied".to_string(), 
                sessiontime: 0
            },
        };
        let response_body = json!(combined_response);
        HttpResponse::Forbidden().json(response_body) // ถ้าตัวนี้จะเป็น Status Code 403
    }
    else{
        let combined_response = studentmanagement_update_response {
            
            user_name: user_name,
            user_surname: user_surname,
            user_id: user_id,
            user_status: user_status,
            user_year: user_year,

            status: status { 
                status: "successful".to_string(), 
                message: "Student Updated Successfully.".to_string(), 
                sessiontime: 3000
            },
        };
        let response_body = json!(combined_response);
        HttpResponse::Created().json(response_body) // ถ้าตัวนี้จะเป็น Status Code 201
    }
}