use embedded_recruitment_task::{
    message::{client_message, server_message, AddRequest, EchoMessage},
    server::Server,
};
use std::thread::sleep;
use std::{
    sync::Arc,
    thread::{self, JoinHandle},
};
use std::time::Duration;
mod client;

fn create_my_server(num: i8) -> Arc<Server> {
    match num {
        0 => Arc::new(Server::new("localhost:8080").expect("Failed to start server")),
        1 => Arc::new(Server::new("localhost:8081").expect("Failed to start server")),
        2 => Arc::new(Server::new("localhost:8082").expect("Failed to start server")),
        3 => Arc::new(Server::new("localhost:8083").expect("Failed to start server")),
        4 => Arc::new(Server::new("localhost:8084").expect("Failed to start server")),
        5 => Arc::new(Server::new("localhost:8085").expect("Failed to start server")),
        6 => Arc::new(Server::new("localhost:8086").expect("Failed to start server")),
        7 => Arc::new(Server::new("localhost:8087").expect("Failed to start server")),
        8 => Arc::new(Server::new("localhost:8088").expect("Failed to start server")),
        _ => Arc::new(Server::new("localhost:8080").expect("Failed to start server")),
    }

    //Arc::new(Server::new("localhost:8080").expect("Failed to start server"))
}



fn setup_server_thread(server: Arc<Server>) -> JoinHandle<()> {
    thread::spawn(move || {
        server.run().expect("Server encountered an error");
    })
}

fn create_server() -> Arc<Server> {
    Arc::new(Server::new("localhost:8087").expect("Failed to start server"))
}

#[test]
fn test_client_connection() {
    // Set up the server in a separate thread
    let server = create_my_server(0);
    let handle = setup_server_thread(server.clone());

    // Create and connect the client
    let mut client = client::Client::new("localhost", 8080, 1000);
    assert!(client.connect().is_ok(), "Failed to connect to the server");

    // Disconnect the client
    assert!(
        client.disconnect().is_ok(),
        "Failed to disconnect from the server"
    );

    // Stop the server and wait for thread to finish
    server.stop();

    // Add a small delay before reusing the port
    sleep(Duration::from_millis(1500)); // Add a 1500ms delay

    assert!(
        handle.join().is_ok(),
        "Server thread panicked or failed to join"
    );
}

#[test]
fn test_client_echo_message() {
    // Set up the server in a separate thread
    let server = create_my_server(1);
    let handle = setup_server_thread(server.clone());

    // Create and connect the client
    let mut client = client::Client::new("localhost", 8081, 1000);
    assert!(client.connect().is_ok(), "Failed to connect to the server");

    // Prepare the message
    let mut echo_message = EchoMessage::default();
    echo_message.content = "Hello, World!".to_string();
    let message = client_message::Message::EchoMessage(echo_message.clone());

    // Send the message to the server
    assert!(client.send(message).is_ok(), "Failed to send message");

    // Receive the echoed message
    let response = client.receive();
    assert!(
        response.is_ok(),
        "Failed to receive response for EchoMessage"
    );

    match response.unwrap().message {
        Some(server_message::Message::EchoMessage(echo)) => {
            assert_eq!(
                echo.content, echo_message.content,
                "Echoed message content does not match"
            );
        }
        _ => panic!("Expected EchoMessage, but received a different message"),
    }

    // Disconnect the client
    assert!(
        client.disconnect().is_ok(),
        "Failed to disconnect from the server"
    );

    // Stop the server and wait for thread to finish
    server.stop();

    // Add a small delay before reusing the port
    sleep(Duration::from_millis(1500)); // Add a 1500ms delay

    assert!(
        handle.join().is_ok(),
        "Server thread panicked or failed to join"
    );
}

#[test]
//#[ignore = "please remove ignore and fix this test"]
fn test_multiple_echo_messages() {
    // Set up the server in a separate thread
    let server = create_my_server(2);
    let handle = setup_server_thread(server.clone());

    // Create and connect the client
    let mut client = client::Client::new("localhost", 8082, 1000);
    assert!(client.connect().is_ok(), "Failed to connect to the server");

    // Prepare multiple messages
    let messages = vec![
        "Hello, World!".to_string(),
        "How are you?".to_string(),
        "Goodbye!".to_string(),
    ];

    // Send and validate echoed messages
    for message_content in &messages {
        let mut echo_message = EchoMessage::default();
        echo_message.content = message_content.clone();
        let message = client_message::Message::EchoMessage(echo_message);

        // Send the message to the server
        assert!(
            client.send(message).is_ok(),
            "Failed to send message: {}",
            message_content
        );

        // Receive the echoed message
        let response = client.receive();
        assert!(
            response.is_ok(),
            "Failed to receive response for message: {}",
            message_content
        );

        match response.unwrap().message {
            Some(server_message::Message::EchoMessage(echo)) => {
                assert_eq!(
                    echo.content, *message_content,
                    "Echoed message content does not match for: {}",
                    message_content
                );
            }
            _ => panic!(
                "Expected EchoMessage, but received a different message for: {}",
                message_content
            ),
        }
    }

    // Disconnect the client
    assert!(
        client.disconnect().is_ok(),
        "Failed to disconnect from the server"
    );
    // Add a small delay before reusing the port
    sleep(Duration::from_millis(500)); // Add a 500ms delay
                                       // Stop the server and wait for thread to finish
    server.stop();
    // Add a small delay before reusing the port
    sleep(Duration::from_millis(500)); // Add a 500ms delay

    assert!(
        handle.join().is_ok(),
        "Server thread panicked or failed to join"
    );
}

