/// Provides a Glue Catalog Database Resource. You can refer to the [Glue Developer Guide](http://docs.aws.amazon.com/glue/latest/dg/populate-data-catalog.html) for a full explanation of the Glue Data Catalog functionality
///
/// ## Example Usage
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let example = catalog_database::create(
///         "example",
///         CatalogDatabaseArgs::builder().name("MyCatalogDatabase").build_struct(),
///     );
/// }
/// ```
///
/// ### Create Table Default Permissions
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let example = catalog_database::create(
///         "example",
///         CatalogDatabaseArgs::builder()
///             .create_table_default_permissions(
///                 vec![
///                     CatalogDatabaseCreateTableDefaultPermission::builder()
///                     .permissions(vec!["SELECT",])
///                     .principal(CatalogDatabaseCreateTableDefaultPermissionPrincipal::builder()
///                     .dataLakePrincipalIdentifier("IAM_ALLOWED_PRINCIPALS")
///                     .build_struct()).build_struct(),
///                 ],
///             )
///             .name("MyCatalogDatabase")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import Glue Catalog Databases using the `catalog_id:name`. If you have not set a Catalog ID specify the AWS Account ID that the database is in. For example:
///
/// ```sh
/// $ pulumi import aws:glue/catalogDatabase:CatalogDatabase database 123456789012:my_database
/// ```
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod catalog_database {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct CatalogDatabaseArgs {
        /// ID of the Glue Catalog to create the database in. If omitted, this defaults to the AWS Account ID.
        #[builder(into, default)]
        pub catalog_id: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Creates a set of default permissions on the table for principals. See `create_table_default_permission` below.
        #[builder(into, default)]
        pub create_table_default_permissions: pulumi_gestalt_rust::InputOrOutput<
            Option<
                Vec<
                    super::super::types::glue::CatalogDatabaseCreateTableDefaultPermission,
                >,
            >,
        >,
        /// Description of the database.
        #[builder(into, default)]
        pub description: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Configuration block that references an entity outside the AWS Glue Data Catalog. See `federated_database` below.
        #[builder(into, default)]
        pub federated_database: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::glue::CatalogDatabaseFederatedDatabase>,
        >,
        /// Location of the database (for example, an HDFS path).
        #[builder(into, default)]
        pub location_uri: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Name of the database. The acceptable characters are lowercase letters, numbers, and the underscore character.
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// List of key-value pairs that define parameters and properties of the database.
        #[builder(into, default)]
        pub parameters: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// Key-value map of resource tags. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        #[builder(into, default)]
        pub tags: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// Configuration block for a target database for resource linking. See `target_database` below.
        #[builder(into, default)]
        pub target_database: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::glue::CatalogDatabaseTargetDatabase>,
        >,
    }
    #[allow(dead_code)]
    pub struct CatalogDatabaseResult {
        /// ARN of the Glue Catalog Database.
        pub arn: pulumi_gestalt_rust::Output<String>,
        /// ID of the Glue Catalog to create the database in. If omitted, this defaults to the AWS Account ID.
        pub catalog_id: pulumi_gestalt_rust::Output<String>,
        /// Creates a set of default permissions on the table for principals. See `create_table_default_permission` below.
        pub create_table_default_permissions: pulumi_gestalt_rust::Output<
            Vec<super::super::types::glue::CatalogDatabaseCreateTableDefaultPermission>,
        >,
        /// Description of the database.
        pub description: pulumi_gestalt_rust::Output<Option<String>>,
        /// Configuration block that references an entity outside the AWS Glue Data Catalog. See `federated_database` below.
        pub federated_database: pulumi_gestalt_rust::Output<
            Option<super::super::types::glue::CatalogDatabaseFederatedDatabase>,
        >,
        /// Location of the database (for example, an HDFS path).
        pub location_uri: pulumi_gestalt_rust::Output<String>,
        /// Name of the database. The acceptable characters are lowercase letters, numbers, and the underscore character.
        pub name: pulumi_gestalt_rust::Output<String>,
        /// List of key-value pairs that define parameters and properties of the database.
        pub parameters: pulumi_gestalt_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// Key-value map of resource tags. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        pub tags: pulumi_gestalt_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// A map of tags assigned to the resource, including those inherited from the provider `default_tags` configuration block.
        pub tags_all: pulumi_gestalt_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        /// Configuration block for a target database for resource linking. See `target_database` below.
        pub target_database: pulumi_gestalt_rust::Output<
            Option<super::super::types::glue::CatalogDatabaseTargetDatabase>,
        >,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: CatalogDatabaseArgs,
    ) -> CatalogDatabaseResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let catalog_id_binding = args.catalog_id.get_output(context);
        let create_table_default_permissions_binding = args
            .create_table_default_permissions
            .get_output(context);
        let description_binding = args.description.get_output(context);
        let federated_database_binding = args.federated_database.get_output(context);
        let location_uri_binding = args.location_uri.get_output(context);
        let name_binding = args.name.get_output(context);
        let parameters_binding = args.parameters.get_output(context);
        let tags_binding = args.tags.get_output(context);
        let target_database_binding = args.target_database.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "aws:glue/catalogDatabase:CatalogDatabase".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "catalogId".into(),
                    value: &catalog_id_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "createTableDefaultPermissions".into(),
                    value: &create_table_default_permissions_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "description".into(),
                    value: &description_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "federatedDatabase".into(),
                    value: &federated_database_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "locationUri".into(),
                    value: &location_uri_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "name".into(),
                    value: &name_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "parameters".into(),
                    value: &parameters_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "tags".into(),
                    value: &tags_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "targetDatabase".into(),
                    value: &target_database_binding.drop_type(),
                },
            ],
        };
        let o = context.register_resource(request);
        CatalogDatabaseResult {
            arn: o.get_field("arn"),
            catalog_id: o.get_field("catalogId"),
            create_table_default_permissions: o
                .get_field("createTableDefaultPermissions"),
            description: o.get_field("description"),
            federated_database: o.get_field("federatedDatabase"),
            location_uri: o.get_field("locationUri"),
            name: o.get_field("name"),
            parameters: o.get_field("parameters"),
            tags: o.get_field("tags"),
            tags_all: o.get_field("tagsAll"),
            target_database: o.get_field("targetDatabase"),
        }
    }
}
