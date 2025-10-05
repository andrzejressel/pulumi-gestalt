/// Allows setting policy to an OpenSearch domain while referencing domain attributes (e.g., ARN).
///
/// ## Example Usage
///
/// ```yaml
/// resources:
///   example:
///     type: aws:opensearch:Domain
///     properties:
///       domainName: tf-test
///       engineVersion: OpenSearch_1.1
///   mainDomainPolicy:
///     type: aws:opensearch:DomainPolicy
///     name: main
///     properties:
///       domainName: ${example.domainName}
///       accessPolicies: ${main.json}
/// variables:
///   main:
///     fn::invoke:
///       function: aws:iam:getPolicyDocument
///       arguments:
///         statements:
///           - effect: Allow
///             principals:
///               - type: '*'
///                 identifiers:
///                   - '*'
///             actions:
///               - es:*
///             resources:
///               - ${example.arn}/*
///             conditions:
///               - test: IpAddress
///                 variable: aws:SourceIp
///                 values:
///                   - 127.0.0.1/32
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
            type_: "aws:opensearch/domainPolicy:DomainPolicy".into(),
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
