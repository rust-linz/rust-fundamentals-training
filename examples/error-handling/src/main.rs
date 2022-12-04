use serde_json::Value;
use tokio::{fs::File, io::AsyncReadExt};

async fn read_and_parse(file_name: &str) -> String {
    let mut f = File::open(file_name).await.unwrap();
    let mut buf = Vec::new();
    f.read_buf(&mut buf).await.unwrap();
    let s = String::from_utf8(buf).unwrap();

    let v: Value = serde_json::from_str(s.as_str()).unwrap();
    v.get("connectionString").unwrap().to_string()
}

async fn read_and_parse_2(file_name: &str) -> Result<String, &'static str> {
    let mut f = File::open(file_name).await.map_err(|_| "cannot open file")?;
    let mut buf = Vec::new();
    f.read_buf(&mut buf).await.map_err(|_| "cannot read from file")?;
    let s = String::from_utf8(buf).map_err(|_| "cannot convert to utf8")?;

    let v: Value = serde_json::from_str(s.as_str()).map_err(|_| "cannot parse json")?;
    match v.get("connectionString") {
        Some(s) => Ok(s.to_string()),
        None => Err("cannot find connectionString"),
    }
}

#[tokio::main]
async fn main() {
    let cs = read_and_parse("./config.json").await;
    println!("{}", cs);

    let cs = read_and_parse_2("./config2.json").await.unwrap();
    println!("{}", cs);
}
