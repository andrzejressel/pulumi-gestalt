/// A collection of resources that are deployed and managed together using
/// a configuration file
///
///
///
/// > **Warning:** This resource is intended only to manage a Deployment resource,
/// and attempts to manage the Deployment's resources in the provider as well
/// will likely result in errors or unexpected behavior as the two tools
/// fight over ownership. We strongly discourage doing so unless you are an
/// experienced user of both tools.
///
/// In addition, due to limitations of the API, the provider will treat
/// deployments in preview as recreate-only for any update operation other
/// than actually deploying an in-preview deployment (i.e. `preview=true` to
/// `preview=false`).
///
/// ## Example Usage
///
/// ### Deployment Manager Deployment Basic
///
///
/// ```yaml
/// resources:
///   deployment:
///     type: gcp:deploymentmanager:Deployment
///     properties:
///       name: my-deployment
///       target:
///         config:
///           content:
///             fn::invoke:
///               function: std:file
///               arguments:
///                 input: path/to/config.yml
///               return: result
///       labels:
///         - key: foo
///           value: bar
/// ```
/// ## Import
///
/// Deployment can be imported using any of these accepted formats:
///
/// * `projects/{{project}}/deployments/{{name}}`
///
/// * `{{project}}/{{name}}`
///
/// * `{{name}}`
///
/// When using the `pulumi import` command, Deployment can be imported using one of the formats above. For example:
///
/// ```sh
/// $ pulumi import gcp:deploymentmanager/deployment:Deployment default projects/{{project}}/deployments/{{name}}
/// ```
///
/// ```sh
/// $ pulumi import gcp:deploymentmanager/deployment:Deployment default {{project}}/{{name}}
/// ```
///
/// ```sh
/// $ pulumi import gcp:deploymentmanager/deployment:Deployment default {{name}}
/// ```
///
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod deployment {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct DeploymentArgs {
        /// Set the policy to use for creating new resources. Only used on create and update. Valid values are 'CREATE_OR_ACQUIRE'
        /// (default) or 'ACQUIRE'. If set to 'ACQUIRE' and resources do not already exist, the deployment will fail. Note that
        /// updating this field does not actually affect the deployment, just how it is updated. Default value: "CREATE_OR_ACQUIRE"
        /// Possible values: ["ACQUIRE", "CREATE_OR_ACQUIRE"]
        #[builder(into, default)]
        pub create_policy: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Set the policy to use for deleting new resources on update/delete. Valid values are 'DELETE' (default) or 'ABANDON'. If
        /// 'DELETE', resource is deleted after removal from Deployment Manager. If 'ABANDON', the resource is only removed from
        /// Deployment Manager and is not actually deleted. Note that updating this field does not actually change the deployment,
        /// just how it is updated. Default value: "DELETE" Possible values: ["ABANDON", "DELETE"]
        #[builder(into, default)]
        pub delete_policy: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Optional user-provided description of deployment.
        #[builder(into, default)]
        pub description: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Key-value pairs to apply to this labels.
        #[builder(into, default)]
        pub labels: pulumi_gestalt_rust::InputOrOutput<
            Option<Vec<super::super::types::deploymentmanager::DeploymentLabel>>,
        >,
        /// Unique name for the deployment
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        #[builder(into, default)]
        pub preview: pulumi_gestalt_rust::InputOrOutput<Option<bool>>,
        #[builder(into, default)]
        pub project: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Parameters that define your deployment, including the deployment
        /// configuration and relevant templates.
        /// Structure is documented below.
        #[builder(into)]
        pub target: pulumi_gestalt_rust::InputOrOutput<
            super::super::types::deploymentmanager::DeploymentTarget,
        >,
    }
    #[allow(dead_code)]
    pub struct DeploymentResult {
        /// Set the policy to use for creating new resources. Only used on create and update. Valid values are 'CREATE_OR_ACQUIRE'
        /// (default) or 'ACQUIRE'. If set to 'ACQUIRE' and resources do not already exist, the deployment will fail. Note that
        /// updating this field does not actually affect the deployment, just how it is updated. Default value: "CREATE_OR_ACQUIRE"
        /// Possible values: ["ACQUIRE", "CREATE_OR_ACQUIRE"]
        pub create_policy: pulumi_gestalt_rust::Output<Option<String>>,
        /// Set the policy to use for deleting new resources on update/delete. Valid values are 'DELETE' (default) or 'ABANDON'. If
        /// 'DELETE', resource is deleted after removal from Deployment Manager. If 'ABANDON', the resource is only removed from
        /// Deployment Manager and is not actually deleted. Note that updating this field does not actually change the deployment,
        /// just how it is updated. Default value: "DELETE" Possible values: ["ABANDON", "DELETE"]
        pub delete_policy: pulumi_gestalt_rust::Output<Option<String>>,
        /// Unique identifier for deployment. Output only.
        pub deployment_id: pulumi_gestalt_rust::Output<String>,
        /// Optional user-provided description of deployment.
        pub description: pulumi_gestalt_rust::Output<Option<String>>,
        /// Key-value pairs to apply to this labels.
        pub labels: pulumi_gestalt_rust::Output<
            Option<Vec<super::super::types::deploymentmanager::DeploymentLabel>>,
        >,
        /// Output only. URL of the manifest representing the last manifest that
        /// was successfully deployed.
        pub manifest: pulumi_gestalt_rust::Output<String>,
        /// Unique name for the deployment
        pub name: pulumi_gestalt_rust::Output<String>,
        pub preview: pulumi_gestalt_rust::Output<Option<bool>>,
        pub project: pulumi_gestalt_rust::Output<String>,
        /// Output only. Server defined URL for the resource.
        pub self_link: pulumi_gestalt_rust::Output<String>,
        /// Parameters that define your deployment, including the deployment
        /// configuration and relevant templates.
        /// Structure is documented below.
        pub target: pulumi_gestalt_rust::Output<
            super::super::types::deploymentmanager::DeploymentTarget,
        >,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: DeploymentArgs,
    ) -> DeploymentResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let create_policy_binding = args.create_policy.get_output(context);
        let delete_policy_binding = args.delete_policy.get_output(context);
        let description_binding = args.description.get_output(context);
        let labels_binding = args.labels.get_output(context);
        let name_binding = args.name.get_output(context);
        let preview_binding = args.preview.get_output(context);
        let project_binding = args.project.get_output(context);
        let target_binding = args.target.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "gcp:deploymentmanager/deployment:Deployment".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "createPolicy".into(),
                    value: &create_policy_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "deletePolicy".into(),
                    value: &delete_policy_binding.drop_type(),
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
                    name: "name".into(),
                    value: &name_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "preview".into(),
                    value: &preview_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "project".into(),
                    value: &project_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "target".into(),
                    value: &target_binding.drop_type(),
                },
            ],
        };
        let o = context.register_resource(request);
        DeploymentResult {
            create_policy: o.get_field("createPolicy"),
            delete_policy: o.get_field("deletePolicy"),
            deployment_id: o.get_field("deploymentId"),
            description: o.get_field("description"),
            labels: o.get_field("labels"),
            manifest: o.get_field("manifest"),
            name: o.get_field("name"),
            preview: o.get_field("preview"),
            project: o.get_field("project"),
            self_link: o.get_field("selfLink"),
            target: o.get_field("target"),
        }
    }
}
