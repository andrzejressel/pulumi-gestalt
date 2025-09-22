#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct VMwareNodePoolConfigTaint {
    /// Available taint effects.
    /// Possible values are: `EFFECT_UNSPECIFIED`, `NO_SCHEDULE`, `PREFER_NO_SCHEDULE`, `NO_EXECUTE`.
    #[builder(into)]
    #[serde(rename = "effect")]
    pub r#effect: Option<String>,
    /// Key associated with the effect.
    #[builder(into)]
    #[serde(rename = "key")]
    pub r#key: String,
    /// Value associated with the effect.
    #[builder(into)]
    #[serde(rename = "value")]
    pub r#value: String,
}
