use reqwest::{Client, header};
use serde_json;

#[tokio::main]

pub async fn get_scopes(search: String, cookie: String, csrf: String) -> Result<Vec<String>, Box<dyn std::error::Error>> {
    let mut headers = header::HeaderMap::new();
    headers.insert("x-csrf-token", csrf.parse().unwrap());
    headers.insert(header::COOKIE, cookie.parse().unwrap());
    headers.insert("Content-Type", "application/json".parse().unwrap());

    let query = "{\"query\":\"query TeamAssets($handle: String!) {\\n  team(handle: $handle) {\\n    in_scope_assets: structured_scopes(\\n      first: 650\\n      archived: false\\n      eligible_for_submission: true\\n    ) {\\n      edges {\\n        node {\\n          id\\n          asset_type\\n          asset_identifier\\n          instruction\\n          max_severity\\n          eligible_for_bounty\\n          labels(first: 100) {\\n            edges {\\n              node {\\n                id\\n                name\\n                __typename\\n              }\\n              __typename\\n            }\\n            __typename\\n          }\\n          __typename\\n        }\\n        __typename\\n      }\\n      __typename\\n    }\\n    __typename\\n  }\\n}\\n\",\"variables\":{\"handle\":\"app_name_variable\"}}".replace("app_name_variable", &search);

    let client = Client::new();
    let res = client.post("https://hackerone.com/graphql")
        .headers(headers)
        .body(query)
        .send()
        .await?
        .text()
        .await?;    
    if res.contains("NOT_FOUND"){
        panic!("Request to get scopes failed check you interenet connection and your query");
    }
    let json: serde_json::Value = serde_json::from_str(&res).expect("couldn't decode response to json");
    let scopes_json = json["data"]["team"]["in_scope_assets"]["edges"].as_array().unwrap();

    let mut scopes: Vec<String> = Vec::new();
    for i in scopes_json.iter() {
        if i["node"]["asset_type"] == "URL" {
            let i = i["node"]["asset_identifier"].as_str().unwrap().to_string();
            scopes.push(i);
        }
    }
    Ok(scopes)
}
