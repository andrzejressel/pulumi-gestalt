#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct FleetDomainJoinInfo {
    /// Fully qualified name of the directory (for example, corp.example.com).
    #[builder(into)]
    #[serde(rename = "directoryName")]
    pub r#directory_name: Option<String>,
    /// Distinguished name of the organizational unit for computer accounts.
    #[builder(into)]
    #[serde(rename = "organizationalUnitDistinguishedName")]
    pub r#organizational_unit_distinguished_name: Option<String>,
}
