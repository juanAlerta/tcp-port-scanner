use tokio::net::TcpStream;
use tokio::time::{timeout, Duration};

#[tokio::main]
async fn main() {
    let ip = "scanme.org"; // IP a escanear
    let ports = 1..100; // Rango de puertos a probar

    for port in ports {
        let address = format!("{}:{}", ip, port);

        // Cada escaneo es una tarea asincrónica
        let scan = async move {
            match timeout(Duration::from_secs(2), TcpStream::connect(&address)).await {
                Ok(Ok(_)) => println!("Puerto abierto: {}", port),
                Ok(Err(_)) | Err(_) => (), // Cerrado o sin respuesta
            }
        };

        // Lanza la tarea
        tokio::spawn(scan);
    }

    // Espera a que todas las tareas finalicen
    tokio::time::sleep(Duration::from_secs(5)).await;
}