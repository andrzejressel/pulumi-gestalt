/// Resource for managing an AWS RDS (Relational Database) RDS Instance State.
///
/// > Destruction of this resource is a no-op and **will not** modify the instance state
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
///     let test = instance_desired_state::create(
///         "test",
///         InstanceDesiredStateArgs::builder()
///             .identifier("${testAwsDbInstance.identifier}")
///             .state("available")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import RDS (Relational Database) RDS Instance State using the `example_id_arg`. For example:
///
/// ```sh
/// $ pulumi import aws:rds/instanceDesiredState:InstanceDesiredState example rds_instance_state-id-12345678
/// ```
#[allow(
    clippy::doc_lazy_continuation,
    clippy::tabs_in_doc_comments,
    clippy::should_implement_trait
)]
pub mod instance_desired_state {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct InstanceDesiredStateArgs {
        /// DB Instance Identifier
        #[builder(into)]
        pub identifier: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Configured state of the DB Instance. Valid values are `available` and `stopped`.
        #[builder(into)]
        pub state: pulumi_gestalt_rust::InputOrOutput<String>,
        #[builder(into, default)]
        pub timeouts: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::rds::InstanceDesiredStateTimeouts>,
        >,
    }
    #[allow(dead_code)]
    pub struct InstanceDesiredStateResult {
        /// Pulumi ID is the provider-assigned unique ID for this managed resource.
        /// It is set during deployments and may be missing (unknown) during planning phases.
        pub id: pulumi_gestalt_rust::Output<String>,
        /// Pulumi URN is the stable logical identity of this resource in the Pulumi stack.
        pub urn: pulumi_gestalt_rust::Output<String>,
        /// DB Instance Identifier
        pub identifier: pulumi_gestalt_rust::Output<String>,
        /// Configured state of the DB Instance. Valid values are `available` and `stopped`.
        pub state: pulumi_gestalt_rust::Output<String>,
        pub timeouts: pulumi_gestalt_rust::Output<
            Option<super::super::types::rds::InstanceDesiredStateTimeouts>,
        >,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, dead_code)]
    pub fn create(
        ctx: &pulumi_gestalt_rust::Context,
        name: &str,
        args: InstanceDesiredStateArgs,
    ) -> InstanceDesiredStateResult {
        __create(ctx, name, args, None)
    }
    ///
    /// Same as `create`, but with additional generic options that control the behavior of the resource registration.
    ///
    #[allow(non_snake_case, dead_code)]
    pub fn create_with_options(
        ctx: &pulumi_gestalt_rust::Context,
        name: &str,
        args: InstanceDesiredStateArgs,
        options: pulumi_gestalt_rust::CustomResourceOptions,
    ) -> InstanceDesiredStateResult {
        __create(ctx, name, args, Some(options))
    }
    #[allow(non_snake_case, unused_imports, dead_code)]
    fn __create(
        ctx: &pulumi_gestalt_rust::Context,
        name: &str,
        args: InstanceDesiredStateArgs,
        options: Option<pulumi_gestalt_rust::CustomResourceOptions>,
    ) -> InstanceDesiredStateResult {
        let identifier_binding = args.identifier.get_output(ctx);
        let state_binding = args.state.get_output(ctx);
        let timeouts_binding = args.timeouts.get_output(ctx);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "aws:rds/instanceDesiredState:InstanceDesiredState".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "identifier".into(),
                    value: &identifier_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "state".into(),
                    value: &state_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "timeouts".into(),
                    value: &timeouts_binding.drop_type(),
                },
            ],
            options,
        };
        let o = ctx.register_resource(request);
        InstanceDesiredStateResult {
            id: o.get_id(),
            urn: o.get_urn(),
            identifier: o.get_field("identifier"),
            state: o.get_field("state"),
            timeouts: o.get_field("timeouts"),
        }
    }
}
