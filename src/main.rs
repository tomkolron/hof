use reqwest::header;
use serde_json;

#[tokio::main]

async fn main() -> Result<(), reqwest::Error> {
    let mut headers = header::HeaderMap::new();
    headers.insert("x-csrf-token", "8yI1bBt25477yWBqWfuUukx5+FM3I+52OgJp4K568o1fnDtYYXLIhC6839osAZnmID6QZlZ4rqBlyGTrLNncMw==".parse().unwrap());
    headers.insert("Content-Type", "application/json".parse().unwrap());
    headers.insert(header::COOKIE, "__Host-session=NXpaWDVZMDFaSnNiazB5cFY0OXpySWVwWXJ6SFN6S0FuUnVxZjhWUzFSUjNyTFFvckNXZjA3cXF0S2xRVjF4T01mUGhjTmt3OFRtQXdWL3hkZVpOdFFDWE5LS0s3S3Z4ZkZUVzRneCtFd2xzNGMvWDlrVUg2ZGh6TDlta1dyTjFVTXc1R1hSNUpqRUYyemoycldFbis4dHVQbEYwQzQvN1VMNWVhc1NCbkx5aitzTFJIS0tQUkZIQ0xaUjZ5TTNSYmhKS2dQUTdPajZ1Yk1QcjZFaENUcmdWMnhKSlJrY1RjSEx1Z3FZSmVUUllLeXc1SXdDK2wxSGRJS1g3aWR4ZnVJNTI3Wk55TGRhY3hwWGNHSjh3UDJ6b3lXYStDZTQ2MkpkVGtFTVlMNzhiNjZpWW5Ndmpjd1lSL0pzcW5oM2xTTFVKa2lycnE2a29DYWFLWXV4MkdBPT0tLUxBMDNKUEpxS0k5dFdtb0x0REJNTnc9PQ%3D%3D--738baf14af5c02e4b72f7c9bc49f5893292874ca; h1_device_id=6ec619d7-4785-4d7d-a6cb-de08e0a104dd".parse().unwrap());

    let app = "reddit";
    let query = "{\"query\":\"query TeamAssets($handle: String!) {\\n  team(handle: $handle) {\\n    in_scope_assets: structured_scopes(\\n      first: 650\\n      archived: false\\n      eligible_for_submission: true\\n    ) {\\n      edges {\\n        node {\\n          id\\n          asset_type\\n          asset_identifier\\n          instruction\\n          max_severity\\n          eligible_for_bounty\\n          labels(first: 100) {\\n            edges {\\n              node {\\n                id\\n                name\\n                __typename\\n              }\\n              __typename\\n            }\\n            __typename\\n          }\\n          __typename\\n        }\\n        __typename\\n      }\\n      __typename\\n    }\\n    __typename\\n  }\\n}\\n\",\"variables\":{\"handle\":\"app_name_variable\"}}".replace("app_name_variable", app);

    let client = reqwest::Client::new();
    let res = client.post("https://hackerone.com/graphql")
        .headers(headers)
        .body(query)
        .send()
        .await?
        .text()
        .await?;
    
    let json: serde_json::Value = serde_json::from_str(&res).expect("error");
    let scopes = &json["data"]["team"]["in_scope_assets"]["edges"];

    for i in scopes.as_array().unwrap() {
        if i["node"]["asset_type"] == "URL" {
            println!("{}", i["node"]["asset_identifier"]);
        }
    }

    // println!("{:?}", scopes.as_arrae().unwrap().len());
    // println!("{a}", a = res);

    Ok(())
}
