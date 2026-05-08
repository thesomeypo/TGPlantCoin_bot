use rusqlite::{params, Connection, Result};

pub fn conectar() -> Result<Connection> {
    Connection::open("plantcoin.db")
}

pub fn iniciar_banco() -> Result<()> {
    let conn = conectar()?;
    conn.execute(
        "CREATE TABLE IF NOT EXISTS usuarios (
            id INTEGER PRIMARY KEY,
            telegram_id INTEGER UNIQUE,
            saldo REAL DEFAULT 0.0,
            conheita_cooldown INTEGER DEFAULT 0
        )",
        [],
    )?;
    Ok(())
}
pub fn ver_saldo(telegram_id: i64) -> Result<f64> {
    let conn = conectar()?;

    let mut stmt = conn.prepare("SELECT saldo FROM usuarios WHERE telegram_id = ?1")?;

    let mut rows = stmt.query([telegram_id])?;

    if let Some(row) = rows.next()? {
        let saldo: f64 = row.get(0)?;
        Ok(saldo)
    } else {
        conn.execute(
            "INSERT INTO usuarios (telegram_id, saldo) VALUES (?1, 0.0)",
            [telegram_id]
        )?;
        Ok(0.0)
    }
}

pub fn adicionar_saldo(telegram_id: i64, valor: f64) -> Result<()> {
    let conn = conectar()?;

    let _ = ver_saldo(telegram_id)?;

    conn.execute(
        "UPDATE usuarios SET saldo = saldo + ?1 WHERE telegram_id = ?2",
        params![valor, telegram_id],
    )?; 
    Ok(())
}