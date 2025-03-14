/// ## Example Usage
///
/// ### Network Security Authorization Policy Basic
///
///
/// ```yaml
/// resources:
///   default:
///     type: gcp:networksecurity:AuthorizationPolicy
///     properties:
///       name: my-authorization-policy
///       labels:
///         foo: bar
///       description: my description
///       action: ALLOW
///       rules:
///         - sources:
///             - principals:
///                 - namespace/*
///               ipBlocks:
///                 - 1.2.3.0/24
/// ```
/// ### Network Security Authorization Policy Destinations
///
///
/// ```yaml
/// resources:
///   default:
///     type: gcp:networksecurity:AuthorizationPolicy
///     properties:
///       name: my-authorization-policy
///       labels:
///         foo: bar
///       description: my description
///       action: ALLOW
///       rules:
///         - sources:
///             - principals:
///                 - namespace/*
///               ipBlocks:
///                 - 1.2.3.0/24
///           destinations:
///             - hosts:
///                 - mydomain.*
///               ports:
///                 - 8080
///               methods:
///                 - GET
///               httpHeaderMatch:
///                 headerName: :method
///                 regexMatch: GET
/// ```
///
/// ## Import
///
/// AuthorizationPolicy can be imported using any of these accepted formats:
///
/// * `projects/{{project}}/locations/{{location}}/authorizationPolicies/{{name}}`
///
/// * `{{project}}/{{location}}/{{name}}`
///
/// * `{{location}}/{{name}}`
///
/// When using the `pulumi import` command, AuthorizationPolicy can be imported using one of the formats above. For example:
///
/// ```sh
/// $ pulumi import gcp:networksecurity/authorizationPolicy:AuthorizationPolicy default projects/{{project}}/locations/{{location}}/authorizationPolicies/{{name}}
/// ```
///
/// ```sh
/// $ pulumi import gcp:networksecurity/authorizationPolicy:AuthorizationPolicy default {{project}}/{{location}}/{{name}}
/// ```
///
/// ```sh
/// $ pulumi import gcp:networksecurity/authorizationPolicy:AuthorizationPolicy default {{location}}/{{name}}
/// ```
///
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod authorization_policy {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct AuthorizationPolicyArgs {
        /// The action to take when a rule match is found. Possible values are "ALLOW" or "DENY".
        /// Possible values are: `ALLOW`, `DENY`.
        #[builder(into)]
        pub action: pulumi_gestalt_rust::InputOrOutput<String>,
        /// A free-text description of the resource. Max length 1024 characters.
        #[builder(into, default)]
        pub description: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Set of label tags associated with the AuthorizationPolicy resource.
        /// **Note**: This field is non-authoritative, and will only manage the labels present in your configuration.
        /// Please refer to the field `effective_labels` for all of the labels present on the resource.
        #[builder(into, default)]
        pub labels: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// The location of the authorization policy.
        /// The default value is `global`.
        #[builder(into, default)]
        pub location: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Name of the AuthorizationPolicy resource.
        ///
        ///
        /// - - -
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The ID of the project in which the resource belongs.
        /// If it is not provided, the provider project is used.
        #[builder(into, default)]
        pub project: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// List of rules to match. Note that at least one of the rules must match in order for the action specified in the 'action' field to be taken.
        /// A rule is a match if there is a matching source and destination. If left blank, the action specified in the action field will be applied on every request.
        /// Structure is documented below.
        #[builder(into, default)]
        pub rules: pulumi_gestalt_rust::InputOrOutput<
            Option<Vec<super::super::types::networksecurity::AuthorizationPolicyRule>>,
        >,
    }
    #[allow(dead_code)]
    pub struct AuthorizationPolicyResult {
        /// The action to take when a rule match is found. Possible values are "ALLOW" or "DENY".
        /// Possible values are: `ALLOW`, `DENY`.
        pub action: pulumi_gestalt_rust::Output<String>,
        /// Time the AuthorizationPolicy was created in UTC.
        pub create_time: pulumi_gestalt_rust::Output<String>,
        /// A free-text description of the resource. Max length 1024 characters.
        pub description: pulumi_gestalt_rust::Output<Option<String>>,
        /// All of labels (key/value pairs) present on the resource in GCP, including the labels configured through Pulumi, other clients and services.
        pub effective_labels: pulumi_gestalt_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        /// Set of label tags associated with the AuthorizationPolicy resource.
        /// **Note**: This field is non-authoritative, and will only manage the labels present in your configuration.
        /// Please refer to the field `effective_labels` for all of the labels present on the resource.
        pub labels: pulumi_gestalt_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// The location of the authorization policy.
        /// The default value is `global`.
        pub location: pulumi_gestalt_rust::Output<Option<String>>,
        /// Name of the AuthorizationPolicy resource.
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
        /// List of rules to match. Note that at least one of the rules must match in order for the action specified in the 'action' field to be taken.
        /// A rule is a match if there is a matching source and destination. If left blank, the action specified in the action field will be applied on every request.
        /// Structure is documented below.
        pub rules: pulumi_gestalt_rust::Output<
            Option<Vec<super::super::types::networksecurity::AuthorizationPolicyRule>>,
        >,
        /// Time the AuthorizationPolicy was updated in UTC.
        pub update_time: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: AuthorizationPolicyArgs,
    ) -> AuthorizationPolicyResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let action_binding = args.action.get_output(context);
        let description_binding = args.description.get_output(context);
        let labels_binding = args.labels.get_output(context);
        let location_binding = args.location.get_output(context);
        let name_binding = args.name.get_output(context);
        let project_binding = args.project.get_output(context);
        let rules_binding = args.rules.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "gcp:networksecurity/authorizationPolicy:AuthorizationPolicy".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "action".into(),
                    value: &action_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "description".into(),
                    value: &description_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "labels".into(),
                    value: &labels_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "location".into(),
                    value: &location_binding.drop_type(),
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
                    name: "rules".into(),
                    value: &rules_binding.drop_type(),
                },
            ],
        };
        let o = context.register_resource(request);
        AuthorizationPolicyResult {
            action: o.get_field("action"),
            create_time: o.get_field("createTime"),
            description: o.get_field("description"),
            effective_labels: o.get_field("effectiveLabels"),
            labels: o.get_field("labels"),
            location: o.get_field("location"),
            name: o.get_field("name"),
            project: o.get_field("project"),
            pulumi_labels: o.get_field("pulumiLabels"),
            rules: o.get_field("rules"),
            update_time: o.get_field("updateTime"),
        }
    }
}
