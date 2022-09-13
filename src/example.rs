use tokio::io::AsyncReadExt;
use tokio_serial::SerialPortBuilderExt;
use std::time::Duration;
use crate::converter::convert;
use crate::parser::Parser;

#[tokio::main]
async fn main() -> tokio_serial::Result<()> {
    let mut port = tokio_serial::new("/dev/serial0", 19200)
        .timeout(Duration::from_secs(5))
        .open_native_async()?;
    
    let mut buf: Vec<u8> = vec![0; 2048];
    let mut parser = Parser::new();

    loop{ 
        if let Ok(r) = port.read(&mut buf).await {
            if let Ok(parsed) = parser.parse_slice(&buf[..r]) {
                println!("{:?}", convert(parsed));
            }
        }

    }
}
