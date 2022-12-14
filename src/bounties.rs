use reqwest::header;
use serde_json;

#[tokio::main]

pub async fn get_bounties(search: String, cookie: String, csrf: String) -> Result<[String; 4], Box<dyn std::error::Error>> {
    // Set headermap
    let mut headers = header::HeaderMap::new();
    headers.insert("x-csrf-token", csrf.parse().unwrap());
    headers.insert("Content-Type", "application/json".parse().unwrap());
    headers.insert(header::COOKIE, cookie.parse().unwrap());

    // Set query to get bouties
    let query = "{\"query\":\"query TeamProfilePage($handle: String!) {\\n  team(handle: $handle) {\\n    id\\n    handle\\n    state\\n    url\\n    external_program {\\n      id\\n      __typename\\n    }\\n    ...TeamProfileLayoutTeam\\n    ...PopularNowTeam\\n    __typename\\n  }\\n  me {\\n    id\\n    __typename\\n  }\\n}\\n\\nfragment TeamProfileLayoutTeam on Team {\\n  id\\n  offers_bounties\\n  credentials_set_up\\n  asset_credentials_set_up\\n  i_am_a_whitelisted_reporter\\n  vpn_enabled\\n  state\\n  submission_requirements_enabled\\n  child_program_directory_enabled\\n  asset_scope_tab_enabled\\n  submission_requirements {\\n    id\\n    mfa_required_at\\n    terms_required_at\\n    __typename\\n  }\\n  child_teams {\\n    total_count\\n    edges {\\n      node {\\n        id\\n        name\\n        handle\\n        offers_bounties\\n        profile_picture(size: medium)\\n        __typename\\n      }\\n      __typename\\n    }\\n    __typename\\n  }\\n  ...BountyTableTeam\\n  ...ProgramIsSandboxedTeam\\n  ...VpnBannerTeam\\n  ...TeamProfileMetricsTeam\\n  ...TeamPolicyTeam\\n  ...TeamProfileTitleTeam\\n  __typename\\n}\\n\\nfragment BountyTableTeam on Team {\\n  id\\n  handle\\n  bounty_table {\\n    id\\n    low_label\\n    medium_label\\n    high_label\\n    critical_label\\n    description\\n    use_range\\n    bounty_table_rows(first: 100) {\\n      edges {\\n        node {\\n          id\\n          low\\n          medium\\n          high\\n          critical\\n          low_minimum\\n          medium_minimum\\n          high_minimum\\n          critical_minimum\\n          smart_rewards_start_at\\n          structured_scope {\\n            id\\n            asset_identifier\\n            __typename\\n          }\\n          updated_at\\n          __typename\\n        }\\n        __typename\\n      }\\n      __typename\\n    }\\n    updated_at\\n    __typename\\n  }\\n  __typename\\n}\\n\\nfragment ProgramIsSandboxedTeam on Team {\\n  id\\n  handle\\n  policy_setting {\\n    id\\n    has_structured_policy\\n    __typename\\n  }\\n  state\\n  __typename\\n}\\n\\nfragment VpnBannerTeam on Team {\\n  id\\n  name\\n  vpn_enabled\\n  vpn_suspended\\n  vpn_blocked_for_me\\n  __typename\\n}\\n\\nfragment TeamProfileMetricsTeam on Team {\\n  id\\n  handle\\n  i_can_view_checklist_checks\\n  ...TopResearchersTeam\\n  ...ProfileMetricsTeam\\n  ...ChecklistCheckIndicatorTeam\\n  __typename\\n}\\n\\nfragment TopResearchersTeam on Team {\\n  id\\n  handle\\n  state\\n  facebook_team\\n  ...ResearchersTeam\\n  __typename\\n}\\n\\nfragment ResearchersTeam on Team {\\n  id\\n  handle\\n  participants(first: 5, order_by: {field: reputation, direction: DESC}) {\\n    total_count\\n    edges {\\n      node {\\n        id\\n        __typename\\n      }\\n      ...TeamProfileUserItemParticipantWithReputationEdge\\n      __typename\\n    }\\n    __typename\\n  }\\n  __typename\\n}\\n\\nfragment TeamProfileUserItemParticipantWithReputationEdge on ParticipantWithReputationEdge {\\n  reputation\\n  node {\\n    id\\n    profile_picture(size: large)\\n    username\\n    url\\n    cleared\\n    __typename\\n  }\\n  __typename\\n}\\n\\nfragment ProfileMetricsTeam on Team {\\n  id\\n  handle\\n  currency\\n  offers_bounties\\n  average_bounty_lower_amount\\n  average_bounty_upper_amount\\n  top_bounty_lower_amount\\n  top_bounty_upper_amount\\n  formatted_total_bounties_paid_prefix\\n  formatted_total_bounties_paid_amount\\n  resolved_report_count\\n  formatted_bounties_paid_last_90_days\\n  reports_received_last_90_days\\n  last_report_resolved_at\\n  most_recent_sla_snapshot {\\n    id\\n    first_response_time: average_time_to_first_program_response\\n    triage_time: average_time_to_report_triage\\n    bounty_time: average_time_to_bounty_awarded\\n    resolution_time: average_time_to_report_resolved\\n    __typename\\n  }\\n  team_display_options {\\n    id\\n    show_response_efficiency_indicator\\n    show_mean_first_response_time\\n    show_mean_report_triage_time\\n    show_mean_bounty_time\\n    show_mean_resolution_time\\n    show_top_bounties\\n    show_average_bounty\\n    show_total_bounties_paid\\n    __typename\\n  }\\n  ...ResponseEfficiencyPercentageTeam\\n  ...BountyMetricTeam\\n  ...HackersThankedTeam\\n  __typename\\n}\\n\\nfragment ResponseEfficiencyPercentageTeam on Team {\\n  id\\n  response_efficiency_percentage\\n  response_efficiency_indicator\\n  team_display_options {\\n    id\\n    show_response_efficiency_indicator\\n    __typename\\n  }\\n  __typename\\n}\\n\\nfragment BountyMetricTeam on Team {\\n  id\\n  bounty_table {\\n    id\\n    __typename\\n  }\\n  offers_bounties\\n  base_bounty\\n  currency\\n  __typename\\n}\\n\\nfragment HackersThankedTeam on Team {\\n  id\\n  participants {\\n    total_count\\n    __typename\\n  }\\n  __typename\\n}\\n\\nfragment ChecklistCheckIndicatorTeam on Team {\\n  id\\n  handle\\n  currency\\n  challenge_setting {\\n    id\\n    stops_at\\n    __typename\\n  }\\n  checklist {\\n    id\\n    total_checks: checklist_checks {\\n      total_count\\n      max_award_amount\\n      min_award_amount\\n      __typename\\n    }\\n    completed_checks: checklist_checks(where: {state: {_eq: completed}}) {\\n      total_count\\n      __typename\\n    }\\n    unclaimed_checks: checklist_checks(where: {state: {_eq: not_claimed}}) {\\n      total_count\\n      __typename\\n    }\\n    __typename\\n  }\\n  __typename\\n}\\n\\nfragment TeamPolicyTeam on Team {\\n  id\\n  handle\\n  policy_setting {\\n    id\\n    policy\\n    last_policy_change_at\\n    __typename\\n  }\\n  external_program {\\n    id\\n    policy_url\\n    policy\\n    __typename\\n  }\\n  attachments {\\n    id\\n    _id\\n    file_name\\n    file_size\\n    content_type\\n    expiring_url\\n    __typename\\n  }\\n  __typename\\n}\\n\\nfragment TeamProfileTitleTeam on Team {\\n  id\\n  offers_bounties\\n  name\\n  __typename\\n}\\n\\nfragment PopularNowTeam on Team {\\n  id\\n  name\\n  state\\n  __typename\\n}\",\"variables\":{\"handle\":\"app_name_variable\"}}".replace("app_name_variable", &search);

    // Make the request
    let client = reqwest::Client::new();
    let res = client.post("https://hackerone.com/graphql")
        .headers(headers)
        .body(query)
        .send()
        .await?
        .text()
        .await?;    

    if res.contains("NOT_FOUND"){
        panic!("Request to get bounties failed check you interenet connection and your query");
    }else if res.contains("\"bounty_table\":null,") {
        return Ok([String::from("none"), "".to_string(), "".to_string(), "".to_string()]);
    }

    // Get json of the request
    let json: serde_json::Value = serde_json::from_str(&res).expect("couldn't decode response to json");

    // Get the bouties out of the json
    let bounties_json = json["data"]["team"]["bounty_table"]["bounty_table_rows"]["edges"].as_array().unwrap();

    // Format bouties into an array
    let bounties: [String; 4] = [bounties_json[0]["node"]["low"].to_string(), bounties_json[0]["node"]["medium"].to_string(), bounties_json[0]["node"]["high"].to_string(), bounties_json[0]["node"]["critical"].to_string()];

    // Return bouties
    return Ok(bounties);
}
