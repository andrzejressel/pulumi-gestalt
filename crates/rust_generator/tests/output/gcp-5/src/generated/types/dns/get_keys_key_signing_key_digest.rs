#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct GetKeysKeySigningKeyDigest {
    /// The base-16 encoded bytes of this digest. Suitable for use in a DS resource record.
    #[builder(into)]
    #[serde(rename = "digest")]
    pub r#digest: Option<String>,
    /// Specifies the algorithm used to calculate this digest. Possible values are `sha1`, `sha256` and `sha384`
    #[builder(into)]
    #[serde(rename = "type")]
    pub r#type_: Option<String>,
}
