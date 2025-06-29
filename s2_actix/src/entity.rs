use actix_web::web::Json;
use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Course {
    pub id: Option<usize>, 
    pub teacher_id: usize, 
    pub name: String, 
    pub time: Option<NaiveDateTime>,
}

impl From<Json<Course>> for Course {
    fn from(course: Json<Course>) -> Self {
        Course {
            id: course.id, 
            teacher_id: course.teacher_id, 
            name: course.name.clone(), 
            time: course.time,
        }
    }
}