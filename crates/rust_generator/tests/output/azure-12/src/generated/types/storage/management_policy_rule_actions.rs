#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct ManagementPolicyRuleActions {
    /// A `base_blob` block as documented below.
    #[builder(into)]
    #[serde(rename = "baseBlob")]
    pub r#base_blob: Option<Box<super::super::types::storage::ManagementPolicyRuleActionsBaseBlob>>,
    /// A `snapshot` block as documented below.
    #[builder(into)]
    #[serde(rename = "snapshot")]
    pub r#snapshot: Option<Box<super::super::types::storage::ManagementPolicyRuleActionsSnapshot>>,
    /// A `version` block as documented below.
    #[builder(into)]
    #[serde(rename = "version")]
    pub r#version: Option<Box<super::super::types::storage::ManagementPolicyRuleActionsVersion>>,
}
