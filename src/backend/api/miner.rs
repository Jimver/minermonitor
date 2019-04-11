/*
File for miner api:
- GH/s (per asic)
- Frequency
- Voltage
- Error rate
- Fan speed (%)
- Temperature(s)
*/
use diesel::query_dsl::{QueryDsl, RunQueryDsl};
use rocket::State;
use rocket_contrib::json::Json;
use serde::{Deserialize, Serialize};

use crate::{CookieStoreState, DbConn};
use crate::backend::probe::cookie_store::CookieStore;
use crate::schema::miners;

#[derive(Serialize, Deserialize, Queryable, Debug, Clone)]
pub struct Miner {
    pub id: i32,
    pub host: String,
    pub username: String,
    pub password: String,
}

#[derive(Serialize, Deserialize, Insertable, AsChangeset, Debug, Clone)]
#[table_name = "miners"]
pub struct NewMiner {
    pub host: String,
    pub username: String,
    pub password: String,
}

#[post("/", data = "<miner>")]
pub fn create(miner: Json<NewMiner>, conn: DbConn) -> Json<bool> {
    use crate::schema::miners::dsl::miners;
    Json(
        diesel::insert_into(miners)
            .values(&*miner)
            .execute(&*conn)
            .is_ok(),
    )
}

#[get("/")]
pub fn read(conn: DbConn) -> Json<Vec<Miner>> {
    use crate::schema::miners::dsl::miners;
    let miners_list = miners.load::<Miner>(&*conn).expect("Error loading miners");
    Json(miners_list)
}

#[put("/<id>", data = "<miner>")]
pub fn update(id: i32, miner: Json<NewMiner>, conn: DbConn) -> Json<Miner> {
    use crate::schema::miners::dsl::miners;
    diesel::update(miners.find(id))
        .set(&*miner)
        .execute(&*conn)
        .expect("Failed to update miner");
    Json(Miner {
        id,
        host: miner.clone().host,
        username: miner.clone().username,
        password: miner.clone().password,
    })
}

#[delete("/<id>")]
pub fn delete(id: i32, conn: DbConn) -> Json<bool> {
    use crate::schema::miners::dsl::miners;
    Json(diesel::delete(miners.find(id)).execute(&*conn).is_ok())
}

pub trait MinerStats {
    fn hash_rate(&self) -> f64;
    fn frequency(&self) -> Vec<f64>;
    fn voltage(&self) -> Vec<f64>;
    fn error_rate(&self) -> Vec<i64>;
    fn fan_speed(&self) -> Vec<i64>;
    fn temperature1(&self) -> Vec<f64>;
    fn temperature2(&self) -> Vec<f64>;
}
