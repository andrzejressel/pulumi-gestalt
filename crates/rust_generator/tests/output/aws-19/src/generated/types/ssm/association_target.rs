#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct AssociationTarget {
    /// Either `InstanceIds` or `tag:Tag Name` to specify an EC2 tag.
    #[builder(into)]
    #[serde(rename = "key")]
    pub r#key: Box<String>,
    /// A list of instance IDs or tag values. AWS currently limits this list size to one value.
    #[builder(into)]
    #[serde(rename = "values")]
    pub r#values: Box<Vec<String>>,
}
