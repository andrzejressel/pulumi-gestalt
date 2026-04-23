/// Provides a resource to manage AWS Device Farm Uploads.
///
/// > **NOTE:** AWS currently has limited regional support for Device Farm (e.g., `us-west-2`). See [AWS Device Farm endpoints and quotas](https://docs.aws.amazon.com/general/latest/gr/devicefarm.html) for information on supported regions.
///
/// ## Example Usage
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let example = project::create(
///         "example",
///         ProjectArgs::builder().name("example").build_struct(),
///     );
///     let exampleUpload = upload::create(
///         "exampleUpload",
///         UploadArgs::builder()
///             .name("example")
///             .project_arn("${example.arn}")
///             .type_("APPIUM_JAVA_TESTNG_TEST_SPEC")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import DeviceFarm Uploads using their ARN. For example:
///
/// ```sh
/// $ pulumi import aws:devicefarm/upload:Upload example arn:aws:devicefarm:us-west-2:123456789012:upload:4fa784c7-ccb4-4dbf-ba4f-02198320daa1
/// ```
#[allow(
    clippy::doc_lazy_continuation,
    clippy::tabs_in_doc_comments,
    clippy::should_implement_trait
)]
pub mod upload {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct UploadArgs {
        /// The upload's content type (for example, application/octet-stream).
        #[builder(into, default)]
        pub content_type: pulumi_gestalt_rust::Input<Option<String>>,
        /// The upload's file name. The name should not contain any forward slashes (/). If you are uploading an iOS app, the file name must end with the .ipa extension. If you are uploading an Android app, the file name must end with the .apk extension. For all others, the file name must end with the .zip file extension.
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::Input<Option<String>>,
        /// The ARN of the project for the upload.
        #[builder(into)]
        pub project_arn: pulumi_gestalt_rust::Input<String>,
        /// The upload's upload type. See [AWS Docs](https://docs.aws.amazon.com/devicefarm/latest/APIReference/API_CreateUpload.html#API_CreateUpload_RequestSyntax) for valid list of values.
        #[builder(into)]
        pub type_: pulumi_gestalt_rust::Input<String>,
    }
    #[allow(dead_code)]
    pub struct UploadResult {
        /// Pulumi ID is the provider-assigned unique ID for this managed resource.
        /// It is set during deployments and may be missing (unknown) during planning phases.
        pub id: pulumi_gestalt_rust::Output<String>,
        /// Pulumi URN is the stable logical identity of this resource in the Pulumi stack.
        pub urn: pulumi_gestalt_rust::Output<String>,
        /// The Amazon Resource Name of this upload.
        pub arn: pulumi_gestalt_rust::Output<String>,
        /// The upload's category.
        pub category: pulumi_gestalt_rust::Output<String>,
        /// The upload's content type (for example, application/octet-stream).
        pub content_type: pulumi_gestalt_rust::Output<Option<String>>,
        /// The upload's metadata. For example, for Android, this contains information that is parsed from the manifest and is displayed in the AWS Device Farm console after the associated app is uploaded.
        pub metadata: pulumi_gestalt_rust::Output<String>,
        /// The upload's file name. The name should not contain any forward slashes (/). If you are uploading an iOS app, the file name must end with the .ipa extension. If you are uploading an Android app, the file name must end with the .apk extension. For all others, the file name must end with the .zip file extension.
        pub name: pulumi_gestalt_rust::Output<String>,
        /// The ARN of the project for the upload.
        pub project_arn: pulumi_gestalt_rust::Output<String>,
        /// The upload's upload type. See [AWS Docs](https://docs.aws.amazon.com/devicefarm/latest/APIReference/API_CreateUpload.html#API_CreateUpload_RequestSyntax) for valid list of values.
        pub type_: pulumi_gestalt_rust::Output<String>,
        /// The presigned Amazon S3 URL that was used to store a file using a PUT request.
        pub url: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, dead_code)]
    pub fn create(
        ctx: &pulumi_gestalt_rust::Context,
        name: &str,
        args: UploadArgs,
    ) -> UploadResult {
        __create(ctx, name, args, None)
    }
    ///
    /// Same as `create`, but with additional generic options that control the behavior of the resource registration.
    ///
    #[allow(non_snake_case, dead_code)]
    pub fn create_with_options(
        ctx: &pulumi_gestalt_rust::Context,
        name: &str,
        args: UploadArgs,
        options: pulumi_gestalt_rust::CustomResourceOptions,
    ) -> UploadResult {
        __create(ctx, name, args, Some(options))
    }
    #[allow(non_snake_case, unused_imports, dead_code)]
    fn __create(
        ctx: &pulumi_gestalt_rust::Context,
        name: &str,
        args: UploadArgs,
        options: Option<pulumi_gestalt_rust::CustomResourceOptions>,
    ) -> UploadResult {
        let content_type_binding = args.content_type.get_output(ctx);
        let name_binding = args.name.get_output(ctx);
        let project_arn_binding = args.project_arn.get_output(ctx);
        let type__binding = args.type_.get_output(ctx);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "aws:devicefarm/upload:Upload".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "contentType".into(),
                    value: &content_type_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "name".into(),
                    value: &name_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "projectArn".into(),
                    value: &project_arn_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "type".into(),
                    value: &type__binding.drop_type(),
                },
            ],
            options,
        };
        let o = ctx.register_resource(request);
        UploadResult {
            id: o.get_id(),
            urn: o.get_urn(),
            arn: o.get_field("arn"),
            category: o.get_field("category"),
            content_type: o.get_field("contentType"),
            metadata: o.get_field("metadata"),
            name: o.get_field("name"),
            project_arn: o.get_field("projectArn"),
            type_: o.get_field("type"),
            url: o.get_field("url"),
        }
    }
}
