use crate::models::{
    Doctor, FullScheduleEntry, NewDoctor, NewPatient, NewScheduleEntry, NewTicket, OptionDoctor,
    OptionPatient, OptionScheduleEntry, OptionTicket, Patient, ScheduleEntry, Ticket, UpdateDoctor,
    UpdatePatient, UpdateScheduleEntry, UpdateTicket,
};
use actix_web::{delete, get, patch, post, web, HttpResponse, Responder};
use sqlx::PgPool;

#[utoipa::path(
    get,
    path = "/patients",
    tag = "Patients",
    responses(
        (status = 200, description = "List of patients", body = [Patient])
    ),
    params(
        ("patient" = OptionPatient, Query, description = "Optional filters")
    )
)]
#[get("/patients")]
pub async fn get_patients(
    pool: web::Data<PgPool>,
    option_patient: web::Query<OptionPatient>,
) -> impl Responder {
    let rows = sqlx::query_as!(
        Patient,
        "SELECT * 
        FROM patients
        WHERE 
            (COALESCE($1, '') = '' OR name = $1) AND
            (COALESCE($2, '') = '' OR surname = $2) AND
            (COALESCE($3, '') = '' OR birth_date = $3) AND
            (COALESCE($4, '') = '' OR phone_number = $4) AND
            (COALESCE($5, '') = '' OR passport_number = $5);",
        option_patient.name,
        option_patient.surname,
        option_patient.birth_date,
        option_patient.phone_number,
        option_patient.passport_number,
    )
    .fetch_all(pool.get_ref())
    .await
    .unwrap();

    web::Json(rows)
}

#[utoipa::path(
    post,
    path = "/patients",
    tag = "Patients",
    request_body = NewPatient,
    responses(
        (status = 201, description = "Entry successfully created", body = Patient),
        (status = 400, description = "Invalid input")
    )
)]
#[post("/patients")]
pub async fn add_patient(
    pool: web::Data<sqlx::PgPool>,
    new_patient: web::Json<NewPatient>,
) -> impl Responder {
    let result = sqlx::query!(
        r#"
        INSERT INTO patients (name, surname, birth_date, phone_number, passport_number)
        VALUES ($1, $2, $3, $4, $5)
        RETURNING id, name, surname, birth_date, phone_number, passport_number
        "#,
        new_patient.name,
        new_patient.surname,
        new_patient.birth_date,
        new_patient.phone_number,
        new_patient.passport_number,
    )
    .fetch_one(pool.get_ref())
    .await;

    match result {
        Ok(patient) => HttpResponse::Created().json(Patient {
            id: patient.id,
            name: patient.name,
            surname: patient.surname,
            birth_date: patient.birth_date,
            phone_number: patient.phone_number,
            passport_number: patient.passport_number,
        }),
        Err(_) => HttpResponse::BadRequest().body("Invalid input"),
    }
}

#[utoipa::path(
    patch,
    path = "/patients",
    tag = "Patients",
    request_body = UpdatePatient,
    responses(
        (status = 200, description = "Entry successfully updated"),
        (status = 404, description = "Entry not found"),
    )
)]
#[patch("/patients")]
pub async fn update_patient(
    pool: web::Data<PgPool>,
    request: web::Json<UpdatePatient>,
) -> impl Responder {
    let result = sqlx::query!(
        r#"
        UPDATE patients
        SET name = COALESCE($1, name),
            surname = COALESCE($2, surname),
            birth_date = COALESCE($3, birth_date),
            phone_number = COALESCE($4, phone_number),
            passport_number = COALESCE($5, passport_number)
       WHERE 
            ($6::TEXT IS NULL OR name = $6) AND
            ($7::TEXT IS NULL OR surname = $7) AND
            ($8::TEXT IS NULL OR birth_date = $8) AND
            ($9::TEXT IS NULL OR phone_number = $9) AND
            ($10::TEXT IS NULL OR passport_number = $10);
        "#,
        request.update_name,
        request.update_surname,
        request.update_birth_date,
        request.update_phone_number,
        request.update_passport_number,
        request.condition_name,
        request.condition_surname,
        request.condition_birth_date,
        request.condition_phone_number,
        request.condition_passport_number
    )
    .execute(pool.get_ref())
    .await;

    match result {
        Ok(_) => HttpResponse::Ok().body("Entry successfully updated"),
        Err(_) => HttpResponse::NotFound().body("Entry not found"),
    }
}

