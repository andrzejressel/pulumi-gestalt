#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct NodeGroupLaunchTemplate {
    /// Identifier of the EC2 Launch Template. Conflicts with `name`.
    #[builder(into)]
    #[serde(rename = "id")]
    pub r#id: Option<String>,
    /// Name of the EC2 Launch Template. Conflicts with `id`.
    #[builder(into)]
    #[serde(rename = "name")]
    pub r#name: Option<String>,
    /// EC2 Launch Template version number. While the API accepts values like `$Default` and `$Latest`, the API will convert the value to the associated version number (e.g., `1`) on read and the provider will show a difference on next plan. Using the `default_version` or `latest_version` attribute of the `aws.ec2.LaunchTemplate` resource or data source is recommended for this argument.
    #[builder(into)]
    #[serde(rename = "version")]
    pub r#version: String,
}
