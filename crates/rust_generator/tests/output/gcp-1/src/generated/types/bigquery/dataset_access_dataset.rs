#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue, pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct DatasetAccessDataset {
    /// The dataset this entry applies to
    /// Structure is documented below.
    #[builder(into)]
    #[serde(rename = "dataset")]
    pub r#dataset: Box<super::super::types::bigquery::DatasetAccessDatasetDataset>,
    /// Which resources in the dataset this entry applies to. Currently, only views are supported,
    /// but additional target types may be added in the future. Possible values: VIEWS
    #[builder(into)]
    #[serde(rename = "targetTypes")]
    pub r#target_types: Vec<String>,
}
