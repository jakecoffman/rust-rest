use std::borrow::Borrow;
use actix_web::{HttpRequest, HttpResponse, Json};
use actix_web::dev::ResourceHandler;
use AppState;

#[derive(Serialize, Deserialize, Debug)]
pub struct User {
    #[serde(default)]
    name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    candy: Option<String>,
}

pub fn routes(r: &mut ResourceHandler<AppState>) {
    r.get().f(list);
    r.post().with(create);
}

pub fn list(req: HttpRequest<AppState>) -> HttpResponse {
    HttpResponse::build_from(&req).json(json!({"Hello": "world!"}))
}

pub fn create(body: Json<User>) -> HttpResponse {
    let user = body.into_inner();
    if user.name == "" {
        return HttpResponse::BadRequest().json(json!({"error": "missing property `name`"}));
    }

    match user.candy.borrow() {
        &None => {}
        &Some(ref candy) => if candy == "pez" { println!("Yum, pez!") }
    };
    HttpResponse::Ok().json(user)
}
