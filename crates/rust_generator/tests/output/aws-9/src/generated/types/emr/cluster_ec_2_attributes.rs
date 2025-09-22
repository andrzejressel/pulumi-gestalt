#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct ClusterEc2Attributes {
    /// String containing a comma separated list of additional Amazon EC2 security group IDs for the master node.
    #[builder(into)]
    #[serde(rename = "additionalMasterSecurityGroups")]
    pub r#additional_master_security_groups: Option<String>,
    /// String containing a comma separated list of additional Amazon EC2 security group IDs for the slave nodes as a comma separated string.
    #[builder(into)]
    #[serde(rename = "additionalSlaveSecurityGroups")]
    pub r#additional_slave_security_groups: Option<String>,
    /// Identifier of the Amazon EC2 EMR-Managed security group for the master node.
    #[builder(into)]
    #[serde(rename = "emrManagedMasterSecurityGroup")]
    pub r#emr_managed_master_security_group: Option<String>,
    /// Identifier of the Amazon EC2 EMR-Managed security group for the slave nodes.
    #[builder(into)]
    #[serde(rename = "emrManagedSlaveSecurityGroup")]
    pub r#emr_managed_slave_security_group: Option<String>,
    /// Instance Profile for EC2 instances of the cluster assume this role.
    #[builder(into)]
    #[serde(rename = "instanceProfile")]
    pub r#instance_profile: String,
    /// Amazon EC2 key pair that can be used to ssh to the master node as the user called `hadoop`.
    #[builder(into)]
    #[serde(rename = "keyName")]
    pub r#key_name: Option<String>,
    /// Identifier of the Amazon EC2 service-access security group - required when the cluster runs on a private subnet.
    #[builder(into)]
    #[serde(rename = "serviceAccessSecurityGroup")]
    pub r#service_access_security_group: Option<String>,
    /// VPC subnet id where you want the job flow to launch. Cannot specify the `cc1.4xlarge` instance type for nodes of a job flow launched in an Amazon VPC.
    #[builder(into)]
    #[serde(rename = "subnetId")]
    pub r#subnet_id: Option<String>,
    /// List of VPC subnet id-s where you want the job flow to launch.  Amazon EMR identifies the best Availability Zone to launch instances according to your fleet specifications.
    /// 
    /// > **NOTE on EMR-Managed security groups:** These security groups will have any missing inbound or outbound access rules added and maintained by AWS, to ensure proper communication between instances in a cluster. The EMR service will maintain these rules for groups provided in `emr_managed_master_security_group` and `emr_managed_slave_security_group`; attempts to remove the required rules may succeed, only for the EMR service to re-add them in a matter of minutes. This may cause this provider to fail to destroy an environment that contains an EMR cluster, because the EMR service does not revoke rules added on deletion, leaving a cyclic dependency between the security groups that prevents their deletion. To avoid this, use the `revoke_rules_on_delete` optional attribute for any Security Group used in `emr_managed_master_security_group` and `emr_managed_slave_security_group`. See [Amazon EMR-Managed Security Groups](http://docs.aws.amazon.com/emr/latest/ManagementGuide/emr-man-sec-groups.html) for more information about the EMR-managed security group rules.
    #[builder(into)]
    #[serde(rename = "subnetIds")]
    pub r#subnet_ids: Option<Vec<String>>,
}
