use actix_web::{web, post, get, put, delete, Responder, HttpResponse};
use serde_json::json;
use crate::models::{user::*, status::*, master::*};
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct register_request {
    master: master,
    user: user,

}

#[derive(Serialize, Deserialize)]
pub struct register_response {
    user: user,
    status: status,
}

#[derive(Serialize, Deserialize)]
pub struct register_response_fail {
    status: status,
}

// register [post]
// (/register)
#[post("/register")] 
async fn post_register(input_data: web::Json<register_request>) -> impl Responder {
    // ค่าเริ่มต้น ที่รับมาแบบ json (ถ้าอยากแก้ไข เติม mut หลัง let)
    let req = input_data.into_inner();

    // request from user
    let master_name = req.master.master_name;
    let master_surname = req.master.master_surname;
    let master_role = req.master.master_role;

    let user_name = req.user.user_name;
    let user_surname = req.user.user_surname;
    let user_id = req.user.user_id;
    let user_password = req.user.user_password;
    let user_role = req.user.user_role;
    let user_status = req.user.user_status;
    let user_year = req.user.user_year;


    if master_role != "admin" {
        let combined_response = register_response_fail {
            status: status { 
                status: "failed".to_string(), 
                message: "access denied".to_string(), 
                sessiontime: 0
            },
        };
        let response_body = json!(combined_response);
        HttpResponse::Unauthorized().json(response_body) // ถ้าตัวนี้จะเป็น Status Code 401
    }
    else{
        let combined_response = register_response {
            user: user { 
                user_name: user_name, 
                user_surname: user_surname, 
                user_id: user_id, 
                user_password: user_password, 
                user_role: user_role,
                user_status: user_status,
                user_year: user_year, 
            },
            status: status { 
                status: "successful".to_string(), 
                message: "registration successful".to_string(), 
                sessiontime: 1000
            },
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