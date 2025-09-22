#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct PreventionDeidentifyTemplateDeidentifyConfigInfoTypeTransformationsTransformationPrimitiveTransformationFixedSizeBucketingConfigUpperBound {
    /// A float value.
    #[builder(into)]
    #[serde(rename = "floatValue")]
    pub r#float_value: Option<f64>,
    /// An integer value (int64 format)
    #[builder(into)]
    #[serde(rename = "integerValue")]
    pub r#integer_value: Option<String>,
}
