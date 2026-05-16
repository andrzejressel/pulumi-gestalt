#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue, pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct ConfigBlockingFunctionsTrigger {
    /// The identifier for this object. Format specified above.
    #[builder(into)]
    #[serde(rename = "eventType")]
    pub r#event_type: String,
    /// HTTP URI trigger for the Cloud Function.
    #[builder(into)]
    #[serde(rename = "functionUri")]
    pub r#function_uri: String,
    /// (Output)
    /// When the trigger was changed.
    #[builder(into)]
    #[serde(rename = "updateTime")]
    pub r#update_time: Option<String>,
}
