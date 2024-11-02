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
        handlers::export_patients,
        handlers::import_patients,

        handlers::get_doctors,
        handlers::add_doctor,
        handlers::update_doctor,
        handlers::delete_doctor,
        handlers::export_doctors,
        handlers::import_doctors,

        handlers::get_tickets,
        handlers::add_ticket,
        handlers::update_ticket,
        handlers::delete_ticket,
        handlers::export_tickets,
        handlers::import_tickets,

        handlers::get_schedule,
        handlers::add_schedule_entry,
        handlers::update_schedule_entry,
        handlers::delete_schedule_entry,
        handlers::export_schedule,
        handlers::import_schedule,
    ),
    components(schemas(
        models::Patient,
        models::NewPatient,
        models::OptionPatient,
        models::UpdatePatient,
        models::Doctor,
        models::NewDoctor,
        models::OptionDoctor,
        models::UpdateDoctor,
        models::Ticket,
        models::NewTicket,
        models::OptionTicket,
        models::UpdateTicket,
        models::ScheduleEntry,
        models::NewScheduleEntry,
        models::OptionScheduleEntry,
        models::UpdateScheduleEntry,
        models::FullScheduleEntry,
    )),
    tags(
        (name = "Patients", description = "Operations related to relation \"patients\""),
        (name = "Doctors", description = "Operations related to relation \"doctors\""),
        (name = "Tickets", description = "Operations related to relation \"tickets\""),
        (name = "Schedule", description = "Operations related to relation \"schedule\"")
    )
)]
pub struct ApiDoc;
