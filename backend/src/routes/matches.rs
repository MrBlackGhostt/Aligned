use crate::models::inputs::SendMessageRequest;
use actix_web::{HttpResponse, Responder, web};
use crate::db::DbState;
use crate::models::outputs::{StatusResponse, Match} ;  
use sqlx::types::Uuid;   

pub async fn get_matches(
    db: web::Data<DbState>,
) -> impl Responder {
    
    let result = sqlx::query_as::<_, Match>(
        "SELECT *
         FROM matches 
         ORDER BY last_message_at DESC NULLS LAST"
    )
    .fetch_all(&db.db)
    .await;

    match result {
        Ok(matches) => HttpResponse::Ok().json(matches),
        Err(e) => {
            eprintln!("Error fetching matches: {:?}", e);
            HttpResponse::InternalServerError().json(StatusResponse {
                status: "error".to_string(),
                message: Some("Failed to fetch matches".to_string()),
            })
        }
    }
}

pub async fn get_messages( db: web::Data<DbState>,path: web::Path<String>) -> impl Responder {
    let match_id = path.into_inner();
    println!("Messages: Get history for match {}", match_id);
    HttpResponse::Ok().body(format!("Messages: Get Chat History for {}", match_id))
}

pub async fn send_message(
    path: web::Path<String>,
    body: web::Json<SendMessageRequest>,
) -> impl Responder {
    let match_id = path.into_inner();
    println!("Messages: Send to match {}: {}", match_id, body.text);
    HttpResponse::Ok().body(format!("Messages: Send Message to {}", match_id))
}