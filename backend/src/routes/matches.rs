use crate::models::inputs::SendMessageRequest;
use crate::AppState;
use actix_web::{HttpResponse, Responder, web};
use serde::Serialize;
use uuid::Uuid;

// Response structure for a single match in the list
#[derive(Serialize, sqlx::FromRow)]
struct MatchListItem {
    match_id: Uuid,
    other_user_id: Uuid,
    other_user_name: Option<String>,
    avatar_url: Option<String>,
    last_message: Option<String>,
    last_message_at: Option<chrono::DateTime<chrono::Utc>>,
    created_at: chrono::DateTime<chrono::Utc>,
}

// API Response wrapper
#[derive(Serialize)]
struct MatchesResponse {
    matches: Vec<MatchListItem>,
}

pub async fn get_matches(
    data: web::Data<AppState>
) -> impl Responder {
    // TODO: Get user_id from JWT token when you implement authentication
    // For now, using a hardcoded UUID for testing
    // Replace this with: let current_user_id = extract_from_jwt_token(&req);
    let current_user_id = Uuid::parse_str("00000000-0000-0000-0000-000000000001")
        .unwrap_or_else(|_| Uuid::nil());
    
    // Execute SQL query to get all matches for this user
    let query_result = sqlx::query_as::<_, MatchListItem>(
        r#"
        SELECT 
            m.id as match_id,
            m.created_at,
            m.last_message,
            m.last_message_at,
            -- Determine which user is the "other" user
            CASE 
                WHEN m.user1_id = $1 THEN m.user2_id 
                ELSE m.user1_id 
            END as other_user_id,
            -- Get the other user's name
            p.name as other_user_name,
            -- Get the other user's first profile image (display_order = 0)
            (
                SELECT url 
                FROM user_images 
                WHERE user_id = CASE 
                    WHEN m.user1_id = $1 THEN m.user2_id 
                    ELSE m.user1_id 
                END 
                AND display_order = 0
                LIMIT 1
            ) as avatar_url
        FROM matches m
        LEFT JOIN profiles p ON p.user_id = CASE 
            WHEN m.user1_id = $1 THEN m.user2_id 
            ELSE m.user1_id 
        END
        WHERE m.user1_id = $1 OR m.user2_id = $1
        ORDER BY m.last_message_at DESC NULLS LAST
        "#
    )
    .bind(current_user_id)  // $1 = current user's ID
    .fetch_all(&data.db)    // Execute query and get all results
    .await;
    
    // Handle the result
    match query_result {
        Ok(matches) => {
            HttpResponse::Ok().json(MatchesResponse { matches })
        },
        Err(e) => {
            // Log the error for debugging
            eprintln!("Database error in get_matches: {:?}", e);
            HttpResponse::InternalServerError().json(serde_json::json!({
                "error": "Failed to fetch matches",
                "details": format!("{}", e)
            }))
        }
    }
}

pub async fn get_messages(path: web::Path<String>) -> impl Responder {
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
