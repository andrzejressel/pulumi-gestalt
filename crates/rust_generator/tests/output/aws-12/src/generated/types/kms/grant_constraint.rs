#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct GrantConstraint {
    /// A list of key-value pairs that must match the encryption context in subsequent cryptographic operation requests. The grant allows the operation only when the encryption context in the request is the same as the encryption context specified in this constraint. Conflicts with `encryption_context_subset`.
    #[builder(into)]
    #[serde(rename = "encryptionContextEquals")]
    pub r#encryption_context_equals: Option<std::collections::HashMap<String, String>>,
    /// A list of key-value pairs that must be included in the encryption context of subsequent cryptographic operation requests. The grant allows the cryptographic operation only when the encryption context in the request includes the key-value pairs specified in this constraint, although it can include additional key-value pairs. Conflicts with `encryption_context_equals`.
    #[builder(into)]
    #[serde(rename = "encryptionContextSubset")]
    pub r#encryption_context_subset: Option<std::collections::HashMap<String, String>>,
}
