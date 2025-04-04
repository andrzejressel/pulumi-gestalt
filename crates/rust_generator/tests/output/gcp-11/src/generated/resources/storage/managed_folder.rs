/// A Google Cloud Storage Managed Folder.
///
/// You can apply Identity and Access Management (IAM) policies to
/// managed folders to grant principals access only to the objects
/// within the managed folder, which lets you more finely control access
/// for specific data sets and tables within a bucket. You can nest
/// managed folders up to 15 levels deep, including the parent managed
/// folder.
///
/// Managed folders can only be created in buckets that have uniform
/// bucket-level access enabled.
///
///
/// To get more information about ManagedFolder, see:
///
/// * [API documentation](https://cloud.google.com/storage/docs/json_api/v1/managedFolder)
/// * How-to Guides
///     * [Official Documentation](https://cloud.google.com/storage/docs/managed-folders)
///
/// ## Example Usage
///
/// ### Storage Managed Folder Basic
///
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let bucket = bucket::create(
///         "bucket",
///         BucketArgs::builder()
///             .location("EU")
///             .name("my-bucket")
///             .uniform_bucket_level_access(true)
///             .build_struct(),
///     );
///     let folder = managed_folder::create(
///         "folder",
///         ManagedFolderArgs::builder()
///             .bucket("${bucket.name}")
///             .force_destroy(true)
///             .name("managed/folder/name/")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// ManagedFolder can be imported using any of these accepted formats:
///
/// * `{{bucket}}/managedFolders/{{name}}`
///
/// * `{{bucket}}/{{name}}`
///
/// When using the `pulumi import` command, ManagedFolder can be imported using one of the formats above. For example:
///
/// ```sh
/// $ pulumi import gcp:storage/managedFolder:ManagedFolder default {{bucket}}/managedFolders/{{name}}
/// ```
///
/// ```sh
/// $ pulumi import gcp:storage/managedFolder:ManagedFolder default {{bucket}}/{{name}}
/// ```
///
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod managed_folder {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct ManagedFolderArgs {
        /// The name of the bucket that contains the managed folder.
        #[builder(into)]
        pub bucket: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Allows the deletion of a managed folder even if contains
        /// objects. If a non-empty managed folder is deleted, any objects
        /// within the folder will remain in a simulated folder with the
        /// same name.
        #[builder(into, default)]
        pub force_destroy: pulumi_gestalt_rust::InputOrOutput<Option<bool>>,
        /// The name of the managed folder expressed as a path. Must include
        /// trailing '/'. For example, `example_dir/example_dir2/`.
        ///
        ///
        /// - - -
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct ManagedFolderResult {
        /// The name of the bucket that contains the managed folder.
        pub bucket: pulumi_gestalt_rust::Output<String>,
        /// The timestamp at which this managed folder was created.
        pub create_time: pulumi_gestalt_rust::Output<String>,
        /// Allows the deletion of a managed folder even if contains
        /// objects. If a non-empty managed folder is deleted, any objects
        /// within the folder will remain in a simulated folder with the
        /// same name.
        pub force_destroy: pulumi_gestalt_rust::Output<Option<bool>>,
        /// The metadata generation of the managed folder.
        pub metageneration: pulumi_gestalt_rust::Output<String>,
        /// The name of the managed folder expressed as a path. Must include
        /// trailing '/'. For example, `example_dir/example_dir2/`.
        ///
        ///
        /// - - -
        pub name: pulumi_gestalt_rust::Output<String>,
        /// The URI of the created resource.
        pub self_link: pulumi_gestalt_rust::Output<String>,
        /// The timestamp at which this managed folder was most recently updated.
        pub update_time: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: ManagedFolderArgs,
    ) -> ManagedFolderResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let bucket_binding = args.bucket.get_output(context);
        let force_destroy_binding = args.force_destroy.get_output(context);
        let name_binding = args.name.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "gcp:storage/managedFolder:ManagedFolder".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "bucket".into(),
                    value: &bucket_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "forceDestroy".into(),
                    value: &force_destroy_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "name".into(),
                    value: &name_binding.drop_type(),
                },
            ],
        };
        let o = context.register_resource(request);
        ManagedFolderResult {
            bucket: o.get_field("bucket"),
            create_time: o.get_field("createTime"),
            force_destroy: o.get_field("forceDestroy"),
            metageneration: o.get_field("metageneration"),
            name: o.get_field("name"),
            self_link: o.get_field("selfLink"),
            update_time: o.get_field("updateTime"),
        }
    }
}
