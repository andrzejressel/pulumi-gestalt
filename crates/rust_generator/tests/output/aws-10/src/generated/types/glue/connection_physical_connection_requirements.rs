#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct ConnectionPhysicalConnectionRequirements {
    /// The availability zone of the connection. This field is redundant and implied by `subnet_id`, but is currently an api requirement.
    #[builder(into)]
    #[serde(rename = "availabilityZone")]
    pub r#availability_zone: Option<String>,
    /// The security group ID list used by the connection.
    #[builder(into)]
    #[serde(rename = "securityGroupIdLists")]
    pub r#security_group_id_lists: Option<Vec<String>>,
    /// The subnet ID used by the connection.
    #[builder(into)]
    #[serde(rename = "subnetId")]
    pub r#subnet_id: Option<String>,
}
