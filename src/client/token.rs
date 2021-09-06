use serde::Deserialize;
use std::fmt::{Display, Formatter, Result as FmtResult};

#[derive(Default)]
pub(super) struct Token {
    pub access: Option<String>,
    pub refresh: Option<String>,
}

impl Token {
    pub(super) fn update(&mut self, response: TokenResponse) {
        self.access = Some(format!("Bearer {}", response.access_token));
        self.refresh = response.refresh_token;
    }
}

pub(super) enum AuthorizationKind {
    User(Authorization),
    Client(Scope),
}

impl Default for AuthorizationKind {
    fn default() -> Self {
        Self::Client(Scope::Public)
    }
}

pub(super) struct Authorization {
    pub code: String,
    pub redirect_uri: String,
}

#[derive(Deserialize)]
pub(super) struct TokenResponse {
    pub access_token: String,
    pub expires_in: u64,
    #[serde(default)]
    pub refresh_token: Option<String>,
    pub token_type: String,
}

#[derive(Copy, Clone, Eq, PartialEq)]
#[non_exhaustive]
pub enum Scope {
    ChatWrite,
    Delegate,
    ForumWrite,
    FriendsRead,
    Identify,
    Lazer,
    Public,
}

impl Display for Scope {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        match self {
            Scope::ChatWrite => f.write_str("chat.write"),
            Scope::Delegate => f.write_str("delegate"),
            Scope::ForumWrite => f.write_str("forum.write"),
            Scope::FriendsRead => f.write_str("friends.read"),
            Scope::Identify => f.write_str("identify"),
            Scope::Lazer => f.write_str("lazer"),
            Scope::Public => f.write_str("public"),
        }
    }
}
