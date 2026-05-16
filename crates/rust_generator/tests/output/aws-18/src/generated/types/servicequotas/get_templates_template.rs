#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue, pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct GetTemplatesTemplate {
    /// Indicates whether the quota is global.
    #[builder(into)]
    #[serde(rename = "globalQuota")]
    pub r#global_quota: bool,
    /// Quota identifier.
    #[builder(into)]
    #[serde(rename = "quotaCode")]
    pub r#quota_code: String,
    /// Quota name.
    #[builder(into)]
    #[serde(rename = "quotaName")]
    pub r#quota_name: String,
    /// AWS Region to which the quota increases apply.
    #[builder(into)]
    #[serde(rename = "region")]
    pub r#region: String,
    /// (Required) Service identifier.
    #[builder(into)]
    #[serde(rename = "serviceCode")]
    pub r#service_code: String,
    /// Service name.
    #[builder(into)]
    #[serde(rename = "serviceName")]
    pub r#service_name: String,
    /// Unit of measurement.
    #[builder(into)]
    #[serde(rename = "unit")]
    pub r#unit: String,
    /// (Required) The new, increased value for the quota.
    #[builder(into)]
    #[serde(rename = "value")]
    pub r#value: f64,
}
