#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct ClusterClusterProfile {
    /// The custom domain for the cluster. For more info, see [Prepare a custom domain for your cluster](https://docs.microsoft.com/azure/openshift/tutorial-create-cluster#prepare-a-custom-domain-for-your-cluster-optional). Changing this forces a new resource to be created.
    #[builder(into)]
    #[serde(rename = "domain")]
    pub r#domain: String,
    /// Whether Federal Information Processing Standard (FIPS) validated cryptographic modules are used. Defaults to `false`. Changing this forces a new resource to be created.
    #[builder(into)]
    #[serde(rename = "fipsEnabled")]
    pub r#fips_enabled: Option<bool>,
    /// The name of a Resource Group which will be created to host VMs of Azure Red Hat OpenShift Cluster. The value cannot contain uppercase characters. Defaults to `aro-{domain}`. Changing this forces a new resource to be created.
    #[builder(into)]
    #[serde(rename = "managedResourceGroupName")]
    pub r#managed_resource_group_name: Option<String>,
    /// The Red Hat pull secret for the cluster. For more info, see [Get a Red Hat pull secret](https://learn.microsoft.com/azure/openshift/tutorial-create-cluster#get-a-red-hat-pull-secret-optional). Changing this forces a new resource to be created.
    #[builder(into)]
    #[serde(rename = "pullSecret")]
    pub r#pull_secret: Option<String>,
    /// The resource group that the cluster profile is attached to.
    #[builder(into)]
    #[serde(rename = "resourceGroupId")]
    pub r#resource_group_id: Option<String>,
    /// The version of the OpenShift cluster. Available versions can be found with the Azure CLI command `az aro get-versions --location <region>`. Changing this forces a new resource to be created.
    #[builder(into)]
    #[serde(rename = "version")]
    pub r#version: String,
}
