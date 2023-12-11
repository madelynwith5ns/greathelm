#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct NamespacedIdentifier {
    /**
     * Namespaces are expected to be in the form of a
     * reverse domain name. Like so: io.github.madelynwith5ns.greathelm
     */
    pub namespace: String,
    pub identifier: String,
}

impl NamespacedIdentifier {
    #![allow(dead_code)]
    pub fn as_text(&self) -> String {
        return format!("{}:{}", self.namespace, self.identifier);
    }
    pub fn parse_text(text: &String) -> Self {
        if text.contains(":") {
            let (namespace, identifier) = text.split_once(":").unwrap();
            return Self {
                namespace: namespace.into(),
                identifier: identifier.into(),
            };
        } else {
            return Self {
                namespace: "unnamespaced".into(),
                identifier: text.clone(),
            };
        }
    }
}
