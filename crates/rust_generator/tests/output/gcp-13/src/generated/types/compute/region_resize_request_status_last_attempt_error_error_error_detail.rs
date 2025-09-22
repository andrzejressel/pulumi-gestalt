#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct RegionResizeRequestStatusLastAttemptErrorErrorErrorDetail {
    /// (Output)
    /// A nested object resource.
    /// Structure is documented below.
    #[builder(into)]
    #[serde(rename = "errorInfos")]
    pub r#error_infos: Option<Vec<super::super::types::compute::RegionResizeRequestStatusLastAttemptErrorErrorErrorDetailErrorInfo>>,
    /// (Output)
    /// A nested object resource.
    /// Structure is documented below.
    #[builder(into)]
    #[serde(rename = "helps")]
    pub r#helps: Option<Vec<super::super::types::compute::RegionResizeRequestStatusLastAttemptErrorErrorErrorDetailHelp>>,
    /// (Output)
    /// A nested object resource.
    /// Structure is documented below.
    #[builder(into)]
    #[serde(rename = "localizedMessages")]
    pub r#localized_messages: Option<Vec<super::super::types::compute::RegionResizeRequestStatusLastAttemptErrorErrorErrorDetailLocalizedMessage>>,
    /// (Output)
    /// A nested object resource.
    /// Structure is documented below.
    #[builder(into)]
    #[serde(rename = "quotaInfos")]
    pub r#quota_infos: Option<Vec<super::super::types::compute::RegionResizeRequestStatusLastAttemptErrorErrorErrorDetailQuotaInfo>>,
}
