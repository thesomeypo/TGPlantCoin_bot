use rusqlite::{Connection, Result, params};

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
            colheita_cooldown INTEGER DEFAULT 0
        )",
        [],
    )?;

    conn.execute(
        "CREATE TABLE IF NOT EXISTS codigos_pendentes (
        codigo TEXT PRIMARY KEY,
        telegram_id INTEGER,
        criado_em INTEGER
        )",
        []
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

pub fn cooldown (telegram_id: i64) -> Result<i64> {
    let conn = conectar()?;

    let mut ultima_vez = conn.prepare("SELECT colheita_cooldown FROM usuarios WHERE telegram_id = ?1")?;
    let mut rows = ultima_vez.query([telegram_id])?;

    if let Some(row) = rows.next()? {
        let cooldown: i64 = row.get(0)?;
        Ok(cooldown)
    } else {
        conn.execute("INSERT INTO usuarios (telegram_id, colheita_cooldown) VALUES (?1, 0)",
            [telegram_id]
        )?;
        Ok(0)
    }
}

pub fn atualizar_cooldown (telegram_id: i64, novo_timer: i64) -> Result<()> {
    let conn = conectar()?;
    let _ = cooldown(telegram_id);

    conn.execute(
        "UPDATE usuarios SET colheita_cooldown = ?1 WHERE telegram_id = ?2",
        params![novo_timer, telegram_id],
    )?;
    Ok(())
}

pub fn salvar_codigo (telegram_id: i64, codigo: &str, agora: i64) -> Result<()> {
    let conn = conectar()?;
    
    conn.execute(
        "INSERT INTO codigos_pendentes (codigo, telegram_id, criado_em) VALUES(?1, ?2, ?3)",
        params![codigo, telegram_id , agora],
    )?;

    Ok(())
}

pub fn limpar_codigo (codigo: &str) -> Result<()> {
    let conn = conectar()?;

    conn.execute(
        "DELETE FROM codigos_pendentes WHERE codigo = ?1",
        [codigo],
    )?;

    Ok(())
}