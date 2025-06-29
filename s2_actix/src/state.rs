use std::sync::Mutex;
use crate::entity::Course;

pub struct AppState {
    pub health_check_response: String, 
    pub visit_count: Mutex<u32>,
    
    pub courses: Mutex<Vec<Course>>, 
    pub course_id: Mutex<usize>, 
}