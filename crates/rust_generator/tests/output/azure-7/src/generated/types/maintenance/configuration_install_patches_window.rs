#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct ConfigurationInstallPatchesWindow {
    /// List of Classification category of patches to be patched. Possible values are `Critical`, `Security`, `UpdateRollup`, `FeaturePack`, `ServicePack`, `Definition`, `Tools` and `Updates`.
    #[builder(into)]
    #[serde(rename = "classificationsToIncludes")]
    pub r#classifications_to_includes: Option<Vec<String>>,
    /// List of KB numbers to be excluded from patching.
    #[builder(into)]
    #[serde(rename = "kbNumbersToExcludes")]
    pub r#kb_numbers_to_excludes: Option<Vec<String>>,
    /// List of KB numbers to be included for patching.
    #[builder(into)]
    #[serde(rename = "kbNumbersToIncludes")]
    pub r#kb_numbers_to_includes: Option<Vec<String>>,
}
