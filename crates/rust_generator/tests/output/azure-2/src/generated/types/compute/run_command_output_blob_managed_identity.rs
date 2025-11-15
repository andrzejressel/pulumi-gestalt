#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct RunCommandOutputBlobManagedIdentity {
    /// The client ID of the managed identity.
    #[builder(into)]
    #[serde(rename = "clientId")]
    pub r#client_id: Option<String>,
    /// The object ID of the managed identity.
    #[builder(into)]
    #[serde(rename = "objectId")]
    pub r#object_id: Option<String>,
}
