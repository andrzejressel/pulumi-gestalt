/// Creates a Microsoft AD domain
///
///
/// To get more information about Domain, see:
///
/// * [API documentation](https://cloud.google.com/managed-microsoft-ad/reference/rest/v1/projects.locations.global.domains)
/// * How-to Guides
///     * [Managed Microsoft Active Directory Quickstart](https://cloud.google.com/managed-microsoft-ad/docs/quickstarts)
///
/// ## Example Usage
///
/// ### Active Directory Domain Basic
///
///
/// ```yaml
/// resources:
///   ad-domain:
///     type: gcp:activedirectory:Domain
///     properties:
///       domainName: tfgen.org.com
///       locations:
///         - us-central1
///       reservedIpRange: 192.168.255.0/24
///       deletionProtection: false
/// ```
///
/// ## Import
///
/// Domain can be imported using any of these accepted formats:
///
/// * `{{project}}/{{name}}`
///
/// * `{{project}} {{name}}`
///
/// * `{{name}}`
///
/// When using the `pulumi import` command, Domain can be imported using one of the formats above. For example:
///
/// ```sh
/// $ pulumi import gcp:activedirectory/domain:Domain default {{project}}/{{name}}
/// ```
///
/// ```sh
/// $ pulumi import gcp:activedirectory/domain:Domain default "{{project}} {{name}}"
/// ```
///
/// ```sh
/// $ pulumi import gcp:activedirectory/domain:Domain default {{name}}
/// ```
///
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod domain {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct DomainArgs {
        /// The name of delegated administrator account used to perform Active Directory operations.
        /// If not specified, setupadmin will be used.
        #[builder(into, default)]
        pub admin: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The full names of the Google Compute Engine networks the domain instance is connected to. The domain is only available on networks listed in authorizedNetworks.
        /// If CIDR subnets overlap between networks, domain creation will fail.
        #[builder(into, default)]
        pub authorized_networks: pulumi_gestalt_rust::InputOrOutput<Option<Vec<String>>>,
        #[builder(into, default)]
        pub deletion_protection: pulumi_gestalt_rust::InputOrOutput<Option<bool>>,
        /// The fully qualified domain name. e.g. mydomain.myorganization.com, with the restrictions
        /// of https://cloud.google.com/managed-microsoft-ad/reference/rest/v1/projects.locations.global.domains.
        ///
        ///
        /// - - -
        #[builder(into)]
        pub domain_name: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Resource labels that can contain user-provided metadata
        /// **Note**: This field is non-authoritative, and will only manage the labels present in your configuration.
        /// Please refer to the field `effective_labels` for all of the labels present on the resource.
        #[builder(into, default)]
        pub labels: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// Locations where domain needs to be provisioned. [regions][compute/docs/regions-zones/]
        /// e.g. us-west1 or us-east4 Service supports up to 4 locations at once. Each location will use a /26 block.
        #[builder(into)]
        pub locations: pulumi_gestalt_rust::InputOrOutput<Vec<String>>,
        /// The ID of the project in which the resource belongs.
        /// If it is not provided, the provider project is used.
        #[builder(into, default)]
        pub project: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The CIDR range of internal addresses that are reserved for this domain. Reserved networks must be /24 or larger.
        /// Ranges must be unique and non-overlapping with existing subnets in authorizedNetworks
        #[builder(into)]
        pub reserved_ip_range: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct DomainResult {
        /// The name of delegated administrator account used to perform Active Directory operations.
        /// If not specified, setupadmin will be used.
        pub admin: pulumi_gestalt_rust::Output<Option<String>>,
        /// The full names of the Google Compute Engine networks the domain instance is connected to. The domain is only available on networks listed in authorizedNetworks.
        /// If CIDR subnets overlap between networks, domain creation will fail.
        pub authorized_networks: pulumi_gestalt_rust::Output<Option<Vec<String>>>,
        pub deletion_protection: pulumi_gestalt_rust::Output<Option<bool>>,
        /// The fully qualified domain name. e.g. mydomain.myorganization.com, with the restrictions
        /// of https://cloud.google.com/managed-microsoft-ad/reference/rest/v1/projects.locations.global.domains.
        ///
        ///
        /// - - -
        pub domain_name: pulumi_gestalt_rust::Output<String>,
        /// All of labels (key/value pairs) present on the resource in GCP, including the labels configured through Pulumi, other clients and services.
        pub effective_labels: pulumi_gestalt_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        /// The fully-qualified domain name of the exposed domain used by clients to connect to the service.
        /// Similar to what would be chosen for an Active Directory set up on an internal network.
        pub fqdn: pulumi_gestalt_rust::Output<String>,
        /// Resource labels that can contain user-provided metadata
        /// **Note**: This field is non-authoritative, and will only manage the labels present in your configuration.
        /// Please refer to the field `effective_labels` for all of the labels present on the resource.
        pub labels: pulumi_gestalt_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// Locations where domain needs to be provisioned. [regions][compute/docs/regions-zones/]
        /// e.g. us-west1 or us-east4 Service supports up to 4 locations at once. Each location will use a /26 block.
        pub locations: pulumi_gestalt_rust::Output<Vec<String>>,
        /// The unique name of the domain using the format: `projects/{project}/locations/global/domains/{domainName}`.
        pub name: pulumi_gestalt_rust::Output<String>,
        /// The ID of the project in which the resource belongs.
        /// If it is not provided, the provider project is used.
        pub project: pulumi_gestalt_rust::Output<String>,
        /// The combination of labels configured directly on the resource
        /// and default labels configured on the provider.
        pub pulumi_labels: pulumi_gestalt_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        /// The CIDR range of internal addresses that are reserved for this domain. Reserved networks must be /24 or larger.
        /// Ranges must be unique and non-overlapping with existing subnets in authorizedNetworks
        pub reserved_ip_range: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: DomainArgs,
    ) -> DomainResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let admin_binding = args.admin.get_output(context);
        let authorized_networks_binding = args.authorized_networks.get_output(context);
        let deletion_protection_binding = args.deletion_protection.get_output(context);
        let domain_name_binding = args.domain_name.get_output(context);
        let labels_binding = args.labels.get_output(context);
        let locations_binding = args.locations.get_output(context);
        let project_binding = args.project.get_output(context);
        let reserved_ip_range_binding = args.reserved_ip_range.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "gcp:activedirectory/domain:Domain".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "admin".into(),
                    value: &admin_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "authorizedNetworks".into(),
                    value: &authorized_networks_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "deletionProtection".into(),
                    value: &deletion_protection_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "domainName".into(),
                    value: &domain_name_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "labels".into(),
                    value: &labels_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "locations".into(),
                    value: &locations_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "project".into(),
                    value: &project_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "reservedIpRange".into(),
                    value: &reserved_ip_range_binding.drop_type(),
                },
            ],
        };
        let o = context.register_resource(request);
        DomainResult {
            admin: o.get_field("admin"),
            authorized_networks: o.get_field("authorizedNetworks"),
            deletion_protection: o.get_field("deletionProtection"),
            domain_name: o.get_field("domainName"),
            effective_labels: o.get_field("effectiveLabels"),
            fqdn: o.get_field("fqdn"),
            labels: o.get_field("labels"),
            locations: o.get_field("locations"),
            name: o.get_field("name"),
            project: o.get_field("project"),
            pulumi_labels: o.get_field("pulumiLabels"),
            reserved_ip_range: o.get_field("reservedIpRange"),
        }
    }
}
