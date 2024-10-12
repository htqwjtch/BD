use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Serialize, Deserialize, ToSchema)]
pub struct Patient {
    pub id: i32,
    pub name: String,
    pub surname: String,
    pub birth_date: String,
    pub phone_number: String,
    pub passport_number: String,
}

#[derive(Serialize, Deserialize, ToSchema)]
pub struct NewPatient {
    pub name: String,
    pub surname: String,
    pub birth_date: String,
    pub phone_number: String,
    pub passport_number: String,
}

#[derive(Serialize, Deserialize, ToSchema)]
pub struct OptionPatient {
    pub name: Option<String>,
    pub surname: Option<String>,
    pub birth_date: Option<String>,
    pub phone_number: Option<String>,
    pub passport_number: Option<String>,
}

#[derive(Serialize, Deserialize, ToSchema)]
pub struct Doctor {
    pub id: i32,
    pub name: String,
    pub surname: String,
    pub speciality: String,
    pub phone_number: String,
    pub passport_number: String,
}

#[derive(Serialize, Deserialize, ToSchema)]
pub struct NewDoctor {
    pub name: String,
    pub surname: String,
    pub speciality: String,
    pub phone_number: String,
    pub passport_number: String,
}

#[derive(Serialize, Deserialize, ToSchema)]
pub struct OptionDoctor {
    pub name: Option<String>,
    pub surname: Option<String>,
    pub speciality: Option<String>,
    pub phone_number: Option<String>,
    pub passport_number: Option<String>,
}

#[derive(Serialize, Deserialize, ToSchema)]
pub struct Ticket {
    pub id: i32,
    pub date: String,
    pub time: String,
    pub office_number: i32,
}

#[derive(Serialize, Deserialize, ToSchema)]
pub struct NewTicket {
    pub date: String,
    pub time: String,
    pub office_number: i32,
}

#[derive(Serialize, Deserialize, ToSchema)]
pub struct OptionTicket {
    pub date: Option<String>,
    pub time: Option<String>,
    pub office_number: Option<i32>,
}

#[derive(Serialize, Deserialize, ToSchema)]
pub struct ScheduleEntry {
    pub id: i32,
    pub ticket_id: i32,
    pub doctor_id: i32,
    pub patient_id: i32,
}

#[derive(Serialize, Deserialize, ToSchema)]
pub struct NewScheduleEntry {
    pub ticket_id: i32,
    pub doctor_id: i32,
    pub patient_id: i32,
}

#[derive(Serialize, Deserialize, ToSchema)]
pub struct OptionScheduleEntry {
    pub ticket_id: Option<i32>,
    pub doctor_id: Option<i32>,
    pub patient_id: Option<i32>,
}
