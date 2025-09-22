#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct RegistrationDnsSettings {
    /// Configuration for an arbitrary DNS provider.
    /// Structure is documented below.
    #[builder(into)]
    #[serde(rename = "customDns")]
    pub r#custom_dns: Option<Box<super::super::types::clouddomains::RegistrationDnsSettingsCustomDns>>,
    /// The list of glue records for this Registration. Commonly empty.
    /// Structure is documented below.
    #[builder(into)]
    #[serde(rename = "glueRecords")]
    pub r#glue_records: Option<Vec<super::super::types::clouddomains::RegistrationDnsSettingsGlueRecord>>,
}
