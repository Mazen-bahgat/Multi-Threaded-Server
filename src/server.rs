use crate::message::EchoMessage;
use log::{error, info, warn};
use prost::Message;
use std::{
    io::{self, ErrorKind, Read, Write},
    net::{TcpListener, TcpStream},
    sync::{
        atomic::{AtomicBool, Ordering},
        Arc,
    },
    thread,
    time::Duration,
};

/*
This struct represents a connected client,
with a TCP stream "stream" for communication and a shared atomic flag "is_running" to check if the server is running.
The old struct did not contain an atomic flag, which is used to check if the server is running or not.
*/
struct Client {
    stream: TcpStream,
    is_running: Arc<AtomicBool>,
}

/*
The implementation of the Client struct contains a new function that creates a new instance of the Client struct,
initializing it with a TCP stream and the shared running state using the atomic flag.
*/

impl Client {
    // This function creates a new instance of the Client struct with a TCP stream and a shared atomic flag.
    pub fn new(stream: TcpStream, is_running: Arc<AtomicBool>) -> Self {
        Client { stream, is_running }
    }

    // This function handles client communication, reading messages, decoding them, and sending a response back to the client.
    pub fn handle(&mut self) -> io::Result<()> {
        let mut buffer = vec![0; 512]; // This buffer is used to read data from the client.

        // This loop reads messages from the client, decodes them, and sends a response back to the client.
        loop {
            if !self.is_running.load(Ordering::SeqCst) {
                // This checks if the server is still running.
                info!("Server is shutting down. Closing client connection.");
                break;
            }

            match self.stream.read(&mut buffer) {
                // This reads data from the client into the buffer.
                Ok(0) => {
                    info!("Client disconnected.");
                    break;
                }

                Ok(bytes_read) => {
                    // This decodes the message from the buffer and sends a response back to the client.
                    buffer.truncate(bytes_read);
                    match EchoMessage::decode(&buffer[..]) {
                        Ok(message) => {
                            info!("Received: {}", message.content);

                            let payload = message.encode_to_vec();
                            self.stream.write_all(&payload)?;
                            self.stream.flush()?;
                        }
                        Err(e) => {
                            error!("Failed to decode message: {}", e);
                            break;
                        }
                    }
                }
                Err(e) if e.kind() == ErrorKind::WouldBlock => {
                    // This handles the case where no data is available to read.
                    thread::sleep(Duration::from_millis(10));
                }
                Err(e) => {
                    // This handles other errors that may occur during reading.
                    error!("Error reading from client: {}", e);
                    break;
                }
            }
        }

        Ok(())
    }
}

/*
This struct represents a TCP server that listens for incoming connections, 
and a shared atomic flag for the server's running state..
*/
pub struct Server { 
    listener: TcpListener,
    is_running: Arc<AtomicBool>,
}

/*
The implementation of the Server struct contains functions to create a new server instance,
run the server, and stop the server .
*/
impl Server {

    // This function creates a new instance of the Server struct with a TCP listener and a shared atomic flag.
    pub fn new(addr: &str) -> io::Result<Self> {
        let listener = TcpListener::bind(addr)?;
        let is_running = Arc::new(AtomicBool::new(false));
        Ok(Server {// This creates a new instance of the Server struct with a TCP listener and a shared atomic flag.
            listener,
            is_running,
        })
    }

    // This function runs the server, accepting incoming connections and handling them in separate threads.
    pub fn run(&self) -> io::Result<()> {
        self.is_running.store(true, Ordering::SeqCst);
        info!("Server is running on {}", self.listener.local_addr()?);

        // This sets the listener to non-blocking mode.
        self.listener.set_nonblocking(true)?;

        // This loop accepts incoming connections and handles them in separate threads.
        while self.is_running.load(Ordering::SeqCst) {
            match self.listener.accept() {
                Ok((stream, addr)) => {
                    info!("New client connected: {}", addr);

                    // This creates a new client instance and handles the client in a separate thread.
                    let is_running = self.is_running.clone();
                    thread::spawn(move || {
                        let mut client = Client::new(stream, is_running);
                        if let Err(e) = client.handle() {
                            error!("Error handling client {}: {}", addr, e);
                        }
                        info!("Client {} disconnected.", addr);
                    });
                }
                Err(ref e) if e.kind() == ErrorKind::WouldBlock => { // This handles the case where no connections are available to accept.
                    thread::sleep(Duration::from_millis(10));
                }
                Err(e) => {
                    error!("Error accepting connection: {}", e);
                }
            }

        }

        info!("Server stopped.");
        Ok(())
    }

    // This function stops the server by setting the shared atomic flag to false.
    pub fn stop(&self) {
        if self.is_running.load(Ordering::SeqCst) {// This checks if the server is running.
            self.is_running.store(false, Ordering::SeqCst);
            info!("Shutdown signal sent.");
        } else {
            warn!("Server was already stopped or not running.");
        }
    }
}
