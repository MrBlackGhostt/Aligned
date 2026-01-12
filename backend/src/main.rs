use actix_web::{App, HttpResponse, HttpServer, Responder, get, post, web};

#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

#[post("/echo")]
async fn echo(req_body: String) -> impl Responder {
    HttpResponse::Ok().body(req_body)
}

async fn manual_hello() -> impl Responder {
    HttpResponse::Ok().body("Hey there!")
}

async fn health_check() -> impl Responder {
    HttpResponse::Ok().body("I'm ok")
}

// Auth Handlers
async fn phone_login() -> impl Responder {
    HttpResponse::Ok().body("Auth: Phone Login Endpoint")
}

async fn phone_verify() -> impl Responder {
    HttpResponse::Ok().body("Auth: Phone Verify Endpoint")
}

// Profile Handlers
async fn get_profile() -> impl Responder {
    HttpResponse::Ok().body("Profile: Get Current Profile")
}

async fn update_profile() -> impl Responder {
    HttpResponse::Ok().body("Profile: Update Profile")
}

async fn upload_profile_images() -> impl Responder {
    HttpResponse::Ok().body("Profile: Upload Images")
}

async fn finalize_profile() -> impl Responder {
    HttpResponse::Ok().body("Profile: Finalize (Go Live)")
}

async fn delete_account() -> impl Responder {
    HttpResponse::Ok().body("Profile: Delete Account")
}

// Feed Handlers
async fn get_feed() -> impl Responder {
    HttpResponse::Ok().body("Feed: Get Recommendations")
}

// Interaction Handlers
async fn interact() -> impl Responder {
    HttpResponse::Ok().body("Interaction: Like/Pass")
}

// Matches & Messages Handlers
async fn get_matches() -> impl Responder {
    HttpResponse::Ok().body("Matches: Get All Matches")
}

async fn get_messages() -> impl Responder {
    HttpResponse::Ok().body("Messages: Get Chat History")
}

async fn send_message() -> impl Responder {
    HttpResponse::Ok().body("Messages: Send Message")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(hello)
            .service(echo)
            .route("/hey", web::get().to(manual_hello))
            .route("/health", web::get().to(health_check))
            // Auth
            .route("/auth/phone/login", web::post().to(phone_login))
            .route("/auth/phone/verify", web::post().to(phone_verify))
            // Profile
            .route("/profile/me", web::get().to(get_profile))
            .route("/profile", web::post().to(update_profile))
            .route("/profile/images", web::post().to(upload_profile_images))
            .route("/profile/finalize", web::post().to(finalize_profile))
            .route("/profile", web::delete().to(delete_account))
            // Feed
            .route("/feed", web::get().to(get_feed))
            // Interactions
            .route("/interact", web::post().to(interact))
            // Matches & Messages
            .route("/matches", web::get().to(get_matches))
            .route("/matches/{id}/messages", web::get().to(get_messages))
            .route("/matches/{id}/messages", web::post().to(send_message))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
