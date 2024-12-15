mod scanner;

#[tokio::main]
async fn main() {
    let ip = "8.8.8.8"; // IP a escanear

    // Llamada a la función
    scanner::scan_ports(ip).await;
}