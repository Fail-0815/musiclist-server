use crate::schema::*;
use diesel::prelude::*;
use uuid::Uuid;
use wishlib::MusicWish;

/// Representation of a MusicWish used with Diesel (the ORM in use) in the Database
#[derive(Queryable)]
pub struct WishData {
    pub id: i32,
    pub title: String,
    pub artist: String,
    pub comment: String,
    pub score: i16,
    pub sessionid: Uuid,
}

#[allow(dead_code)] // For some reason all functions here will be marked as dead code. But they are not.
impl WishData {
    /// Creates a new MusicWish from WishData
    /// Intended to be used *only* when new data was inserted into the DB
    pub fn new_music_wish(&self) -> MusicWish {
        MusicWish {
            id: self.id,
            title: self.title.clone(),
            artist: self.artist.clone(),
            comment: self.comment.clone(),
            voted: true,
            score: 1,
        }
    }

    /// Creates a new MusicWish from WishData
    /// requires further information:
    /// # Arguments
    /// * `voted` - A bool if this has been voted on by the requesting session
    /// * `score` - Total sum of votes the entry has recieved
    pub fn music_wish(&self, voted: bool, score: usize) -> MusicWish {
        MusicWish {
            id: self.id,
            title: self.title.clone(),
            artist: self.artist.clone(),
            comment: self.comment.clone(),
            voted,
            score,
        }
    }
}

#[derive(Insertable)]
#[diesel(table_name = wishes)]
pub struct NewWish {
    pub title: String,
    pub artist: String,
    pub comment: String,
    pub sessionid: Uuid,
}

#[derive(Queryable, Insertable)]
#[diesel{table_name = sessions}]
pub struct Session {
    pub id: Uuid,
}

#[derive(Queryable, Insertable)]
#[diesel{table_name = votes}]
pub struct Vote {
    pub sessionid: Uuid,
    pub wishid: i32,
}
