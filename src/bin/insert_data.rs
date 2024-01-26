use sqlx::sqlite::SqlitePoolOptions;

// lots of potentioal improvements here,
// due to time constraints this is a very basic implementation
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Connecting to database...");
    let pool = SqlitePoolOptions::new()
        .max_connections(5)
        .connect("sqlite:./db.sqlite")
        .await
        .unwrap();

    println!("Running migrations...");
    sqlx::migrate!("./migrations").run(&pool).await.unwrap();

    println!("Inserting data into database...");

    let mut rdr = csv::Reader::from_path("./data/historical_prices.csv")?;

    for result in rdr.records() {
        let record = result?;

        let date = record.get(1).unwrap().to_owned();
        let price = record.get(2).unwrap().to_owned();
        let instrument_name = record.get(3).unwrap().to_owned();

        let _result = sqlx::query(
            "INSERT INTO prices (date, price, instrument_name)
        VALUES ($1, $2, $3)",
        )
        .bind(date)
        .bind(price)
        .bind(instrument_name)
        .execute(&pool)
        .await?;
    }

    println!("Done!");
    Ok(())
}
