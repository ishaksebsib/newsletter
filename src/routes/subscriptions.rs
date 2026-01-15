use actix_web::{web, HttpResponse};
use serde::Deserialize;
use sqlx::{
    query,
    types::{chrono::Utc, Uuid},
    PgPool,
};

#[derive(Deserialize)]
pub struct FormData {
    name: String,
    email: String,
}

pub async fn subscribe(form: web::Form<FormData>, pool: web::Data<PgPool>) -> HttpResponse {
    let request_id = Uuid::new_v4();
    let request_span = tracing::info_span!("Adding a new subscriber.",
        %request_id,subscriber_email = %form.email,subscriber_name = %form.name);
    let _request_span_guard = request_span.enter();
    log::info!(
        "Adding '{}' '{}' as a new subscriber.",
        form.email,
        form.name
    );
    log::info!("Saving new subscriber details in the database");
    match query!(
        r#"
INSERT INTO subscriptions (id, email, name, subscribed_at)
VALUES ($1, $2, $3, $4)
"#,
        Uuid::new_v4(),
        form.email,
        form.name,
        Utc::now()
    )
    .execute(pool.get_ref())
    .await
    {
        Ok(_) => {
            log::info!("New subscriber details have been saved");
            HttpResponse::Ok().finish()
        }
        Err(e) => {
            log::error!("Failed to save new subscriber details in the database: {e}");
            HttpResponse::InternalServerError().finish()
        }
    }
}
