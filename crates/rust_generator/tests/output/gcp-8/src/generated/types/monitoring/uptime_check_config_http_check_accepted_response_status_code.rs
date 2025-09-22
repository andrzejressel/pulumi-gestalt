#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct UptimeCheckConfigHttpCheckAcceptedResponseStatusCode {
    /// A class of status codes to accept.
    /// Possible values are: `STATUS_CLASS_1XX`, `STATUS_CLASS_2XX`, `STATUS_CLASS_3XX`, `STATUS_CLASS_4XX`, `STATUS_CLASS_5XX`, `STATUS_CLASS_ANY`.
    #[builder(into)]
    #[serde(rename = "statusClass")]
    pub r#status_class: Option<String>,
    /// A status code to accept.
    #[builder(into)]
    #[serde(rename = "statusValue")]
    pub r#status_value: Option<i32>,
}