#[utoipa::path(
    delete,
    path = "/patients",
    tag = "Patients",
    request_body = OptionPatient,
    responses(
        (status = 204, description = "Entry successfully deleted"),
        (status = 404, description = "Entry not found")
    )
)]
#[delete("/patients")]
pub async fn delete_patient(
    pool: web::Data<sqlx::PgPool>,
    option_patient: web::Json<OptionPatient>,
) -> impl Responder {
    let result = sqlx::query!(
        r#"
        DELETE FROM patients
        WHERE 
            (COALESCE($1, '') = '' OR name = $1) AND
            (COALESCE($2, '') = '' OR surname = $2) AND
            (COALESCE($3, '') = '' OR birth_date = $3) AND
            (COALESCE($4, '') = '' OR phone_number = $4) AND
            (COALESCE($5, '') = '' OR passport_number = $5);
        "#,
        option_patient.name,
        option_patient.surname,
        option_patient.birth_date,
        option_patient.phone_number,
        option_patient.passport_number,
    )
    .execute(pool.get_ref())
    .await;

    match result {
        Ok(rows) if rows.rows_affected() > 0 => HttpResponse::NoContent().finish(),
        _ => HttpResponse::NotFound().body("Entry not found"),
    }
}

#[utoipa::path(
    get,
    path = "/doctors",
    tag = "Doctors",
    responses(
        (status = 200, description = "List of doctors", body = [Doctor])
    ),
    params(
        ("doctor" = OptionDoctor, Query, description = "Optional filters")
    )
    
)]
#[get("/doctors")]
pub async fn get_doctors(
    pool: web::Data<PgPool>,
    option_doctor: web::Query<OptionDoctor>
) -> impl Responder {
    let rows = sqlx::query_as!(
        Doctor,
        "SELECT * 
        FROM doctors
        WHERE 
            (COALESCE($1, '') = '' OR name = $1) AND
            (COALESCE($2, '') = '' OR surname = $2) AND
            (COALESCE($3, '') = '' OR speciality = $3) AND
            (COALESCE($4, '') = '' OR phone_number = $4) AND
            (COALESCE($5, '') = '' OR passport_number = $5);",
        option_doctor.name,
        option_doctor.surname,
        option_doctor.speciality,
        option_doctor.phone_number,
        option_doctor.passport_number,
    )
    .fetch_all(pool.get_ref())
    .await
    .unwrap();

    web::Json(rows)
}

#[utoipa::path(
    post,
    path = "/doctors",
    tag = "Doctors",
    request_body = NewDoctor,
    responses(
        (status = 201, description = "Entry successfully created", body = Doctor),
        (status = 400, description = "Invalid input")
    )
)]
#[post("/doctors")]
pub async fn add_doctor(
    pool: web::Data<sqlx::PgPool>,
    new_doctor: web::Json<NewDoctor>,
) -> impl Responder {
    let result = sqlx::query!(
        r#"
        INSERT INTO doctors (name, surname, speciality, phone_number, passport_number)
        VALUES ($1, $2, $3, $4, $5)
        RETURNING id, name, surname, speciality, phone_number, passport_number
        "#,
        new_doctor.name,
        new_doctor.surname,
        new_doctor.speciality,
        new_doctor.phone_number,
        new_doctor.passport_number,
    )
    .fetch_one(pool.get_ref())
    .await;

    match result {
        Ok(doctor) => HttpResponse::Created().json(Doctor {
            id: doctor.id,
            name: doctor.name,
            surname: doctor.surname,
            speciality: doctor.speciality,
            phone_number: doctor.phone_number,
            passport_number: doctor.passport_number,
        }),
        Err(_) => HttpResponse::BadRequest().body("Invalid input"),
    }
}

