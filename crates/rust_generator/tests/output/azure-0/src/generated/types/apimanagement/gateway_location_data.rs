#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct GatewayLocationData {
    /// The city or locality where the resource is located.
    #[builder(into)]
    #[serde(rename = "city")]
    pub r#city: Option<String>,
    /// The district, state, or province where the resource is located.
    #[builder(into)]
    #[serde(rename = "district")]
    pub r#district: Option<String>,
    /// A canonical name for the geographic or physical location.
    #[builder(into)]
    #[serde(rename = "name")]
    pub r#name: String,
    /// The country or region where the resource is located.
    #[builder(into)]
    #[serde(rename = "region")]
    pub r#region: Option<String>,
}
