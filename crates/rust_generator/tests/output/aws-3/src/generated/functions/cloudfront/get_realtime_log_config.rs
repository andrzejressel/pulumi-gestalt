#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod get_realtime_log_config {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetRealtimeLogConfigArgs {
        /// Unique name to identify this real-time log configuration.
        #[builder(into)]
        pub name: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct GetRealtimeLogConfigResult {
        /// ARN (Amazon Resource Name) of the CloudFront real-time log configuration.
        pub arn: pulumi_gestalt_rust::Output<String>,
        /// (Required) Amazon Kinesis data streams where real-time log data is sent.
        pub endpoints: pulumi_gestalt_rust::Output<
            Vec<super::super::super::types::cloudfront::GetRealtimeLogConfigEndpoint>,
        >,
        /// (Required) Fields that are included in each real-time log record. See the [AWS documentation](https://docs.aws.amazon.com/AmazonCloudFront/latest/DeveloperGuide/real-time-logs.html#understand-real-time-log-config-fields) for supported values.
        pub fields: pulumi_gestalt_rust::Output<Vec<String>>,
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_gestalt_rust::Output<String>,
        pub name: pulumi_gestalt_rust::Output<String>,
        /// (Required) Sampling rate for this real-time log configuration. The sampling rate determines the percentage of viewer requests that are represented in the real-time log data. An integer between `1` and `100`, inclusive.
        pub sampling_rate: pulumi_gestalt_rust::Output<i32>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(
        context: &pulumi_gestalt_rust::Context,
        args: GetRealtimeLogConfigArgs,
    ) -> GetRealtimeLogConfigResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let name_binding = args.name.get_output(context);
        let request = pulumi_gestalt_rust::InvokeResourceRequest {
            token: "aws:cloudfront/getRealtimeLogConfig:getRealtimeLogConfig".into(),
            version: super::super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "name".into(),
                    value: &name_binding.drop_type(),
                },
            ],
        };
        let o = context.invoke_resource(request);
        GetRealtimeLogConfigResult {
            arn: o.get_field("arn"),
            endpoints: o.get_field("endpoints"),
            fields: o.get_field("fields"),
            id: o.get_field("id"),
            name: o.get_field("name"),
            sampling_rate: o.get_field("samplingRate"),
        }
    }
}
