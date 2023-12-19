use std::fmt::Display;

/**
 * NamespacedIdentifiers identify some component (builder, generator, action) in the case it is
 * ambiguous.
 */
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
    /**
     * Parses a NamespacedIdentifier back from a text form.
     */
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

impl Display for NamespacedIdentifier {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}:{}", self.namespace, self.identifier)
    }
}
