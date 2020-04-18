use std::sync::Arc;
use std::time::Duration;
use tokio::sync::Mutex;
use paho_mqtt as mqtt;

pub struct MqttAPI {
    pub endpoint: String,
    pub timeout: Duration,
    pub ssl_ca: String,
    pub ssl_cert: String,
    pub ssl_key: String,
    client: Arc<Mutex<mqtt::AsyncClient>>,
}

impl MqttAPI {
    pub fn new(endpoint: String, client_id: String,
               ca_path: String, cert_path: String, key_path: String)
        -> Result<MqttAPI, Box<dyn std::error::Error>> {
        // Client options
        let opts = mqtt::CreateOptionsBuilder::new()
            .server_uri(endpoint.clone())
            .client_id(client_id)
            .finalize();

        Ok(MqttAPI {
            endpoint: endpoint,
            timeout: Duration::from_secs(5),
            ssl_ca: ca_path,
            ssl_cert: cert_path,
            ssl_key: key_path,
            client: Arc::new(Mutex::new(mqtt::AsyncClient::new(opts).unwrap())),
        })
    }

    pub async fn start(&mut self) {
        self.client.lock().await
            .set_connection_lost_callback(|_|
                println!("Connection lost with MQTT server, reconnecting asap..."));
        let ssl_opts = mqtt::ssl_options::SslOptionsBuilder::new()
            .trust_store(self.ssl_ca.as_str())
            .key_store(self.ssl_cert.as_str())
            .private_key(self.ssl_key.as_str())
            .finalize();
        let opts = mqtt::ConnectOptionsBuilder::new()
            .keep_alive_interval(self.timeout)
            .max_inflight(100)
            .will_message(mqtt::message::Message::new(
                    String::from("/server/logs"), "Server disconnected".as_bytes(), 1))
            .ssl_options(ssl_opts)
            .automatic_reconnect(Duration::from_millis(100), Duration::from_secs(2))
            .finalize();
        match self.client.lock().await
            .connect(opts)
            .wait_for(Duration::from_secs(5)) {
                Ok((s, i, b)) => println!("Connected to {:}, status: {:}, bool: {:}", s, i, b),
                Err(e) => println!("{:}", e),
            };
    }
}
