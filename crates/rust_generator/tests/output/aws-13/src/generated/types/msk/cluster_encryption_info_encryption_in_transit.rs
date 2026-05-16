#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue, pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct ClusterEncryptionInfoEncryptionInTransit {
    /// Encryption setting for data in transit between clients and brokers. Valid values: `TLS`, `TLS_PLAINTEXT`, and `PLAINTEXT`. Default value is `TLS`.
    #[builder(into)]
    #[serde(rename = "clientBroker")]
    pub r#client_broker: Option<String>,
    /// Whether data communication among broker nodes is encrypted. Default value: `true`.
    #[builder(into)]
    #[serde(rename = "inCluster")]
    pub r#in_cluster: Option<bool>,
}
