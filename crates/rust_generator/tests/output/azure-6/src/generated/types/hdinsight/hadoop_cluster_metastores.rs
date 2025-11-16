#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct HadoopClusterMetastores {
    /// An `ambari` block as defined below.
    #[builder(into)]
    #[serde(rename = "ambari")]
    pub r#ambari: Option<Box<super::super::types::hdinsight::HadoopClusterMetastoresAmbari>>,
    /// A `hive` block as defined below.
    #[builder(into)]
    #[serde(rename = "hive")]
    pub r#hive: Option<Box<super::super::types::hdinsight::HadoopClusterMetastoresHive>>,
    /// An `oozie` block as defined below.
    #[builder(into)]
    #[serde(rename = "oozie")]
    pub r#oozie: Option<Box<super::super::types::hdinsight::HadoopClusterMetastoresOozie>>,
}
