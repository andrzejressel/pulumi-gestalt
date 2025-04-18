#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod get_web_app {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetWebAppArgs {
        /// The app_ip of name of the Firebase webApp.
        ///
        ///
        /// - - -
        #[builder(into)]
        pub app_id: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The ID of the project in which the resource belongs.
        /// If it is not provided, the provider project is used.
        #[builder(into, default)]
        pub project: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct GetWebAppResult {
        pub api_key_id: pulumi_gestalt_rust::Output<String>,
        /// Immutable. The globally unique, Firebase-assigned identifier of the App.
        /// This identifier should be treated as an opaque token, as the data format is not specified.
        pub app_id: pulumi_gestalt_rust::Output<String>,
        pub app_urls: pulumi_gestalt_rust::Output<Vec<String>>,
        pub deletion_policy: pulumi_gestalt_rust::Output<String>,
        pub display_name: pulumi_gestalt_rust::Output<String>,
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_gestalt_rust::Output<String>,
        /// The fully qualified resource name of the App, for example:
        /// projects/projectId/webApps/appId
        pub name: pulumi_gestalt_rust::Output<String>,
        pub project: pulumi_gestalt_rust::Output<Option<String>>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(
        context: &pulumi_gestalt_rust::Context,
        args: GetWebAppArgs,
    ) -> GetWebAppResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let app_id_binding = args.app_id.get_output(context);
        let project_binding = args.project.get_output(context);
        let request = pulumi_gestalt_rust::InvokeResourceRequest {
            token: "gcp:firebase/getWebApp:getWebApp".into(),
            version: super::super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "appId".into(),
                    value: &app_id_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "project".into(),
                    value: &project_binding.drop_type(),
                },
            ],
        };
        let o = context.invoke_resource(request);
        GetWebAppResult {
            api_key_id: o.get_field("apiKeyId"),
            app_id: o.get_field("appId"),
            app_urls: o.get_field("appUrls"),
            deletion_policy: o.get_field("deletionPolicy"),
            display_name: o.get_field("displayName"),
            id: o.get_field("id"),
            name: o.get_field("name"),
            project: o.get_field("project"),
        }
    }
}
