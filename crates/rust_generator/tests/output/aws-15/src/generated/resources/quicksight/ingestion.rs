/// Resource for managing an AWS QuickSight Ingestion.
///
/// ## Example Usage
///
/// ### Basic Usage
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let example = ingestion::create(
///         "example",
///         IngestionArgs::builder()
///             .data_set_id("${exampleAwsQuicksightDataSet.dataSetId}")
///             .ingestion_id("example-id")
///             .ingestion_type("FULL_REFRESH")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import QuickSight Ingestion using the AWS account ID, data set ID, and ingestion ID separated by commas (`,`). For example:
///
/// ```sh
/// $ pulumi import aws:quicksight/ingestion:Ingestion example 123456789012,example-dataset-id,example-ingestion-id
/// ```
#[allow(
    clippy::doc_lazy_continuation,
    clippy::tabs_in_doc_comments,
    clippy::should_implement_trait
)]
pub mod ingestion {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct IngestionArgs {
        /// AWS account ID.
        #[builder(into, default)]
        pub aws_account_id: pulumi_gestalt_rust::Input<Option<String>>,
        /// ID of the dataset used in the ingestion.
        #[builder(into)]
        pub data_set_id: pulumi_gestalt_rust::Input<String>,
        /// ID for the ingestion.
        #[builder(into)]
        pub ingestion_id: pulumi_gestalt_rust::Input<String>,
        /// Type of ingestion to be created. Valid values are `INCREMENTAL_REFRESH` and `FULL_REFRESH`.
        ///
        /// The following arguments are optional:
        #[builder(into)]
        pub ingestion_type: pulumi_gestalt_rust::Input<String>,
    }
    #[allow(dead_code)]
    pub struct IngestionResult {
        /// Pulumi ID is the provider-assigned unique ID for this managed resource.
        /// It is set during deployments and may be missing (unknown) during planning phases.
        pub id: pulumi_gestalt_rust::Output<String>,
        /// Pulumi URN is the stable logical identity of this resource in the Pulumi stack.
        pub urn: pulumi_gestalt_rust::Output<String>,
        /// ARN of the Ingestion.
        pub arn: pulumi_gestalt_rust::Output<String>,
        /// AWS account ID.
        pub aws_account_id: pulumi_gestalt_rust::Output<String>,
        /// ID of the dataset used in the ingestion.
        pub data_set_id: pulumi_gestalt_rust::Output<String>,
        /// ID for the ingestion.
        pub ingestion_id: pulumi_gestalt_rust::Output<String>,
        /// Ingestion status.
        pub ingestion_status: pulumi_gestalt_rust::Output<String>,
        /// Type of ingestion to be created. Valid values are `INCREMENTAL_REFRESH` and `FULL_REFRESH`.
        ///
        /// The following arguments are optional:
        pub ingestion_type: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, dead_code)]
    pub fn create(
        ctx: &pulumi_gestalt_rust::Context,
        name: &str,
        args: IngestionArgs,
    ) -> IngestionResult {
        __create(ctx, name, args, None)
    }
    ///
    /// Same as `create`, but with additional generic options that control the behavior of the resource registration.
    ///
    #[allow(non_snake_case, dead_code)]
    pub fn create_with_options(
        ctx: &pulumi_gestalt_rust::Context,
        name: &str,
        args: IngestionArgs,
        options: pulumi_gestalt_rust::CustomResourceOptions,
    ) -> IngestionResult {
        __create(ctx, name, args, Some(options))
    }
    #[allow(non_snake_case, unused_imports, dead_code)]
    fn __create(
        ctx: &pulumi_gestalt_rust::Context,
        name: &str,
        args: IngestionArgs,
        options: Option<pulumi_gestalt_rust::CustomResourceOptions>,
    ) -> IngestionResult {
        let aws_account_id_binding = args.aws_account_id.get_output(ctx);
        let data_set_id_binding = args.data_set_id.get_output(ctx);
        let ingestion_id_binding = args.ingestion_id.get_output(ctx);
        let ingestion_type_binding = args.ingestion_type.get_output(ctx);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "aws:quicksight/ingestion:Ingestion".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "awsAccountId".into(),
                    value: &aws_account_id_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "dataSetId".into(),
                    value: &data_set_id_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "ingestionId".into(),
                    value: &ingestion_id_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "ingestionType".into(),
                    value: &ingestion_type_binding.drop_type(),
                },
            ],
            options,
        };
        let o = ctx.register_resource(request);
        IngestionResult {
            id: o.get_id(),
            urn: o.get_urn(),
            arn: o.get_field("arn"),
            aws_account_id: o.get_field("awsAccountId"),
            data_set_id: o.get_field("dataSetId"),
            ingestion_id: o.get_field("ingestionId"),
            ingestion_status: o.get_field("ingestionStatus"),
            ingestion_type: o.get_field("ingestionType"),
        }
    }
}