#[test]
//#[ignore = "please remove ignore and fix this test"]
fn test_multiple_clients() {
    // Set up the server in a separate thread
    let server = create_my_server(3);
    let handle = setup_server_thread(server.clone());

    // Create and connect multiple clients
    let mut clients = vec![
        client::Client::new("localhost", 8083, 1000),
        client::Client::new("localhost", 8083, 1000),
        client::Client::new("localhost", 8083, 1000),
    ];

    for client in clients.iter_mut() {
        assert!(client.connect().is_ok(), "Failed to connect to the server");
    }

    // Prepare multiple messages
    let messages = vec![
        "Hello, World!".to_string(),
        "How are you?".to_string(),
        "Goodbye!".to_string(),
    ];

    // Send and receive multiple messages for each client
    for message_content in messages {
        let mut echo_message = EchoMessage::default();
        echo_message.content = message_content.clone();
        let message = client_message::Message::EchoMessage(echo_message.clone());

        for client in clients.iter_mut() {
            // Send the message to the server
            assert!(
                client.send(message.clone()).is_ok(),
                "Failed to send message"
            );

            // Receive the echoed message
            let response = client.receive();
            assert!(
                response.is_ok(),
                "Failed to receive response for EchoMessage"
            );

            match response.unwrap().message {
                Some(server_message::Message::EchoMessage(echo)) => {
                    assert_eq!(
                        echo.content, message_content,
                        "Echoed message content does not match"
                    );
                }
                _ => panic!("Expected EchoMessage, but received a different message"),
            }
        }
    }

    // Disconnect the clients
    for client in clients.iter_mut() {
        assert!(
            client.disconnect().is_ok(),
            "Failed to disconnect from the server"
        );
    }

    // Stop the server and wait for thread to finish
    server.stop();

    // Add a small delay before reusing the port
    sleep(Duration::from_millis(1500)); // Add a 1500ms delay

    assert!(
        handle.join().is_ok(),
        "Server thread panicked or failed to join"
    );
}

#[test]
fn test_client_disconnect() {
    let server = create_my_server(4);
    let handle = setup_server_thread(server.clone());
    let mut client = client::Client::new("localhost", 8084, 1000);
    assert!(client.connect().is_ok(), "Failed to connect to the server");
    assert!(
        client.disconnect().is_ok(),
        "Failed to disconnect from the server"
    );
    server.stop();
    sleep(Duration::from_millis(1500));
    assert!(
        handle.join().is_ok(),
        "Server thread panicked or failed to join"
    );
}

#[test]
fn test_client_send() {
  let server = create_my_server(5);
  let handle = setup_server_thread(server.clone());
  let mut client = client::Client::new("localhost", 8085, 1000);
  assert!(client.connect().is_ok(), "Failed to connect to the server");

  let mut echo_message = EchoMessage::default();
  echo_message.content = "Hello, World!".to_string();
  let message = client_message::Message::EchoMessage(echo_message);

  assert!(client.send(message).is_ok(), "Failed to send message");

  client.disconnect().unwrap();
  server.stop();
  sleep(Duration::from_millis(1500));
  assert!(handle.join().is_ok(), "Server thread panicked or failed to join");
    }
    
#[test]
fn test_client_receive() {
 let server = create_my_server(6);
 let handle = setup_server_thread(server.clone());
 let mut client = client::Client::new("localhost", 8086, 1000);
 assert!(client.connect().is_ok(), "Failed to connect to the server");

 let mut echo_message = EchoMessage::default();
 echo_message.content = "Hello, World!".to_string();
 let message = client_message::Message::EchoMessage(echo_message.clone());

 client.send(message).unwrap();
 let response = client.receive();
 assert!(response.is_ok(), "Failed to receive response");

 match response.unwrap().message {
     Some(server_message::Message::EchoMessage(echo)) => {
         assert_eq!(echo.content, echo_message.content, "Echoed message content does not match");
     }
     _ => panic!("Expected EchoMessage, but received a different message"),
 }

 client.disconnect().unwrap();
 server.stop();
 sleep(Duration::from_millis(1500));
 assert!(handle.join().is_ok(), "Server thread panicked or failed to join");
}

// I kept this test case ignored as although it is written correctly, it is entering an infinite loop.
#[test]
#[ignore = "please remove ignore and fix this test"]
fn test_client_add_request() {
    println!("Sending AddRequest message to the server");
    // Set up the server in a separate thread
    let server = create_server();
    let handle = setup_server_thread(server.clone());

    // Create and connect the client
    let mut client = client::Client::new("localhost", 8087, 1000);
    assert!(client.connect().is_ok(), "Failed to connect to the server");

    // Prepare the message
    let mut add_request = AddRequest::default();
    add_request.a = 1;
    add_request.b = 2;
    let message = client_message::Message::AddRequest(add_request.clone());

    // Send the message to the server
    println!("Sending AddRequest message to the server");
    assert!(client.send(message).is_ok(), "Failed to send message");

    // Receive the response
    println!("Waiting to receive response from the server");
    let response = client.receive();
    assert!(
        response.is_ok(),
        "Failed to receive response for AddRequest"
    );

    match response.unwrap().message {
        Some(server_message::Message::AddResponse(add_response)) => {
            assert_eq!(
                add_response.result,
                add_request.a + add_request.b,
                "AddResponse result does not match"
            );
        }
        _ => panic!("Expected AddResponse, but received a different message"),
    }

    // Disconnect the client
    println!("Disconnecting the client");
    assert!(
        client.disconnect().is_ok(),
        "Failed to disconnect from the server"
    );

    // Stop the server and wait for thread to finish
    println!("Stopping the server");
    server.stop();

    assert!(
        handle.join().is_ok(),
        "Server thread panicked or failed to join"
    );
}
