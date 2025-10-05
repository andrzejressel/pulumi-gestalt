/// Represents a single field in the database.
/// Fields are grouped by their "Collection Group", which represent all collections
/// in the database with the same id.
///
///
/// To get more information about Field, see:
///
/// * [API documentation](https://cloud.google.com/firestore/docs/reference/rest/v1/projects.databases.collectionGroups.fields)
/// * How-to Guides
///     * [Official Documentation](https://cloud.google.com/firestore/docs/query-data/indexing)
///
/// > **Warning:** This resource creates a Firestore Single Field override on a project that
///  already has a Firestore database. If you haven't already created it, you may
/// create a `gcp.firestore.Database` resource with `location_id` set to your
/// chosen location.
///
/// ## Example Usage
///
/// ### Firestore Field Basic
///
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let basic = field::create(
///         "basic",
///         FieldArgs::builder()
///             .collection("chatrooms__8493")
///             .database("${database.name}")
///             .field("basic")
///             .index_config(
///                 FieldIndexConfig::builder()
///                     .indexes(
///                         vec![
///                             FieldIndexConfigIndex::builder().order("ASCENDING")
///                             .queryScope("COLLECTION_GROUP").build_struct(),
///                             FieldIndexConfigIndex::builder().arrayConfig("CONTAINS")
///                             .build_struct(),
///                         ],
///                     )
///                     .build_struct(),
///             )
///             .project("my-project-name")
///             .build_struct(),
///     );
///     let database = database::create(
///         "database",
///         DatabaseArgs::builder()
///             .delete_protection_state("DELETE_PROTECTION_ENABLED")
///             .deletion_policy("DELETE")
///             .location_id("nam5")
///             .name("database-id")
///             .project("my-project-name")
///             .type_("FIRESTORE_NATIVE")
///             .build_struct(),
///     );
/// }
/// ```
/// ### Firestore Field Timestamp
///
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let database = database::create(
///         "database",
///         DatabaseArgs::builder()
///             .delete_protection_state("DELETE_PROTECTION_ENABLED")
///             .deletion_policy("DELETE")
///             .location_id("nam5")
///             .name("database-id")
///             .project("my-project-name")
///             .type_("FIRESTORE_NATIVE")
///             .build_struct(),
///     );
///     let timestamp = field::create(
///         "timestamp",
///         FieldArgs::builder()
///             .collection("chatrooms")
///             .database("${database.name}")
///             .field("timestamp")
///             .index_config(FieldIndexConfig::builder().build_struct())
///             .project("my-project-name")
///             .ttl_config(FieldTtlConfig::builder().build_struct())
///             .build_struct(),
///     );
/// }
/// ```
/// ### Firestore Field Match Override
///
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let database = database::create(
///         "database",
///         DatabaseArgs::builder()
///             .delete_protection_state("DELETE_PROTECTION_ENABLED")
///             .deletion_policy("DELETE")
///             .location_id("nam5")
///             .name("database-id")
///             .project("my-project-name")
///             .type_("FIRESTORE_NATIVE")
///             .build_struct(),
///     );
///     let matchOverride = field::create(
///         "matchOverride",
///         FieldArgs::builder()
///             .collection("chatrooms__9106")
///             .database("${database.name}")
///             .field("field_with_same_configuration_as_ancestor")
///             .index_config(
///                 FieldIndexConfig::builder()
///                     .indexes(
///                         vec![
///                             FieldIndexConfigIndex::builder().order("ASCENDING")
///                             .build_struct(), FieldIndexConfigIndex::builder()
///                             .order("DESCENDING").build_struct(),
///                             FieldIndexConfigIndex::builder().arrayConfig("CONTAINS")
///                             .build_struct(),
///                         ],
///                     )
///                     .build_struct(),
///             )
///             .project("my-project-name")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// Field can be imported using any of these accepted formats:
///
/// * `{{name}}`
///
/// When using the `pulumi import` command, Field can be imported using one of the formats above. For example:
///
/// ```sh
/// $ pulumi import gcp:firestore/field:Field default {{name}}
/// ```
///
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod field {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct FieldArgs {
        /// The id of the collection group to configure.
        #[builder(into)]
        pub collection: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The Firestore database id. Defaults to `"(default)"`.
        #[builder(into, default)]
        pub database: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The id of the field to configure.
        ///
        ///
        /// - - -
        #[builder(into)]
        pub field: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The single field index configuration for this field.
        /// Creating an index configuration for this field will override any inherited configuration with the
        /// indexes specified. Configuring the index configuration with an empty block disables all indexes on
        /// the field.
        /// Structure is documented below.
        #[builder(into, default)]
        pub index_config: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::firestore::FieldIndexConfig>,
        >,
        /// The ID of the project in which the resource belongs.
        /// If it is not provided, the provider project is used.
        #[builder(into, default)]
        pub project: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The TTL configuration for this Field. If set to an empty block (i.e. `ttl_config {}`), a TTL policy is configured based on the field. If unset, a TTL policy is not configured (or will be disabled upon updating the resource).
        /// Structure is documented below.
        #[builder(into, default)]
        pub ttl_config: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::firestore::FieldTtlConfig>,
        >,
    }
    #[allow(dead_code)]
    pub struct FieldResult {
        /// Pulumi ID is the provider-assigned unique ID for this managed resource.
        /// It is set during deployments and may be missing (unknown) during planning phases.
        pub id: pulumi_gestalt_rust::Output<String>,
        /// The id of the collection group to configure.
        pub collection: pulumi_gestalt_rust::Output<String>,
        /// The Firestore database id. Defaults to `"(default)"`.
        pub database: pulumi_gestalt_rust::Output<Option<String>>,
        /// The id of the field to configure.
        ///
        ///
        /// - - -
        pub field: pulumi_gestalt_rust::Output<String>,
        /// The single field index configuration for this field.
        /// Creating an index configuration for this field will override any inherited configuration with the
        /// indexes specified. Configuring the index configuration with an empty block disables all indexes on
        /// the field.
        /// Structure is documented below.
        pub index_config: pulumi_gestalt_rust::Output<
            Option<super::super::types::firestore::FieldIndexConfig>,
        >,
        /// The name of this field. Format:
        /// `projects/{{project}}/databases/{{database}}/collectionGroups/{{collection}}/fields/{{field}}`
        pub name: pulumi_gestalt_rust::Output<String>,
        /// The ID of the project in which the resource belongs.
        /// If it is not provided, the provider project is used.
        pub project: pulumi_gestalt_rust::Output<String>,
        /// The TTL configuration for this Field. If set to an empty block (i.e. `ttl_config {}`), a TTL policy is configured based on the field. If unset, a TTL policy is not configured (or will be disabled upon updating the resource).
        /// Structure is documented below.
        pub ttl_config: pulumi_gestalt_rust::Output<
            Option<super::super::types::firestore::FieldTtlConfig>,
        >,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: FieldArgs,
    ) -> FieldResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let collection_binding = args.collection.get_output(context);
        let database_binding = args.database.get_output(context);
        let field_binding = args.field.get_output(context);
        let index_config_binding = args.index_config.get_output(context);
        let project_binding = args.project.get_output(context);
        let ttl_config_binding = args.ttl_config.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "gcp:firestore/field:Field".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "collection".into(),
                    value: &collection_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "database".into(),
                    value: &database_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "field".into(),
                    value: &field_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "indexConfig".into(),
                    value: &index_config_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "project".into(),
                    value: &project_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "ttlConfig".into(),
                    value: &ttl_config_binding.drop_type(),
                },
            ],
        };
        let o = context.register_resource(request);
        FieldResult {
            id: o.get_field("id"),
            collection: o.get_field("collection"),
            database: o.get_field("database"),
            field: o.get_field("field"),
            index_config: o.get_field("indexConfig"),
            name: o.get_field("name"),
            project: o.get_field("project"),
            ttl_config: o.get_field("ttlConfig"),
        }
    }
}
