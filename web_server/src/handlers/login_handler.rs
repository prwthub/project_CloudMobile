use actix_web::{web, post, get, put, delete, Responder, HttpResponse};
use serde_json::json;
use crate::models::{user::*, status::*};
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct login_request {
    user_id: String,
    user_password: String,
}

#[derive(Serialize, Deserialize)]
pub struct login_response {
    status: status,
}

// login [post]
// (/login)
#[post("/login")] 
async fn post_login(input_data: web::Json<login_request>) -> impl Responder {
    // ค่าเริ่มต้น ที่รับมาแบบ json (ถ้าอยากแก้ไข เติม mut หลัง let)
    let req = input_data.into_inner();

    // request from user
    let user_id = req.user_id;
    let user_password = req.user_password;

    // status
    // let status = "";
    // let message = "";
    // let sessiontime = 0;

    if user_id != "6303051613050" {
        let combined_response = login_response {
            // user: user { 
            //     user_name: "".to_string(), 
            //     user_surname: "".to_string(), 
            //     user_id: "".to_string(), 
            //     user_password: "".to_string(), 
            //     user_role: "".to_string(), 
            // },
            // status: status { 
            //     status: "failed".to_string(), 
            //     message: "invalid login".to_string(), 
            //     sessiontime: 0
            // },
            status: status { 
                status: "failed".to_string(), 
                message: "Incorrect Login Infomation, Please Try Again.".to_string(), 
                sessiontime: 0
            }
        };
        let response_body = json!(combined_response);
        HttpResponse::Unauthorized().json(response_body) // ถ้าตัวนี้จะเป็น Status Code 401
    }
    else{
        let combined_response = login_response {
            status: status {  
                status: "successful".to_string(), 
                message: "student login successfully.".to_string(), 
                sessiontime: 1000
            }
        };
        let response_body = json!(combined_response);
        HttpResponse::Created().json(response_body) // ถ้าตัวนี้จะเป็น Status Code 201
    }

    // let combined_response = CombinedResponse {
    //     status: "".to_string(),
    //     message: name.to_string(),
    //     sessiontime: 1000,
    // };
    // let response_body = json!(combined_response);

    // if name == "invalid" {
    //     // HttpResponse::Ok().json(response_body) // ถ้าตัวนี้จะเป็น Status Code 200  
    //     HttpResponse::ExpectationFailed().json(response_body) // ถ้าตัวนี้จะเป็น Status Code 404 
    // }
    // else{
    //     HttpResponse::Created().json(response_body) // ถ้าตัวนี้จะเป็น Status Code 201
    // }


}