use self::models::*;
use diesel::pg::PgConnection;
use diesel::prelude::*;
use dotenvy::dotenv;
use rocket::http::CookieJar;
use std::env;
use std::str::FromStr;
use uuid::Uuid;
use wishlib::MusicWish;

pub mod models;
pub mod schema;

pub fn establish_connection() -> PgConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    PgConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}

pub fn create_wish(conn: &mut PgConnection, wish: MusicWish, sid: uuid::Uuid) -> WishData {
    use crate::schema::wishes;

    let new_wish = NewWish {
        title: wish.title,
        artist: wish.artist,
        comment: wish.comment,
        sessionid: sid,
    };

    let resultwish: WishData = diesel::insert_into(wishes::table)
        .values(&new_wish)
        .get_result(conn)
        .expect("Error saving new wish");

    vote(conn, resultwish.id, sid);

    resultwish
}

pub fn get_uuid_from_cookies(cookies: &CookieJar<'_>) -> Option<Uuid> {
    match cookies.get("sessionid") {
        Some(cookie) => match Uuid::from_str(cookie.value()) {
            Ok(sid) => Some(sid),
            Err(_) => None,
        },
        None => None,
    }
}

pub fn get_wishes(conn: &mut PgConnection, sid: Uuid) -> Vec<MusicWish> {
    use crate::schema::wishes::dsl::*;

    wishes
        .load::<WishData>(conn)
        .expect("Error loading wishes")
        .iter()
        .map(|x| x.music_wish(voted(conn, x.id, sid), get_votes(conn, x.id)))
        .collect::<Vec<MusicWish>>()
}

pub fn check_session(conn: &mut PgConnection, sid: Uuid) -> bool {
    use crate::schema::sessions::dsl::*;
    match sessions.find(sid).first::<Session>(conn) {
        Ok(_) => true,
        Err(_) => false,
    }
}

pub fn new_session(conn: &mut PgConnection) -> Uuid {
    use crate::schema::sessions;

    let sid = uuid::Uuid::new_v4();
    let new_session = Session { id: sid };
    diesel::insert_into(sessions::table)
        .values(&new_session)
        .execute(conn)
        .expect("Error saving Session");
    sid
}

pub fn get_votes(conn: &mut PgConnection, wish_to_query: i32) -> usize {
    use crate::schema::votes::dsl::*;

    let res = votes
        .filter(wishid.eq(wish_to_query))
        .select((wishid, sessionid))
        .execute(conn)
        .expect("Error Counting Votes");

    res
}

pub fn vote(conn: &mut PgConnection, wish_to_vote: i32, sid: uuid::Uuid) -> bool {
    use crate::schema::votes::{dsl::*, table};

    if !voted(conn, wish_to_vote, sid) {
        // not voted yet, add vote
        diesel::insert_into(table)
            .values(Vote {
                sessionid: sid,
                wishid: wish_to_vote,
            })
            .execute(conn)
            .expect("Error Voting");
        true
    } else {
        // already voted for, remove vote
        diesel::delete(
            votes
                .filter(wishid.eq(wish_to_vote))
                .filter(sessionid.eq(sid)),
        )
        .execute(conn)
        .expect("Error removing Vote");
        false
    }
}

pub fn voted(conn: &mut PgConnection, wish_to_vote: i32, sid: uuid::Uuid) -> bool {
    use crate::schema::votes::dsl::*;
    let voted = votes
        .filter(wishid.eq(wish_to_vote))
        .filter(sessionid.eq(sid))
        .select((wishid, sessionid))
        .execute(conn)
        .expect("Error Couting Votes");
    voted > 0
}
