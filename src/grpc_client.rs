use services::{
    payment_service_client::PaymentServiceClient, PaymentRequest,    
    transaction_service_client::TransactionServiceClient, TransactionRequest,
    chat_service_client::ChatServiceClient, ChatMessage
};

use tonic::transport::Channel;
use tokio::sync::mpsc;
use tokio_stream::wrappers::ReceiverStream;
use tokio::sync::mpsc::{Sender, Receiver};
use tokio::io::{self, AsyncBufReadExt};

pub mod services {
    tonic::include_proto!("services");
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut payment_client = PaymentServiceClient::connect("http://[::1]:50051").await?;
    let pay_req = tonic::Request::new(PaymentRequest {
        user_id: "user_123".to_string(),
        amount: 100.0,
    });

    let response = payment_client.process_payment(pay_req).await?;
    println!("RESPONSE={:?}", response.into_inner());

    let mut transaction_client = TransactionServiceClient::connect("http://[::1]:50051").await?;
    let tx_req = tonic::Request::new(TransactionRequest {
        user_id: "user_123".to_string(),
    });

    let mut tx_stream = transaction_client.get_transaction_history(tx_req).await?.into_inner();
    while let Some(transaction) = tx_stream.message().await? {
        println!("Transaction: {:?}", transaction);
    }

    let channel = Channel::from_static("http://[::1]:50051").connect().await?;
    let mut chat_client = ChatServiceClient::new(channel);
    let (tx, rx): (Sender<ChatMessage>, Receiver<ChatMessage>) = mpsc::channel(32);

    let tx_clone = tx.clone();
    tokio::spawn(async move {
        let stdin = io::stdin();
        let mut reader = io::BufReader::new(stdin).lines();
    
        while let Ok(Some(line)) = reader.next_line().await {
            if line.trim().is_empty() {
                continue;
            }
            let message = ChatMessage {
                user_id: "user_123".to_string(),
                message: line,
            };
    
            if tx_clone.send(message).await.is_err() {
                eprintln!("Failed to send message to server.");
                break;
            }
        }
    });

    let chat_req = tonic::Request::new(ReceiverStream::new(rx));
    let mut chat_stream = chat_client.chat(chat_req).await?.into_inner();

    while let Some(response) = chat_stream.message().await? {
        println!("Server says: {:?}", response);
    }

    Ok(())
}