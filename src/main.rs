use anyhow::Result;

#[tokio::main]
async fn main() -> Result<()> {
    #[cfg(feature = "baseline")]
    main_baseline().await?;

    #[cfg(feature = "fjall")]
    main_fjall().await?;

    #[cfg(feature = "sled")]
    main_sled().await?;

    #[cfg(feature = "libsql")]
    main_libsql().await?;

    #[cfg(feature = "sqlite")]
    main_sqlite().await?;

    Ok(())
}

#[cfg(feature = "baseline")]
async fn main_baseline() -> Result<()> {
    tokio::fs::write("storage_baseline", "hello world").await?;
    println!(
        "{}",
        String::from_utf8(tokio::fs::read("storage_baseline").await?)?
    );

    Ok(())
}

#[cfg(feature = "fjall")]
async fn main_fjall() -> Result<()> {
    let keyspace = fjall::Config::new("storage_fjall").open()?;
    let items = keyspace.open_partition("items", fjall::PartitionCreateOptions::default())?;

    items.insert("hello", "world")?;
    println!(
        "{}",
        String::from_utf8(items.get("hello")?.expect("missing value").to_vec())?
    );
    keyspace.persist(fjall::PersistMode::SyncAll)?;

    Ok(())
}

#[cfg(feature = "sled")]
async fn main_sled() -> Result<()> {
    let db = sled::open("storage_sled")?;
    db.insert("hello", "world")?;
    println!(
        "{}",
        String::from_utf8(db.get("hello")?.expect("missing value").to_vec())?
    );
    drop(db);

    Ok(())
}

#[cfg(feature = "libsql")]
async fn main_libsql() -> Result<()> {
    let db = libsql::Builder::new_local("storage_libsql.sqlite")
        .build()
        .await?;

    let conn = db.connect()?;

    conn.query("select 1; select 1;", ()).await.unwrap();

    conn.execute(
        "CREATE TABLE IF NOT EXISTS kvstore (key TEXT, value TEXT)",
        (),
    )
    .await
    .unwrap();

    let mut stmt = conn
        .prepare("INSERT INTO kvstore (key, value) VALUES (?1, ?2)")
        .await
        .unwrap();

    stmt.execute(["hello", "world"]).await.unwrap();

    let mut stmt = conn
        .prepare("SELECT * FROM kvstore WHERE key = ?1")
        .await
        .unwrap();

    let mut rows = stmt.query(["hello"]).await.unwrap();

    let row = rows.next().await.unwrap().unwrap();

    let value = row.get_value(0).unwrap();

    println!("Row: {:?}", value);

    Ok(())
}

#[cfg(feature = "sqlite")]
async fn main_sqlite() -> Result<()> {
    let conn = sqlite::open("storage_sqlite.sqlite")?;
    conn.execute("select 1; select 1;")?;
    conn.execute("CREATE TABLE IF NOT EXISTS kvstore (key TEXT, value TEXT);")?;
    let mut stmt = conn.prepare("INSERT INTO kvstore (key, value) VALUES (?1, ?2);")?;
    stmt.bind(("hello", "world"))?;
    let mut stmt = conn.prepare("SELECT * FROM kvstore WHERE key = ?1;")?;

    for row in stmt.into_iter().bind((1, "hello"))?.map(|row| row.unwrap()) {
        println!("Row: {:?}", row.read::<&str, _>("value"));
    }

    Ok(())
}