#[utoipa::path(
    patch,
    path = "/doctors",
    tag = "Doctors",
    request_body = UpdateDoctor,
    responses(
        (status = 200, description = "Entry successfully updated"),
        (status = 404, description = "Entry not found"),
    )
)]
#[patch("/doctors")]
pub async fn update_doctor(
    pool: web::Data<PgPool>,
    request: web::Json<UpdateDoctor>,
) -> impl Responder {
    let result = sqlx::query!(
        r#"
        UPDATE doctors
        SET name = COALESCE($1, name),
            surname = COALESCE($2, surname),
            speciality = COALESCE($3, speciality),
            phone_number = COALESCE($4, phone_number),
            passport_number = COALESCE($5, passport_number)
       WHERE 
            ($6::TEXT IS NULL OR name = $6) AND
            ($7::TEXT IS NULL OR surname = $7) AND
            ($8::TEXT IS NULL OR speciality = $8) AND
            ($9::TEXT IS NULL OR phone_number = $9) AND
            ($10::TEXT IS NULL OR passport_number = $10);
        "#,
        request.update_name,
        request.update_surname,
        request.update_speciality,
        request.update_phone_number,
        request.update_passport_number,
        request.condition_name,
        request.condition_surname,
        request.condition_speciality,
        request.condition_phone_number,
        request.condition_passport_number
    )
    .execute(pool.get_ref())
    .await;

    match result {
        Ok(_) => HttpResponse::Ok().body("Entry successfully updated"),
        Err(_) => HttpResponse::NotFound().body("Entry not found"),
    }
}

#[utoipa::path(
    delete,
    path = "/doctors",
    tag = "Doctors",
    request_body = OptionDoctor,
    responses(
        (status = 204, description = "Entry successfully deleted"),
        (status = 404, description = "Entry not found")
    )
)]
#[delete("/doctors")]
pub async fn delete_doctor(
    pool: web::Data<sqlx::PgPool>,
    option_doctor: web::Json<OptionDoctor>,
) -> impl Responder {
    let result = sqlx::query!(
        r#"
        DELETE FROM doctors
        WHERE 
            (COALESCE($1, '') = '' OR name = $1) AND
            (COALESCE($2, '') = '' OR surname = $2) AND
            (COALESCE($3, '') = '' OR speciality = $3) AND
            (COALESCE($4, '') = '' OR phone_number = $4) AND
            (COALESCE($5, '') = '' OR passport_number = $5);
        "#,
        option_doctor.name,
        option_doctor.surname,
        option_doctor.speciality,
        option_doctor.phone_number,
        option_doctor.passport_number,
    )
    .execute(pool.get_ref())
    .await;

    match result {
        Ok(rows) if rows.rows_affected() > 0 => HttpResponse::NoContent().finish(),
        _ => HttpResponse::NotFound().body("Entry not found"),
    }
}

#[utoipa::path(
    get,
    path = "/tickets",
    tag = "Tickets",
    responses(
        (status = 200, description = "List of tickets", body = [Ticket])
    ),
    params(
        ("ticket" = OptionTicket, Query, description = "Optional filters")
    )
)]
#[get("/tickets")]
pub async fn get_tickets(
    pool: web::Data<PgPool>,
    option_ticket: web::Query<OptionTicket>
) -> impl Responder {
    let rows = sqlx::query_as!(
        Ticket,
        "SELECT * 
        FROM tickets
        WHERE 
            (COALESCE($1, '') = '' OR date = $1) AND
            (COALESCE($2, '') = '' OR time = $2) AND
            (COALESCE($3, 0) = 0 OR office_number = $3);",
        option_ticket.date,
        option_ticket.time,
        option_ticket.office_number
    )
    .fetch_all(pool.get_ref())
    .await
    .unwrap();

    web::Json(rows)
}

#[utoipa::path(
    post,
    path = "/tickets",
    tag = "Tickets",
    request_body = NewTicket,
    responses(
        (status = 201, description = "Entry successfully created", body = Ticket),
        (status = 400, description = "Invalid input")
    )
)]
#[post("/tickets")]
pub async fn add_ticket(
    pool: web::Data<sqlx::PgPool>,
    new_ticket: web::Json<NewTicket>,
) -> impl Responder {
    let result = sqlx::query!(
        r#"
        INSERT INTO tickets (date, time, office_number)
        VALUES ($1, $2, $3)
        RETURNING id, date, time, office_number
        "#,
        new_ticket.date,
        new_ticket.time,
        new_ticket.office_number,
    )
    .fetch_one(pool.get_ref())
    .await;

    match result {
        Ok(ticket) => HttpResponse::Created().json(Ticket {
            id: ticket.id,
            date: ticket.date,
            time: ticket.time,
            office_number: ticket.office_number,
        }),
        Err(_) => HttpResponse::BadRequest().body("Invalid input"),
    }
}

