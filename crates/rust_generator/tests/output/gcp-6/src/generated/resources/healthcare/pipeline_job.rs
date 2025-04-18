/// PipelineJobs are Long Running Operations on Healthcare API to Map or Reconcile
/// incoming data into FHIR format
///
///
/// To get more information about PipelineJob, see:
///
/// * [API documentation](https://cloud.google.com/healthcare-api/healthcare-data-engine/docs/reference/rest/v1/projects.locations.datasets.pipelineJobs)
/// * How-to Guides
///     * [Creating a PipelineJob](https://cloud.google.com/healthcare-api/private/healthcare-data-engine/docs/reference/rest/v1/projects.locations.datasets.pipelineJobs#PipelineJob)
///
/// ## Example Usage
///
/// ### Healthcare Pipeline Job Reconciliation
///
///
/// ```yaml
/// resources:
///   example-pipeline:
///     type: gcp:healthcare:PipelineJob
///     properties:
///       name: example_pipeline_job
///       location: us-central1
///       dataset: ${dataset.id}
///       disableLineage: true
///       reconciliationPipelineJob:
///         mergeConfig:
///           description: sample description for reconciliation rules
///           whistleConfigSource:
///             uri: gs://${bucket.name}/${mergeFile.name}
///             importUriPrefix: gs://${bucket.name}
///         matchingUriPrefix: gs://${bucket.name}
///         fhirStoreDestination: ${dataset.id}/fhirStores/${fhirstore.name}
///   dataset:
///     type: gcp:healthcare:Dataset
///     properties:
///       name: example_dataset
///       location: us-central1
///   fhirstore:
///     type: gcp:healthcare:FhirStore
///     properties:
///       name: fhir_store
///       dataset: ${dataset.id}
///       version: R4
///       enableUpdateCreate: true
///       disableReferentialIntegrity: true
///   bucket:
///     type: gcp:storage:Bucket
///     properties:
///       name: example_bucket_name
///       location: us-central1
///       uniformBucketLevelAccess: true
///   mergeFile:
///     type: gcp:storage:BucketObject
///     name: merge_file
///     properties:
///       name: merge.wstl
///       content: ' '
///       bucket: ${bucket.name}
///   hsa:
///     type: gcp:storage:BucketIAMMember
///     properties:
///       bucket: ${bucket.name}
///       role: roles/storage.objectUser
///       member: serviceAccount:service-${project.number}@gcp-sa-healthcare.iam.gserviceaccount.com
/// variables:
///   project:
///     fn::invoke:
///       function: gcp:organizations:getProject
///       arguments: {}
/// ```
/// ### Healthcare Pipeline Job Backfill
///
///
/// ```yaml
/// resources:
///   example-pipeline:
///     type: gcp:healthcare:PipelineJob
///     properties:
///       name: example_backfill_pipeline
///       location: us-central1
///       dataset: ${dataset.id}
///       backfillPipelineJob:
///         mappingPipelineJob: ${dataset.id}/pipelinejobs/example_mapping_pipeline
///   dataset:
///     type: gcp:healthcare:Dataset
///     properties:
///       name: example_dataset
///       location: us-central1
/// ```
/// ### Healthcare Pipeline Job Whistle Mapping
///
///
/// ```yaml
/// resources:
///   example-mapping-pipeline:
///     type: gcp:healthcare:PipelineJob
///     properties:
///       name: example_mapping_pipeline_job
///       location: us-central1
///       dataset: ${dataset.id}
///       disableLineage: true
///       labels:
///         example_label_key: example_label_value
///       mappingPipelineJob:
///         mappingConfig:
///           whistleConfigSource:
///             uri: gs://${bucket.name}/${mappingFile.name}
///             importUriPrefix: gs://${bucket.name}
///           description: example description for mapping configuration
///         fhirStreamingSource:
///           fhirStore: ${dataset.id}/fhirStores/${sourceFhirstore.name}
///           description: example description for streaming fhirstore
///         fhirStoreDestination: ${dataset.id}/fhirStores/${destFhirstore.name}
///   dataset:
///     type: gcp:healthcare:Dataset
///     properties:
///       name: example_dataset
///       location: us-central1
///   sourceFhirstore:
///     type: gcp:healthcare:FhirStore
///     name: source_fhirstore
///     properties:
///       name: source_fhir_store
///       dataset: ${dataset.id}
///       version: R4
///       enableUpdateCreate: true
///       disableReferentialIntegrity: true
///   destFhirstore:
///     type: gcp:healthcare:FhirStore
///     name: dest_fhirstore
///     properties:
///       name: dest_fhir_store
///       dataset: ${dataset.id}
///       version: R4
///       enableUpdateCreate: true
///       disableReferentialIntegrity: true
///   bucket:
///     type: gcp:storage:Bucket
///     properties:
///       name: example_bucket_name
///       location: us-central1
///       uniformBucketLevelAccess: true
///   mappingFile:
///     type: gcp:storage:BucketObject
///     name: mapping_file
///     properties:
///       name: mapping.wstl
///       content: ' '
///       bucket: ${bucket.name}
///   hsa:
///     type: gcp:storage:BucketIAMMember
///     properties:
///       bucket: ${bucket.name}
///       role: roles/storage.objectUser
///       member: serviceAccount:service-${project.number}@gcp-sa-healthcare.iam.gserviceaccount.com
/// variables:
///   project:
///     fn::invoke:
///       function: gcp:organizations:getProject
///       arguments: {}
/// ```
/// ### Healthcare Pipeline Job Mapping Recon Dest
///
///
/// ```yaml
/// resources:
///   recon:
///     type: gcp:healthcare:PipelineJob
///     properties:
///       name: example_recon_pipeline_job
///       location: us-central1
///       dataset: ${dataset.id}
///       disableLineage: true
///       reconciliationPipelineJob:
///         mergeConfig:
///           description: sample description for reconciliation rules
///           whistleConfigSource:
///             uri: gs://${bucket.name}/${mergeFile.name}
///             importUriPrefix: gs://${bucket.name}
///         matchingUriPrefix: gs://${bucket.name}
///         fhirStoreDestination: ${dataset.id}/fhirStores/${destFhirstore.name}
///   example-mapping-pipeline:
///     type: gcp:healthcare:PipelineJob
///     properties:
///       name: example_mapping_pipeline_job
///       location: us-central1
///       dataset: ${dataset.id}
///       disableLineage: true
///       labels:
///         example_label_key: example_label_value
///       mappingPipelineJob:
///         mappingConfig:
///           whistleConfigSource:
///             uri: gs://${bucket.name}/${mappingFile.name}
///             importUriPrefix: gs://${bucket.name}
///           description: example description for mapping configuration
///         fhirStreamingSource:
///           fhirStore: ${dataset.id}/fhirStores/${sourceFhirstore.name}
///           description: example description for streaming fhirstore
///         reconciliationDestination: true
///     options:
///       dependsOn:
///         - ${recon}
///   dataset:
///     type: gcp:healthcare:Dataset
///     properties:
///       name: example_dataset
///       location: us-central1
///   sourceFhirstore:
///     type: gcp:healthcare:FhirStore
///     name: source_fhirstore
///     properties:
///       name: source_fhir_store
///       dataset: ${dataset.id}
///       version: R4
///       enableUpdateCreate: true
///       disableReferentialIntegrity: true
///   destFhirstore:
///     type: gcp:healthcare:FhirStore
///     name: dest_fhirstore
///     properties:
///       name: dest_fhir_store
///       dataset: ${dataset.id}
///       version: R4
///       enableUpdateCreate: true
///       disableReferentialIntegrity: true
///   bucket:
///     type: gcp:storage:Bucket
///     properties:
///       name: example_bucket_name
///       location: us-central1
///       uniformBucketLevelAccess: true
///   mappingFile:
///     type: gcp:storage:BucketObject
///     name: mapping_file
///     properties:
///       name: mapping.wstl
///       content: ' '
///       bucket: ${bucket.name}
///   mergeFile:
///     type: gcp:storage:BucketObject
///     name: merge_file
///     properties:
///       name: merge.wstl
///       content: ' '
///       bucket: ${bucket.name}
///   hsa:
///     type: gcp:storage:BucketIAMMember
///     properties:
///       bucket: ${bucket.name}
///       role: roles/storage.objectUser
///       member: serviceAccount:service-${project.number}@gcp-sa-healthcare.iam.gserviceaccount.com
/// variables:
///   project:
///     fn::invoke:
///       function: gcp:organizations:getProject
///       arguments: {}
/// ```
///
/// ## Import
///
/// PipelineJob can be imported using any of these accepted formats:
///
/// * `{{dataset}}/pipelineJobs/{{name}}`
///
/// * `{{dataset}}/pipelineJobs?pipelineJobId={{name}}`
///
/// * `{{name}}`
///
/// When using the `pulumi import` command, PipelineJob can be imported using one of the formats above. For example:
///
/// ```sh
/// $ pulumi import gcp:healthcare/pipelineJob:PipelineJob default {{dataset}}/pipelineJobs/{{name}}
/// ```
///
/// ```sh
/// $ pulumi import gcp:healthcare/pipelineJob:PipelineJob default {{dataset}}/pipelineJobs?pipelineJobId={{name}}
/// ```
///
/// ```sh
/// $ pulumi import gcp:healthcare/pipelineJob:PipelineJob default {{name}}
/// ```
///
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod pipeline_job {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct PipelineJobArgs {
        /// Specifies the backfill configuration.
        /// Structure is documented below.
        #[builder(into, default)]
        pub backfill_pipeline_job: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::healthcare::PipelineJobBackfillPipelineJob>,
        >,
        /// Healthcare Dataset under which the Pipeline Job is to run
        ///
        ///
        /// - - -
        #[builder(into)]
        pub dataset: pulumi_gestalt_rust::InputOrOutput<String>,
        /// If true, disables writing lineage for the pipeline.
        #[builder(into, default)]
        pub disable_lineage: pulumi_gestalt_rust::InputOrOutput<Option<bool>>,
        /// User-supplied key-value pairs used to organize Pipeline Jobs.
        /// Label keys must be between 1 and 63 characters long, have a UTF-8 encoding of
        /// maximum 128 bytes, and must conform to the following PCRE regular expression:
        /// [\p{Ll}\p{Lo}][\p{Ll}\p{Lo}\p{N}_-]{0,62}
        /// Label values are optional, must be between 1 and 63 characters long, have a
        /// UTF-8 encoding of maximum 128 bytes, and must conform to the following PCRE
        /// regular expression: [\p{Ll}\p{Lo}\p{N}_-]{0,63}
        /// No more than 64 labels can be associated with a given pipeline.
        /// An object containing a list of "key": value pairs.
        /// Example: { "name": "wrench", "mass": "1.3kg", "count": "3" }.
        ///
        /// **Note**: This field is non-authoritative, and will only manage the labels present in your configuration.
        /// Please refer to the field `effective_labels` for all of the labels present on the resource.
        #[builder(into, default)]
        pub labels: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// Location where the Pipeline Job is to run
        #[builder(into)]
        pub location: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Specifies mapping configuration.
        /// Structure is documented below.
        #[builder(into, default)]
        pub mapping_pipeline_job: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::healthcare::PipelineJobMappingPipelineJob>,
        >,
        /// Specifies the name of the pipeline job. This field is user-assigned.
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Specifies reconciliation configuration.
        /// Structure is documented below.
        #[builder(into, default)]
        pub reconciliation_pipeline_job: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::healthcare::PipelineJobReconciliationPipelineJob>,
        >,
    }
    #[allow(dead_code)]
    pub struct PipelineJobResult {
        /// Specifies the backfill configuration.
        /// Structure is documented below.
        pub backfill_pipeline_job: pulumi_gestalt_rust::Output<
            Option<super::super::types::healthcare::PipelineJobBackfillPipelineJob>,
        >,
        /// Healthcare Dataset under which the Pipeline Job is to run
        ///
        ///
        /// - - -
        pub dataset: pulumi_gestalt_rust::Output<String>,
        /// If true, disables writing lineage for the pipeline.
        pub disable_lineage: pulumi_gestalt_rust::Output<Option<bool>>,
        /// All of labels (key/value pairs) present on the resource in GCP, including the labels configured through Pulumi, other clients and services.
        pub effective_labels: pulumi_gestalt_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        /// User-supplied key-value pairs used to organize Pipeline Jobs.
        /// Label keys must be between 1 and 63 characters long, have a UTF-8 encoding of
        /// maximum 128 bytes, and must conform to the following PCRE regular expression:
        /// [\p{Ll}\p{Lo}][\p{Ll}\p{Lo}\p{N}_-]{0,62}
        /// Label values are optional, must be between 1 and 63 characters long, have a
        /// UTF-8 encoding of maximum 128 bytes, and must conform to the following PCRE
        /// regular expression: [\p{Ll}\p{Lo}\p{N}_-]{0,63}
        /// No more than 64 labels can be associated with a given pipeline.
        /// An object containing a list of "key": value pairs.
        /// Example: { "name": "wrench", "mass": "1.3kg", "count": "3" }.
        ///
        /// **Note**: This field is non-authoritative, and will only manage the labels present in your configuration.
        /// Please refer to the field `effective_labels` for all of the labels present on the resource.
        pub labels: pulumi_gestalt_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// Location where the Pipeline Job is to run
        pub location: pulumi_gestalt_rust::Output<String>,
        /// Specifies mapping configuration.
        /// Structure is documented below.
        pub mapping_pipeline_job: pulumi_gestalt_rust::Output<
            Option<super::super::types::healthcare::PipelineJobMappingPipelineJob>,
        >,
        /// Specifies the name of the pipeline job. This field is user-assigned.
        pub name: pulumi_gestalt_rust::Output<String>,
        /// The combination of labels configured directly on the resource
        /// and default labels configured on the provider.
        pub pulumi_labels: pulumi_gestalt_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        /// Specifies reconciliation configuration.
        /// Structure is documented below.
        pub reconciliation_pipeline_job: pulumi_gestalt_rust::Output<
            Option<super::super::types::healthcare::PipelineJobReconciliationPipelineJob>,
        >,
        /// The fully qualified name of this dataset
        pub self_link: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: PipelineJobArgs,
    ) -> PipelineJobResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let backfill_pipeline_job_binding = args
            .backfill_pipeline_job
            .get_output(context);
        let dataset_binding = args.dataset.get_output(context);
        let disable_lineage_binding = args.disable_lineage.get_output(context);
        let labels_binding = args.labels.get_output(context);
        let location_binding = args.location.get_output(context);
        let mapping_pipeline_job_binding = args.mapping_pipeline_job.get_output(context);
        let name_binding = args.name.get_output(context);
        let reconciliation_pipeline_job_binding = args
            .reconciliation_pipeline_job
            .get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "gcp:healthcare/pipelineJob:PipelineJob".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "backfillPipelineJob".into(),
                    value: &backfill_pipeline_job_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "dataset".into(),
                    value: &dataset_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "disableLineage".into(),
                    value: &disable_lineage_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "labels".into(),
                    value: &labels_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "location".into(),
                    value: &location_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "mappingPipelineJob".into(),
                    value: &mapping_pipeline_job_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "name".into(),
                    value: &name_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "reconciliationPipelineJob".into(),
                    value: &reconciliation_pipeline_job_binding.drop_type(),
                },
            ],
        };
        let o = context.register_resource(request);
        PipelineJobResult {
            backfill_pipeline_job: o.get_field("backfillPipelineJob"),
            dataset: o.get_field("dataset"),
            disable_lineage: o.get_field("disableLineage"),
            effective_labels: o.get_field("effectiveLabels"),
            labels: o.get_field("labels"),
            location: o.get_field("location"),
            mapping_pipeline_job: o.get_field("mappingPipelineJob"),
            name: o.get_field("name"),
            pulumi_labels: o.get_field("pulumiLabels"),
            reconciliation_pipeline_job: o.get_field("reconciliationPipelineJob"),
            self_link: o.get_field("selfLink"),
        }
    }
}
