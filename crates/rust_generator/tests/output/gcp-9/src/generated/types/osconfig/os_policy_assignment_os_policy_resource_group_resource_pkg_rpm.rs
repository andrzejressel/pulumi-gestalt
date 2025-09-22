#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct OsPolicyAssignmentOsPolicyResourceGroupResourcePkgRpm {
    /// Whether dependencies should also be installed. -
    /// install when false: `rpm --upgrade --replacepkgs package.rpm` - install when
    /// true: `yum -y install package.rpm` or `zypper -y install package.rpm`
    #[builder(into)]
    #[serde(rename = "pullDeps")]
    pub r#pull_deps: Option<bool>,
    /// An rpm package. Structure is
    /// documented below.
    #[builder(into)]
    #[serde(rename = "source")]
    pub r#source: Box<super::super::types::osconfig::OsPolicyAssignmentOsPolicyResourceGroupResourcePkgRpmSource>,
}
