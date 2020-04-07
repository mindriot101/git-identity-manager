use std::path::PathBuf;

#[derive(Default, Debug, Clone, PartialEq)]
pub struct Identity {
    pub id: String,
    pub name: String,
    pub email: String,
    pub signing_key: Option<String>,
    pub ssh_key: Option<PathBuf>,
}

impl Identity {
    pub(crate) fn iter(&self) -> IdentityIterator {
        let mut items = vec![
            ("user.name".to_string(), self.name.clone()),
            ("user.email".to_string(), self.email.clone()),
        ];

        if let Some(key) = &self.signing_key {
            items.push(("user.signingkey".to_string(), key.clone()));
        }

        if let Some(key) = &self.ssh_key {
            let key_str = key.as_path().to_str().unwrap();
            items.push(("user.sshkey".to_string(), key_str.to_string()));
        }

        IdentityIterator { idx: 0, items }
    }
}

pub(crate) struct IdentityIterator {
    idx: usize,
    items: Vec<(String, String)>,
}

impl Iterator for IdentityIterator {
    type Item = (String, String);

    fn next(&mut self) -> Option<Self::Item> {
        if self.idx >= self.items.len() {
            return None;
        }

        let i = self.items[self.idx].clone();
        self.idx += 1;
        Some(i)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_identity_iterator() {
        let identity = Identity {
            id: "foobar.personal".to_string(),
            name: "A User".to_string(),
            email: "user@example.com".to_string(),
            signing_key: Some("abc".to_string()),
            ssh_key: Some(PathBuf::from("/a/b/c/id_rsa")),
        };

        let keys = identity.iter().collect::<Vec<_>>();
        assert_eq!(
            keys,
            vec![
                ("user.name".to_string(), "A User".to_string()),
                ("user.email".to_string(), "user@example.com".to_string()),
                ("user.signingkey".to_string(), "abc".to_string()),
                ("user.sshkey".to_string(), "/a/b/c/id_rsa".to_string()),
            ]
        );
    }
}
