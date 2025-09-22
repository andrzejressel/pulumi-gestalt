#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct AccessApplicationSaasAppRefreshTokenOption {
    /// How long a refresh token will be valid for after creation. Valid units are `m`, `h` and `d`. Must be longer than 1m.
    #[builder(into)]
    #[serde(rename = "lifetime")]
    pub r#lifetime: Option<String>,
}
