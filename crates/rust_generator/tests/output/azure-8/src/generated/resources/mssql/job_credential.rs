/// Manages an Elastic Job Credential.
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
///     let exampleJobCredential = job_credential::create(
///         "exampleJobCredential",
///         JobCredentialArgs::builder()
///             .job_agent_id("${exampleJobAgent.id}")
///             .name("example-credential")
///             .password("MyP4ssw0rd!!!")
///             .username("my-username")
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
/// Elastic Job Credentials can be imported using the `resource id`, e.g.
///
/// ```sh
/// $ pulumi import azure:mssql/jobCredential:JobCredential example /subscriptions/00000000-0000-0000-0000-000000000000/resourceGroups/mygroup1/providers/Microsoft.Sql/servers/myserver1/jobAgents/myjobagent1/credentials/credential1
/// ```
///
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod job_credential {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct JobCredentialArgs {
        /// The ID of the Elastic Job Agent. Changing this forces a new Elastic Job Credential to be created.
        #[builder(into)]
        pub job_agent_id: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The name which should be used for this Elastic Job Credential. Changing this forces a new Elastic Job Credential to be created.
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The password part of the credential.
        #[builder(into)]
        pub password: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The username part of the credential.
        #[builder(into)]
        pub username: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct JobCredentialResult {
        /// The ID of the Elastic Job Agent. Changing this forces a new Elastic Job Credential to be created.
        pub job_agent_id: pulumi_gestalt_rust::Output<String>,
        /// The name which should be used for this Elastic Job Credential. Changing this forces a new Elastic Job Credential to be created.
        pub name: pulumi_gestalt_rust::Output<String>,
        /// The password part of the credential.
        pub password: pulumi_gestalt_rust::Output<String>,
        /// The username part of the credential.
        pub username: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: JobCredentialArgs,
    ) -> JobCredentialResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let job_agent_id_binding = args.job_agent_id.get_output(context);
        let name_binding = args.name.get_output(context);
        let password_binding = args.password.get_output(context);
        let username_binding = args.username.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "azure:mssql/jobCredential:JobCredential".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "jobAgentId".into(),
                    value: &job_agent_id_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "name".into(),
                    value: &name_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "password".into(),
                    value: &password_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "username".into(),
                    value: &username_binding.drop_type(),
                },
            ],
        };
        let o = context.register_resource(request);
        JobCredentialResult {
            job_agent_id: o.get_field("jobAgentId"),
            name: o.get_field("name"),
            password: o.get_field("password"),
            username: o.get_field("username"),
        }
    }
}
