#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct PreventionDeidentifyTemplateDeidentifyConfigImageTransformationsTransformRedactionColor {
    /// The amount of blue in the color as a value in the interval [0, 1].
    #[builder(into)]
    #[serde(rename = "blue")]
    pub r#blue: Option<f64>,
    /// The amount of green in the color as a value in the interval [0, 1].
    #[builder(into)]
    #[serde(rename = "green")]
    pub r#green: Option<f64>,
    /// The amount of red in the color as a value in the interval [0, 1].
    #[builder(into)]
    #[serde(rename = "red")]
    pub r#red: Option<f64>,
}
