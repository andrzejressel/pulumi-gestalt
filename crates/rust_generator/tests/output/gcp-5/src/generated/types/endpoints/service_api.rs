#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct ServiceApi {
    /// A list of Method objects; structure is documented below.
    #[builder(into)]
    #[serde(rename = "methods")]
    pub r#methods: Option<Vec<super::super::types::endpoints::ServiceApiMethod>>,
    /// The simple name of the endpoint as described in the config.
    #[builder(into)]
    #[serde(rename = "name")]
    pub r#name: Option<String>,
    /// `SYNTAX_PROTO2` or `SYNTAX_PROTO3`.
    #[builder(into)]
    #[serde(rename = "syntax")]
    pub r#syntax: Option<String>,
    /// A version string for this api. If specified, will have the form major-version.minor-version, e.g. `1.10`.
    #[builder(into)]
    #[serde(rename = "version")]
    pub r#version: Option<String>,
}
