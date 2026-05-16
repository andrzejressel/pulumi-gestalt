#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue, pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct PolicyAdmissionWhitelistPattern {
    /// An image name pattern to whitelist, in the form
    /// `registry/path/to/image`. This supports a trailing * as a
    /// wildcard, but this is allowed only in text after the registry/
    /// part.
    #[builder(into)]
    #[serde(rename = "namePattern")]
    pub r#name_pattern: String,
}
