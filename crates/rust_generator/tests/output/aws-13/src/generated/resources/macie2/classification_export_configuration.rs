/// Provides a resource to manage an [Amazon Macie Classification Export Configuration](https://docs.aws.amazon.com/macie/latest/APIReference/classification-export-configuration.html).
///
/// ## Example Usage
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let example = account::create("example", AccountArgs::builder().build_struct());
///     let exampleClassificationExportConfiguration = classification_export_configuration::create(
///         "exampleClassificationExportConfiguration",
///         ClassificationExportConfigurationArgs::builder()
///             .s_3_destination(
///                 ClassificationExportConfigurationS3Destination::builder()
///                     .bucketName("${exampleAwsS3Bucket.bucket}")
///                     .keyPrefix("exampleprefix/")
///                     .kmsKeyArn("${exampleAwsKmsKey.arn}")
///                     .build_struct(),
///             )
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import `aws_macie2_classification_export_configuration` using the account ID and region. For example:
///
/// ```sh
/// $ pulumi import aws:macie2/classificationExportConfiguration:ClassificationExportConfiguration example 123456789012:us-west-2
/// ```
#[allow(
    clippy::doc_lazy_continuation,
    clippy::tabs_in_doc_comments,
    clippy::should_implement_trait
)]
pub mod classification_export_configuration {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct ClassificationExportConfigurationArgs {
        /// Configuration block for a S3 Destination. Defined below
        #[builder(into, default)]
        pub s3_destination: pulumi_gestalt_rust::Input<
            Option<
                super::super::types::macie2::ClassificationExportConfigurationS3Destination,
            >,
        >,
    }
    #[allow(dead_code)]
    pub struct ClassificationExportConfigurationResult {
        /// Pulumi ID is the provider-assigned unique ID for this managed resource.
        /// It is set during deployments and may be missing (unknown) during planning phases.
        pub id: pulumi_gestalt_rust::Output<String>,
        /// Pulumi URN is the stable logical identity of this resource in the Pulumi stack.
        pub urn: pulumi_gestalt_rust::Output<String>,
        /// Configuration block for a S3 Destination. Defined below
        pub s3_destination: pulumi_gestalt_rust::Output<
            Option<
                super::super::types::macie2::ClassificationExportConfigurationS3Destination,
            >,
        >,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, dead_code)]
    pub fn create(
        ctx: &pulumi_gestalt_rust::Context,
        name: &str,
        args: ClassificationExportConfigurationArgs,
    ) -> ClassificationExportConfigurationResult {
        __create(ctx, name, args, None)
    }
    ///
    /// Same as `create`, but with additional generic options that control the behavior of the resource registration.
    ///
    #[allow(non_snake_case, dead_code)]
    pub fn create_with_options(
        ctx: &pulumi_gestalt_rust::Context,
        name: &str,
        args: ClassificationExportConfigurationArgs,
        options: pulumi_gestalt_rust::CustomResourceOptions,
    ) -> ClassificationExportConfigurationResult {
        __create(ctx, name, args, Some(options))
    }
    #[allow(non_snake_case, unused_imports, dead_code)]
    fn __create(
        ctx: &pulumi_gestalt_rust::Context,
        name: &str,
        args: ClassificationExportConfigurationArgs,
        options: Option<pulumi_gestalt_rust::CustomResourceOptions>,
    ) -> ClassificationExportConfigurationResult {
        let s3_destination_binding = args.s3_destination.get_output(ctx);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "aws:macie2/classificationExportConfiguration:ClassificationExportConfiguration"
                .into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "s3Destination".into(),
                    value: &s3_destination_binding.drop_type(),
                },
            ],
            options,
        };
        let o = ctx.register_resource(request);
        ClassificationExportConfigurationResult {
            id: o.get_id(),
            urn: o.get_urn(),
            s3_destination: o.get_field("s3Destination"),
        }
    }
}
