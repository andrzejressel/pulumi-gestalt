/// Provides a S3 bucket [inventory configuration](https://docs.aws.amazon.com/AmazonS3/latest/dev/storage-inventory.html) resource.
///
/// > This resource cannot be used with S3 directory buckets.
///
/// ## Example Usage
///
/// ### Add inventory configuration
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let inventory = bucket_v_2::create(
///         "inventory",
///         BucketV2Args::builder().bucket("my-tf-inventory-bucket").build_struct(),
///     );
///     let test = bucket_v_2::create(
///         "test",
///         BucketV2Args::builder().bucket("my-tf-test-bucket").build_struct(),
///     );
///     let testInventory = inventory::create(
///         "testInventory",
///         InventoryArgs::builder()
///             .bucket("${test.id}")
///             .destination(
///                 InventoryDestination::builder()
///                     .bucket(
///                         InventoryDestinationBucket::builder()
///                             .bucketArn("${inventory.arn}")
///                             .format("ORC")
///                             .build_struct(),
///                     )
///                     .build_struct(),
///             )
///             .included_object_versions("All")
///             .name("EntireBucketDaily")
///             .schedule(InventorySchedule::builder().frequency("Daily").build_struct())
///             .build_struct(),
///     );
/// }
/// ```
///
/// ### Add inventory configuration with S3 object prefix
///
/// ```yaml
/// resources:
///   test:
///     type: aws:s3:BucketV2
///     properties:
///       bucket: my-tf-test-bucket
///   inventory:
///     type: aws:s3:BucketV2
///     properties:
///       bucket: my-tf-inventory-bucket
///   test-prefix:
///     type: aws:s3:Inventory
///     properties:
///       bucket: ${test.id}
///       name: DocumentsWeekly
///       includedObjectVersions: All
///       schedule:
///         frequency: Daily
///       filter:
///         prefix: documents/
///       destination:
///         bucket:
///           format: ORC
///           bucketArn: ${inventory.arn}
///           prefix: inventory
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import S3 bucket inventory configurations using `bucket:inventory`. For example:
///
/// ```sh
/// $ pulumi import aws:s3/inventory:Inventory my-bucket-entire-bucket my-bucket:EntireBucket
/// ```
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod inventory {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct InventoryArgs {
        /// Name of the source bucket that inventory lists the objects for.
        #[builder(into)]
        pub bucket: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Contains information about where to publish the inventory results (documented below).
        #[builder(into)]
        pub destination: pulumi_gestalt_rust::InputOrOutput<
            super::super::types::s3::InventoryDestination,
        >,
        /// Specifies whether the inventory is enabled or disabled.
        #[builder(into, default)]
        pub enabled: pulumi_gestalt_rust::InputOrOutput<Option<bool>>,
        /// Specifies an inventory filter. The inventory only includes objects that meet the filter's criteria (documented below).
        #[builder(into, default)]
        pub filter: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::s3::InventoryFilter>,
        >,
        /// Object versions to include in the inventory list. Valid values: `All`, `Current`.
        #[builder(into)]
        pub included_object_versions: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Unique identifier of the inventory configuration for the bucket.
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// List of optional fields that are included in the inventory results. Please refer to the S3 [documentation](https://docs.aws.amazon.com/AmazonS3/latest/API/API_InventoryConfiguration.html#AmazonS3-Type-InventoryConfiguration-OptionalFields) for more details.
        #[builder(into, default)]
        pub optional_fields: pulumi_gestalt_rust::InputOrOutput<Option<Vec<String>>>,
        /// Specifies the schedule for generating inventory results (documented below).
        #[builder(into)]
        pub schedule: pulumi_gestalt_rust::InputOrOutput<
            super::super::types::s3::InventorySchedule,
        >,
    }
    #[allow(dead_code)]
    pub struct InventoryResult {
        /// Name of the source bucket that inventory lists the objects for.
        pub bucket: pulumi_gestalt_rust::Output<String>,
        /// Contains information about where to publish the inventory results (documented below).
        pub destination: pulumi_gestalt_rust::Output<
            super::super::types::s3::InventoryDestination,
        >,
        /// Specifies whether the inventory is enabled or disabled.
        pub enabled: pulumi_gestalt_rust::Output<Option<bool>>,
        /// Specifies an inventory filter. The inventory only includes objects that meet the filter's criteria (documented below).
        pub filter: pulumi_gestalt_rust::Output<
            Option<super::super::types::s3::InventoryFilter>,
        >,
        /// Object versions to include in the inventory list. Valid values: `All`, `Current`.
        pub included_object_versions: pulumi_gestalt_rust::Output<String>,
        /// Unique identifier of the inventory configuration for the bucket.
        pub name: pulumi_gestalt_rust::Output<String>,
        /// List of optional fields that are included in the inventory results. Please refer to the S3 [documentation](https://docs.aws.amazon.com/AmazonS3/latest/API/API_InventoryConfiguration.html#AmazonS3-Type-InventoryConfiguration-OptionalFields) for more details.
        pub optional_fields: pulumi_gestalt_rust::Output<Option<Vec<String>>>,
        /// Specifies the schedule for generating inventory results (documented below).
        pub schedule: pulumi_gestalt_rust::Output<
            super::super::types::s3::InventorySchedule,
        >,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: InventoryArgs,
    ) -> InventoryResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let bucket_binding = args.bucket.get_output(context);
        let destination_binding = args.destination.get_output(context);
        let enabled_binding = args.enabled.get_output(context);
        let filter_binding = args.filter.get_output(context);
        let included_object_versions_binding = args
            .included_object_versions
            .get_output(context);
        let name_binding = args.name.get_output(context);
        let optional_fields_binding = args.optional_fields.get_output(context);
        let schedule_binding = args.schedule.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "aws:s3/inventory:Inventory".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "bucket".into(),
                    value: &bucket_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "destination".into(),
                    value: &destination_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "enabled".into(),
                    value: &enabled_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "filter".into(),
                    value: &filter_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "includedObjectVersions".into(),
                    value: &included_object_versions_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "name".into(),
                    value: &name_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "optionalFields".into(),
                    value: &optional_fields_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "schedule".into(),
                    value: &schedule_binding.drop_type(),
                },
            ],
        };
        let o = context.register_resource(request);
        InventoryResult {
            bucket: o.get_field("bucket"),
            destination: o.get_field("destination"),
            enabled: o.get_field("enabled"),
            filter: o.get_field("filter"),
            included_object_versions: o.get_field("includedObjectVersions"),
            name: o.get_field("name"),
            optional_fields: o.get_field("optionalFields"),
            schedule: o.get_field("schedule"),
        }
    }
}
