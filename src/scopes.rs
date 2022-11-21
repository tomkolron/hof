use reqwest::{Client, header};
use serde_json;

#[tokio::main]
pub async fn get_scopes(search: String, cookie: String, csrf: String) -> Result<Vec<String>, Box<dyn std::error::Error>> {
    // Set headerMap
    let mut headers = header::HeaderMap::new();
    headers.insert("x-csrf-token", csrf.parse().unwrap());
    headers.insert(header::COOKIE, cookie.parse().unwrap());
    headers.insert("Content-Type", "application/json".parse().unwrap());

    // Set query
    let query = "{\"query\":\"query TeamAssets($handle: String!) {\\n  team(handle: $handle) {\\n    in_scope_assets: structured_scopes(\\n      first: 650\\n      archived: false\\n      eligible_for_submission: true\\n    ) {\\n      edges {\\n        node {\\n          id\\n          asset_type\\n          asset_identifier\\n          instruction\\n          max_severity\\n          eligible_for_bounty\\n          labels(first: 100) {\\n            edges {\\n              node {\\n                id\\n                name\\n                __typename\\n              }\\n              __typename\\n            }\\n            __typename\\n          }\\n          __typename\\n        }\\n        __typename\\n      }\\n      __typename\\n    }\\n    __typename\\n  }\\n}\\n\",\"variables\":{\"handle\":\"app_name_variable\"}}".replace("app_name_variable", &search);

    // Create reqwest client
    let client = Client::new();

    // Set res to a return of a post request 
    let res = client.post("https://hackerone.com/graphql")
        .headers(headers)
        .body(query)
        .send()
        .await?
        .text()
        .await?;    

    // Check if request was successful
    if res.contains("NOT_FOUND"){
        panic!("Request to get scopes failed check you interenet connection and your query");
    }

    // Set json object
    let json: serde_json::Value = serde_json::from_str(&res).expect("couldn't decode response to json");

    // Get the scopes out of the json
    let scopes_json = json["data"]["team"]["in_scope_assets"]["edges"].as_array().unwrap();

    // Declare empty scopes vector
    let mut scopes: Vec<String> = Vec::new();

    // Loop through all scopes json
    for i in scopes_json.iter() {
        // Check if scope is of type url
        if i["node"]["asset_type"] == "URL" {
            //Push scope to vector
            let i = i["node"]["asset_identifier"].as_str().unwrap().to_string();
            scopes.push(i);
        }
    }
    
    // Return scopes
    return Ok(scopes);
}
