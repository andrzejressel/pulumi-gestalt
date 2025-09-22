#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct GcPolicyMaxAge {
    /// Number of days before applying GC policy.
    #[builder(into)]
    #[serde(rename = "days")]
    pub r#days: Option<i32>,
    /// Duration before applying GC policy (ex. "8h"). This is required when `days` isn't set
    /// 
    /// -----
    #[builder(into)]
    #[serde(rename = "duration")]
    pub r#duration: Option<String>,
}
