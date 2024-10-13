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
pub struct UpdatePatient {
    pub update_name: Option<String>,
    pub update_surname: Option<String>,
    pub update_birth_date: Option<String>,
    pub update_phone_number: Option<String>,
    pub update_passport_number: Option<String>,
    pub condition_name: Option<String>,
    pub condition_surname: Option<String>,
    pub condition_birth_date: Option<String>,
    pub condition_phone_number: Option<String>,
    pub condition_passport_number: Option<String>,
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
pub struct UpdateDoctor {
    pub update_name: Option<String>,
    pub update_surname: Option<String>,
    pub update_speciality: Option<String>,
    pub update_phone_number: Option<String>,
    pub update_passport_number: Option<String>,
    pub condition_name: Option<String>,
    pub condition_surname: Option<String>,
    pub condition_speciality: Option<String>,
    pub condition_phone_number: Option<String>,
    pub condition_passport_number: Option<String>,
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
pub struct UpdateTicket {
    pub update_date: Option<String>,
    pub update_time: Option<String>,
    pub update_office_number: Option<i32>,
    pub condition_date: Option<String>,
    pub condition_time: Option<String>,
    pub condition_office_number: Option<i32>,
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

#[derive(Serialize, Deserialize, ToSchema)]
pub struct UpdateScheduleEntry {
    pub update_ticket_id: Option<i32>,
    pub update_doctor_id: Option<i32>,
    pub update_patient_id: Option<i32>,
    pub condition_ticket_id: Option<i32>,
    pub condition_doctor_id: Option<i32>,
    pub condition_patient_id: Option<i32>,
}

#[derive(Serialize, Deserialize, ToSchema)]
pub struct FullScheduleEntry {
    pub schedule_id: i32,

    pub ticket_id: i32,
    pub ticket_date: String,
    pub ticket_time: String,
    pub ticket_office_number: i32,

    pub doctor_id: i32,
    pub doctor_name: String,
    pub doctor_surname: String,
    pub doctor_speciality: String,
    pub doctor_phone_number: String,
    pub doctor_passport_number: String,

    pub patient_id: i32,
    pub patient_name: String,
    pub patient_surname: String,
    pub patient_birth_date: String,
    pub patient_phone_number: String,
    pub patient_passport_number: String,
}
