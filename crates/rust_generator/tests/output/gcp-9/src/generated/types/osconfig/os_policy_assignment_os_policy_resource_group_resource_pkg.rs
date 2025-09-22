#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct OsPolicyAssignmentOsPolicyResourceGroupResourcePkg {
    /// A package managed by Apt. Structure is
    /// documented below.
    #[builder(into)]
    #[serde(rename = "apt")]
    pub r#apt: Option<Box<super::super::types::osconfig::OsPolicyAssignmentOsPolicyResourceGroupResourcePkgApt>>,
    /// A deb package file. Structure is
    /// documented below.
    #[builder(into)]
    #[serde(rename = "deb")]
    pub r#deb: Option<Box<super::super::types::osconfig::OsPolicyAssignmentOsPolicyResourceGroupResourcePkgDeb>>,
    /// The desired state the agent should maintain for
    /// this package. Possible values are: `DESIRED_STATE_UNSPECIFIED`, `INSTALLED`,
    /// `REMOVED`.
    #[builder(into)]
    #[serde(rename = "desiredState")]
    pub r#desired_state: String,
    /// A package managed by GooGet. Structure is
    /// documented below.
    #[builder(into)]
    #[serde(rename = "googet")]
    pub r#googet: Option<Box<super::super::types::osconfig::OsPolicyAssignmentOsPolicyResourceGroupResourcePkgGooget>>,
    /// An MSI package. Structure is
    /// documented below.
    #[builder(into)]
    #[serde(rename = "msi")]
    pub r#msi: Option<Box<super::super::types::osconfig::OsPolicyAssignmentOsPolicyResourceGroupResourcePkgMsi>>,
    /// An rpm package file. Structure is
    /// documented below.
    #[builder(into)]
    #[serde(rename = "rpm")]
    pub r#rpm: Option<Box<super::super::types::osconfig::OsPolicyAssignmentOsPolicyResourceGroupResourcePkgRpm>>,
    /// A package managed by YUM. Structure is
    /// documented below.
    #[builder(into)]
    #[serde(rename = "yum")]
    pub r#yum: Option<Box<super::super::types::osconfig::OsPolicyAssignmentOsPolicyResourceGroupResourcePkgYum>>,
    /// A package managed by Zypper. Structure is
    /// documented below.
    #[builder(into)]
    #[serde(rename = "zypper")]
    pub r#zypper: Option<Box<super::super::types::osconfig::OsPolicyAssignmentOsPolicyResourceGroupResourcePkgZypper>>,
}
