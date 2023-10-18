use crate::models::models::Person;

pub async fn get_person() -> Person {
    // mock data 
    Person {
        first_name: "John".to_string(),
        last_name: "Doe".to_string(),
    }
}
