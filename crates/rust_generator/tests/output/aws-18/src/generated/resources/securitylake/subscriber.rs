/// Resource for managing an AWS Security Lake Subscriber.
///
/// > **NOTE:** The underlying `aws.securitylake.DataLake` must be configured before creating the `aws.securitylake.Subscriber`. Use a `depends_on` statement.
///
/// ## Example Usage
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let example = subscriber::create(
///         "example",
///         SubscriberArgs::builder()
///             .access_type("S3")
///             .source(
///                 SubscriberSource::builder()
///                     .awsLogSourceResource(
///                         SubscriberSourceAwsLogSourceResource::builder()
///                             .sourceName("ROUTE53")
///                             .sourceVersion("1.0")
///                             .build_struct(),
///                     )
///                     .build_struct(),
///             )
///             .subscriber_identity(
///                 SubscriberSubscriberIdentity::builder()
///                     .externalId("example")
///                     .principal("1234567890")
///                     .build_struct(),
///             )
///             .subscriber_name("example-name")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import Security Lake subscriber using the subscriber ID. For example:
///
/// ```sh
/// $ pulumi import aws:securitylake/subscriber:Subscriber example 9f3bfe79-d543-474d-a93c-f3846805d208
/// ```
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod subscriber {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct SubscriberArgs {
        /// The Amazon S3 or Lake Formation access type.
        #[builder(into, default)]
        pub access_type: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The supported AWS services from which logs and events are collected. Security Lake supports log and event collection for natively supported AWS services. See `source` Blocks below.
        #[builder(into, default)]
        pub source: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::securitylake::SubscriberSource>,
        >,
        /// The description for your subscriber account in Security Lake.
        #[builder(into, default)]
        pub subscriber_description: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The AWS identity used to access your data. See `subscriber_identity` Block below.
        #[builder(into, default)]
        pub subscriber_identity: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::securitylake::SubscriberSubscriberIdentity>,
        >,
        /// The name of your Security Lake subscriber account.
        #[builder(into, default)]
        pub subscriber_name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Key-value map of resource tags. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        #[builder(into, default)]
        pub tags: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
        #[builder(into, default)]
        pub timeouts: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::securitylake::SubscriberTimeouts>,
        >,
    }
    #[allow(dead_code)]
    pub struct SubscriberResult {
        /// The Amazon S3 or Lake Formation access type.
        pub access_type: pulumi_gestalt_rust::Output<String>,
        /// ARN of the Data Lake.
        pub arn: pulumi_gestalt_rust::Output<String>,
        /// The Amazon Resource Name (ARN) which uniquely defines the AWS RAM resource share. Before accepting the RAM resource share invitation, you can view details related to the RAM resource share.
        pub resource_share_arn: pulumi_gestalt_rust::Output<String>,
        /// The name of the resource share.
        pub resource_share_name: pulumi_gestalt_rust::Output<String>,
        /// The ARN of the IAM role to be used by the entity putting logs into your custom source partition.
        pub role_arn: pulumi_gestalt_rust::Output<String>,
        /// The ARN for the Amazon Security Lake Amazon S3 bucket.
        pub s3_bucket_arn: pulumi_gestalt_rust::Output<String>,
        /// The supported AWS services from which logs and events are collected. Security Lake supports log and event collection for natively supported AWS services. See `source` Blocks below.
        pub source: pulumi_gestalt_rust::Output<
            Option<super::super::types::securitylake::SubscriberSource>,
        >,
        /// The description for your subscriber account in Security Lake.
        pub subscriber_description: pulumi_gestalt_rust::Output<Option<String>>,
        /// The subscriber endpoint to which exception messages are posted.
        pub subscriber_endpoint: pulumi_gestalt_rust::Output<String>,
        /// The AWS identity used to access your data. See `subscriber_identity` Block below.
        pub subscriber_identity: pulumi_gestalt_rust::Output<
            Option<super::super::types::securitylake::SubscriberSubscriberIdentity>,
        >,
        /// The name of your Security Lake subscriber account.
        pub subscriber_name: pulumi_gestalt_rust::Output<Option<String>>,
        /// The subscriber status of the Amazon Security Lake subscriber account.
        pub subscriber_status: pulumi_gestalt_rust::Output<String>,
        /// Key-value map of resource tags. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        pub tags: pulumi_gestalt_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// A map of tags assigned to the resource, including those inherited from the provider `default_tags` configuration block.
        pub tags_all: pulumi_gestalt_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        pub timeouts: pulumi_gestalt_rust::Output<
            Option<super::super::types::securitylake::SubscriberTimeouts>,
        >,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: SubscriberArgs,
    ) -> SubscriberResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let access_type_binding = args.access_type.get_output(context);
        let source_binding = args.source.get_output(context);
        let subscriber_description_binding = args
            .subscriber_description
            .get_output(context);
        let subscriber_identity_binding = args.subscriber_identity.get_output(context);
        let subscriber_name_binding = args.subscriber_name.get_output(context);
        let tags_binding = args.tags.get_output(context);
        let timeouts_binding = args.timeouts.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "aws:securitylake/subscriber:Subscriber".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "accessType".into(),
                    value: &access_type_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "source".into(),
                    value: &source_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "subscriberDescription".into(),
                    value: &subscriber_description_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "subscriberIdentity".into(),
                    value: &subscriber_identity_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "subscriberName".into(),
                    value: &subscriber_name_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "tags".into(),
                    value: &tags_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "timeouts".into(),
                    value: &timeouts_binding.drop_type(),
                },
            ],
        };
        let o = context.register_resource(request);
        SubscriberResult {
            access_type: o.get_field("accessType"),
            arn: o.get_field("arn"),
            resource_share_arn: o.get_field("resourceShareArn"),
            resource_share_name: o.get_field("resourceShareName"),
            role_arn: o.get_field("roleArn"),
            s3_bucket_arn: o.get_field("s3BucketArn"),
            source: o.get_field("source"),
            subscriber_description: o.get_field("subscriberDescription"),
            subscriber_endpoint: o.get_field("subscriberEndpoint"),
            subscriber_identity: o.get_field("subscriberIdentity"),
            subscriber_name: o.get_field("subscriberName"),
            subscriber_status: o.get_field("subscriberStatus"),
            tags: o.get_field("tags"),
            tags_all: o.get_field("tagsAll"),
            timeouts: o.get_field("timeouts"),
        }
    }
}
