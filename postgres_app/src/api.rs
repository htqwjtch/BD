use crate::handlers;
use crate::models;
use utoipa::OpenApi;

#[derive(OpenApi)]
#[openapi(
    paths(
        handlers::get_patients,
        handlers::add_patient,
        handlers::update_patient,
        handlers::delete_patient,
        handlers::get_doctors,
        handlers::add_doctor,
        handlers::update_doctor,
        handlers::delete_doctor,
        handlers::get_tickets,
        handlers::add_ticket,
        handlers::update_ticket,
        handlers::delete_ticket,
        handlers::get_schedule,
        handlers::add_schedule_entry,
        handlers::update_schedule_entry,
        handlers::delete_schedule_entry,
    ),
    components(schemas(
        models::Patient,
        models::NewPatient,
        models::OptionPatient,
        models::Doctor,
        models::NewDoctor,
        models::OptionDoctor,
        models::Ticket,
        models::NewTicket,
        models::OptionTicket,
        models::ScheduleEntry,
        models::NewScheduleEntry,
        models::OptionScheduleEntry,
    ))
)]
pub struct ApiDoc;
