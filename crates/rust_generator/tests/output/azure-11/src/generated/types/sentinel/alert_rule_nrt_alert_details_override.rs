#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct AlertRuleNrtAlertDetailsOverride {
    /// The format containing columns name(s) to override the description of this Sentinel Alert Rule.
    #[builder(into)]
    #[serde(rename = "descriptionFormat")]
    pub r#description_format: Option<String>,
    /// The format containing columns name(s) to override the name of this Sentinel Alert Rule.
    #[builder(into)]
    #[serde(rename = "displayNameFormat")]
    pub r#display_name_format: Option<String>,
    /// A list of `dynamic_property` blocks as defined below.
    #[builder(into)]
    #[serde(rename = "dynamicProperties")]
    pub r#dynamic_properties: Option<Vec<super::super::types::sentinel::AlertRuleNrtAlertDetailsOverrideDynamicProperty>>,
    /// The column name to take the alert severity from.
    #[builder(into)]
    #[serde(rename = "severityColumnName")]
    pub r#severity_column_name: Option<String>,
    /// The column name to take the alert tactics from.
    #[builder(into)]
    #[serde(rename = "tacticsColumnName")]
    pub r#tactics_column_name: Option<String>,
}
