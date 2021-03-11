use async_std::net::TcpStream;
use async_std::prelude::*;
use uuid::Uuid;

const BIND_ADDR: &str = "0.0.0.0:1337";
// const BIND_ADDR: &str = "tracing.2020.ctfcompetition.com:1337";

fn to_bytes(b: &[u8]) -> [u8; 16] {
    let mut result = [0u8; 16];
    for i in 0..15 {
        result[i] = b[i];
    }
    result
}

#[async_std::main]
async fn main() -> std::io::Result<()> {
    env_logger::init();
    use std::time::Instant;

    let values: Vec<(String, Uuid)> = vec![
        (
            "8888888888888888".to_owned(),
            Uuid::from_bytes(to_bytes("8888888888888888".as_bytes())),
        ),
        (
            "9999999999999999".to_owned(),
            Uuid::from_bytes(to_bytes("9999999999999999".as_bytes())),
        ),
        (
            "1111000000000000".to_owned(),
            Uuid::from_bytes(to_bytes("1111000000000000".as_bytes())),
        ),
        (
            "1110000000000000".to_owned(),
            Uuid::from_bytes(to_bytes("1110000000000000".as_bytes())),
        ),
        (
            "1100000000000000".to_owned(),
            Uuid::from_bytes(to_bytes("1100000000000000".as_bytes())),
        ),
        (
            "1000000000000000".to_owned(),
            Uuid::from_bytes(to_bytes("1000000000000000".as_bytes())),
        ),
        (
            "0000000000000000".to_owned(),
            Uuid::from_bytes(to_bytes("0000000000000000".as_bytes())),
        ),
    ];

    let mut stream = TcpStream::connect(BIND_ADDR).await?;
    for (v, u) in values {
        stream.write_all(v.as_bytes()).await?;
        log::info!("v: {}, u:{:?}", v, u);
    }
    stream.shutdown(std::net::Shutdown::Write)?;
    let mut buf = vec![0u8; 1024];
    let n = stream.read(&mut buf).await?;
    let mut bufbla: [u8; 4] = [0, 0, 0, 0];
    bufbla[0] = buf[0];
    bufbla[1] = buf[1];
    bufbla[2] = buf[2];
    bufbla[3] = buf[3];
    let mut new = vec![1u8; 4];

    let start = Instant::now();
    let b = stream.read(&mut new).await?;
    log::info!("read: {}, buf: {:?}", n, u32::from_be_bytes(bufbla));
    log::info!(
        "shutdown: {}, buf: {:?}, time: {:?}",
        b,
        new,
        start.elapsed()
    );

    Ok(())
}
