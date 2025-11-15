#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct ContainerVolume {
    /// The path in the container where the volume will be mounted.
    #[builder(into)]
    #[serde(rename = "containerPath")]
    pub r#container_path: Option<String>,
    /// The container where the volume is coming from.
    #[builder(into)]
    #[serde(rename = "fromContainer")]
    pub r#from_container: Option<String>,
    /// The path on the host where the volume is coming from.
    #[builder(into)]
    #[serde(rename = "hostPath")]
    pub r#host_path: Option<String>,
    /// If `true`, this volume will be readonly. Defaults to `false`.
    #[builder(into)]
    #[serde(rename = "readOnly")]
    pub r#read_only: Option<bool>,
    /// The name of the docker volume which should be mounted.
    #[builder(into)]
    #[serde(rename = "volumeName")]
    pub r#volume_name: Option<String>,
}
