/// Provides a lightsail resource access to a bucket.
///
/// ## Import
///
/// Using `pulumi import`, import `aws_lightsail_bucket_resource_access` using the `id` attribute. For example:
///
/// ```sh
/// $ pulumi import aws:lightsail/bucketResourceAccess:BucketResourceAccess test example-bucket,example-instance
/// ```
#[allow(
    clippy::doc_lazy_continuation,
    clippy::tabs_in_doc_comments,
    clippy::should_implement_trait
)]
pub mod bucket_resource_access {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct BucketResourceAccessArgs {
        /// The name of the bucket to grant access to.
        #[builder(into)]
        pub bucket_name: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The name of the resource to be granted bucket access.
        #[builder(into)]
        pub resource_name: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct BucketResourceAccessResult {
        /// Pulumi ID is the provider-assigned unique ID for this managed resource.
        /// It is set during deployments and may be missing (unknown) during planning phases.
        pub id: pulumi_gestalt_rust::Output<String>,
        /// Pulumi URN is the stable logical identity of this resource in the Pulumi stack.
        pub urn: pulumi_gestalt_rust::Output<String>,
        /// The name of the bucket to grant access to.
        pub bucket_name: pulumi_gestalt_rust::Output<String>,
        /// The name of the resource to be granted bucket access.
        pub resource_name: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, dead_code)]
    pub fn create(
        ctx: &pulumi_gestalt_rust::Context,
        name: &str,
        args: BucketResourceAccessArgs,
    ) -> BucketResourceAccessResult {
        __create(ctx, name, args, None)
    }
    ///
    /// Same as `create`, but with additional generic options that control the behavior of the resource registration.
    ///
    #[allow(non_snake_case, dead_code)]
    pub fn create_with_options(
        ctx: &pulumi_gestalt_rust::Context,
        name: &str,
        args: BucketResourceAccessArgs,
        options: pulumi_gestalt_rust::CustomResourceOptions,
    ) -> BucketResourceAccessResult {
        __create(ctx, name, args, Some(options))
    }
    #[allow(non_snake_case, unused_imports, dead_code)]
    fn __create(
        ctx: &pulumi_gestalt_rust::Context,
        name: &str,
        args: BucketResourceAccessArgs,
        options: Option<pulumi_gestalt_rust::CustomResourceOptions>,
    ) -> BucketResourceAccessResult {
        let bucket_name_binding = args.bucket_name.get_output(ctx);
        let resource_name_binding = args.resource_name.get_output(ctx);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "aws:lightsail/bucketResourceAccess:BucketResourceAccess".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "bucketName".into(),
                    value: &bucket_name_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "resourceName".into(),
                    value: &resource_name_binding.drop_type(),
                },
            ],
            options,
        };
        let o = ctx.register_resource(request);
        BucketResourceAccessResult {
            id: o.get_id(),
            urn: o.get_urn(),
            bucket_name: o.get_field("bucketName"),
            resource_name: o.get_field("resourceName"),
        }
    }
}
