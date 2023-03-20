#![feature(proc_macro_hygiene, decl_macro, let_chains)]

#[macro_use]
extern crate rocket;
mod models;
mod schema;

use diesel::prelude::*;
use rocket::http::{Cookie, CookieJar};
use rocket::serde::json::Json;
use rocket::State;
use server::*;
use std::sync::Mutex;
use wishlib::{LoginResult, MusicWish};

#[get("/wishlist")]
fn wishlist(
    db: &State<Mutex<PgConnection>>,
    cookies: &CookieJar<'_>,
) -> Json<Option<Vec<MusicWish>>> {
    let connection = &mut *db.lock().unwrap();
    let session = get_uuid_from_cookies(cookies);

    Json(
        if let Some(id) = session && check_session(connection, id){
            let results = get_wishes(connection, id);
            Some(results)
        } else {
            None
        }
    )
}

#[post("/wishlist", format = "json", data = "<wish>")]
fn post_wish(
    db: &State<Mutex<PgConnection>>,
    wish: Json<MusicWish>,
    cookies: &CookieJar<'_>,
) -> Json<Option<MusicWish>> {
    let connection = &mut *db.lock().unwrap();

    let session = get_uuid_from_cookies(cookies);
    Json( 
        if let Some(sid) = session && check_session(connection, sid) {
            // crate Wish
            let wish = (*wish).clone();
            Some(create_wish(connection, wish, sid).new_music_wish())
        } else {
            None
        }
    )
}

#[post("/wishlist/<wishid>/vote")]
fn vote_wish(
    db: &State<Mutex<PgConnection>>,
    wishid: i32,
    cookies: &CookieJar<'_>,
) -> Json<Option<bool>> {
    let connection = &mut *db.lock().unwrap();

    let session = get_uuid_from_cookies(cookies);
    Json(
        if let Some(sid) = session && check_session(connection, sid) {
            Some(vote(connection, wishid, sid))
        } else {
            None
        }
    )
}

#[post("/login", format = "Json", data = "<password>")]
fn login(
    password: Json<String>,
    cookies: &CookieJar<'_>,
    db: &State<Mutex<PgConnection>>,
) -> Json<LoginResult> {
    let connection = &mut *db.lock().unwrap();

    // check password
    if *password == "Welt Hallo" {
        let session = get_uuid_from_cookies(cookies);
        if let Some(id) = session && check_session(connection, id) {
                // what to do if already logged in?
                // currently: Nothing, irelevant of entered password.
                Json(LoginResult{status: true, message: id.to_string()})
            } else {
                let sessionid = new_session(connection);

                cookies.add(
                    Cookie::build("sessionid", sessionid.to_string())
                        .path("/")
                        .finish(),
                );
                Json(LoginResult{status: true, message: sessionid.to_string()})
            }
    } else {
        Json(LoginResult {
            status: false,
            message: "Password incorrect".to_string(),
        })
    }
}

#[get("/login", format = "Json")]
fn login_get(db: &State<Mutex<PgConnection>>, cookies: &CookieJar<'_>) -> Json<bool> {
    let connection = &mut *db.lock().unwrap();
    let session = get_uuid_from_cookies(cookies);
    if let Some(id) = session && check_session(connection, id) {
        Json(true)
    } else {
        Json(false)
    }
}

#[launch]
fn rocket() -> _ {
    let connection = establish_connection();
    rocket::build().manage(Mutex::new(connection)).mount(
        "/api",
        routes![wishlist, post_wish, login, login_get, vote_wish],
    )
}
