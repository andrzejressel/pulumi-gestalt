/// Provides a resource to manage an S3 Object Lambda Access Point.
/// An Object Lambda access point is associated with exactly one standard access point and thus one Amazon S3 bucket.
///
/// ## Example Usage
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let example = bucket_v_2::create(
///         "example",
///         BucketV2Args::builder().bucket("example").build_struct(),
///     );
///     let exampleAccessPoint = access_point::create(
///         "exampleAccessPoint",
///         AccessPointArgs::builder().bucket("${example.id}").name("example").build_struct(),
///     );
///     let exampleObjectLambdaAccessPoint = object_lambda_access_point::create(
///         "exampleObjectLambdaAccessPoint",
///         ObjectLambdaAccessPointArgs::builder()
///             .configuration(
///                 ObjectLambdaAccessPointConfiguration::builder()
///                     .supportingAccessPoint("${exampleAccessPoint.arn}")
///                     .transformationConfigurations(
///                         vec![
///                             ObjectLambdaAccessPointConfigurationTransformationConfiguration::builder()
///                             .actions(vec!["GetObject",])
///                             .contentTransformation(ObjectLambdaAccessPointConfigurationTransformationConfigurationContentTransformation::builder()
///                             .awsLambda(ObjectLambdaAccessPointConfigurationTransformationConfigurationContentTransformationAwsLambda::builder()
///                             .functionArn("${exampleAwsLambdaFunction.arn}")
///                             .build_struct()).build_struct()).build_struct(),
///                         ],
///                     )
///                     .build_struct(),
///             )
///             .name("example")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import Object Lambda Access Points using the `account_id` and `name`, separated by a colon (`:`). For example:
///
/// ```sh
/// $ pulumi import aws:s3control/objectLambdaAccessPoint:ObjectLambdaAccessPoint example 123456789012:example
/// ```
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod object_lambda_access_point {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct ObjectLambdaAccessPointArgs {
        /// The AWS account ID for the owner of the bucket for which you want to create an Object Lambda Access Point. Defaults to automatically determined account ID of the AWS provider.
        #[builder(into, default)]
        pub account_id: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// A configuration block containing details about the Object Lambda Access Point. See Configuration below for more details.
        #[builder(into)]
        pub configuration: pulumi_gestalt_rust::InputOrOutput<
            super::super::types::s3control::ObjectLambdaAccessPointConfiguration,
        >,
        /// The name for this Object Lambda Access Point.
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct ObjectLambdaAccessPointResult {
        /// Pulumi ID is the provider-assigned unique ID for this managed resource.
        /// It is set during deployments and may be missing (unknown) during planning phases.
        pub id: pulumi_gestalt_rust::Output<String>,
        /// The AWS account ID for the owner of the bucket for which you want to create an Object Lambda Access Point. Defaults to automatically determined account ID of the AWS provider.
        pub account_id: pulumi_gestalt_rust::Output<String>,
        /// Alias for the S3 Object Lambda Access Point.
        pub alias: pulumi_gestalt_rust::Output<String>,
        /// Amazon Resource Name (ARN) of the Object Lambda Access Point.
        pub arn: pulumi_gestalt_rust::Output<String>,
        /// A configuration block containing details about the Object Lambda Access Point. See Configuration below for more details.
        pub configuration: pulumi_gestalt_rust::Output<
            super::super::types::s3control::ObjectLambdaAccessPointConfiguration,
        >,
        /// The name for this Object Lambda Access Point.
        pub name: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: ObjectLambdaAccessPointArgs,
    ) -> ObjectLambdaAccessPointResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let account_id_binding = args.account_id.get_output(context);
        let configuration_binding = args.configuration.get_output(context);
        let name_binding = args.name.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "aws:s3control/objectLambdaAccessPoint:ObjectLambdaAccessPoint"
                .into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "accountId".into(),
                    value: &account_id_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "configuration".into(),
                    value: &configuration_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "name".into(),
                    value: &name_binding.drop_type(),
                },
            ],
        };
        let o = context.register_resource(request);
        ObjectLambdaAccessPointResult {
            id: o.get_field("id"),
            account_id: o.get_field("accountId"),
            alias: o.get_field("alias"),
            arn: o.get_field("arn"),
            configuration: o.get_field("configuration"),
            name: o.get_field("name"),
        }
    }
}
