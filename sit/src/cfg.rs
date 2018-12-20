//! Client configuration
#[cfg(feature = "git")]
use std::path::PathBuf;
use serde_derive::{Serialize, Deserialize};

#[derive(Clone, Serialize, Deserialize)]
pub struct Author {
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
}

use std::fmt::Display;
impl Display for Author {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter) -> Result<(), ::std::fmt::Error> {
        write!(fmt, "{}", self.name)?;
        match self.email {
            Some(ref email) => write!(fmt, " <{}>", email),
            None => Ok(())
        }
    }
}

impl Author {
    #[cfg(feature = "git")]
    pub fn from_gitconfig(path: PathBuf) -> Option<Author> {
        use git2;
        let mut gitconfig = match git2::Config::open_default() {
            Err(_) => return None,
            Ok(config) => config,
        };
        gitconfig.add_file(&path, git2::ConfigLevel::Local, true).ok()?;
        let name = match gitconfig.get_string("user.name") {
            Ok(name) => name,
            Err(_) => return None,
        };
        let email = match gitconfig.get_string("user.email") {
            Ok(email) => Some(email),
            Err(_) => None,
        };
        Some(Author {
            name,
            email
        })
    }
}

use std::collections::HashMap;
#[derive(Default, Clone, Serialize, Deserialize)]
pub struct JMESPathConfig {
    #[serde(default, skip_serializing_if = "HashMap::is_empty")]
    pub filters: HashMap<String, String>,
    #[serde(default, skip_serializing_if = "HashMap::is_empty")]
    pub queries: HashMap<String, String>,
}

impl JMESPathConfig {
    pub fn is_empty(&self) -> bool {
        self.filters.is_empty() && self.queries.is_empty()
    }
}

#[derive(Default, Clone, Serialize, Deserialize)]
pub struct Signing {
    #[serde(default)]
    pub enabled: bool,
    #[serde(default)]
    pub key: Option<String>,
    #[serde(default)]
    pub gnupg: Option<String>,
}

impl Signing {
    pub fn is_none(&self) -> bool {
        !self.enabled && self.key.is_none() && self.gnupg.is_none()
    }
}

#[derive(Serialize, Clone, Deserialize)]
pub struct ExtensibleConfiguration<T> {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub author: Option<Author>,
    #[serde(default, skip_serializing_if = "JMESPathConfig::is_empty")]
    pub items: JMESPathConfig,
    #[serde(default, skip_serializing_if = "JMESPathConfig::is_empty")]
    pub records: JMESPathConfig,
    #[serde(default, skip_serializing_if = "Signing::is_none")]
    pub signing: Signing,
    #[serde(default, flatten)]
    pub extra: T,
}

use serde_json;
pub type Configuration = ExtensibleConfiguration<HashMap<String, serde_json::Value>>;
