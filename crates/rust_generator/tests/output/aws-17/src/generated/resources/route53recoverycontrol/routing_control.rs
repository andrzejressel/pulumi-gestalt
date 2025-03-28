/// Provides an AWS Route 53 Recovery Control Config Routing Control.
///
/// ## Example Usage
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let example = routing_control::create(
///         "example",
///         RoutingControlArgs::builder()
///             .cluster_arn(
///                 "arn:aws:route53-recovery-control::881188118811:cluster/8d47920e-d789-437d-803a-2dcc4b204393",
///             )
///             .name("tinlicker")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let example = routing_control::create(
///         "example",
///         RoutingControlArgs::builder()
///             .cluster_arn(
///                 "arn:aws:route53-recovery-control::881188118811:cluster/8d47920e-d789-437d-803a-2dcc4b204393",
///             )
///             .control_panel_arn(
///                 "arn:aws:route53-recovery-control::428113431245:controlpanel/abd5fbfc052d4844a082dbf400f61da8",
///             )
///             .name("thomasoliver")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import Route53 Recovery Control Config Routing Control using the routing control arn. For example:
///
/// ```sh
/// $ pulumi import aws:route53recoverycontrol/routingControl:RoutingControl mycontrol arn:aws:route53-recovery-control::313517334327:controlpanel/abd5fbfc052d4844a082dbf400f61da8/routingcontrol/d5d90e587870494b
/// ```
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod routing_control {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct RoutingControlArgs {
        /// ARN of the cluster in which this routing control will reside.
        #[builder(into)]
        pub cluster_arn: pulumi_gestalt_rust::InputOrOutput<String>,
        /// ARN of the control panel in which this routing control will reside.
        #[builder(into, default)]
        pub control_panel_arn: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The name describing the routing control.
        ///
        /// The following arguments are optional:
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct RoutingControlResult {
        /// ARN of the routing control.
        pub arn: pulumi_gestalt_rust::Output<String>,
        /// ARN of the cluster in which this routing control will reside.
        pub cluster_arn: pulumi_gestalt_rust::Output<String>,
        /// ARN of the control panel in which this routing control will reside.
        pub control_panel_arn: pulumi_gestalt_rust::Output<String>,
        /// The name describing the routing control.
        ///
        /// The following arguments are optional:
        pub name: pulumi_gestalt_rust::Output<String>,
        /// Status of routing control. `PENDING` when it is being created/updated, `PENDING_DELETION` when it is being deleted, and `DEPLOYED` otherwise.
        pub status: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: RoutingControlArgs,
    ) -> RoutingControlResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let cluster_arn_binding = args.cluster_arn.get_output(context);
        let control_panel_arn_binding = args.control_panel_arn.get_output(context);
        let name_binding = args.name.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "aws:route53recoverycontrol/routingControl:RoutingControl".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "clusterArn".into(),
                    value: &cluster_arn_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "controlPanelArn".into(),
                    value: &control_panel_arn_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "name".into(),
                    value: &name_binding.drop_type(),
                },
            ],
        };
        let o = context.register_resource(request);
        RoutingControlResult {
            arn: o.get_field("arn"),
            cluster_arn: o.get_field("clusterArn"),
            control_panel_arn: o.get_field("controlPanelArn"),
            name: o.get_field("name"),
            status: o.get_field("status"),
        }
    }
}
