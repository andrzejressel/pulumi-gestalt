#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct TokenPasswordPassword2 {
    /// The expiration date of the password in RFC3339 format. If not specified, the password never expires. Changing this forces a new resource to be created.
    #[builder(into)]
    #[serde(rename = "expiry")]
    pub r#expiry: Option<String>,
    /// The value of the password (Sensitive).
    #[builder(into)]
    #[serde(rename = "value")]
    pub r#value: Option<String>,
}
