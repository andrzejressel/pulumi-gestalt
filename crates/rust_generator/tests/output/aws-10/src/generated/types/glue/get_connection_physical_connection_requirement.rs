#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct GetConnectionPhysicalConnectionRequirement {
    #[builder(into)]
    #[serde(rename = "availabilityZone")]
    pub r#availability_zone: String,
    #[builder(into)]
    #[serde(rename = "securityGroupIdLists")]
    pub r#security_group_id_lists: Vec<String>,
    #[builder(into)]
    #[serde(rename = "subnetId")]
    pub r#subnet_id: String,
}
