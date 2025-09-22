#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct VirtualMachineGroupWsfcDomainProfile {
    /// The account name used for creating cluster. Changing this forces a new resource to be created.
    #[builder(into)]
    #[serde(rename = "clusterBootstrapAccountName")]
    pub r#cluster_bootstrap_account_name: Option<String>,
    /// The account name used for operating cluster. Changing this forces a new resource to be created.
    #[builder(into)]
    #[serde(rename = "clusterOperatorAccountName")]
    pub r#cluster_operator_account_name: Option<String>,
    /// The subnet type of the SQL Virtual Machine cluster. Possible values are `MultiSubnet` and `SingleSubnet`. Changing this forces a new resource to be created.
    #[builder(into)]
    #[serde(rename = "clusterSubnetType")]
    pub r#cluster_subnet_type: String,
    /// The fully qualified name of the domain. Changing this forces a new resource to be created.
    #[builder(into)]
    #[serde(rename = "fqdn")]
    pub r#fqdn: String,
    /// The organizational Unit path in which the nodes and cluster will be present. Changing this forces a new resource to be created.
    #[builder(into)]
    #[serde(rename = "organizationalUnitPath")]
    pub r#organizational_unit_path: Option<String>,
    /// The account name under which SQL service will run on all participating SQL virtual machines in the cluster. Changing this forces a new resource to be created.
    #[builder(into)]
    #[serde(rename = "sqlServiceAccountName")]
    pub r#sql_service_account_name: Option<String>,
    /// The primary key of the Storage Account.
    #[builder(into)]
    #[serde(rename = "storageAccountPrimaryKey")]
    pub r#storage_account_primary_key: Option<String>,
    /// The SAS URL to the Storage Container of the witness storage account. Changing this forces a new resource to be created.
    #[builder(into)]
    #[serde(rename = "storageAccountUrl")]
    pub r#storage_account_url: Option<String>,
}
