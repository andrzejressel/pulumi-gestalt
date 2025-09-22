#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct SoftwareUpdateConfigurationWindows {
    /// Specifies the list of update classification. Possible values are `Unclassified`, `Critical`, `Security`, `UpdateRollup`, `FeaturePack`, `ServicePack`, `Definition`, `Tools` and `Updates`.
    /// 
    /// > **NOTE:** The `classifications_included` property will become `Required` in version 4.0 of the Provider.
    #[builder(into)]
    #[serde(rename = "classificationsIncludeds")]
    pub r#classifications_includeds: Vec<String>,
    /// Specifies a list of knowledge base numbers excluded.
    #[builder(into)]
    #[serde(rename = "excludedKnowledgeBaseNumbers")]
    pub r#excluded_knowledge_base_numbers: Option<Vec<String>>,
    /// Specifies a list of knowledge base numbers included.
    #[builder(into)]
    #[serde(rename = "includedKnowledgeBaseNumbers")]
    pub r#included_knowledge_base_numbers: Option<Vec<String>>,
    /// Specifies the reboot settings after software update, possible values are `IfRequired`, `Never`, `RebootOnly` and `Always`. Defaults to `IfRequired`.
    #[builder(into)]
    #[serde(rename = "reboot")]
    pub r#reboot: Option<String>,
}
