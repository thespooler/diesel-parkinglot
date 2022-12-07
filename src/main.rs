use diesel::{r2d2::ConnectionManager, PgConnection, r2d2::Pool};

#[tokio::main]
pub async fn main() -> std::io::Result<()> {
    let manager = ConnectionManager::<PgConnection>::new("");
    let _ = Pool::builder().build(manager).unwrap();
    Ok(())
}
