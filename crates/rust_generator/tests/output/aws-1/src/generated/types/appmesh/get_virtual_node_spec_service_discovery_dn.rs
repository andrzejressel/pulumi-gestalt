#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct GetVirtualNodeSpecServiceDiscoveryDn {
    #[builder(into)]
    #[serde(rename = "hostname")]
    pub r#hostname: String,
    #[builder(into)]
    #[serde(rename = "ipPreference")]
    pub r#ip_preference: String,
    #[builder(into)]
    #[serde(rename = "responseType")]
    pub r#response_type: String,
}
