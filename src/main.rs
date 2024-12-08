mod scanner;

#[tokio::main]
async fn main() {
    let ip = "scanme.org"; // IP a escanear

    // Llamada a la función
    scanner::scan_ports(ip).await;
}