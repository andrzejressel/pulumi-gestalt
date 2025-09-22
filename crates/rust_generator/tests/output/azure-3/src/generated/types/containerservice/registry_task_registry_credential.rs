#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct RegistryTaskRegistryCredential {
    /// One or more `custom` blocks as defined above.
    #[builder(into)]
    #[serde(rename = "customs")]
    pub r#customs: Option<Vec<super::super::types::containerservice::RegistryTaskRegistryCredentialCustom>>,
    /// One `source` block as defined below.
    #[builder(into)]
    #[serde(rename = "source")]
    pub r#source: Box<Option<super::super::types::containerservice::RegistryTaskRegistryCredentialSource>>,
}
