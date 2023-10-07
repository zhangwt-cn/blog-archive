use serde::{Deserialize, Serialize};


#[derive(Deserialize, Serialize)]
pub struct IssuesResponse {
    #[serde(default)]
    pub url: String,
    #[serde(default)]
    pub repository_url: String,
    #[serde(default)]
    pub labels_url: String,
    #[serde(default)]
    pub comments_url: String,
    #[serde(default)]
    pub events_url: String,
    #[serde(default)]
    pub html_url: String,
    pub id: i64,
    #[serde(default)]
    pub node_id: String,
    pub number: i64,
    #[serde(default)]
    pub title: String,
    pub user: User,
    pub labels: Vec<Label>,
    #[serde(default)]
    pub state: String,
    pub locked: bool,
    pub comments: i64,
    #[serde(default)]
    pub created_at: String,
    #[serde(default)]
    pub updated_at: String,
    pub closed_at: Option<String>,
    #[serde(default)]
    pub author_association: String,
    pub active_lock_reason: Option<String>,
    #[serde(skip)]
    pub body: String,
    pub performed_via_github_app: Option<String>,
}
#[derive(Deserialize, Serialize)]
pub struct User {
    pub login: String,
    pub id: i64,
    pub node_id: String,
    pub avatar_url: String,
    pub gravatar_id: String,
    pub url: String,
    pub html_url: String,
    pub followers_url: String,
    pub following_url: String,
    pub gists_url: String,
    pub starred_url: String,
    pub subscriptions_url: String,
    pub organizations_url: String,
    pub repos_url: String,
    pub events_url: String,
    pub received_events_url: String,
    pub r#type: String,
    pub site_admin: bool,
}

#[derive(Deserialize, Serialize)]
pub struct Label {
    pub id: i64,
    pub node_id: String,
    pub url: String,
    pub name: String,
    pub color: String,
    pub default: bool,
    pub description: Option<String>,
}