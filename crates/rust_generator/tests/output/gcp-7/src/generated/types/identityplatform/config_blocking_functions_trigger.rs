#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct ConfigBlockingFunctionsTrigger {
    /// The identifier for this object. Format specified above.
    #[builder(into)]
    #[serde(rename = "eventType")]
    pub r#event_type: Box<String>,
    /// HTTP URI trigger for the Cloud Function.
    #[builder(into)]
    #[serde(rename = "functionUri")]
    pub r#function_uri: Box<String>,
    /// (Output)
    /// When the trigger was changed.
    #[builder(into, default)]
    #[serde(rename = "updateTime")]
    pub r#update_time: Box<Option<String>>,
}
