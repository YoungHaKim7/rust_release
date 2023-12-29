// use std::future::Future;

// struct Url;

// struct HtmlBody;

// // trait HttpService {
// //     async fn fetch(&self, url: Url) -> HtmlBody;
// // }

// // trait HttpService {
// //     fn fetch(&self, url: Url) -> impl Future<Output = HtmlBody>;
// // }

// trait HttpService {
//     async fn fetch(&self, url: Url) -> HtmlBody;
// }

// fn main() {
//     println!("url : {}", HttpService.fetch("127.0.0.0".to_string()));
// }

use std::future::Future;

// Define the structures with appropriate fields
#[derive(Debug)]
struct Url(String);
#[derive(Debug)]
struct HtmlBody(String);

// Define the trait for HTTP services
trait HttpService {
    async fn fetch(&self, url: Url) -> HtmlBody;
}

// Create a concrete implementation of HttpService (replace with your actual implementation)
struct MyHttpService;
impl HttpService for MyHttpService {
    async fn fetch(&self, url: Url) -> HtmlBody {
        // Perform the HTTP request here
        // ...
        HtmlBody("Example HTML content".to_string()) // Placeholder for actual response
    }
}

#[tokio::main]
async fn main() {
    // Create an instance of the HTTP service
    let http_service = MyHttpService;

    // Call the fetch method asynchronously
    let fetched_body = async { http_service.fetch(Url("127.0.0.0".to_string())).await };

    // Print the fetched HTML body (requires an executor for async runtime)
    println!("URL: {:?}", fetched_body.await);
}
