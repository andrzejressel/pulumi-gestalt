#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct AuthomationRuleActionIncident {
    /// The classification of the incident, when closing it. Possible values are: `BenignPositive_SuspiciousButExpected`, `FalsePositive_InaccurateData`, `FalsePositive_IncorrectAlertLogic`, `TruePositive_SuspiciousActivity` and `Undetermined`.
    /// 
    /// > **Note:** The `classification` is required when `status` is `Closed`.
    #[builder(into)]
    #[serde(rename = "classification")]
    pub r#classification: Option<String>,
    /// The comment why the incident is to be closed.
    /// 
    /// > **Note:** The `classification_comment` is allowed to set only when `status` is `Closed`.
    #[builder(into)]
    #[serde(rename = "classificationComment")]
    pub r#classification_comment: Option<String>,
    /// Specifies a list of labels to add to the incident.
    #[builder(into)]
    #[serde(rename = "labels")]
    pub r#labels: Option<Vec<String>>,
    /// The execution order of this action.
    #[builder(into)]
    #[serde(rename = "order")]
    pub r#order: i32,
    /// The object ID of the entity this incident is assigned to.
    #[builder(into)]
    #[serde(rename = "ownerId")]
    pub r#owner_id: Option<String>,
    /// The severity to add to the incident. Possible values are `High`, `Informational`, `Low` and `Medium`.
    /// 
    /// > **Note:**: At least one of `status`, `labels`, `owner_id` and `severity` has to be set.
    #[builder(into)]
    #[serde(rename = "severity")]
    pub r#severity: Option<String>,
    /// The status to set to the incident. Possible values are: `Active`, `Closed`, `New`.
    #[builder(into)]
    #[serde(rename = "status")]
    pub r#status: Option<String>,
}
