#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue, pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct LakeAssetStatus {
    /// Number of active assets.
    #[builder(into)]
    #[serde(rename = "activeAssets")]
    pub r#active_assets: Option<i32>,
    /// Number of assets that are in process of updating the security policy on attached resources.
    #[builder(into)]
    #[serde(rename = "securityPolicyApplyingAssets")]
    pub r#security_policy_applying_assets: Option<i32>,
    /// Output only. The time when the lake was last updated.
    #[builder(into)]
    #[serde(rename = "updateTime")]
    pub r#update_time: Option<String>,
}
