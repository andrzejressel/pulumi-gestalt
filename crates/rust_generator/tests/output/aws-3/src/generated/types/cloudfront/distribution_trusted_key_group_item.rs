#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct DistributionTrustedKeyGroupItem {
    /// ID of the key group that contains the public keys.
    #[builder(into)]
    #[serde(rename = "keyGroupId")]
    pub r#key_group_id: Option<String>,
    /// Set of active CloudFront key pairs associated with the signer account
    #[builder(into)]
    #[serde(rename = "keyPairIds")]
    pub r#key_pair_ids: Option<Vec<String>>,
}
