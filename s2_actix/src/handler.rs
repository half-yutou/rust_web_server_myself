use crate::entity::Course;
use crate::state::AppState;
use actix_web::{get, web, HttpResponse, Responder};
use chrono::Utc;

pub async fn hey() -> impl Responder {
    HttpResponse::Ok().body("hey !")
}

#[get("/health")]
pub async fn health_check_handler(
    app_state: web::Data<AppState>
) -> impl Responder {
    let health_check_response = &(app_state.health_check_response);
    let mut visit_count = app_state.visit_count.lock().unwrap();
    let response = format!("{} {} times", health_check_response, visit_count);
    *visit_count += 1;
    HttpResponse::Ok().json(&response)
}

pub async fn add_course(
    app_state: web::Data<AppState>,
    new_course: web::Json<Course>,
) -> impl Responder {
    println!("Received new course");
    let mut id_gen = app_state.course_id.lock().unwrap();
    *id_gen += 1;
    let id = id_gen.clone();
    let c = Course {
        id: Some(id),
        teacher_id: new_course.teacher_id,
        name: new_course.name.clone(),
        time: Some(Utc::now().naive_utc()),
    };
    app_state.courses.lock().unwrap().push(c);
    HttpResponse::Ok().json("Course add success!")
}

pub async fn get_course_from_teacher_id(
    app_state: web::Data<AppState>,
    params: web::Path<(usize,)>, 
) -> impl Responder {
    let teacher_id = params.0;
    let filtered_courses = app_state.courses.lock().unwrap()
        .iter()
        .filter(|course| course.teacher_id == teacher_id)
        .cloned()
        .collect::<Vec<Course>>();
    
    match filtered_courses.len() { 
        0 => HttpResponse::Ok().json("empty!"), 
        _ => HttpResponse::Ok().json(filtered_courses), 
    }
}