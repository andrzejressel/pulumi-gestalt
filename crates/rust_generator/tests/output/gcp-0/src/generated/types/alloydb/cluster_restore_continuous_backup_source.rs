#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue, pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct ClusterRestoreContinuousBackupSource {
    /// The name of the source cluster that this cluster is restored from.
    #[builder(into)]
    #[serde(rename = "cluster")]
    pub r#cluster: String,
    /// The point in time that this cluster is restored to, in RFC 3339 format.
    #[builder(into)]
    #[serde(rename = "pointInTime")]
    pub r#point_in_time: String,
}
