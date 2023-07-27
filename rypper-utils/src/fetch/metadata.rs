use reqwest::{Client, Response, header::{ACCEPT, HeaderMap}};

pub const REPO_HEADER: &str = "application/xml, application/metalink+xml, application/metalink4+xml";

// Q: should this be a struct or just util functions?
// Function to send the custom request
pub async fn send_custom_request(url: &str) -> Result<Response, reqwest::Error> {
    // Create a new reqwest Client
    let client = Client::new();

    // Create custom Accept headers
    let mut headers = HeaderMap::new();
    // TODO: the following headers should be a constant?
    
    headers.insert(
        ACCEPT,
        REPO_HEADER
            .parse().unwrap(),
    );

    // Build the request with the custom headers
    let request_builder = client.get(url).headers(headers);

    // Send the request and await the response
    let response = request_builder.send().await?;

    Ok(response)
}

// #[tokio::main]
// async fn main() {
//     let url = "https://download.opensuse.org/tumbleweed/repo/oss/repodata/repomd.xml";
//     let response = send_custom_request(url).await;

//     match response {
//         Ok(response) => {
//             // Process the response here
//             println!("Response Status: {}", response.status());
//             let r: String = match response.text_with_charset("utf-8").await {
//                 Ok(r) => r,
//                 Err(e) => {
//                     panic!("Invalid response text: {}", e);
//                 }
//             };
//             println!("Response text: {:#?}", r);
//             // ... other handling of the response ...
//         }
//         Err(e) => {
//             eprintln!("Error: {}", e);
//         }
//     }
// }
