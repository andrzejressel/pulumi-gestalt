/// Manages an EventGrid Domain Topic
///
/// ## Example Usage
///
/// ```yaml
/// resources:
///   example:
///     type: azure:core:ResourceGroup
///     properties:
///       name: example-resources
///       location: West Europe
///   exampleDomain:
///     type: azure:eventgrid:Domain
///     name: example
///     properties:
///       name: my-eventgrid-domain
///       location: ${example.location}
///       resourceGroupName: ${example.name}
///       tags:
///         environment: Production
///   exampleDomainTopic:
///     type: azure:eventgrid:DomainTopic
///     name: example
///     properties:
///       name: my-eventgrid-domain-topic
///       domainName: ${exampleDomain.name}
///       resourceGroupName: ${example.name}
/// ```
///
/// ## Import
///
/// EventGrid Domain Topics can be imported using the `resource id`, e.g.
///
/// ```sh
/// $ pulumi import azure:eventgrid/domainTopic:DomainTopic topic1 /subscriptions/00000000-0000-0000-0000-000000000000/resourceGroups/group1/providers/Microsoft.EventGrid/domains/domain1/topics/topic1
/// ```
///
#[allow(
    clippy::doc_lazy_continuation,
    clippy::tabs_in_doc_comments,
    clippy::should_implement_trait
)]
pub mod domain_topic {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct DomainTopicArgs {
        /// Specifies the name of the EventGrid Domain. Changing this forces a new resource to be created.
        #[builder(into)]
        pub domain_name: pulumi_gestalt_rust::Input<String>,
        /// Specifies the name of the EventGrid Domain Topic resource. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::Input<Option<String>>,
        /// The name of the resource group in which the EventGrid Domain exists. Changing this forces a new resource to be created.
        #[builder(into)]
        pub resource_group_name: pulumi_gestalt_rust::Input<String>,
    }
    #[allow(dead_code)]
    pub struct DomainTopicResult {
        /// Pulumi ID is the provider-assigned unique ID for this managed resource.
        /// It is set during deployments and may be missing (unknown) during planning phases.
        pub id: pulumi_gestalt_rust::Output<String>,
        /// Pulumi URN is the stable logical identity of this resource in the Pulumi stack.
        pub urn: pulumi_gestalt_rust::Output<String>,
        /// Specifies the name of the EventGrid Domain. Changing this forces a new resource to be created.
        pub domain_name: pulumi_gestalt_rust::Output<String>,
        /// Specifies the name of the EventGrid Domain Topic resource. Changing this forces a new resource to be created.
        pub name: pulumi_gestalt_rust::Output<String>,
        /// The name of the resource group in which the EventGrid Domain exists. Changing this forces a new resource to be created.
        pub resource_group_name: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, dead_code)]
    pub fn create(
        ctx: &pulumi_gestalt_rust::Context,
        name: &str,
        args: DomainTopicArgs,
    ) -> DomainTopicResult {
        __create(ctx, name, args, None)
    }
    ///
    /// Same as `create`, but with additional generic options that control the behavior of the resource registration.
    ///
    #[allow(non_snake_case, dead_code)]
    pub fn create_with_options(
        ctx: &pulumi_gestalt_rust::Context,
        name: &str,
        args: DomainTopicArgs,
        options: pulumi_gestalt_rust::CustomResourceOptions,
    ) -> DomainTopicResult {
        __create(ctx, name, args, Some(options))
    }
    #[allow(non_snake_case, unused_imports, dead_code)]
    fn __create(
        ctx: &pulumi_gestalt_rust::Context,
        name: &str,
        args: DomainTopicArgs,
        options: Option<pulumi_gestalt_rust::CustomResourceOptions>,
    ) -> DomainTopicResult {
        let domain_name_binding = args.domain_name.get_output(ctx);
        let name_binding = args.name.get_output(ctx);
        let resource_group_name_binding = args.resource_group_name.get_output(ctx);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "azure:eventgrid/domainTopic:DomainTopic".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "domainName".into(),
                    value: &domain_name_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "name".into(),
                    value: &name_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "resourceGroupName".into(),
                    value: &resource_group_name_binding.drop_type(),
                },
            ],
            options,
        };
        let o = ctx.register_resource(request);
        DomainTopicResult {
            id: o.get_id(),
            urn: o.get_urn(),
            domain_name: o.get_field("domainName"),
            name: o.get_field("name"),
            resource_group_name: o.get_field("resourceGroupName"),
        }
    }
}