#[utoipa::path(
    patch,
    path = "/tickets",
    tag = "Tickets",
    request_body = UpdateTicket,
    responses(
        (status = 200, description = "Entry successfully updated"),
        (status = 404, description = "Entry not found"),
    )
)]
#[patch("/tickets")]
pub async fn update_ticket(
    pool: web::Data<PgPool>,
    request: web::Json<UpdateTicket>,
) -> impl Responder {
    let result = sqlx::query!(
        r#"
        UPDATE tickets
        SET date = COALESCE($1, date),
            time = COALESCE($2, time),
            office_number = COALESCE($3, office_number)
       WHERE 
            ($4::TEXT IS NULL OR date = $4) AND
            ($5::TEXT IS NULL OR time = $5) AND
            ($6::INT IS NULL OR office_number = $6);
        "#,
        request.update_date,
        request.update_time,
        request.update_office_number,
        request.condition_date,
        request.condition_time,
        request.condition_office_number,
    )
    .execute(pool.get_ref())
    .await;

    match result {
        Ok(_) => HttpResponse::Ok().body("Entry successfully updated"),
        Err(_) => HttpResponse::NotFound().body("Entry not found"),
    }
}

#[utoipa::path(
    delete,
    path = "/tickets",
    tag = "Tickets",
    request_body = OptionTicket,
    responses(
        (status = 204, description = "Entry successfully deleted"),
        (status = 404, description = "Entry not found")
    )
)]
#[delete("/tickets")]
pub async fn delete_ticket(
    pool: web::Data<sqlx::PgPool>,
    option_ticket: web::Json<OptionTicket>,
) -> impl Responder {
    let result = sqlx::query!(
        r#"
        DELETE FROM tickets
        WHERE 
            (COALESCE($1, '') = '' OR date = $1) AND
            (COALESCE($2, '') = '' OR time = $2) AND
            (COALESCE($3, 0) = 0 OR office_number = $3);
        "#,
        option_ticket.date,
        option_ticket.time,
        option_ticket.office_number,
    )
    .execute(pool.get_ref())
    .await;

    match result {
        Ok(rows) if rows.rows_affected() > 0 => HttpResponse::NoContent().finish(),
        _ => HttpResponse::NotFound().body("Entry not found"),
    }
}

#[utoipa::path(
    get,
    path = "/schedule",
    tag = "Schedule",
    responses(
        (status = 200, description = "Schedule", body = [FullScheduleEntry])
    ),
    params(
        ("schedule entry" = OptionScheduleEntry, Query, description = "Optional filters")
    )
)]
#[get("/schedule")]
pub async fn get_schedule(
    pool: web::Data<PgPool>,
    option_schedule_entry: web::Query<OptionScheduleEntry>
) -> impl Responder {
    let rows = sqlx::query_as!(
        FullScheduleEntry,
        "SELECT schedule.id as schedule_id, tickets.id as ticket_id, doctors.id as doctor_id, patients.id as patient_id,
        tickets.date as ticket_date, tickets.time as ticket_time, tickets.office_number as ticket_office_number,
        doctors.name as doctor_name, doctors.surname as doctor_surname, doctors.speciality as doctor_speciality,
        doctors.phone_number as doctor_phone_number, doctors.passport_number as doctor_passport_number,
        patients.name as patient_name, patients.surname as patient_surname, patients.birth_date as patient_birth_date,
        patients.phone_number as patient_phone_number, patients.passport_number as patient_passport_number
        FROM schedule
        JOIN tickets ON schedule.ticket_id = tickets.id
        JOIN doctors ON schedule.doctor_id = doctors.id
        JOIN patients ON schedule.patient_id = patients.id
        WHERE 
            (COALESCE($1, 0) = 0 OR ticket_id = $1) AND
            (COALESCE($2, 0) = 0 OR doctor_id = $2) AND
            (COALESCE($3, 0) = 0 OR patient_id = $3);",
        option_schedule_entry.ticket_id,
        option_schedule_entry.doctor_id,
        option_schedule_entry.patient_id
    )
    .fetch_all(pool.get_ref())
    .await
    .unwrap();

    web::Json(rows)
}

