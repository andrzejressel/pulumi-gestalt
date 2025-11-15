/// Associates a Direct Connect Connection with a LAG.
///
/// ## Example Usage
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let example = connection::create(
///         "example",
///         ConnectionArgs::builder()
///             .bandwidth("1Gbps")
///             .location("EqSe2-EQ")
///             .name("example")
///             .build_struct(),
///     );
///     let exampleConnectionAssociation = connection_association::create(
///         "exampleConnectionAssociation",
///         ConnectionAssociationArgs::builder()
///             .connection_id("${example.id}")
///             .lag_id("${exampleLinkAggregationGroup.id}")
///             .build_struct(),
///     );
///     let exampleLinkAggregationGroup = link_aggregation_group::create(
///         "exampleLinkAggregationGroup",
///         LinkAggregationGroupArgs::builder()
///             .connections_bandwidth("1Gbps")
///             .location("EqSe2-EQ")
///             .name("example")
///             .build_struct(),
///     );
/// }
/// ```
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub mod connection_association {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct ConnectionAssociationArgs {
        /// The ID of the connection.
        #[builder(into)]
        pub connection_id: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The ID of the LAG with which to associate the connection.
        #[builder(into)]
        pub lag_id: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct ConnectionAssociationResult {
        /// Pulumi ID is the provider-assigned unique ID for this managed resource.
        /// It is set during deployments and may be missing (unknown) during planning phases.
        pub id: pulumi_gestalt_rust::Output<String>,
        /// The ID of the connection.
        pub connection_id: pulumi_gestalt_rust::Output<String>,
        /// The ID of the LAG with which to associate the connection.
        pub lag_id: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: ConnectionAssociationArgs,
    ) -> ConnectionAssociationResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let connection_id_binding = args.connection_id.get_output(context);
        let lag_id_binding = args.lag_id.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "aws:directconnect/connectionAssociation:ConnectionAssociation"
                .into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "connectionId".into(),
                    value: &connection_id_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "lagId".into(),
                    value: &lag_id_binding.drop_type(),
                },
            ],
        };
        let o = context.register_resource(request);
        ConnectionAssociationResult {
            id: o.get_field("id"),
            connection_id: o.get_field("connectionId"),
            lag_id: o.get_field("lagId"),
        }
    }
}
