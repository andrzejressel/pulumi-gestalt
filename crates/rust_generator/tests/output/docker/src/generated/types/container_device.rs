#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct ContainerDevice {
    /// The path in the container where the device will be bound.
    #[builder(into)]
    #[serde(rename = "containerPath")]
    pub r#container_path: Option<String>,
    /// The path on the host where the device is located.
    #[builder(into)]
    #[serde(rename = "hostPath")]
    pub r#host_path: String,
    /// The cgroup permissions given to the container to access the device. Defaults to `rwm`.
    #[builder(into)]
    #[serde(rename = "permissions")]
    pub r#permissions: Option<String>,
}
