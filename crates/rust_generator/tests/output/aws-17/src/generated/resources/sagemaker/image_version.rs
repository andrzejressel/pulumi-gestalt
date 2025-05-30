/// Provides a SageMaker Image Version resource.
///
/// ## Example Usage
///
/// ### Basic usage
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let test = image_version::create(
///         "test",
///         ImageVersionArgs::builder()
///             .base_image("012345678912.dkr.ecr.us-west-2.amazonaws.com/image:latest")
///             .image_name("${testAwsSagemakerImage.id}")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import SageMaker Image Versions using the `name`. For example:
///
/// ```sh
/// $ pulumi import aws:sagemaker/imageVersion:ImageVersion test_image my-code-repo
/// ```
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod image_version {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct ImageVersionArgs {
        /// The registry path of the container image on which this image version is based.
        #[builder(into)]
        pub base_image: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The name of the image. Must be unique to your account.
        #[builder(into)]
        pub image_name: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct ImageVersionResult {
        /// The Amazon Resource Name (ARN) assigned by AWS to this Image Version.
        pub arn: pulumi_gestalt_rust::Output<String>,
        /// The registry path of the container image on which this image version is based.
        pub base_image: pulumi_gestalt_rust::Output<String>,
        /// The registry path of the container image that contains this image version.
        pub container_image: pulumi_gestalt_rust::Output<String>,
        /// The Amazon Resource Name (ARN) of the image the version is based on.
        pub image_arn: pulumi_gestalt_rust::Output<String>,
        /// The name of the image. Must be unique to your account.
        pub image_name: pulumi_gestalt_rust::Output<String>,
        pub version: pulumi_gestalt_rust::Output<i32>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: ImageVersionArgs,
    ) -> ImageVersionResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let base_image_binding = args.base_image.get_output(context);
        let image_name_binding = args.image_name.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "aws:sagemaker/imageVersion:ImageVersion".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "baseImage".into(),
                    value: &base_image_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "imageName".into(),
                    value: &image_name_binding.drop_type(),
                },
            ],
        };
        let o = context.register_resource(request);
        ImageVersionResult {
            arn: o.get_field("arn"),
            base_image: o.get_field("baseImage"),
            container_image: o.get_field("containerImage"),
            image_arn: o.get_field("imageArn"),
            image_name: o.get_field("imageName"),
            version: o.get_field("version"),
        }
    }
}
