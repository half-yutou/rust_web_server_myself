use actix_web::{web, App, HttpServer};
use std::sync::Mutex;

#[path = "../state.rs"]
mod state;

#[path = "../handler.rs"]
mod handler;

#[path = "../entity.rs"]
mod entity;

fn course_router_cfg(cfg: &mut web::ServiceConfig) {
    cfg.service(web::scope("/course")
        .route("/add", web::post().to(handler::add_course))
        .route("/{teacher_id}", web::get().to(handler::get_course_from_teacher_id))
    );
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let shared_data = web::Data::new(
        state::AppState {
            health_check_response: "I'm Ok".to_string(),
            visit_count: Mutex::new(0),
            courses: Mutex::new(vec![]), 
            course_id: Mutex::new(0), 
        }
    );
    /*
    三种路由配置方式
    1. 直接使用route(pattern, handler)
    2. 使用service(handler),此时handler需要加上#[get("/health")]标记（类似sb注解）
    3. 使用configure(router_cfg),需要额外写一个cfg,好处在于可以更清晰定义路由组
     */
    let app = move || App::new()
        .app_data(shared_data.clone())
        .configure(course_router_cfg)
        .service(handler::health_check_handler)
        .route("/hey", web::get().to(handler::hey));

    HttpServer::new(app)
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
}