#[utoipa::path(
    post,
    path = "/schedule",
    tag = "Schedule",
    request_body = NewScheduleEntry,
    responses(
        (status = 201, description = "Entry successfully created", body = ScheduleEntry),
        (status = 400, description = "Invalid input")
    )
)]
#[post("/schedule")]
pub async fn add_schedule_entry(
    pool: web::Data<sqlx::PgPool>,
    new_schedule_entry: web::Json<NewScheduleEntry>,
) -> impl Responder {
    let result = sqlx::query!(
        r#"
        INSERT INTO schedule (ticket_id, doctor_id, patient_id)
        VALUES ($1, $2, $3)
        RETURNING id, ticket_id, doctor_id, patient_id
        "#,
        new_schedule_entry.ticket_id,
        new_schedule_entry.doctor_id,
        new_schedule_entry.patient_id,
    )
    .fetch_one(pool.get_ref())
    .await;

    match result {
        Ok(schedule_entry) => HttpResponse::Created().json(ScheduleEntry {
            id: schedule_entry.id,
            ticket_id: schedule_entry.ticket_id,
            doctor_id: schedule_entry.doctor_id,
            patient_id: schedule_entry.patient_id,
        }),
        Err(_) => HttpResponse::BadRequest().body("Invalid input"),
    }
}

#[utoipa::path(
    patch,
    path = "/schedule",
    tag = "Schedule",
    request_body = UpdateScheduleEntry,
    responses(
        (status = 200, description = "Entry successfully updated"),
        (status = 404, description = "Entry not found"),
    )
)]
#[patch("/schedule")]
pub async fn update_schedule_entry(
    pool: web::Data<PgPool>,
    request: web::Json<UpdateScheduleEntry>,
) -> impl Responder {
    let result = sqlx::query!(
        r#"
        UPDATE schedule
        SET ticket_id = COALESCE($1, ticket_id),
            doctor_id = COALESCE($2, doctor_id),
            patient_id = COALESCE($3, patient_id)
       WHERE 
            ($4::INT IS NULL OR ticket_id = $4) AND
            ($5::INT IS NULL OR doctor_id = $5) AND
            ($6::INT IS NULL OR patient_id = $6);
        "#,
        request.update_ticket_id,
        request.update_doctor_id,
        request.update_patient_id,
        request.condition_ticket_id,
        request.condition_doctor_id,
        request.condition_patient_id,
    )
    .execute(pool.get_ref())
    .await;

    match result {
        Ok(_) => HttpResponse::Ok().body("Entry successfully updated"),
        Err(_) => HttpResponse::NotFound().body("Entry not found"),
    }
}

#[utoipa::path(
    delete,
    path = "/schedule",
    tag = "Schedule",
    request_body = OptionScheduleEntry,
    responses(
        (status = 204, description = "Entry successfully deleted"),
        (status = 404, description = "Entry not found")
    )
)]
#[delete("/schedule")]
pub async fn delete_schedule_entry(
    pool: web::Data<sqlx::PgPool>,
    option_schedule_entry: web::Json<OptionScheduleEntry>,
) -> impl Responder {
    let result = sqlx::query!(
        r#"
        DELETE FROM schedule
        WHERE 
            (COALESCE($1, 0) = 0 OR ticket_id = $1) AND
            (COALESCE($2, 0) = 0 OR doctor_id = $2) AND
            (COALESCE($3, 0) = 0 OR patient_id = $3);
        "#,
        option_schedule_entry.ticket_id,
        option_schedule_entry.doctor_id,
        option_schedule_entry.patient_id
    )
    .execute(pool.get_ref())
    .await;

    match result {
        Ok(rows) if rows.rows_affected() > 0 => HttpResponse::NoContent().finish(),
        _ => HttpResponse::NotFound().body("Entry not found"),
    }
}
