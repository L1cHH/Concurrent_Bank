use uuid::Uuid;
use chrono::NaiveDate;

//By now, we are using UUID for identifying users in our bank system
//For person name, second name, etc. We are using just String, maybe later
// it will be split into three types for filtering
//Also we use NaiveDate for convenient formatting.
pub struct Client {
    client_id: Uuid,
    first_name: String,
    second_name: String,
    third_name: String,
    birth_day: NaiveDate,
}