use std::vec;

use actix_web::{web, post, get, put, delete, Responder, HttpResponse};
use serde_json::json;
use crate::models::{user::*, status::*, subjects::*, master::*};
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct student {
    user_id: String,
    user_name: String,
    user_surname: String,
    user_role: String
}

#[derive(Serialize, Deserialize)]
pub struct studentmanage_response {
    user: user,
}

#[derive(Serialize, Deserialize)]
pub struct studentmanage_response_fail {
    status: status
}

// student manage [get]
// (/studentmanage)
#[get("/studentmanagement")]
async fn get_studentmanagement() -> impl Responder {
    let master_role = "admin";
    if master_role != "admin" {
        let combined_response = studentmanage_response_fail {
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
        // let combined_response = studentmanage_response {
        //     user1: student { 
        //         user_name: "perawit".to_string(), 
        //         user_surname: "anansukatham".to_string(), 
        //         user_id: "6303051613050".to_string(), 
        //         user_role: "student".to_string(),
        //     },
        //     user2: student { 
        //         user_name: "somchai".to_string(), 
        //         user_surname: "jaidee".to_string(), 
        //         user_id: "6303051613051".to_string(), 
        //         user_role: "student".to_string(),
        //     },
            
        // };
        let userlist = vec![
            student {
                user_name: "perawit".to_string(), 
                user_surname: "anansukatham".to_string(), 
                user_id: "6303051613050".to_string(), 
                user_role: "student".to_string(),
            },
            student {
                user_name: "somchai".to_string(), 
                user_surname: "jaidee".to_string(), 
                user_id: "6303051613051".to_string(), 
                user_role: "student".to_string(),
            }
        ];
        let user_response = json!(userlist);
        HttpResponse::Ok().json(user_response) // ถ้าตัวนี้จะเป็น Status Code 200
        //let response_body = json!(combined_response);
        //HttpResponse::Ok().json(response_body) // ถ้าตัวนี้จะเป็น Status Code 201
    
    }
    
}