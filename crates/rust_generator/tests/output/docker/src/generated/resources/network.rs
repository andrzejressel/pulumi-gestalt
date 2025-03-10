/// <!-- Bug: Type and Name are switched -->
/// `docker.Network` provides a docker network resource.
///
/// ## Example Usage
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let privateNetwork = network::create(
///         "privateNetwork",
///         NetworkArgs::builder().build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// ### Example
///
/// Assuming you created a `network` as follows
///
/// ```shell
/// docker network create foo
/// ````
///
/// prints the long ID
///
/// ```text
/// 87b57a9b91ecab2db2a6dbf38df74c67d7c7108cbe479d6576574ec2cd8c2d73
/// ```
///
/// you provide the definition for the resource as follows
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let foo = network::create("foo", NetworkArgs::builder().name("foo").build_struct());
/// }
/// ```
///
/// then the import command is as follows
///
/// ```sh
/// $ pulumi import docker:index/network:Network foo 87b57a9b91ecab2db2a6dbf38df74c67d7c7108cbe479d6576574ec2cd8c2d73
/// ```
///
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod network {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct NetworkArgs {
        /// Enable manual container attachment to the network.
        #[builder(into, default)]
        pub attachable: pulumi_gestalt_rust::InputOrOutput<Option<bool>>,
        /// Requests daemon to check for networks with same name.
        #[builder(into, default)]
        pub check_duplicate: pulumi_gestalt_rust::InputOrOutput<Option<bool>>,
        /// The driver of the Docker network. Possible values are `bridge`, `host`, `overlay`, `macvlan`. See [network docs](https://docs.docker.com/network/#network-drivers) for more details.
        #[builder(into, default)]
        pub driver: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Create swarm routing-mesh network. Defaults to `false`.
        #[builder(into, default)]
        pub ingress: pulumi_gestalt_rust::InputOrOutput<Option<bool>>,
        /// Whether the network is internal.
        #[builder(into, default)]
        pub internal: pulumi_gestalt_rust::InputOrOutput<Option<bool>>,
        /// The IPAM configuration options
        #[builder(into, default)]
        pub ipam_configs: pulumi_gestalt_rust::InputOrOutput<
            Option<Vec<super::types::NetworkIpamConfig>>,
        >,
        /// Driver used by the custom IP scheme of the network. Defaults to `default`
        #[builder(into, default)]
        pub ipam_driver: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Provide explicit options to the IPAM driver. Valid options vary with `ipam_driver` and refer to that driver's documentation for more details.
        #[builder(into, default)]
        pub ipam_options: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// Enable IPv6 networking. Defaults to `false`.
        #[builder(into, default)]
        pub ipv6: pulumi_gestalt_rust::InputOrOutput<Option<bool>>,
        /// User-defined key/value metadata
        #[builder(into, default)]
        pub labels: pulumi_gestalt_rust::InputOrOutput<
            Option<Vec<super::types::NetworkLabel>>,
        >,
        /// The name of the Docker network.
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Only available with bridge networks. See [bridge options docs](https://docs.docker.com/engine/reference/commandline/network_create/#bridge-driver-options) for more details.
        #[builder(into, default)]
        pub options: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
    }
    #[allow(dead_code)]
    pub struct NetworkResult {
        /// Enable manual container attachment to the network.
        pub attachable: pulumi_gestalt_rust::Output<Option<bool>>,
        /// Requests daemon to check for networks with same name.
        pub check_duplicate: pulumi_gestalt_rust::Output<Option<bool>>,
        /// The driver of the Docker network. Possible values are `bridge`, `host`, `overlay`, `macvlan`. See [network docs](https://docs.docker.com/network/#network-drivers) for more details.
        pub driver: pulumi_gestalt_rust::Output<String>,
        /// Create swarm routing-mesh network. Defaults to `false`.
        pub ingress: pulumi_gestalt_rust::Output<Option<bool>>,
        /// Whether the network is internal.
        pub internal: pulumi_gestalt_rust::Output<bool>,
        /// The IPAM configuration options
        pub ipam_configs: pulumi_gestalt_rust::Output<
            Vec<super::types::NetworkIpamConfig>,
        >,
        /// Driver used by the custom IP scheme of the network. Defaults to `default`
        pub ipam_driver: pulumi_gestalt_rust::Output<Option<String>>,
        /// Provide explicit options to the IPAM driver. Valid options vary with `ipam_driver` and refer to that driver's documentation for more details.
        pub ipam_options: pulumi_gestalt_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// Enable IPv6 networking. Defaults to `false`.
        pub ipv6: pulumi_gestalt_rust::Output<Option<bool>>,
        /// User-defined key/value metadata
        pub labels: pulumi_gestalt_rust::Output<Option<Vec<super::types::NetworkLabel>>>,
        /// The name of the Docker network.
        pub name: pulumi_gestalt_rust::Output<String>,
        /// Only available with bridge networks. See [bridge options docs](https://docs.docker.com/engine/reference/commandline/network_create/#bridge-driver-options) for more details.
        pub options: pulumi_gestalt_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        /// Scope of the network. One of `swarm`, `global`, or `local`.
        pub scope: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: NetworkArgs,
    ) -> NetworkResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let attachable_binding = args.attachable.get_output(context);
        let check_duplicate_binding = args.check_duplicate.get_output(context);
        let driver_binding = args.driver.get_output(context);
        let ingress_binding = args.ingress.get_output(context);
        let internal_binding = args.internal.get_output(context);
        let ipam_configs_binding = args.ipam_configs.get_output(context);
        let ipam_driver_binding = args.ipam_driver.get_output(context);
        let ipam_options_binding = args.ipam_options.get_output(context);
        let ipv6_binding = args.ipv6.get_output(context);
        let labels_binding = args.labels.get_output(context);
        let name_binding = args.name.get_output(context);
        let options_binding = args.options.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "docker:index/network:Network".into(),
            name: name.to_string(),
            version: super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "attachable".into(),
                    value: &attachable_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "checkDuplicate".into(),
                    value: &check_duplicate_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "driver".into(),
                    value: &driver_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "ingress".into(),
                    value: &ingress_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "internal".into(),
                    value: &internal_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "ipamConfigs".into(),
                    value: &ipam_configs_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "ipamDriver".into(),
                    value: &ipam_driver_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "ipamOptions".into(),
                    value: &ipam_options_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "ipv6".into(),
                    value: &ipv6_binding.drop_type(),
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
                    name: "options".into(),
                    value: &options_binding.drop_type(),
                },
            ],
        };
        let o = context.register_resource(request);
        NetworkResult {
            attachable: o.get_field("attachable"),
            check_duplicate: o.get_field("checkDuplicate"),
            driver: o.get_field("driver"),
            ingress: o.get_field("ingress"),
            internal: o.get_field("internal"),
            ipam_configs: o.get_field("ipamConfigs"),
            ipam_driver: o.get_field("ipamDriver"),
            ipam_options: o.get_field("ipamOptions"),
            ipv6: o.get_field("ipv6"),
            labels: o.get_field("labels"),
            name: o.get_field("name"),
            options: o.get_field("options"),
            scope: o.get_field("scope"),
        }
    }
}
