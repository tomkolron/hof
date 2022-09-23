use reqwest::header;
use serde_json;

#[tokio::main]

pub async fn get_scopes(search: String) -> Result<Vec<String>, Box<dyn std::error::Error>> {
    let mut headers = header::HeaderMap::new();
    headers.insert("x-csrf-token", "SamsFQNKt6SGoZGASqEVzOrk6Kr6aQ0L6fTCXVEq+WM511+bPXkgcJjnB9Ncha61QEAK3P3A8OSJZKXTVAvi3w==".parse().unwrap());
    headers.insert("Content-Type", "application/json".parse().unwrap());
    headers.insert(header::COOKIE, "__Host-session=Q3JoYnBVSGxqUlJiQy94aDZtVFJsaXdzdFFYb0lVWnI4NEZxdTczb3pjZVBZWjhiUGNkOWlOcXBMQ2l5YkhYTEZzWDVVa085N0pRVXpxV3ZnUjlrVjNhR3IwY25qSzQxZVNxYWFxdnB0NExxRW1QWWlaVkdjUlNlWVdTYkJ0eTBkdXNCL3liWTBuQ1NHbnBOV2VpNnVHUitrRjA5YlJlVXdpSHZLaDJ0QlIwY2Vvclg4WTZkSWtSN3grTjVjbUFyQ05tYzBoeE1wR1hVVmZScDFXUjR5N0RKRjFnZktjREpoa21lRitTRGRlYWRvcS9zdnZ4MkE1QUVUVkxKZ1laU3VYbzErOTRFL1FDa09wNWVaL2hLZTZYais4Q0Y3a2F6UGV2cmJhUlpFVHAxMnJ4OHZrdDVlRjhZSHI2WGN3NTNCS2xuQVlGQk1TVVB0MDJwNzM5REd3PT0tLXMzeW9kUU1Vb0Z5aUdYSHIwaFRPYmc9PQ%3D%3D--47432d9b6126a2bf31b341e51b0615c2fa4559ca; h1_device_id=fd44accc-11f0-4aa9-a41b-9e96e6026e4f".parse().unwrap());

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
        panic!("Request to get scopes failed check you interenet connection and your query");
    }
    let json: serde_json::Value = serde_json::from_str(&res).expect("couldn't decode response to json");
    let scopes_json = json["data"]["team"]["in_scope_assets"]["edges"].as_array().unwrap();

    let mut scopes: Vec<String> = Vec::new();
    for i in scopes_json.iter() {
        if i["node"]["asset_type"] == "URL" {
            let mut i = i["node"]["asset_identifier"].to_string();
            i.pop();
            i.remove(0);
            scopes.push(i);
        }
    }
    Ok(scopes)
}
