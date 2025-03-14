/// A BeyondCorp AppGateway resource represents a BeyondCorp protected AppGateway to a remote application. It creates
/// all the necessary GCP components needed for creating a BeyondCorp protected AppGateway. Multiple connectors can be
/// authorised for a single AppGateway.
///
///
/// To get more information about AppGateway, see:
///
/// * [API documentation](https://cloud.google.com/beyondcorp/docs/reference/rest#rest-resource:-v1.projects.locations.appgateways)
/// * How-to Guides
///     * [Official Documentation](https://cloud.google.com/beyondcorp-enterprise/docs/enable-app-connector)
///
/// ## Example Usage
///
/// ### Beyondcorp App Gateway Basic
///
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let appGateway = app_gateway::create(
///         "appGateway",
///         AppGatewayArgs::builder()
///             .host_type("GCP_REGIONAL_MIG")
///             .name("my-app-gateway")
///             .region("us-central1")
///             .type_("TCP_PROXY")
///             .build_struct(),
///     );
/// }
/// ```
/// ### Beyondcorp App Gateway Full
///
///
/// ```yaml
/// resources:
///   appGateway:
///     type: gcp:beyondcorp:AppGateway
///     name: app_gateway
///     properties:
///       name: my-app-gateway
///       type: TCP_PROXY
///       region: us-central1
///       displayName: some display name
///       labels:
///         foo: bar
///         bar: baz
///       hostType: GCP_REGIONAL_MIG
/// ```
///
/// ## Import
///
/// AppGateway can be imported using any of these accepted formats:
///
/// * `projects/{{project}}/locations/{{region}}/appGateways/{{name}}`
///
/// * `{{project}}/{{region}}/{{name}}`
///
/// * `{{region}}/{{name}}`
///
/// * `{{name}}`
///
/// When using the `pulumi import` command, AppGateway can be imported using one of the formats above. For example:
///
/// ```sh
/// $ pulumi import gcp:beyondcorp/appGateway:AppGateway default projects/{{project}}/locations/{{region}}/appGateways/{{name}}
/// ```
///
/// ```sh
/// $ pulumi import gcp:beyondcorp/appGateway:AppGateway default {{project}}/{{region}}/{{name}}
/// ```
///
/// ```sh
/// $ pulumi import gcp:beyondcorp/appGateway:AppGateway default {{region}}/{{name}}
/// ```
///
/// ```sh
/// $ pulumi import gcp:beyondcorp/appGateway:AppGateway default {{name}}
/// ```
///
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod app_gateway {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct AppGatewayArgs {
        /// An arbitrary user-provided name for the AppGateway.
        #[builder(into, default)]
        pub display_name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The type of hosting used by the AppGateway.
        /// Default value is `HOST_TYPE_UNSPECIFIED`.
        /// Possible values are: `HOST_TYPE_UNSPECIFIED`, `GCP_REGIONAL_MIG`.
        #[builder(into, default)]
        pub host_type: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Resource labels to represent user provided metadata.
        ///
        /// **Note**: This field is non-authoritative, and will only manage the labels present in your configuration.
        /// Please refer to the field `effective_labels` for all of the labels present on the resource.
        #[builder(into, default)]
        pub labels: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// ID of the AppGateway.
        ///
        ///
        /// - - -
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The ID of the project in which the resource belongs.
        /// If it is not provided, the provider project is used.
        #[builder(into, default)]
        pub project: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The region of the AppGateway.
        #[builder(into, default)]
        pub region: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The type of network connectivity used by the AppGateway.
        /// Default value is `TYPE_UNSPECIFIED`.
        /// Possible values are: `TYPE_UNSPECIFIED`, `TCP_PROXY`.
        #[builder(into, default)]
        pub type_: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct AppGatewayResult {
        /// A list of connections allocated for the Gateway.
        /// Structure is documented below.
        pub allocated_connections: pulumi_gestalt_rust::Output<
            Vec<super::super::types::beyondcorp::AppGatewayAllocatedConnection>,
        >,
        /// An arbitrary user-provided name for the AppGateway.
        pub display_name: pulumi_gestalt_rust::Output<Option<String>>,
        /// All of labels (key/value pairs) present on the resource in GCP, including the labels configured through Pulumi, other clients and services.
        pub effective_labels: pulumi_gestalt_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        /// The type of hosting used by the AppGateway.
        /// Default value is `HOST_TYPE_UNSPECIFIED`.
        /// Possible values are: `HOST_TYPE_UNSPECIFIED`, `GCP_REGIONAL_MIG`.
        pub host_type: pulumi_gestalt_rust::Output<Option<String>>,
        /// Resource labels to represent user provided metadata.
        ///
        /// **Note**: This field is non-authoritative, and will only manage the labels present in your configuration.
        /// Please refer to the field `effective_labels` for all of the labels present on the resource.
        pub labels: pulumi_gestalt_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// ID of the AppGateway.
        ///
        ///
        /// - - -
        pub name: pulumi_gestalt_rust::Output<String>,
        /// The ID of the project in which the resource belongs.
        /// If it is not provided, the provider project is used.
        pub project: pulumi_gestalt_rust::Output<String>,
        /// The combination of labels configured directly on the resource
        /// and default labels configured on the provider.
        pub pulumi_labels: pulumi_gestalt_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        /// The region of the AppGateway.
        pub region: pulumi_gestalt_rust::Output<Option<String>>,
        /// Represents the different states of a AppGateway.
        pub state: pulumi_gestalt_rust::Output<String>,
        /// The type of network connectivity used by the AppGateway.
        /// Default value is `TYPE_UNSPECIFIED`.
        /// Possible values are: `TYPE_UNSPECIFIED`, `TCP_PROXY`.
        pub type_: pulumi_gestalt_rust::Output<Option<String>>,
        /// Server-defined URI for this resource.
        pub uri: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: AppGatewayArgs,
    ) -> AppGatewayResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let display_name_binding = args.display_name.get_output(context);
        let host_type_binding = args.host_type.get_output(context);
        let labels_binding = args.labels.get_output(context);
        let name_binding = args.name.get_output(context);
        let project_binding = args.project.get_output(context);
        let region_binding = args.region.get_output(context);
        let type__binding = args.type_.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "gcp:beyondcorp/appGateway:AppGateway".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "displayName".into(),
                    value: &display_name_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "hostType".into(),
                    value: &host_type_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "labels".into(),
                    value: &labels_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "name".into(),
                    value: &name_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "project".into(),
                    value: &project_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "region".into(),
                    value: &region_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "type".into(),
                    value: &type__binding.drop_type(),
                },
            ],
        };
        let o = context.register_resource(request);
        AppGatewayResult {
            allocated_connections: o.get_field("allocatedConnections"),
            display_name: o.get_field("displayName"),
            effective_labels: o.get_field("effectiveLabels"),
            host_type: o.get_field("hostType"),
            labels: o.get_field("labels"),
            name: o.get_field("name"),
            project: o.get_field("project"),
            pulumi_labels: o.get_field("pulumiLabels"),
            region: o.get_field("region"),
            state: o.get_field("state"),
            type_: o.get_field("type"),
            uri: o.get_field("uri"),
        }
    }
}
