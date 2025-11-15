#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct AccountSharePropertiesRetentionPolicy {
    /// Specifies the number of days that the `azure.storage.Share` should be retained, between `1` and `365` days. Defaults to `7`.
    #[builder(into)]
    #[serde(rename = "days")]
    pub r#days: Option<i32>,
}
