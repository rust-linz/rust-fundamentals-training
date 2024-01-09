use serde_json::Value;
use thiserror::Error;
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

async fn read_and_parse_3(file_name: &str) -> anyhow::Result<String> {
    let mut f = File::open(file_name).await?;
    let mut buf = Vec::new();
    f.read_buf(&mut buf).await?;
    let s = String::from_utf8(buf)?;

    let v: Value = serde_json::from_str(s.as_str())?;
    match v.get("connectionString") {
        Some(s) => Ok(s.to_string()),
        None => Err(anyhow::anyhow!("cannot find connectionString")),
    }
}

#[derive(Error, Debug)]
enum ParseError {
    #[error("error while reading file")]
    Io(#[from] std::io::Error),
    #[error("error while converting file content to utf8")]
    Utf8(#[from] std::string::FromUtf8Error),
    #[error("error while parsing file content")]
    Json(#[from] serde_json::Error),
    #[error("other error")]
    Unknown,
}

async fn read_and_parse_4(file_name: &str) -> Result<String, ParseError> {
    let mut f = File::open(file_name).await?;
    let mut buf = Vec::new();
    f.read_buf(&mut buf).await?;
    let s = String::from_utf8(buf)?;

    let v: Value = serde_json::from_str(s.as_str())?;
    match v.get("connectionString") {
        Some(s) => Ok(s.to_string()),
        None => Err(ParseError::Unknown),
    }
}

#[tokio::main]
async fn main() {
    let cs = read_and_parse("./config.json").await;
    println!("{}", cs);

    let cs = read_and_parse_2("./config.json").await.unwrap();
    println!("{}", cs);

    let cs = read_and_parse_3("./config.json").await;
    match cs {
        Ok(cs) => println!("{}", cs),
        Err(e) => {
            if let Some(io_err) = e.downcast_ref::<std::io::Error>() {
                println!("IO Error: {io_err}");
            } else if let Some(json_err) = e.downcast_ref::<serde_json::Error>() {
                println!("JSON error: {json_err}");
            } else {
                println!("Other error: {e}");
            }
        }
    }

    let cs = read_and_parse_4("./config.json").await;
    match cs {
        Ok(cs) => println!("{}", cs),
        Err(e) => {
            match e {
                ParseError::Io(io_err) => println!("IO Error: {io_err}"),
                ParseError::Json(json_err) => println!("JSON error: {json_err}"),
                ParseError::Utf8(utf8_err) => println!("UTF8 error: {utf8_err}"),
                ParseError::Unknown => println!("Other error"),
            }
        }
    }
}
