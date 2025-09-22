#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct AiDeploymentResourcePoolDedicatedResourcesMachineSpec {
    /// The number of accelerators to attach to the machine.
    #[builder(into)]
    #[serde(rename = "acceleratorCount")]
    pub r#accelerator_count: Option<i32>,
    /// The type of accelerator(s) that may be attached to the machine as per accelerator_count. See possible values [here](https://cloud.google.com/vertex-ai/docs/reference/rest/v1/MachineSpec#AcceleratorType).
    #[builder(into)]
    #[serde(rename = "acceleratorType")]
    pub r#accelerator_type: Option<String>,
    /// The type of the machine. See the [list of machine types supported for prediction](https://cloud.google.com/vertex-ai/docs/predictions/configure-compute#machine-types).
    #[builder(into)]
    #[serde(rename = "machineType")]
    pub r#machine_type: Option<String>,
}
