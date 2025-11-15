#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct OsPolicyAssignmentInstanceFilter {
    /// Target all VMs in the project. If true, no other criteria
    /// is permitted.
    #[builder(into)]
    #[serde(rename = "all")]
    pub r#all: Option<bool>,
    /// List of label sets used for VM exclusion. If
    /// the list has more than one label set, the VM is excluded if any of the label
    /// sets are applicable for the VM. Structure is
    /// documented below.
    #[builder(into)]
    #[serde(rename = "exclusionLabels")]
    pub r#exclusion_labels: Option<Vec<super::super::types::osconfig::OsPolicyAssignmentInstanceFilterExclusionLabel>>,
    /// List of label sets used for VM inclusion. If
    /// the list has more than one `LabelSet`, the VM is included if any of the
    /// label sets are applicable for the VM. Structure is
    /// documented below.
    #[builder(into)]
    #[serde(rename = "inclusionLabels")]
    pub r#inclusion_labels: Option<Vec<super::super::types::osconfig::OsPolicyAssignmentInstanceFilterInclusionLabel>>,
    /// List of inventories to select VMs. A VM is
    /// selected if its inventory data matches at least one of the following
    /// inventories. Structure is documented below.
    #[builder(into)]
    #[serde(rename = "inventories")]
    pub r#inventories: Option<Vec<super::super::types::osconfig::OsPolicyAssignmentInstanceFilterInventory>>,
}
