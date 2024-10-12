use crate::models::{
    Doctor, NewDoctor, NewPatient, NewScheduleEntry, NewTicket, OptionDoctor, OptionPatient,
    OptionScheduleEntry, OptionTicket, Patient, ScheduleEntry, Ticket,
};
use actix_web::{delete, get, patch, post, web, HttpResponse, Responder};
use sqlx::PgPool;

#[utoipa::path(
    get,
    path = "/patients/select/all",
    responses(
        (status = 200, description = "List of patients", body = [Patient])
    )
)]
#[get("/patients/select/all")]
pub async fn get_patients(pool: web::Data<PgPool>) -> impl Responder {
    let rows = sqlx::query_as!(Patient, "SELECT * FROM patients")
        .fetch_all(pool.get_ref())
        .await
        .unwrap();

    web::Json(rows)
}

#[utoipa::path(
    post,
    path = "/patients/add",
    request_body = NewPatient,
    responses(
        (status = 201, description = "Entry successfully created", body = Patient),
        (status = 400, description = "Invalid input")
    )
)]
#[post("/patients/add")]
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
    path = "/patients/update/{id}",
    params(
        ("id" = i32, Path, description = "Patient ID")
    ),
    request_body = OptionPatient,
    responses(
        (status = 200, description = "Entry successfully updated"),
        (status = 404, description = "Entry not found"),
    )
)]
#[patch("/patients/update/{id}")]
pub async fn update_patient(
    pool: web::Data<PgPool>,
    patient_id: web::Path<i32>,
    option_patient: web::Json<OptionPatient>,
) -> impl Responder {
    let result = sqlx::query!(
        r#"
        UPDATE patients
        SET name = COALESCE($1, name),
            surname = COALESCE($2, surname),
            birth_date = COALESCE($3, birth_date),
            phone_number = COALESCE($4, phone_number),
            passport_number = COALESCE($5, passport_number)
        WHERE id = $6
        "#,
        option_patient.name,
        option_patient.surname,
        option_patient.birth_date,
        option_patient.phone_number,
        option_patient.passport_number,
        *patient_id
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
    path = "/patients/delete/{id}",
    params(
        ("id" = i32, Path, description = "Patient ID")
    ),
    responses(
        (status = 204, description = "Entry successfully deleted"),
        (status = 404, description = "Entry not found")
    )
)]
#[delete("/patients/delete/{id}")]
pub async fn delete_patient(
    pool: web::Data<sqlx::PgPool>,
    patient_id: web::Path<i32>,
) -> impl Responder {
    let result = sqlx::query!(
        r#"
        DELETE FROM patients
        WHERE id = $1
        "#,
        *patient_id
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
    path = "/doctors/select/all",
    responses(
        (status = 200, description = "List of doctors", body = [Doctor])
    )
)]
#[get("/doctors/select/all")]
pub async fn get_doctors(pool: web::Data<PgPool>) -> impl Responder {
    let rows = sqlx::query_as!(Doctor, "SELECT * FROM doctors")
        .fetch_all(pool.get_ref())
        .await
        .unwrap();

    web::Json(rows)
}

#[utoipa::path(
    post,
    path = "/doctors/add",
    request_body = NewDoctor,
    responses(
        (status = 201, description = "Entry successfully created", body = Doctor),
        (status = 400, description = "Invalid input")
    )
)]
#[post("/doctors/add")]
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
    path = "/doctors/update/{id}",
    params(
        ("id" = i32, Path, description = "Doctor ID")
    ),
    request_body = OptionDoctor,
    responses(
        (status = 200, description = "Entry successfully updated"),
        (status = 404, description = "Entry not found"),
    )
)]
#[patch("/doctors/update/{id}")]
pub async fn update_doctor(
    pool: web::Data<PgPool>,
    doctor_id: web::Path<i32>,
    option_doctor: web::Json<OptionDoctor>,
) -> impl Responder {
    let result = sqlx::query!(
        r#"
        UPDATE doctors
        SET name = COALESCE($1, name),
            surname = COALESCE($2, surname),
            speciality = COALESCE($3, speciality),
            phone_number = COALESCE($4, phone_number),
            passport_number = COALESCE($5, passport_number)
        WHERE id = $6
        "#,
        option_doctor.name,
        option_doctor.surname,
        option_doctor.speciality,
        option_doctor.phone_number,
        option_doctor.passport_number,
        *doctor_id
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
    path = "/doctors/delete/{id}",
    params(
        ("id" = i32, Path, description = "Doctor ID")
    ),
    responses(
        (status = 204, description = "Entry successfully deleted"),
        (status = 404, description = "Entry not found")
    )
)]
#[delete("/doctors/delete/{id}")]
pub async fn delete_doctor(
    pool: web::Data<sqlx::PgPool>,
    doctor_id: web::Path<i32>,
) -> impl Responder {
    let result = sqlx::query!(
        r#"
        DELETE FROM patients
        WHERE id = $1
        "#,
        *doctor_id
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
    path = "/tickets/select/all",
    responses(
        (status = 200, description = "List of tickets", body = [Ticket])
    )
)]
#[get("/tickets/select/all")]
pub async fn get_tickets(pool: web::Data<PgPool>) -> impl Responder {
    let rows = sqlx::query_as!(Ticket, "SELECT * FROM tickets")
        .fetch_all(pool.get_ref())
        .await
        .unwrap();

    web::Json(rows)
}

