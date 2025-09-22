#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct FeatureVariationValue {
    /// If this feature uses the Boolean variation type, this field contains the Boolean value of this variation.
    #[builder(into)]
    #[serde(rename = "boolValue")]
    pub r#bool_value: Option<String>,
    /// If this feature uses the double integer variation type, this field contains the double integer value of this variation.
    #[builder(into)]
    #[serde(rename = "doubleValue")]
    pub r#double_value: Option<String>,
    /// If this feature uses the long variation type, this field contains the long value of this variation. Minimum value of `-9007199254740991`. Maximum value of `9007199254740991`.
    #[builder(into)]
    #[serde(rename = "longValue")]
    pub r#long_value: Option<String>,
    /// If this feature uses the string variation type, this field contains the string value of this variation. Minimum length of `0`. Maximum length of `512`.
    #[builder(into)]
    #[serde(rename = "stringValue")]
    pub r#string_value: Option<String>,
}
