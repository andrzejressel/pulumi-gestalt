#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue, pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct BlockchainNodesEthereumDetailsValidatorConfig {
    /// URLs for MEV-relay services to use for block building. When set, a managed MEV-boost service is configured on the beacon client.
    #[builder(into)]
    #[serde(rename = "mevRelayUrls")]
    pub r#mev_relay_urls: Option<Vec<String>>,
}
