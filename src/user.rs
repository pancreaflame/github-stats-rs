//! For getting user information.

use serde::Deserialize;

use crate::Result;

/// Represents that stats of a [Github] user.
///
/// [Github]: https://github.com/
#[derive(Debug, Deserialize)]
pub struct User {
    login: String,
    id: u64,
    node_id: String,
    avatar_url: String,
    gravatar_id: String,
    html_url: String,
    r#type: String,
}

impl User {
    /// Creates a new `User`
    ///
    /// # Example
    ///
    /// ```
    /// use github_stats::User;
    ///
    /// let user = User::new("rust-lang");
    /// ```
    pub fn new(user: &str) -> Result<Self> {
        const URL: &str = "https://api.github.com/users";
        let url = format!("{}/{}", URL, user);
        let user: User = reqwest::get(&url)?.json()?;

        Ok(user)
    }
    pub fn login(&self) -> &str {
        &self.login
    }
    pub fn id(&self) -> u64 {
        self.id
    }
    pub fn node_id(&self) -> &str {
        &self.node_id
    }
    pub fn avatar_url(&self) -> &str {
        &self.avatar_url
    }
    pub fn gravatar_id(&self) -> &str {
        &self.gravatar_id
    }
    /// Actual link to the user's page.
    pub fn html_url(&self) -> &str {
        &self.html_url
    }
    /// *Use `r#type` to avoid conflict with `type` keyword.*
    pub fn r#type(&self) -> &str {
        &self.r#type
    }
}
