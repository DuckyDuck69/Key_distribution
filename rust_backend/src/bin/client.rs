/// Entry point for a simple gRPC client that interacts with the KvStore service.
/// 
/// Performs the following operations sequentially:
/// 1) Connects to the KvStore server running at 127.0.0.1:50051.
/// 2) Sends a "PutRequest" to insert a key-value pair ("k1" -> "hello").
/// 3) Sends a "GetRequest" to retrieve the value for "k1".
/// 4) Sends a "DeleteRequest" to remove "k1".
/// 5) Attempts another "GetRequest" to confirm that "k1" was deleted.
 



use rust_backend::kvstore::kv_store_client::KvStoreClient;
use rust_backend::kvstore::{PutRequest, GetRequest, DeleteRequest};

#[tokio::main]
async fn main() -> anyhow::Result<()> {   
    let mut client = KvStoreClient::connect("http://13.236.5.1:50051").await?;

    // PUT
    let put = PutRequest {
        //key: String::from("k1")   ->alternative way to write 
        key: "k1".into(),   //into convert covert one type to another, in this case &str to String   
        value: b"this key is from -$(date +%s)".to_vec(), //the b means "make this a byte string    "
    };
    println!("Put: {:?}", client.put(put).await?);

    // GET
    let get = GetRequest { key: "k1".into() };
    println!("Get: {:?}", client.get(get).await?);

    //DELETE
    let del = DeleteRequest { key: "k1".into() };
    println!("Delete: {:?}", client.delete(del).await?);

    // GET 
    let get2 = GetRequest { key: "k1".into() };
    println!("Get after delete: {:?}", client.get(get2).await?);

    Ok(())
}