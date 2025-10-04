/// Allows configuring a single GCP resource that should be inside the `status` block of a service perimeter.
/// This resource is intended to be used in cases where it is not possible to compile a full list
/// of projects to include in a `gcp.accesscontextmanager.ServicePerimeter` resource,
/// to enable them to be added separately.
/// If your perimeter is in dry-run mode use `gcp.accesscontextmanager.ServicePerimeterDryRunResource` instead.
///
/// > **Note:** If this resource is used alongside a `gcp.accesscontextmanager.ServicePerimeter` resource,
/// the service perimeter resource must have a `lifecycle` block with `ignore_changes = [status[0].resources]` so
/// they don't fight over which resources should be in the policy.
///
///
/// To get more information about ServicePerimeterResource, see:
///
/// * [API documentation](https://cloud.google.com/access-context-manager/docs/reference/rest/v1/accessPolicies.servicePerimeters)
/// * How-to Guides
///     * [Service Perimeter Quickstart](https://cloud.google.com/vpc-service-controls/docs/quickstart)
///
/// > **Warning:** If you are using User ADCs (Application Default Credentials) with this resource,
/// you must specify a `billing_project` and set `user_project_override` to true
/// in the provider configuration. Otherwise the ACM API will return a 403 error.
/// Your account must have the `serviceusage.services.use` permission on the
/// `billing_project` you defined.
///
/// ## Example Usage
///
/// ### Access Context Manager Service Perimeter Resource Basic
///
///
/// ```yaml
/// resources:
///   service-perimeter-resource:
///     type: gcp:accesscontextmanager:ServicePerimeterResource
///     properties:
///       perimeterName: ${["service-perimeter-resourceServicePerimeter"].name}
///       resource: projects/987654321
///   service-perimeter-resourceServicePerimeter:
///     type: gcp:accesscontextmanager:ServicePerimeter
///     name: service-perimeter-resource
///     properties:
///       parent: accessPolicies/${["access-policy"].name}
///       name: accessPolicies/${["access-policy"].name}/servicePerimeters/restrict_all
///       title: restrict_all
///       status:
///         restrictedServices:
///           - storage.googleapis.com
///   access-policy:
///     type: gcp:accesscontextmanager:AccessPolicy
///     properties:
///       parent: organizations/123456789
///       title: my policy
/// ```
///
/// ## Import
///
/// ServicePerimeterResource can be imported using any of these accepted formats:
///
/// * `{{perimeter_name}}/{{resource}}`
///
/// When using the `pulumi import` command, ServicePerimeterResource can be imported using one of the formats above. For example:
///
/// ```sh
/// $ pulumi import gcp:accesscontextmanager/servicePerimeterResource:ServicePerimeterResource default {{perimeter_name}}/{{resource}}
/// ```
///
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod service_perimeter_resource {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct ServicePerimeterResourceArgs {
        /// The name of the Service Perimeter to add this resource to.
        ///
        ///
        /// - - -
        #[builder(into)]
        pub perimeter_name: pulumi_gestalt_rust::InputOrOutput<String>,
        /// A GCP resource that is inside of the service perimeter.
        /// Currently only projects are allowed.
        /// Format: projects/{project_number}
        #[builder(into)]
        pub resource: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct ServicePerimeterResourceResult {
        /// Pulumi ID is the provider-assigned unique ID for this managed resource.
        /// It is set during deployments and may be missing (unknown) during planning phases.
        pub id: pulumi_gestalt_rust::Output<String>,
        /// The name of the Service Perimeter to add this resource to.
        ///
        ///
        /// - - -
        pub perimeter_name: pulumi_gestalt_rust::Output<String>,
        /// A GCP resource that is inside of the service perimeter.
        /// Currently only projects are allowed.
        /// Format: projects/{project_number}
        pub resource: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: ServicePerimeterResourceArgs,
    ) -> ServicePerimeterResourceResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let perimeter_name_binding = args.perimeter_name.get_output(context);
        let resource_binding = args.resource.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "gcp:accesscontextmanager/servicePerimeterResource:ServicePerimeterResource"
                .into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "perimeterName".into(),
                    value: &perimeter_name_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "resource".into(),
                    value: &resource_binding.drop_type(),
                },
            ],
        };
        let o = context.register_resource(request);
        ServicePerimeterResourceResult {
            id: o.get_field("id"),
            perimeter_name: o.get_field("perimeterName"),
            resource: o.get_field("resource"),
        }
    }
}
