#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct ConnectionEventingConfigRegistrationDestinationConfig {
    /// destinations for the connection
    /// Structure is documented below.
    #[builder(into)]
    #[serde(rename = "destinations")]
    pub r#destinations: Option<Vec<super::super::types::integrationconnectors::ConnectionEventingConfigRegistrationDestinationConfigDestination>>,
    /// Key for the connection
    #[builder(into)]
    #[serde(rename = "key")]
    pub r#key: Option<String>,
}
