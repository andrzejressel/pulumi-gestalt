#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct PreventionDeidentifyTemplateDeidentifyConfigRecordTransformationsFieldTransformationPrimitiveTransformationBucketingConfigBucket {
    /// Upper bound of the range, exclusive; type must match min.
    /// The `max` block must only contain one argument. See the `bucketing_config` block description for more information about choosing a data type.
    /// Structure is documented below.
    #[builder(into)]
    #[serde(rename = "max")]
    pub r#max: Option<Box<super::super::types::dataloss::PreventionDeidentifyTemplateDeidentifyConfigRecordTransformationsFieldTransformationPrimitiveTransformationBucketingConfigBucketMax>>,
    /// Lower bound of the range, inclusive. Type should be the same as max if used.
    /// The `min` block must only contain one argument. See the `bucketing_config` block description for more information about choosing a data type.
    /// Structure is documented below.
    #[builder(into)]
    #[serde(rename = "min")]
    pub r#min: Option<Box<super::super::types::dataloss::PreventionDeidentifyTemplateDeidentifyConfigRecordTransformationsFieldTransformationPrimitiveTransformationBucketingConfigBucketMin>>,
    /// Replacement value for this bucket.
    /// The `replacement_value` block must only contain one argument.
    /// Structure is documented below.
    #[builder(into)]
    #[serde(rename = "replacementValue")]
    pub r#replacement_value: Box<super::super::types::dataloss::PreventionDeidentifyTemplateDeidentifyConfigRecordTransformationsFieldTransformationPrimitiveTransformationBucketingConfigBucketReplacementValue>,
}
