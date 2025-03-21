/// A representation of a collection of database items organized in a way that allows for approximate nearest neighbor (a.k.a ANN) algorithms search.
///
///
/// To get more information about Index, see:
///
/// * [API documentation](https://cloud.google.com/vertex-ai/docs/reference/rest/v1/projects.locations.indexes/)
///
/// ## Example Usage
///
/// ### Vertex Ai Index
///
///
/// ```yaml
/// resources:
///   bucket:
///     type: gcp:storage:Bucket
///     properties:
///       name: vertex-ai-index-test
///       location: us-central1
///       uniformBucketLevelAccess: true
///   # The sample data comes from the following link:
///   # https://cloud.google.com/vertex-ai/docs/matching-engine/filtering#specify-namespaces-tokens
///   data:
///     type: gcp:storage:BucketObject
///     properties:
///       name: contents/data.json
///       bucket: ${bucket.name}
///       content: |
///         {"id": "42", "embedding": [0.5, 1.0], "restricts": [{"namespace": "class", "allow": ["cat", "pet"]},{"namespace": "category", "allow": ["feline"]}]}
///         {"id": "43", "embedding": [0.6, 1.0], "restricts": [{"namespace": "class", "allow": ["dog", "pet"]},{"namespace": "category", "allow": ["canine"]}]}
///   index:
///     type: gcp:vertex:AiIndex
///     properties:
///       labels:
///         foo: bar
///       region: us-central1
///       displayName: test-index
///       description: index for test
///       metadata:
///         contentsDeltaUri: gs://${bucket.name}/contents
///         config:
///           dimensions: 2
///           approximateNeighborsCount: 150
///           shardSize: SHARD_SIZE_SMALL
///           distanceMeasureType: DOT_PRODUCT_DISTANCE
///           algorithmConfig:
///             treeAhConfig:
///               leafNodeEmbeddingCount: 500
///               leafNodesToSearchPercent: 7
///       indexUpdateMethod: BATCH_UPDATE
/// ```
/// ### Vertex Ai Index Streaming
///
///
/// ```yaml
/// resources:
///   bucket:
///     type: gcp:storage:Bucket
///     properties:
///       name: vertex-ai-index-test
///       location: us-central1
///       uniformBucketLevelAccess: true
///   # The sample data comes from the following link:
///   # https://cloud.google.com/vertex-ai/docs/matching-engine/filtering#specify-namespaces-tokens
///   data:
///     type: gcp:storage:BucketObject
///     properties:
///       name: contents/data.json
///       bucket: ${bucket.name}
///       content: |
///         {"id": "42", "embedding": [0.5, 1.0], "restricts": [{"namespace": "class", "allow": ["cat", "pet"]},{"namespace": "category", "allow": ["feline"]}]}
///         {"id": "43", "embedding": [0.6, 1.0], "restricts": [{"namespace": "class", "allow": ["dog", "pet"]},{"namespace": "category", "allow": ["canine"]}]}
///   index:
///     type: gcp:vertex:AiIndex
///     properties:
///       labels:
///         foo: bar
///       region: us-central1
///       displayName: test-index
///       description: index for test
///       metadata:
///         contentsDeltaUri: gs://${bucket.name}/contents
///         config:
///           dimensions: 2
///           shardSize: SHARD_SIZE_LARGE
///           distanceMeasureType: COSINE_DISTANCE
///           featureNormType: UNIT_L2_NORM
///           algorithmConfig:
///             bruteForceConfig: {}
///       indexUpdateMethod: STREAM_UPDATE
/// ```
///
/// ## Import
///
/// Index can be imported using any of these accepted formats:
///
/// * `projects/{{project}}/locations/{{region}}/indexes/{{name}}`
///
/// * `{{project}}/{{region}}/{{name}}`
///
/// * `{{region}}/{{name}}`
///
/// * `{{name}}`
///
/// When using the `pulumi import` command, Index can be imported using one of the formats above. For example:
///
/// ```sh
/// $ pulumi import gcp:vertex/aiIndex:AiIndex default projects/{{project}}/locations/{{region}}/indexes/{{name}}
/// ```
///
/// ```sh
/// $ pulumi import gcp:vertex/aiIndex:AiIndex default {{project}}/{{region}}/{{name}}
/// ```
///
/// ```sh
/// $ pulumi import gcp:vertex/aiIndex:AiIndex default {{region}}/{{name}}
/// ```
///
/// ```sh
/// $ pulumi import gcp:vertex/aiIndex:AiIndex default {{name}}
/// ```
///
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod ai_index {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct AiIndexArgs {
        /// The description of the Index.
        #[builder(into, default)]
        pub description: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The display name of the Index. The name can be up to 128 characters long and can consist of any UTF-8 characters.
        ///
        ///
        /// - - -
        #[builder(into)]
        pub display_name: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The update method to use with this Index. The value must be the followings. If not set, BATCH_UPDATE will be used by default.
        /// * BATCH_UPDATE: user can call indexes.patch with files on Cloud Storage of datapoints to update.
        /// * STREAM_UPDATE: user can call indexes.upsertDatapoints/DeleteDatapoints to update the Index and the updates will be applied in corresponding DeployedIndexes in nearly real-time.
        #[builder(into, default)]
        pub index_update_method: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The labels with user-defined metadata to organize your Indexes.
        /// **Note**: This field is non-authoritative, and will only manage the labels present in your configuration.
        /// Please refer to the field `effective_labels` for all of the labels present on the resource.
        #[builder(into, default)]
        pub labels: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// An additional information about the Index
        /// Structure is documented below.
        #[builder(into, default)]
        pub metadata: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::vertex::AiIndexMetadata>,
        >,
        /// The ID of the project in which the resource belongs.
        /// If it is not provided, the provider project is used.
        #[builder(into, default)]
        pub project: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The region of the index. eg us-central1
        #[builder(into, default)]
        pub region: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct AiIndexResult {
        /// The timestamp of when the Index was created in RFC3339 UTC "Zulu" format, with nanosecond resolution and up to nine fractional digits.
        pub create_time: pulumi_gestalt_rust::Output<String>,
        /// The pointers to DeployedIndexes created from this Index. An Index can be only deleted if all its DeployedIndexes had been undeployed first.
        /// Structure is documented below.
        pub deployed_indexes: pulumi_gestalt_rust::Output<
            Vec<super::super::types::vertex::AiIndexDeployedIndex>,
        >,
        /// The description of the Index.
        pub description: pulumi_gestalt_rust::Output<Option<String>>,
        /// The display name of the Index. The name can be up to 128 characters long and can consist of any UTF-8 characters.
        ///
        ///
        /// - - -
        pub display_name: pulumi_gestalt_rust::Output<String>,
        /// All of labels (key/value pairs) present on the resource in GCP, including the labels configured through Pulumi, other clients and services.
        pub effective_labels: pulumi_gestalt_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        /// Used to perform consistent read-modify-write updates.
        pub etag: pulumi_gestalt_rust::Output<String>,
        /// Stats of the index resource.
        /// Structure is documented below.
        pub index_stats: pulumi_gestalt_rust::Output<
            Vec<super::super::types::vertex::AiIndexIndexStat>,
        >,
        /// The update method to use with this Index. The value must be the followings. If not set, BATCH_UPDATE will be used by default.
        /// * BATCH_UPDATE: user can call indexes.patch with files on Cloud Storage of datapoints to update.
        /// * STREAM_UPDATE: user can call indexes.upsertDatapoints/DeleteDatapoints to update the Index and the updates will be applied in corresponding DeployedIndexes in nearly real-time.
        pub index_update_method: pulumi_gestalt_rust::Output<Option<String>>,
        /// The labels with user-defined metadata to organize your Indexes.
        /// **Note**: This field is non-authoritative, and will only manage the labels present in your configuration.
        /// Please refer to the field `effective_labels` for all of the labels present on the resource.
        pub labels: pulumi_gestalt_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// An additional information about the Index
        /// Structure is documented below.
        pub metadata: pulumi_gestalt_rust::Output<
            Option<super::super::types::vertex::AiIndexMetadata>,
        >,
        /// Points to a YAML file stored on Google Cloud Storage describing additional information about the Index, that is specific to it. Unset if the Index does not have any additional information.
        pub metadata_schema_uri: pulumi_gestalt_rust::Output<String>,
        /// The resource name of the Index.
        pub name: pulumi_gestalt_rust::Output<String>,
        /// The ID of the project in which the resource belongs.
        /// If it is not provided, the provider project is used.
        pub project: pulumi_gestalt_rust::Output<String>,
        /// The combination of labels configured directly on the resource
        /// and default labels configured on the provider.
        pub pulumi_labels: pulumi_gestalt_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        /// The region of the index. eg us-central1
        pub region: pulumi_gestalt_rust::Output<Option<String>>,
        /// The timestamp of when the Index was last updated in RFC3339 UTC "Zulu" format, with nanosecond resolution and up to nine fractional digits.
        pub update_time: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: AiIndexArgs,
    ) -> AiIndexResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let description_binding = args.description.get_output(context);
        let display_name_binding = args.display_name.get_output(context);
        let index_update_method_binding = args.index_update_method.get_output(context);
        let labels_binding = args.labels.get_output(context);
        let metadata_binding = args.metadata.get_output(context);
        let project_binding = args.project.get_output(context);
        let region_binding = args.region.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "gcp:vertex/aiIndex:AiIndex".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "description".into(),
                    value: &description_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "displayName".into(),
                    value: &display_name_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "indexUpdateMethod".into(),
                    value: &index_update_method_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "labels".into(),
                    value: &labels_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "metadata".into(),
                    value: &metadata_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "project".into(),
                    value: &project_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "region".into(),
                    value: &region_binding.drop_type(),
                },
            ],
        };
        let o = context.register_resource(request);
        AiIndexResult {
            create_time: o.get_field("createTime"),
            deployed_indexes: o.get_field("deployedIndexes"),
            description: o.get_field("description"),
            display_name: o.get_field("displayName"),
            effective_labels: o.get_field("effectiveLabels"),
            etag: o.get_field("etag"),
            index_stats: o.get_field("indexStats"),
            index_update_method: o.get_field("indexUpdateMethod"),
            labels: o.get_field("labels"),
            metadata: o.get_field("metadata"),
            metadata_schema_uri: o.get_field("metadataSchemaUri"),
            name: o.get_field("name"),
            project: o.get_field("project"),
            pulumi_labels: o.get_field("pulumiLabels"),
            region: o.get_field("region"),
            update_time: o.get_field("updateTime"),
        }
    }
}
