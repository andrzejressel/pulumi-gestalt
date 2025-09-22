#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct PipelineJobMappingPipelineJob {
    /// If set, the mapping pipeline will write snapshots to this
    /// FHIR store without assigning stable IDs. You must
    /// grant your pipeline project's Cloud Healthcare Service
    /// Agent serviceaccount healthcare.fhirResources.executeBundle
    /// and healthcare.fhirResources.create permissions on the
    /// destination store. The destination store must set
    /// [disableReferentialIntegrity][FhirStore.disable_referential_integrity]
    /// to true. The destination store must use FHIR version R4.
    /// Format: project/{projectID}/locations/{locationID}/datasets/{datasetName}/fhirStores/{fhirStoreID}.
    #[builder(into)]
    #[serde(rename = "fhirStoreDestination")]
    pub r#fhir_store_destination: Option<String>,
    /// A streaming FHIR data source.
    /// Structure is documented below.
    #[builder(into)]
    #[serde(rename = "fhirStreamingSource")]
    pub r#fhir_streaming_source: Option<Box<super::super::types::healthcare::PipelineJobMappingPipelineJobFhirStreamingSource>>,
    /// The location of the mapping configuration.
    /// Structure is documented below.
    #[builder(into)]
    #[serde(rename = "mappingConfig")]
    pub r#mapping_config: Box<super::super::types::healthcare::PipelineJobMappingPipelineJobMappingConfig>,
    /// If set to true, a mapping pipeline will send output snapshots
    /// to the reconciliation pipeline in its dataset. A reconciliation
    /// pipeline must exist in this dataset before a mapping pipeline
    /// with a reconciliation destination can be created.
    #[builder(into)]
    #[serde(rename = "reconciliationDestination")]
    pub r#reconciliation_destination: Option<bool>,
}
