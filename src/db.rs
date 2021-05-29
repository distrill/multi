use rusqlite::{params, Connection, Result};
use rusqlite::Error::{QueryReturnedNoRows};

use crate::player::Player;

pub struct Db {
    conn: Connection
}

impl Db {
    fn init(&self) -> Result<()> {
        self.conn.execute(
            "CREATE TABLE IF NOT EXISTS player (
                id TEXT PRIMARY KEY,
                pos_x INTEGER NOT NULL,  
                pos_y INTEGER_NOT_NULL,
                color_r REAL NOT NULL,
                color_g REAL NOT NULL,
                color_b REAL NOT NULL
              )",
            [],
        )?;  
        Ok(())
    }

    pub fn new() -> Result<Db> {
	let path = "./data/meta.db3";
	let conn = Connection::open(&path)?;

        let db = Db { conn };
        db.init()?;
        Ok(db)
    }

    // how can we make this generic?
    pub fn insert_player(&self, player: &Player) -> Result<()> {
         self.conn.execute(
	    "INSERT INTO player (
                id, pos_x, pos_y, color_r, color_g, color_b
            ) VALUES (
                ?1, ?2, ?3, ?4, ?5, ?6
            )",
	    params![
                player.id,
                player.pos.x,
                player.pos.y,
                player.settings.color.0,
                player.settings.color.1,
                player.settings.color.2,
            ],
	)?;
        Ok(())
    }

    // how can we make this generic?
    pub fn fetch_player(&self, id: String) -> Result<Option<Player>> {
	match self.conn.query_row("
                SELECT
                    id, pos_x, pos_y, color_r, color_g, color_b
                FROM
                    player
                WHERE id = ?1
            ",
	    params![id],
	    |row| {
                let id = row.get_unwrap(0);
                let pos = (row.get_unwrap(1), row.get_unwrap(2));
                let color = (row.get_unwrap(3), row.get_unwrap(4), row.get_unwrap(5));
                Ok(Player::new(id, pos, color))
            },
	) {
            Ok(player) => Ok(Some(player)),
            Err(QueryReturnedNoRows) => Ok(None),
            Err(err) => Err(err),
        }
    }
}
