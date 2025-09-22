#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct GetPolicyRuleAction {
    /// A `base_blob` block as documented below.
    #[builder(into)]
    #[serde(rename = "baseBlobs")]
    pub r#base_blobs: Vec<super::super::types::storage::GetPolicyRuleActionBaseBlob>,
    /// A `snapshot` block as documented below.
    #[builder(into)]
    #[serde(rename = "snapshots")]
    pub r#snapshots: Vec<super::super::types::storage::GetPolicyRuleActionSnapshot>,
    /// A `version` block as documented below.
    #[builder(into)]
    #[serde(rename = "versions")]
    pub r#versions: Vec<super::super::types::storage::GetPolicyRuleActionVersion>,
}
