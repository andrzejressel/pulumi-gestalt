#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct RouterNatLogConfig {
    /// Indicates whether or not to export logs.
    #[builder(into)]
    #[serde(rename = "enable")]
    pub r#enable: bool,
    /// Specifies the desired filtering of logs on this NAT.
    /// Possible values are: `ERRORS_ONLY`, `TRANSLATIONS_ONLY`, `ALL`.
    #[builder(into)]
    #[serde(rename = "filter")]
    pub r#filter: String,
}
