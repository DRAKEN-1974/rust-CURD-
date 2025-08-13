use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use dotenvy::dotenv;
use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::env;

#[derive(Deserialize)]
struct UserRequest {
    name: String,
    email: String,
    age: i32,
}

#[derive(Serialize)]
struct SupabaseUser {
    name: String,
    email: String,
    age: i32,
}

async fn create_user(
    client: web::Data<Client>,
    user: web::Json<UserRequest>,
) -> impl Responder {
    let supabase_url = env::var("SUPABASE_URL").expect("SUPABASE_URL must be set");
    let supabase_key = env::var("SUPABASE_ANON_KEY").expect("SUPABASE_ANON_KEY must be set");

    let url = format!("{}/rest/v1/users", supabase_url);

    let supabase_user = SupabaseUser {
        name: user.name.clone(),
        email: user.email.clone(),
        age: user.age,
    };

    let res = client
        .post(&url)
        .header("apikey", &supabase_key)
        .header("Authorization", format!("Bearer {}", supabase_key))
        .header("Content-Type", "application/json")
        
        .query(&[("return", "representation")])
        .json(&supabase_user)
        .send()
        .await;

    match res {
        Ok(resp) if resp.status().is_success() => {
           
            let body = resp.json::<Vec<SupabaseUser>>().await.unwrap_or_default();
            HttpResponse::Created().json(body)
        }
        Ok(resp) => {
            let status = resp.status();
            let body = resp.text().await.unwrap_or_default();
            HttpResponse::build(status).body(format!("Failed: {}", body))
        }
        Err(e) => HttpResponse::InternalServerError().body(format!("Error: {}", e)),
    }
}

async fn get_users(client: web::Data<Client>) -> impl Responder {
    let supabase_url = env::var("SUPABASE_URL").expect("SUPABASE_URL must be set");
    let supabase_key = env::var("SUPABASE_ANON_KEY").expect("SUPABASE_ANON_KEY must be set");

    let url = format!("{}/rest/v1/users", supabase_url);

    let res = client
        .get(&url)
        .header("apikey", &supabase_key)
        .header("Authorization", format!("Bearer {}", supabase_key))
        .header("Accept", "application/json")
        .send()
        .await;

    match res {
        Ok(resp) if resp.status().is_success() => {
            let body = resp.json::<Vec<SupabaseUser>>().await.unwrap_or_default();
            HttpResponse::Ok().json(body)
        }
        Ok(resp) => {
            let status = resp.status();
            let body = resp.text().await.unwrap_or_default();
            HttpResponse::build(status).body(format!("Failed: {}", body))
        }
        Err(e) => HttpResponse::InternalServerError().body(format!("Error: {}", e)),
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();

    let port: u16 = env::var("APP_PORT")
        .unwrap_or_else(|_| "8080".to_string())
        .parse()
        .expect("APP_PORT must be a number");

    let client = Client::new();

    println!("Starting server on http://localhost:{}", port);

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(client.clone()))
            .route("/users", web::post().to(create_user))
            .route("/users", web::get().to(get_users))
    })
    .bind(("127.0.0.1", port))?
    .run()
    .await
}
