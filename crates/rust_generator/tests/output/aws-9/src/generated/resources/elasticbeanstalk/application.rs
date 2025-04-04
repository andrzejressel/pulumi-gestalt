/// Provides an Elastic Beanstalk Application Resource. Elastic Beanstalk allows
/// you to deploy and manage applications in the AWS cloud without worrying about
/// the infrastructure that runs those applications.
///
/// This resource creates an application that has one configuration template named
/// `default`, and no application versions
///
/// ## Example Usage
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let tftest = application::create(
///         "tftest",
///         ApplicationArgs::builder()
///             .appversion_lifecycle(
///                 ApplicationAppversionLifecycle::builder()
///                     .deleteSourceFromS3(true)
///                     .maxCount(128)
///                     .serviceRole("${beanstalkService.arn}")
///                     .build_struct(),
///             )
///             .description("tf-test-desc")
///             .name("tf-test-name")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import Elastic Beanstalk Applications using the `name`. For example:
///
/// ```sh
/// $ pulumi import aws:elasticbeanstalk/application:Application tf_test tf-test-name
/// ```
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod application {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct ApplicationArgs {
        #[builder(into, default)]
        pub appversion_lifecycle: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::elasticbeanstalk::ApplicationAppversionLifecycle>,
        >,
        /// Short description of the application
        #[builder(into, default)]
        pub description: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The name of the application, must be unique within your account
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Key-value map of tags for the Elastic Beanstalk Application. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        #[builder(into, default)]
        pub tags: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
    }
    #[allow(dead_code)]
    pub struct ApplicationResult {
        pub appversion_lifecycle: pulumi_gestalt_rust::Output<
            Option<super::super::types::elasticbeanstalk::ApplicationAppversionLifecycle>,
        >,
        /// The ARN assigned by AWS for this Elastic Beanstalk Application.
        pub arn: pulumi_gestalt_rust::Output<String>,
        /// Short description of the application
        pub description: pulumi_gestalt_rust::Output<Option<String>>,
        /// The name of the application, must be unique within your account
        pub name: pulumi_gestalt_rust::Output<String>,
        /// Key-value map of tags for the Elastic Beanstalk Application. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        pub tags: pulumi_gestalt_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// A map of tags assigned to the resource, including those inherited from the provider `default_tags` configuration block.
        pub tags_all: pulumi_gestalt_rust::Output<
            std::collections::HashMap<String, String>,
        >,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: ApplicationArgs,
    ) -> ApplicationResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let appversion_lifecycle_binding = args.appversion_lifecycle.get_output(context);
        let description_binding = args.description.get_output(context);
        let name_binding = args.name.get_output(context);
        let tags_binding = args.tags.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "aws:elasticbeanstalk/application:Application".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "appversionLifecycle".into(),
                    value: &appversion_lifecycle_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "description".into(),
                    value: &description_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "name".into(),
                    value: &name_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "tags".into(),
                    value: &tags_binding.drop_type(),
                },
            ],
        };
        let o = context.register_resource(request);
        ApplicationResult {
            appversion_lifecycle: o.get_field("appversionLifecycle"),
            arn: o.get_field("arn"),
            description: o.get_field("description"),
            name: o.get_field("name"),
            tags: o.get_field("tags"),
            tags_all: o.get_field("tagsAll"),
        }
    }
}
