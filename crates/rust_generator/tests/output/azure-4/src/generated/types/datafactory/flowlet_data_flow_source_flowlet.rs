#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct FlowletDataFlowSourceFlowlet {
    /// Specifies the reference data flow parameters from dataset.
    #[builder(into)]
    #[serde(rename = "datasetParameters")]
    pub r#dataset_parameters: Option<String>,
    /// The name for the Data Factory Flowlet.
    #[builder(into)]
    #[serde(rename = "name")]
    pub r#name: String,
    /// A map of parameters to associate with the Data Factory Flowlet.
    #[builder(into)]
    #[serde(rename = "parameters")]
    pub r#parameters: Option<std::collections::HashMap<String, String>>,
}
