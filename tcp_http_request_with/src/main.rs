use std::{net::TcpStream, io::Write};

fn main() -> std::io::Result<()> {
    let host = "www.benyaamin.com";
    let conn_url = format!("{}:80", &host);
    let mut conn = TcpStream::connect(&conn_url)?;

    conn.write_all(b"GET / HTTP/1.1")?;
    conn.write(b"/r/n")?;

    conn.write_all(format!("HOST: {}", host).as_bytes())?;
    conn.write_all(b"/r/n/r/n")?;

    std::io::copy(&mut conn, &mut std::io::stdout())?;

    Ok(())
}
