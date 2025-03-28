/// Manages an Elastic Job Agent.
///
/// ## Example Usage
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let example = resource_group::create(
///         "example",
///         ResourceGroupArgs::builder()
///             .location("northeurope")
///             .name("example")
///             .build_struct(),
///     );
///     let exampleDatabase = database::create(
///         "exampleDatabase",
///         DatabaseArgs::builder()
///             .collation("SQL_Latin1_General_CP1_CI_AS")
///             .name("example-db")
///             .server_id("${exampleServer.id}")
///             .sku_name("S1")
///             .build_struct(),
///     );
///     let exampleJobAgent = job_agent::create(
///         "exampleJobAgent",
///         JobAgentArgs::builder()
///             .database_id("${exampleDatabase.id}")
///             .location("${example.location}")
///             .name("example-job-agent")
///             .build_struct(),
///     );
///     let exampleServer = server::create(
///         "exampleServer",
///         ServerArgs::builder()
///             .administrator_login("4dm1n157r470r")
///             .administrator_login_password("4-v3ry-53cr37-p455w0rd")
///             .location("${example.location}")
///             .name("example-server")
///             .resource_group_name("${example.name}")
///             .version("12.0")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// Elastic Job Agents can be imported using the `id`, e.g.
///
/// ```sh
/// $ pulumi import azure:mssql/jobAgent:JobAgent example /subscriptions/00000000-0000-0000-0000-000000000000/resourceGroups/mygroup1/providers/Microsoft.Sql/servers/myserver1/jobAgents/myjobagent1
/// ```
///
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod job_agent {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct JobAgentArgs {
        /// The ID of the database to store metadata for the Elastic Job Agent. Changing this forces a new Elastic Job Agent to be created.
        #[builder(into)]
        pub database_id: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The Azure Region where the Elastic Job Agent should exist. Changing this forces a new Elastic Job Agent to be created.
        #[builder(into, default)]
        pub location: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The name which should be used for this Elastic Job Agent. Changing this forces a new Elastic Job Agent to be created.
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// A mapping of tags which should be assigned to the Database.
        #[builder(into, default)]
        pub tags: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
    }
    #[allow(dead_code)]
    pub struct JobAgentResult {
        /// The ID of the database to store metadata for the Elastic Job Agent. Changing this forces a new Elastic Job Agent to be created.
        pub database_id: pulumi_gestalt_rust::Output<String>,
        /// The Azure Region where the Elastic Job Agent should exist. Changing this forces a new Elastic Job Agent to be created.
        pub location: pulumi_gestalt_rust::Output<String>,
        /// The name which should be used for this Elastic Job Agent. Changing this forces a new Elastic Job Agent to be created.
        pub name: pulumi_gestalt_rust::Output<String>,
        /// A mapping of tags which should be assigned to the Database.
        pub tags: pulumi_gestalt_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: JobAgentArgs,
    ) -> JobAgentResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let database_id_binding = args.database_id.get_output(context);
        let location_binding = args.location.get_output(context);
        let name_binding = args.name.get_output(context);
        let tags_binding = args.tags.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "azure:mssql/jobAgent:JobAgent".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "databaseId".into(),
                    value: &database_id_binding.drop_type(),
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
                    name: "tags".into(),
                    value: &tags_binding.drop_type(),
                },
            ],
        };
        let o = context.register_resource(request);
        JobAgentResult {
            database_id: o.get_field("databaseId"),
            location: o.get_field("location"),
            name: o.get_field("name"),
            tags: o.get_field("tags"),
        }
    }
}
