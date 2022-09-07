use reqwest::header;
use serde_json;

#[tokio::main]

async fn main() -> Result<(), reqwest::Error> {
    let mut headers = header::HeaderMap::new();
    headers.insert("authority", "hackerone.com".parse().unwrap());
    headers.insert("accept", "*/*".parse().unwrap());
    headers.insert("accept-language", "he-IL,he;q=0.9,en-US;q=0.8,en;q=0.7".parse().unwrap());
    headers.insert("content-type", "application/json".parse().unwrap());
    headers.insert(header::COOKIE, "h1_device_id=fbec298b-6004-4a1a-82dc-627a9621ab56; AMP_MKTG_b7cba2c14c=JTdCJTdE; AMP_b7cba2c14c=JTdCJTIyb3B0T3V0JTIyJTNBZmFsc2UlMkMlMjJkZXZpY2VJZCUyMiUzQSUyMmU0MGMxYmE3LTlkYzgtNDJjZS1hZDIyLTE1YjA3Y2Y1NDg5NiUyMiUyQyUyMmxhc3RFdmVudFRpbWUlMjIlM0ExNjYyNDY2MDU3NTkyJTJDJTIyc2Vzc2lvbklkJTIyJTNBMTY2MjQ2NjA0MDA5OSU3RA==; _dd_s=rum=0&expire=1662466956999; __Host-session=bmROWXZHSkRvcHhuNE1ZWk1YbmxhZjhGa0pDeEN6U0pET0NUY0x5WUlROUJVcUxkbkNrQXhjbFh4VHFJWVoySkVYbnRLNWYzUWZYRnRvRVAyRFdnNzVVK0tLOHBIeGVRS3ljUWM1R1dSV1VBWUJaK1ppRVNiVFEyeSthNlVrMTBSU3FEQlJkNDZ6aHBXU3YxYTBFdnlRdkNjTXRrMzEyc3pJYTVQMm1BOFdYUnlGNVVlYTcrTFJSR3E4UHJ1dGdFREFseFRRclpuYWVubEtyem5IRE16a2szTUZZZy9OVVFWYjE4Qkx5d3ZWU3FrUUh1REE0Mjc4d284bTkrODNKTWNGd1h1bHlpdU9QR08xaGNJL21BbDRNRlpCOWdXenRmZGs1bWtHQTdYbnA5cEs1enJZTkVUT0FrazJlN1N6YWhZUTlrNDRCWHVJN2pVZDF1R05Fc3lRPT0tLUIweG5FMnZNbXJnU2szWWpCNUNYUWc9PQ%3D%3D--8b6bdd30756ca4ba641ec0e08f38497a2d619ce1".parse().unwrap());
    headers.insert("origin", "https://hackerone.com".parse().unwrap());
    headers.insert("referer", "https://hackerone.com/reddit?type=team".parse().unwrap());
    headers.insert("sec-ch-ua", "\"Chromium\";v=\"104\", \" Not A;Brand\";v=\"99\", \"Google Chrome\";v=\"104\"".parse().unwrap());
    headers.insert("sec-ch-ua-mobile", "?0".parse().unwrap());
    headers.insert("sec-ch-ua-platform", "\"Linux\"".parse().unwrap());
    headers.insert("sec-fetch-dest", "empty".parse().unwrap());
    headers.insert("sec-fetch-mode", "cors".parse().unwrap());
    headers.insert("sec-fetch-site", "same-origin".parse().unwrap());
    headers.insert("sec-gpc", "1".parse().unwrap());
    headers.insert("user-agent", "Mozilla/5.0 (X11; Linux x86_64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/104.0.0.0 Safari/537.36".parse().unwrap());
    headers.insert("x-csrf-token", "NnPKpLx9O/Hpvm9er01K4IFQPHgzCTf5rNRCG5465/9GDTkqgk6sJff4+Q25afGZK/TeDjSgyhbMRCWVmxv8Qw==".parse().unwrap());

    let client = reqwest::Client::new();
    let res = client.post("https://hackerone.com/graphql")
        .headers(headers)
        .body("{\"operationName\":\"TeamAssets\",\"variables\":{\"handle\":\"reddit\"},\"query\":\"query TeamAssets($handle: String!) {\\n  me {\\n    id\\n    membership(team_handle: $handle) {\\n      id\\n      permissions\\n      __typename\\n    }\\n    __typename\\n  }\\n  team(handle: $handle) {\\n    id\\n    handle\\n    structured_scope_versions(archived: false) {\\n      max_updated_at\\n      __typename\\n    }\\n    in_scope_assets: structured_scopes(\\n      first: 650\\n      archived: false\\n      eligible_for_submission: true\\n    ) {\\n      edges {\\n        node {\\n          id\\n          asset_type\\n          asset_identifier\\n          instruction\\n          max_severity\\n          eligible_for_bounty\\n          labels(first: 100) {\\n            edges {\\n              node {\\n                id\\n                name\\n                __typename\\n              }\\n              __typename\\n            }\\n            __typename\\n          }\\n          __typename\\n        }\\n        __typename\\n      }\\n      __typename\\n    }\\n    out_scope_assets: structured_scopes(\\n      first: 650\\n      archived: false\\n      eligible_for_submission: false\\n    ) {\\n      edges {\\n        node {\\n          id\\n          asset_type\\n          asset_identifier\\n          instruction\\n          __typename\\n        }\\n        __typename\\n      }\\n      __typename\\n    }\\n    __typename\\n  }\\n}\\n\"}")
        .send()
        .await?
        .text()
        .await?;
    
    let json: serde_json::Value = serde_json::from_str(&res).expect("error");

    println!("{:?}", json["data"]);

    Ok(())
}
