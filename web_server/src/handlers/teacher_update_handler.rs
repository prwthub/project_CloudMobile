use actix_web::{web, post, get, put, delete, Responder, HttpResponse};
use serde_json::json;
use crate::models::{user::*, status::*, master::*, subjects::*};
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct teacher_update_request {
    master_role: String,

    user_id: String,
    user_name: String,
    user_surname: String,

    subject: subjects
}

#[derive(Serialize, Deserialize)]
pub struct teacher_update_response {
    user_id: String,
    user_name: String,
    user_surname: String,

    subject: subjects,

    status: status,
}

#[derive(Serialize, Deserialize)]
pub struct teacher_update_response_fail {
    status: status,
}

// teacher update [put]
// (/teacher/update)
#[put("/teacher/update")]
async fn put_teacher_update(input_data: web::Json<teacher_update_request>) -> impl Responder {
    // ค่าเริ่มต้น ที่รับมาแบบ json (ถ้าอยากแก้ไข เติม mut หลัง let)
    let req = input_data.into_inner();

    let user_name = req.user_name;
    let user_surname = req.user_surname;
    let user_id = req.user_id;

    let python = req.subject.python;
    let java = req.subject.java;
    let matlab = req.subject.matlab;
    let rust = req.subject.rust;

    let master_role = req.master_role;

    if master_role != "teacher"{
        let combined_response = teacher_update_response_fail {
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
        let combined_response = teacher_update_response {
            
            user_name: user_name,
            user_surname: user_surname,
            user_id: user_id,

            subject: subjects { 
                python: python, 
                java: java, 
                matlab: matlab, 
                rust: rust 
            },
            status: status { 
                status: "successful".to_string(), 
                message: "Grade Updated Successfully.".to_string(), 
                sessiontime: 3000
            },
        };
        let response_body = json!(combined_response);
        HttpResponse::Created().json(response_body) // ถ้าตัวนี้จะเป็น Status Code 201
    }
}