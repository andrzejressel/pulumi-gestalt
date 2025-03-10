/// ## Example Usage
///
/// ### Artifact Registry Vpcsc Config
///
///
/// ```yaml
/// resources:
///   my-config:
///     type: gcp:artifactregistry:VpcscConfig
///     properties:
///       location: us-central1
///       vpcscPolicy: ALLOW
/// ```
///
/// ## Import
///
/// VPCSCConfig can be imported using any of these accepted formats:
///
/// * `projects/{{project}}/locations/{{location}}/vpcscConfig/{{name}}`
///
/// * `{{project}}/{{location}}/{{name}}`
///
/// * `{{location}}/{{name}}`
///
/// When using the `pulumi import` command, VPCSCConfig can be imported using one of the formats above. For example:
///
/// ```sh
/// $ pulumi import gcp:artifactregistry/vpcscConfig:VpcscConfig default projects/{{project}}/locations/{{location}}/vpcscConfig/{{name}}
/// ```
///
/// ```sh
/// $ pulumi import gcp:artifactregistry/vpcscConfig:VpcscConfig default {{project}}/{{location}}/{{name}}
/// ```
///
/// ```sh
/// $ pulumi import gcp:artifactregistry/vpcscConfig:VpcscConfig default {{location}}/{{name}}
/// ```
///
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod vpcsc_config {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct VpcscConfigArgs {
        /// The name of the location this config is located in.
        #[builder(into, default)]
        pub location: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The ID of the project in which the resource belongs.
        /// If it is not provided, the provider project is used.
        #[builder(into, default)]
        pub project: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The VPC SC policy for project and location.
        /// Possible values are: `DENY`, `ALLOW`.
        #[builder(into, default)]
        pub vpcsc_policy: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct VpcscConfigResult {
        /// The name of the location this config is located in.
        pub location: pulumi_gestalt_rust::Output<String>,
        /// The name of the project's VPC SC Config.
        /// Always of the form: projects/{project}/location/{location}/vpcscConfig
        pub name: pulumi_gestalt_rust::Output<String>,
        /// The ID of the project in which the resource belongs.
        /// If it is not provided, the provider project is used.
        pub project: pulumi_gestalt_rust::Output<String>,
        /// The VPC SC policy for project and location.
        /// Possible values are: `DENY`, `ALLOW`.
        pub vpcsc_policy: pulumi_gestalt_rust::Output<Option<String>>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: VpcscConfigArgs,
    ) -> VpcscConfigResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let location_binding = args.location.get_output(context);
        let project_binding = args.project.get_output(context);
        let vpcsc_policy_binding = args.vpcsc_policy.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "gcp:artifactregistry/vpcscConfig:VpcscConfig".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "location".into(),
                    value: &location_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "project".into(),
                    value: &project_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "vpcscPolicy".into(),
                    value: &vpcsc_policy_binding.drop_type(),
                },
            ],
        };
        let o = context.register_resource(request);
        VpcscConfigResult {
            location: o.get_field("location"),
            name: o.get_field("name"),
            project: o.get_field("project"),
            vpcsc_policy: o.get_field("vpcscPolicy"),
        }
    }
}