#[utoipa::path(
    post,
    path = "/tickets/add",
    request_body = NewTicket,
    responses(
        (status = 201, description = "Entry successfully created", body = Ticket),
        (status = 400, description = "Invalid input")
    )
)]
#[post("/tickets/add")]
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
    path = "/tickets/update/{id}",
    params(
        ("id" = i32, Path, description = "Ticket ID")
    ),
    request_body = OptionTicket,
    responses(
        (status = 200, description = "Entry successfully updated"),
        (status = 404, description = "Entry not found"),
    )
)]
#[patch("/tickets/update/{id}")]
pub async fn update_ticket(
    pool: web::Data<PgPool>,
    ticket_id: web::Path<i32>,
    option_ticket: web::Json<OptionTicket>,
) -> impl Responder {
    let result = sqlx::query!(
        r#"
        UPDATE tickets
        SET date = COALESCE($1, date),
            time = COALESCE($2, time),
            office_number = COALESCE($3, office_number)
        WHERE id = $4
        "#,
        option_ticket.date,
        option_ticket.time,
        option_ticket.office_number,
        *ticket_id
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
    path = "/tickets/delete/{id}",
    params(
        ("id" = i32, Path, description = "Ticket ID")
    ),
    responses(
        (status = 204, description = "Entry successfully deleted"),
        (status = 404, description = "Entry not found")
    )
)]
#[delete("/tickets/delete/{id}")]
pub async fn delete_ticket(
    pool: web::Data<sqlx::PgPool>,
    ticket_id: web::Path<i32>,
) -> impl Responder {
    let result = sqlx::query!(
        r#"
        DELETE FROM tickets
        WHERE id = $1
        "#,
        *ticket_id
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
    path = "/schedule/select/all",
    responses(
        (status = 200, description = "Schedule", body = [ScheduleEntry])
    )
)]
#[get("/schedule/select/all")]
pub async fn get_schedule(pool: web::Data<PgPool>) -> impl Responder {
    let rows = sqlx::query_as!(ScheduleEntry, "SELECT * FROM schedule")
        .fetch_all(pool.get_ref())
        .await
        .unwrap();

    web::Json(rows)
}

#[utoipa::path(
    post,
    path = "/schedule/add",
    request_body = NewTicket,
    responses(
        (status = 201, description = "Entry successfully created", body = Ticket),
        (status = 400, description = "Invalid input")
    )
)]
#[post("/schedule/add")]
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
    path = "/schedule/update/{id}",
    params(
        ("id" = i32, Path, description = "Schedule Enrty ID")
    ),
    request_body = OptionScheduleEntry,
    responses(
        (status = 200, description = "Entry successfully updated"),
        (status = 404, description = "Entry not found"),
    )
)]
#[patch("/schedule/update/{id}")]
pub async fn update_schedule_entry(
    pool: web::Data<PgPool>,
    schedule_entry_id: web::Path<i32>,
    option_schedule_entry: web::Json<OptionScheduleEntry>,
) -> impl Responder {
    let result = sqlx::query!(
        r#"
        UPDATE schedule
        SET ticket_id = COALESCE($1, ticket_id),
            doctor_id = COALESCE($2, doctor_id),
            patient_id = COALESCE($3, patient_id)
        WHERE id = $4
        "#,
        option_schedule_entry.ticket_id,
        option_schedule_entry.doctor_id,
        option_schedule_entry.patient_id,
        *schedule_entry_id
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
    path = "/schedule/delete/{id}",
    params(
        ("id" = i32, Path, description = "Schedule Entry ID")
    ),
    responses(
        (status = 204, description = "Entry successfully deleted"),
        (status = 404, description = "Entry not found")
    )
)]
#[delete("/schedule/delete/{id}")]
pub async fn delete_schedule_entry(
    pool: web::Data<sqlx::PgPool>,
    schedule_entry_id: web::Path<i32>,
) -> impl Responder {
    let result = sqlx::query!(
        r#"
        DELETE FROM schedule
        WHERE id = $1
        "#,
        *schedule_entry_id
    )
    .execute(pool.get_ref())
    .await;

    match result {
        Ok(rows) if rows.rows_affected() > 0 => HttpResponse::NoContent().finish(),
        _ => HttpResponse::NotFound().body("Entry not found"),
    }
}
