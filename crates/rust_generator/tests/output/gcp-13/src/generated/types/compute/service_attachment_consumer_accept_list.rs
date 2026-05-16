#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue, pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct ServiceAttachmentConsumerAcceptList {
    /// The number of consumer forwarding rules the consumer project can
    /// create.
    #[builder(into)]
    #[serde(rename = "connectionLimit")]
    pub r#connection_limit: i32,
    /// The network that is allowed to connect to this service attachment.
    /// Only one of project_id_or_num and network_url may be set.
    #[builder(into)]
    #[serde(rename = "networkUrl")]
    pub r#network_url: Option<String>,
    /// A project that is allowed to connect to this service attachment.
    /// Only one of project_id_or_num and network_url may be set.
    #[builder(into)]
    #[serde(rename = "projectIdOrNum")]
    pub r#project_id_or_num: Option<String>,
}
