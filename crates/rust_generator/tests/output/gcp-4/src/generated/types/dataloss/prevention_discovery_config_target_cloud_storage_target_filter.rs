#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct PreventionDiscoveryConfigTargetCloudStorageTargetFilter {
    /// The bucket to scan. Targets including this can only include one target (the target with this bucket). This enables profiling the contents of a single bucket, while the other options allow for easy profiling of many buckets within a project or an organization.
    /// Structure is documented below.
    #[builder(into)]
    #[serde(rename = "cloudStorageResourceReference")]
    pub r#cloud_storage_resource_reference: Option<Box<super::super::types::dataloss::PreventionDiscoveryConfigTargetCloudStorageTargetFilterCloudStorageResourceReference>>,
    /// A specific set of buckets for this filter to apply to.
    /// Structure is documented below.
    #[builder(into)]
    #[serde(rename = "collection")]
    pub r#collection: Option<Box<super::super::types::dataloss::PreventionDiscoveryConfigTargetCloudStorageTargetFilterCollection>>,
    /// Match discovery resources not covered by any other filter.
    #[builder(into)]
    #[serde(rename = "others")]
    pub r#others: Option<Box<super::super::types::dataloss::PreventionDiscoveryConfigTargetCloudStorageTargetFilterOthers>>,
}
