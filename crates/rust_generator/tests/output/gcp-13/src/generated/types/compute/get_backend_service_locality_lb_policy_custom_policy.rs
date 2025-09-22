#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct GetBackendServiceLocalityLbPolicyCustomPolicy {
    /// An optional, arbitrary JSON object with configuration data, understood
    /// by a locally installed custom policy implementation.
    #[builder(into)]
    #[serde(rename = "data")]
    pub r#data: String,
    /// The name of the Backend Service.
    /// 
    /// - - -
    #[builder(into)]
    #[serde(rename = "name")]
    pub r#name: String,
}
