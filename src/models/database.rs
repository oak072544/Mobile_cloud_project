use crate::models::course::Course;

// Define an in-memory database of courses
pub struct Database {
    pub courses: Vec<Course>,
}

impl Database {
    pub fn new() -> Database {
        Database {
            courses: vec![
                Course {
                    id: 1,
                    name: "Introduction to Rust".to_string(),
                    description: "Learn the basics of Rust programming language.".to_string(),
                },
                Course {
                    id: 2,
                    name: "Advanced Rust".to_string(),
                    description: "Take your Rust skills to the next level with advanced concepts and techniques.".to_string(),
                },
            ],
        }
    }

    // Retrieve a course by its ID
    pub fn get_course(&self, id: i32) -> Option<&Course> {
        self.courses.iter().find(|&course| course.id == id)
    }

    // Retrieve all courses
    pub fn get_courses(&self) -> &Vec<Course> {
        &self.courses
    }
}
............