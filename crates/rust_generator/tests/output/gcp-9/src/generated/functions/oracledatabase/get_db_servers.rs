#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod get_db_servers {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetDbServersArgs {
        /// The Exadata Infrastructure id.
        #[builder(into)]
        pub cloud_exadata_infrastructure: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The location of resource.
        #[builder(into)]
        pub location: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The project to which the resource belongs. If it
        /// is not provided, the provider project is used.
        #[builder(into, default)]
        pub project: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct GetDbServersResult {
        pub cloud_exadata_infrastructure: pulumi_gestalt_rust::Output<String>,
        pub db_servers: pulumi_gestalt_rust::Output<
            Vec<super::super::super::types::oracledatabase::GetDbServersDbServer>,
        >,
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_gestalt_rust::Output<String>,
        pub location: pulumi_gestalt_rust::Output<String>,
        pub project: pulumi_gestalt_rust::Output<Option<String>>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(
        context: &pulumi_gestalt_rust::Context,
        args: GetDbServersArgs,
    ) -> GetDbServersResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let cloud_exadata_infrastructure_binding = args
            .cloud_exadata_infrastructure
            .get_output(context);
        let location_binding = args.location.get_output(context);
        let project_binding = args.project.get_output(context);
        let request = pulumi_gestalt_rust::InvokeResourceRequest {
            token: "gcp:oracledatabase/getDbServers:getDbServers".into(),
            version: super::super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "cloudExadataInfrastructure".into(),
                    value: &cloud_exadata_infrastructure_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "location".into(),
                    value: &location_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "project".into(),
                    value: &project_binding.drop_type(),
                },
            ],
        };
        let o = context.invoke_resource(request);
        GetDbServersResult {
            cloud_exadata_infrastructure: o.get_field("cloudExadataInfrastructure"),
            db_servers: o.get_field("dbServers"),
            id: o.get_field("id"),
            location: o.get_field("location"),
            project: o.get_field("project"),
        }
    }
}
