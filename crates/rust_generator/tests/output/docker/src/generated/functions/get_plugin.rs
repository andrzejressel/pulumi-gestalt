#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod get_plugin {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetPluginArgs {
        /// The alias of the Docker plugin. If the tag is omitted, `:latest` is complemented to the attribute value.
        #[builder(into, default)]
        pub alias: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The ID of the plugin, which has precedence over the `alias` of both are given
        #[builder(into, default)]
        pub id: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct GetPluginResult {
        /// The alias of the Docker plugin. If the tag is omitted, `:latest` is complemented to the attribute value.
        pub alias: pulumi_gestalt_rust::Output<Option<String>>,
        /// If `true` the plugin is enabled
        pub enabled: pulumi_gestalt_rust::Output<bool>,
        /// The environment variables in the form of `KEY=VALUE`, e.g. `DEBUG=0`
        pub envs: pulumi_gestalt_rust::Output<Vec<String>>,
        /// If true, grant all permissions necessary to run the plugin
        pub grant_all_permissions: pulumi_gestalt_rust::Output<bool>,
        /// The ID of the plugin, which has precedence over the `alias` of both are given
        pub id: pulumi_gestalt_rust::Output<Option<String>>,
        /// The plugin name. If the tag is omitted, `:latest` is complemented to the attribute value.
        pub name: pulumi_gestalt_rust::Output<String>,
        /// The Docker Plugin Reference
        pub plugin_reference: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(
        context: &pulumi_gestalt_rust::Context,
        args: GetPluginArgs,
    ) -> GetPluginResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let alias_binding = args.alias.get_output(context);
        let id_binding = args.id.get_output(context);
        let request = pulumi_gestalt_rust::InvokeResourceRequest {
            token: "docker:index/getPlugin:getPlugin".into(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "alias".into(),
                    value: &alias_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "id".into(),
                    value: &id_binding.drop_type(),
                },
            ],
        };
        let o = context.invoke_resource(request);
        GetPluginResult {
            alias: o.get_field("alias"),
            enabled: o.get_field("enabled"),
            envs: o.get_field("envs"),
            grant_all_permissions: o.get_field("grantAllPermissions"),
            id: o.get_field("id"),
            name: o.get_field("name"),
            plugin_reference: o.get_field("pluginReference"),
        }
    }
}
