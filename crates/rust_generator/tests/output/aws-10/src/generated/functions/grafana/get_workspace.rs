#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod get_workspace {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetWorkspaceArgs {
        /// Tags assigned to the resource
        #[builder(into, default)]
        pub tags: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// Grafana workspace ID.
        #[builder(into)]
        pub workspace_id: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct GetWorkspaceResult {
        /// (Required) Type of account access for the workspace. Valid values are `CURRENT_ACCOUNT` and `ORGANIZATION`. If `ORGANIZATION` is specified, then `organizational_units` must also be present.
        pub account_access_type: pulumi_gestalt_rust::Output<String>,
        /// ARN of the Grafana workspace.
        pub arn: pulumi_gestalt_rust::Output<String>,
        /// (Required) Authentication providers for the workspace. Valid values are `AWS_SSO`, `SAML`, or both.
        pub authentication_providers: pulumi_gestalt_rust::Output<Vec<String>>,
        /// Creation date of the Grafana workspace.
        pub created_date: pulumi_gestalt_rust::Output<String>,
        /// Data sources for the workspace.
        pub data_sources: pulumi_gestalt_rust::Output<Vec<String>>,
        /// Workspace description.
        pub description: pulumi_gestalt_rust::Output<String>,
        /// Endpoint of the Grafana workspace.
        pub endpoint: pulumi_gestalt_rust::Output<String>,
        /// Version of Grafana running on the workspace.
        pub grafana_version: pulumi_gestalt_rust::Output<String>,
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_gestalt_rust::Output<String>,
        /// Last updated date of the Grafana workspace.
        pub last_updated_date: pulumi_gestalt_rust::Output<String>,
        /// Grafana workspace name.
        pub name: pulumi_gestalt_rust::Output<String>,
        /// The notification destinations.
        pub notification_destinations: pulumi_gestalt_rust::Output<Vec<String>>,
        /// The role name that the workspace uses to access resources through Amazon Organizations.
        pub organization_role_name: pulumi_gestalt_rust::Output<String>,
        /// The Amazon Organizations organizational units that the workspace is authorized to use data sources from.
        pub organizational_units: pulumi_gestalt_rust::Output<Vec<String>>,
        /// Permission type of the workspace.
        pub permission_type: pulumi_gestalt_rust::Output<String>,
        /// IAM role ARN that the workspace assumes.
        pub role_arn: pulumi_gestalt_rust::Output<String>,
        pub saml_configuration_status: pulumi_gestalt_rust::Output<String>,
        /// AWS CloudFormation stack set name that provisions IAM roles to be used by the workspace.
        pub stack_set_name: pulumi_gestalt_rust::Output<String>,
        /// Status of the Grafana workspace.
        pub status: pulumi_gestalt_rust::Output<String>,
        /// Tags assigned to the resource
        pub tags: pulumi_gestalt_rust::Output<std::collections::HashMap<String, String>>,
        pub workspace_id: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(
        context: &pulumi_gestalt_rust::Context,
        args: GetWorkspaceArgs,
    ) -> GetWorkspaceResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let tags_binding = args.tags.get_output(context);
        let workspace_id_binding = args.workspace_id.get_output(context);
        let request = pulumi_gestalt_rust::InvokeResourceRequest {
            token: "aws:grafana/getWorkspace:getWorkspace".into(),
            version: super::super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "tags".into(),
                    value: &tags_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "workspaceId".into(),
                    value: &workspace_id_binding.drop_type(),
                },
            ],
        };
        let o = context.invoke_resource(request);
        GetWorkspaceResult {
            account_access_type: o.get_field("accountAccessType"),
            arn: o.get_field("arn"),
            authentication_providers: o.get_field("authenticationProviders"),
            created_date: o.get_field("createdDate"),
            data_sources: o.get_field("dataSources"),
            description: o.get_field("description"),
            endpoint: o.get_field("endpoint"),
            grafana_version: o.get_field("grafanaVersion"),
            id: o.get_field("id"),
            last_updated_date: o.get_field("lastUpdatedDate"),
            name: o.get_field("name"),
            notification_destinations: o.get_field("notificationDestinations"),
            organization_role_name: o.get_field("organizationRoleName"),
            organizational_units: o.get_field("organizationalUnits"),
            permission_type: o.get_field("permissionType"),
            role_arn: o.get_field("roleArn"),
            saml_configuration_status: o.get_field("samlConfigurationStatus"),
            stack_set_name: o.get_field("stackSetName"),
            status: o.get_field("status"),
            tags: o.get_field("tags"),
            workspace_id: o.get_field("workspaceId"),
        }
    }
}
