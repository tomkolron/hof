use reqwest::header;
use serde_json;

#[tokio::main]

pub async fn get_scopes(search: String) -> Result<Vec<String>, Box<dyn std::error::Error>> {
    let mut headers = header::HeaderMap::new();
    headers.insert("x-csrf-token", "8yI1bBt25477yWBqWfuUukx5+FM3I+52OgJp4K568o1fnDtYYXLIhC6839osAZnmID6QZlZ4rqBlyGTrLNncMw==".parse().unwrap());
    headers.insert("Content-Type", "application/json".parse().unwrap());
    headers.insert(header::COOKIE, "__Host-session=NXpaWDVZMDFaSnNiazB5cFY0OXpySWVwWXJ6SFN6S0FuUnVxZjhWUzFSUjNyTFFvckNXZjA3cXF0S2xRVjF4T01mUGhjTmt3OFRtQXdWL3hkZVpOdFFDWE5LS0s3S3Z4ZkZUVzRneCtFd2xzNGMvWDlrVUg2ZGh6TDlta1dyTjFVTXc1R1hSNUpqRUYyemoycldFbis4dHVQbEYwQzQvN1VMNWVhc1NCbkx5aitzTFJIS0tQUkZIQ0xaUjZ5TTNSYmhKS2dQUTdPajZ1Yk1QcjZFaENUcmdWMnhKSlJrY1RjSEx1Z3FZSmVUUllLeXc1SXdDK2wxSGRJS1g3aWR4ZnVJNTI3Wk55TGRhY3hwWGNHSjh3UDJ6b3lXYStDZTQ2MkpkVGtFTVlMNzhiNjZpWW5Ndmpjd1lSL0pzcW5oM2xTTFVKa2lycnE2a29DYWFLWXV4MkdBPT0tLUxBMDNKUEpxS0k5dFdtb0x0REJNTnc9PQ%3D%3D--738baf14af5c02e4b72f7c9bc49f5893292874ca; h1_device_id=6ec619d7-4785-4d7d-a6cb-de08e0a104dd".parse().unwrap());

    let query = "{\"query\":\"query TeamAssets($handle: String!) {\\n  team(handle: $handle) {\\n    in_scope_assets: structured_scopes(\\n      first: 650\\n      archived: false\\n      eligible_for_submission: true\\n    ) {\\n      edges {\\n        node {\\n          id\\n          asset_type\\n          asset_identifier\\n          instruction\\n          max_severity\\n          eligible_for_bounty\\n          labels(first: 100) {\\n            edges {\\n              node {\\n                id\\n                name\\n                __typename\\n              }\\n              __typename\\n            }\\n            __typename\\n          }\\n          __typename\\n        }\\n        __typename\\n      }\\n      __typename\\n    }\\n    __typename\\n  }\\n}\\n\",\"variables\":{\"handle\":\"app_name_variable\"}}".replace("app_name_variable", &search);

    let client = reqwest::Client::new();
    let res = client.post("https://hackerone.com/graphql")
        .headers(headers)
        .body(query)
        .send()
        .await?
        .text()
        .await?;    
    if res.contains("NOT_FOUND"){
        panic!("couldn't get request");
    }
    let json: serde_json::Value = serde_json::from_str(&res).expect("couldn't decode response to json");
    let scopes_json = json["data"]["team"]["in_scope_assets"]["edges"].as_array().unwrap();
    let mut scopes: Vec<String> = Vec::new();

    for i in scopes_json.iter() {
        if i["node"]["asset_type"] == "URL" {
            scopes.push(i["node"]["asset_identifier"].to_string());
        }
    }
    Ok(scopes)
}
