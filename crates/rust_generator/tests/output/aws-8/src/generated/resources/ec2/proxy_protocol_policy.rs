/// Provides a proxy protocol policy, which allows an ELB to carry a client connection information to a backend.
///
/// ## Example Usage
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let lb = load_balancer::create(
///         "lb",
///         LoadBalancerArgs::builder()
///             .availability_zones(vec!["us-east-1a",])
///             .listeners(
///                 vec![
///                     LoadBalancerListener::builder().instancePort(25)
///                     .instanceProtocol("tcp").lbPort(25).lbProtocol("tcp").build_struct(),
///                     LoadBalancerListener::builder().instancePort(587)
///                     .instanceProtocol("tcp").lbPort(587).lbProtocol("tcp")
///                     .build_struct(),
///                 ],
///             )
///             .name("test-lb")
///             .build_struct(),
///     );
///     let smtp = proxy_protocol_policy::create(
///         "smtp",
///         ProxyProtocolPolicyArgs::builder()
///             .instance_ports(vec!["25", "587",])
///             .load_balancer("${lb.name}")
///             .build_struct(),
///     );
/// }
/// ```
#[allow(
    clippy::doc_lazy_continuation,
    clippy::tabs_in_doc_comments,
    clippy::should_implement_trait
)]
pub mod proxy_protocol_policy {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct ProxyProtocolPolicyArgs {
        /// List of instance ports to which the policy
        /// should be applied. This can be specified if the protocol is SSL or TCP.
        #[builder(into)]
        pub instance_ports: pulumi_gestalt_rust::InputOrOutput<Vec<String>>,
        /// The load balancer to which the policy
        /// should be attached.
        #[builder(into)]
        pub load_balancer: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct ProxyProtocolPolicyResult {
        /// Pulumi ID is the provider-assigned unique ID for this managed resource.
        /// It is set during deployments and may be missing (unknown) during planning phases.
        pub id: pulumi_gestalt_rust::Output<String>,
        /// Pulumi URN is the stable logical identity of this resource in the Pulumi stack.
        pub urn: pulumi_gestalt_rust::Output<String>,
        /// List of instance ports to which the policy
        /// should be applied. This can be specified if the protocol is SSL or TCP.
        pub instance_ports: pulumi_gestalt_rust::Output<Vec<String>>,
        /// The load balancer to which the policy
        /// should be attached.
        pub load_balancer: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, dead_code)]
    pub fn create(
        ctx: &pulumi_gestalt_rust::Context,
        name: &str,
        args: ProxyProtocolPolicyArgs,
    ) -> ProxyProtocolPolicyResult {
        __create(ctx, name, args, None)
    }
    ///
    /// Same as `create`, but with additional generic options that control the behavior of the resource registration.
    ///
    #[allow(non_snake_case, dead_code)]
    pub fn create_with_options(
        ctx: &pulumi_gestalt_rust::Context,
        name: &str,
        args: ProxyProtocolPolicyArgs,
        options: pulumi_gestalt_rust::CustomResourceOptions,
    ) -> ProxyProtocolPolicyResult {
        __create(ctx, name, args, Some(options))
    }
    #[allow(non_snake_case, unused_imports, dead_code)]
    fn __create(
        ctx: &pulumi_gestalt_rust::Context,
        name: &str,
        args: ProxyProtocolPolicyArgs,
        options: Option<pulumi_gestalt_rust::CustomResourceOptions>,
    ) -> ProxyProtocolPolicyResult {
        let instance_ports_binding = args.instance_ports.get_output(ctx);
        let load_balancer_binding = args.load_balancer.get_output(ctx);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "aws:ec2/proxyProtocolPolicy:ProxyProtocolPolicy".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "instancePorts".into(),
                    value: &instance_ports_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "loadBalancer".into(),
                    value: &load_balancer_binding.drop_type(),
                },
            ],
            options,
        };
        let o = ctx.register_resource(request);
        ProxyProtocolPolicyResult {
            id: o.get_id(),
            urn: o.get_urn(),
            instance_ports: o.get_field("instancePorts"),
            load_balancer: o.get_field("loadBalancer"),
        }
    }
}
