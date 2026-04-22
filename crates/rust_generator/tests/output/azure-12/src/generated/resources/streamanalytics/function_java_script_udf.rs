/// Manages a JavaScript UDF Function within Stream Analytics Streaming Job.
///
/// ## Example Usage
///
/// ```yaml
/// resources:
///   exampleFunctionJavaScriptUDF:
///     type: azure:streamanalytics:FunctionJavaScriptUDF
///     name: example
///     properties:
///       name: example-javascript-function
///       streamAnalyticsJobName: ${exampleGetJob.name}
///       resourceGroupName: ${exampleGetJob.resourceGroupName}
///       script: |
///         function getRandomNumber(in) {
///           return in;
///         }
///       inputs:
///         - type: bigint
///       output:
///         type: bigint
/// variables:
///   example:
///     fn::invoke:
///       function: azure:core:getResourceGroup
///       arguments:
///         name: example-resources
///   exampleGetJob:
///     fn::invoke:
///       function: azure:streamanalytics:getJob
///       arguments:
///         name: example-job
///         resourceGroupName: ${example.name}
/// ```
///
/// ## Import
///
/// Stream Analytics JavaScript UDF Functions can be imported using the `resource id`, e.g.
///
/// ```sh
/// $ pulumi import azure:streamanalytics/functionJavaScriptUDF:FunctionJavaScriptUDF example /subscriptions/00000000-0000-0000-0000-000000000000/resourceGroups/group1/providers/Microsoft.StreamAnalytics/streamingJobs/job1/functions/func1
/// ```
///
#[allow(
    clippy::doc_lazy_continuation,
    clippy::tabs_in_doc_comments,
    clippy::should_implement_trait
)]
pub mod function_java_script_udf {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct FunctionJavaScriptUDFArgs {
        /// One or more `input` blocks as defined below.
        #[builder(into)]
        pub inputs: pulumi_gestalt_rust::Input<
            Vec<super::super::types::streamanalytics::FunctionJavaScriptUdfInput>,
        >,
        /// The name of the JavaScript UDF Function. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::Input<Option<String>>,
        /// An `output` blocks as defined below.
        #[builder(into)]
        pub output: pulumi_gestalt_rust::Input<
            super::super::types::streamanalytics::FunctionJavaScriptUdfOutput,
        >,
        /// The name of the Resource Group where the Stream Analytics Job exists. Changing this forces a new resource to be created.
        #[builder(into)]
        pub resource_group_name: pulumi_gestalt_rust::Input<String>,
        /// The JavaScript of this UDF Function.
        #[builder(into)]
        pub script: pulumi_gestalt_rust::Input<String>,
        /// The name of the Stream Analytics Job where this Function should be created. Changing this forces a new resource to be created.
        #[builder(into)]
        pub stream_analytics_job_name: pulumi_gestalt_rust::Input<String>,
    }
    #[allow(dead_code)]
    pub struct FunctionJavaScriptUDFResult {
        /// Pulumi ID is the provider-assigned unique ID for this managed resource.
        /// It is set during deployments and may be missing (unknown) during planning phases.
        pub id: pulumi_gestalt_rust::Output<String>,
        /// Pulumi URN is the stable logical identity of this resource in the Pulumi stack.
        pub urn: pulumi_gestalt_rust::Output<String>,
        /// One or more `input` blocks as defined below.
        pub inputs: pulumi_gestalt_rust::Output<
            Vec<super::super::types::streamanalytics::FunctionJavaScriptUdfInput>,
        >,
        /// The name of the JavaScript UDF Function. Changing this forces a new resource to be created.
        pub name: pulumi_gestalt_rust::Output<String>,
        /// An `output` blocks as defined below.
        pub output: pulumi_gestalt_rust::Output<
            super::super::types::streamanalytics::FunctionJavaScriptUdfOutput,
        >,
        /// The name of the Resource Group where the Stream Analytics Job exists. Changing this forces a new resource to be created.
        pub resource_group_name: pulumi_gestalt_rust::Output<String>,
        /// The JavaScript of this UDF Function.
        pub script: pulumi_gestalt_rust::Output<String>,
        /// The name of the Stream Analytics Job where this Function should be created. Changing this forces a new resource to be created.
        pub stream_analytics_job_name: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, dead_code)]
    pub fn create(
        ctx: &pulumi_gestalt_rust::Context,
        name: &str,
        args: FunctionJavaScriptUDFArgs,
    ) -> FunctionJavaScriptUDFResult {
        __create(ctx, name, args, None)
    }
    ///
    /// Same as `create`, but with additional generic options that control the behavior of the resource registration.
    ///
    #[allow(non_snake_case, dead_code)]
    pub fn create_with_options(
        ctx: &pulumi_gestalt_rust::Context,
        name: &str,
        args: FunctionJavaScriptUDFArgs,
        options: pulumi_gestalt_rust::CustomResourceOptions,
    ) -> FunctionJavaScriptUDFResult {
        __create(ctx, name, args, Some(options))
    }
    #[allow(non_snake_case, unused_imports, dead_code)]
    fn __create(
        ctx: &pulumi_gestalt_rust::Context,
        name: &str,
        args: FunctionJavaScriptUDFArgs,
        options: Option<pulumi_gestalt_rust::CustomResourceOptions>,
    ) -> FunctionJavaScriptUDFResult {
        let inputs_binding = args.inputs.get_output(ctx);
        let name_binding = args.name.get_output(ctx);
        let output_binding = args.output.get_output(ctx);
        let resource_group_name_binding = args.resource_group_name.get_output(ctx);
        let script_binding = args.script.get_output(ctx);
        let stream_analytics_job_name_binding = args
            .stream_analytics_job_name
            .get_output(ctx);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "azure:streamanalytics/functionJavaScriptUDF:FunctionJavaScriptUDF"
                .into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "inputs".into(),
                    value: &inputs_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "name".into(),
                    value: &name_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "output".into(),
                    value: &output_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "resourceGroupName".into(),
                    value: &resource_group_name_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "script".into(),
                    value: &script_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "streamAnalyticsJobName".into(),
                    value: &stream_analytics_job_name_binding.drop_type(),
                },
            ],
            options,
        };
        let o = ctx.register_resource(request);
        FunctionJavaScriptUDFResult {
            id: o.get_id(),
            urn: o.get_urn(),
            inputs: o.get_field("inputs"),
            name: o.get_field("name"),
            output: o.get_field("output"),
            resource_group_name: o.get_field("resourceGroupName"),
            script: o.get_field("script"),
            stream_analytics_job_name: o.get_field("streamAnalyticsJobName"),
        }
    }
}
