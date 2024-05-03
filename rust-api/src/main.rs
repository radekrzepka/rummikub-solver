use actix_web::{post, App, HttpResponse, HttpServer, Responder};

mod brute_force;
mod game;
mod tile;

use brute_force::find_best_game_brute_force;
use game::deserialize_game_from_string;

#[post("/")]
async fn find_best_game(req_body: String) -> impl Responder {
  let game = match deserialize_game_from_string(req_body) {
    Ok(game) => game,
    Err(e) => return HttpResponse::BadRequest().body(format!("Failed to deserialize game: {}", e)),
  };

  match find_best_game_brute_force(&game) {
    Some(best_game) => return HttpResponse::Ok().json(best_game),
    None => return HttpResponse::BadRequest().body("No best game could be determined."),
  }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
  HttpServer::new(|| App::new().service(find_best_game))
    .bind("0.0.0.0:3002")?
    .run()
    .await
}
