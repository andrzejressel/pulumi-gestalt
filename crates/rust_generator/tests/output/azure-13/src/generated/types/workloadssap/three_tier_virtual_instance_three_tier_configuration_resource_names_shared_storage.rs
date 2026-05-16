#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue, pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct ThreeTierVirtualInstanceThreeTierConfigurationResourceNamesSharedStorage {
    /// The full name of the Shared Storage Account. Changing this forces a new resource to be created.
    #[builder(into)]
    #[serde(rename = "accountName")]
    pub r#account_name: Option<String>,
    /// The full name of Private Endpoint for the Shared Storage Account. Changing this forces a new resource to be created.
    #[builder(into)]
    #[serde(rename = "privateEndpointName")]
    pub r#private_endpoint_name: Option<String>,
}
