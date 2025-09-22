#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct HBaseClusterMetastores {
    /// An `ambari` block as defined below.
    #[builder(into)]
    #[serde(rename = "ambari")]
    pub r#ambari: Box<Option<super::super::types::hdinsight::HBaseClusterMetastoresAmbari>>,
    /// A `hive` block as defined below.
    #[builder(into)]
    #[serde(rename = "hive")]
    pub r#hive: Box<Option<super::super::types::hdinsight::HBaseClusterMetastoresHive>>,
    /// An `oozie` block as defined below.
    #[builder(into)]
    #[serde(rename = "oozie")]
    pub r#oozie: Box<Option<super::super::types::hdinsight::HBaseClusterMetastoresOozie>>,
}
