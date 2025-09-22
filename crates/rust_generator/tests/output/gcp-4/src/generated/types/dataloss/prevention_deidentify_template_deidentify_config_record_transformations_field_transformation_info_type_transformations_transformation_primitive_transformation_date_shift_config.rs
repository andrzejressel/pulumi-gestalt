#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct PreventionDeidentifyTemplateDeidentifyConfigRecordTransformationsFieldTransformationInfoTypeTransformationsTransformationPrimitiveTransformationDateShiftConfig {
    /// Points to the field that contains the context, for example, an entity id.
    /// If set, must also set cryptoKey. If set, shift will be consistent for the given context.
    /// Structure is documented below.
    #[builder(into)]
    #[serde(rename = "context")]
    pub r#context: Box<Option<super::super::types::dataloss::PreventionDeidentifyTemplateDeidentifyConfigRecordTransformationsFieldTransformationInfoTypeTransformationsTransformationPrimitiveTransformationDateShiftConfigContext>>,
    /// Causes the shift to be computed based on this key and the context. This results in the same shift for the same context and cryptoKey. If set, must also set context. Can only be applied to table items.
    /// Structure is documented below.
    #[builder(into)]
    #[serde(rename = "cryptoKey")]
    pub r#crypto_key: Box<Option<super::super::types::dataloss::PreventionDeidentifyTemplateDeidentifyConfigRecordTransformationsFieldTransformationInfoTypeTransformationsTransformationPrimitiveTransformationDateShiftConfigCryptoKey>>,
    /// For example, -5 means shift date to at most 5 days back in the past.
    #[builder(into)]
    #[serde(rename = "lowerBoundDays")]
    pub r#lower_bound_days: i32,
    /// Range of shift in days. Actual shift will be selected at random within this range (inclusive ends). Negative means shift to earlier in time. Must not be more than 365250 days (1000 years) each direction.
    /// For example, 3 means shift date to at most 3 days into the future.
    #[builder(into)]
    #[serde(rename = "upperBoundDays")]
    pub r#upper_bound_days: i32,
}
