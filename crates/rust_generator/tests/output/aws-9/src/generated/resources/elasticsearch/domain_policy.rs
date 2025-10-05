/// Allows setting policy to an Elasticsearch domain while referencing domain attributes (e.g., ARN)
///
/// ## Example Usage
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let example = domain::create(
///         "example",
///         DomainArgs::builder()
///             .domain_name("tf-test")
///             .elasticsearch_version("2.3")
///             .build_struct(),
///     );
///     let main = domain_policy::create(
///         "main",
///         DomainPolicyArgs::builder()
///             .access_policies(
///                 "{\n    \"Version\": \"2012-10-17\",\n    \"Statement\": [\n        {\n            \"Action\": \"es:*\",\n            \"Principal\": \"*\",\n            \"Effect\": \"Allow\",\n            \"Condition\": {\n                \"IpAddress\": {\"aws:SourceIp\": \"127.0.0.1/32\"}\n            },\n            \"Resource\": \"${example.arn}/*\"\n        }\n    ]\n}",
///             )
///             .domain_name("${example.domainName}")
///             .build_struct(),
///     );
/// }
/// ```
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod domain_policy {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct DomainPolicyArgs {
        /// IAM policy document specifying the access policies for the domain
        #[builder(into)]
        pub access_policies: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Name of the domain.
        #[builder(into)]
        pub domain_name: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct DomainPolicyResult {
        /// Pulumi ID is the provider-assigned unique ID for this managed resource.
        /// It is set during deployments and may be missing (unknown) during planning phases.
        pub id: pulumi_gestalt_rust::Output<String>,
        /// IAM policy document specifying the access policies for the domain
        pub access_policies: pulumi_gestalt_rust::Output<String>,
        /// Name of the domain.
        pub domain_name: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: DomainPolicyArgs,
    ) -> DomainPolicyResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let access_policies_binding = args.access_policies.get_output(context);
        let domain_name_binding = args.domain_name.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "aws:elasticsearch/domainPolicy:DomainPolicy".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "accessPolicies".into(),
                    value: &access_policies_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "domainName".into(),
                    value: &domain_name_binding.drop_type(),
                },
            ],
        };
        let o = context.register_resource(request);
        DomainPolicyResult {
            id: o.get_field("id"),
            access_policies: o.get_field("accessPolicies"),
            domain_name: o.get_field("domainName"),
        }
    }
}
