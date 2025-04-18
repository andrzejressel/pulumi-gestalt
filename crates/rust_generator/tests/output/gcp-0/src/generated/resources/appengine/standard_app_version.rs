/// Standard App Version resource to create a new version of standard GAE Application.
/// Learn about the differences between the standard environment and the flexible environment
/// at https://cloud.google.com/appengine/docs/the-appengine-environments.
/// Currently supporting Zip and File Containers.
///
///
/// To get more information about StandardAppVersion, see:
///
/// * [API documentation](https://cloud.google.com/appengine/docs/admin-api/reference/rest/v1/apps.services.versions)
/// * How-to Guides
///     * [Official Documentation](https://cloud.google.com/appengine/docs/standard)
///
/// ## Example Usage
///
/// ### App Engine Standard App Version
///
///
/// ```yaml
/// resources:
///   customServiceAccount:
///     type: gcp:serviceaccount:Account
///     name: custom_service_account
///     properties:
///       accountId: my-account
///       displayName: Custom Service Account
///   gaeApi:
///     type: gcp:projects:IAMMember
///     name: gae_api
///     properties:
///       project: ${customServiceAccount.project}
///       role: roles/compute.networkUser
///       member: serviceAccount:${customServiceAccount.email}
///   storageViewer:
///     type: gcp:projects:IAMMember
///     name: storage_viewer
///     properties:
///       project: ${customServiceAccount.project}
///       role: roles/storage.objectViewer
///       member: serviceAccount:${customServiceAccount.email}
///   myappV1:
///     type: gcp:appengine:StandardAppVersion
///     name: myapp_v1
///     properties:
///       versionId: v1
///       service: myapp
///       runtime: nodejs20
///       entrypoint:
///         shell: node ./app.js
///       deployment:
///         zip:
///           sourceUrl: https://storage.googleapis.com/${bucket.name}/${object.name}
///       envVariables:
///         port: '8080'
///       automaticScaling:
///         maxConcurrentRequests: 10
///         minIdleInstances: 1
///         maxIdleInstances: 3
///         minPendingLatency: 1s
///         maxPendingLatency: 5s
///         standardSchedulerSettings:
///           targetCpuUtilization: 0.5
///           targetThroughputUtilization: 0.75
///           minInstances: 2
///           maxInstances: 10
///       deleteServiceOnDestroy: true
///       serviceAccount: ${customServiceAccount.email}
///   myappV2:
///     type: gcp:appengine:StandardAppVersion
///     name: myapp_v2
///     properties:
///       versionId: v2
///       service: myapp
///       runtime: nodejs20
///       appEngineApis: true
///       entrypoint:
///         shell: node ./app.js
///       deployment:
///         zip:
///           sourceUrl: https://storage.googleapis.com/${bucket.name}/${object.name}
///       envVariables:
///         port: '8080'
///       basicScaling:
///         maxInstances: 5
///       noopOnDestroy: true
///       serviceAccount: ${customServiceAccount.email}
///   bucket:
///     type: gcp:storage:Bucket
///     properties:
///       name: appengine-static-content
///       location: US
///   object:
///     type: gcp:storage:BucketObject
///     properties:
///       name: hello-world.zip
///       bucket: ${bucket.name}
///       source:
///         fn::FileAsset: ./test-fixtures/hello-world.zip
/// ```
///
/// ## Import
///
/// StandardAppVersion can be imported using any of these accepted formats:
///
/// * `apps/{{project}}/services/{{service}}/versions/{{version_id}}`
///
/// * `{{project}}/{{service}}/{{version_id}}`
///
/// * `{{service}}/{{version_id}}`
///
/// When using the `pulumi import` command, StandardAppVersion can be imported using one of the formats above. For example:
///
/// ```sh
/// $ pulumi import gcp:appengine/standardAppVersion:StandardAppVersion default apps/{{project}}/services/{{service}}/versions/{{version_id}}
/// ```
///
/// ```sh
/// $ pulumi import gcp:appengine/standardAppVersion:StandardAppVersion default {{project}}/{{service}}/{{version_id}}
/// ```
///
/// ```sh
/// $ pulumi import gcp:appengine/standardAppVersion:StandardAppVersion default {{service}}/{{version_id}}
/// ```
///
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod standard_app_version {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct StandardAppVersionArgs {
        /// Allows App Engine second generation runtimes to access the legacy bundled services.
        #[builder(into, default)]
        pub app_engine_apis: pulumi_gestalt_rust::InputOrOutput<Option<bool>>,
        /// Automatic scaling is based on request rate, response latencies, and other application metrics.
        #[builder(into, default)]
        pub automatic_scaling: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::appengine::StandardAppVersionAutomaticScaling>,
        >,
        /// Basic scaling creates instances when your application receives requests. Each instance will be shut down when the
        /// application becomes idle. Basic scaling is ideal for work that is intermittent or driven by user activity.
        #[builder(into, default)]
        pub basic_scaling: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::appengine::StandardAppVersionBasicScaling>,
        >,
        /// If set to 'true', the service will be deleted if it is the last version.
        #[builder(into, default)]
        pub delete_service_on_destroy: pulumi_gestalt_rust::InputOrOutput<Option<bool>>,
        /// Code and application artifacts that make up this version.
        /// Structure is documented below.
        #[builder(into)]
        pub deployment: pulumi_gestalt_rust::InputOrOutput<
            super::super::types::appengine::StandardAppVersionDeployment,
        >,
        /// The entrypoint for the application.
        /// Structure is documented below.
        #[builder(into)]
        pub entrypoint: pulumi_gestalt_rust::InputOrOutput<
            super::super::types::appengine::StandardAppVersionEntrypoint,
        >,
        /// Environment variables available to the application.
        #[builder(into, default)]
        pub env_variables: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// An ordered list of URL-matching patterns that should be applied to incoming requests. The first matching URL handles the
        /// request and other request handlers are not attempted.
        #[builder(into, default)]
        pub handlers: pulumi_gestalt_rust::InputOrOutput<
            Option<Vec<super::super::types::appengine::StandardAppVersionHandler>>,
        >,
        /// A list of the types of messages that this application is able to receive. Possible values: ["INBOUND_SERVICE_MAIL",
        /// "INBOUND_SERVICE_MAIL_BOUNCE", "INBOUND_SERVICE_XMPP_ERROR", "INBOUND_SERVICE_XMPP_MESSAGE",
        /// "INBOUND_SERVICE_XMPP_SUBSCRIBE", "INBOUND_SERVICE_XMPP_PRESENCE", "INBOUND_SERVICE_CHANNEL_PRESENCE",
        /// "INBOUND_SERVICE_WARMUP"]
        #[builder(into, default)]
        pub inbound_services: pulumi_gestalt_rust::InputOrOutput<Option<Vec<String>>>,
        /// Instance class that is used to run this version. Valid values are AutomaticScaling: F1, F2, F4, F4_1G BasicScaling or
        /// ManualScaling: B1, B2, B4, B4_1G, B8 Defaults to F1 for AutomaticScaling and B2 for ManualScaling and BasicScaling. If
        /// no scaling is specified, AutomaticScaling is chosen.
        #[builder(into, default)]
        pub instance_class: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Configuration for third-party Python runtime libraries that are required by the application.
        #[builder(into, default)]
        pub libraries: pulumi_gestalt_rust::InputOrOutput<
            Option<Vec<super::super::types::appengine::StandardAppVersionLibrary>>,
        >,
        /// A service with manual scaling runs continuously, allowing you to perform complex initialization and rely on the state of
        /// its memory over time.
        #[builder(into, default)]
        pub manual_scaling: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::appengine::StandardAppVersionManualScaling>,
        >,
        /// If set to 'true', the application version will not be deleted.
        #[builder(into, default)]
        pub noop_on_destroy: pulumi_gestalt_rust::InputOrOutput<Option<bool>>,
        #[builder(into, default)]
        pub project: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Desired runtime. Example python27.
        #[builder(into)]
        pub runtime: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The version of the API in the given runtime environment. Please see the app.yaml reference for valid values at
        /// 'https://cloud.google.com/appengine/docs/standard/<language>/config/appref'\ Substitute '<language>' with 'python',
        /// 'java', 'php', 'ruby', 'go' or 'nodejs'.
        #[builder(into, default)]
        pub runtime_api_version: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// AppEngine service resource
        #[builder(into)]
        pub service: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The identity that the deployed version will run as. Admin API will use the App Engine Appspot service account as default
        /// if this field is neither provided in app.yaml file nor through CLI flag.
        #[builder(into, default)]
        pub service_account: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Whether multiple requests can be dispatched to this version at once.
        #[builder(into, default)]
        pub threadsafe: pulumi_gestalt_rust::InputOrOutput<Option<bool>>,
        /// Relative name of the version within the service. For example, 'v1'. Version names can contain only lowercase letters,
        /// numbers, or hyphens. Reserved names,"default", "latest", and any name with the prefix "ah-".
        #[builder(into, default)]
        pub version_id: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Enables VPC connectivity for standard apps.
        #[builder(into, default)]
        pub vpc_access_connector: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::appengine::StandardAppVersionVpcAccessConnector>,
        >,
    }
    #[allow(dead_code)]
    pub struct StandardAppVersionResult {
        /// Allows App Engine second generation runtimes to access the legacy bundled services.
        pub app_engine_apis: pulumi_gestalt_rust::Output<Option<bool>>,
        /// Automatic scaling is based on request rate, response latencies, and other application metrics.
        pub automatic_scaling: pulumi_gestalt_rust::Output<
            Option<super::super::types::appengine::StandardAppVersionAutomaticScaling>,
        >,
        /// Basic scaling creates instances when your application receives requests. Each instance will be shut down when the
        /// application becomes idle. Basic scaling is ideal for work that is intermittent or driven by user activity.
        pub basic_scaling: pulumi_gestalt_rust::Output<
            Option<super::super::types::appengine::StandardAppVersionBasicScaling>,
        >,
        /// If set to 'true', the service will be deleted if it is the last version.
        pub delete_service_on_destroy: pulumi_gestalt_rust::Output<Option<bool>>,
        /// Code and application artifacts that make up this version.
        /// Structure is documented below.
        pub deployment: pulumi_gestalt_rust::Output<
            super::super::types::appengine::StandardAppVersionDeployment,
        >,
        /// The entrypoint for the application.
        /// Structure is documented below.
        pub entrypoint: pulumi_gestalt_rust::Output<
            super::super::types::appengine::StandardAppVersionEntrypoint,
        >,
        /// Environment variables available to the application.
        pub env_variables: pulumi_gestalt_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// An ordered list of URL-matching patterns that should be applied to incoming requests. The first matching URL handles the
        /// request and other request handlers are not attempted.
        pub handlers: pulumi_gestalt_rust::Output<
            Vec<super::super::types::appengine::StandardAppVersionHandler>,
        >,
        /// A list of the types of messages that this application is able to receive. Possible values: ["INBOUND_SERVICE_MAIL",
        /// "INBOUND_SERVICE_MAIL_BOUNCE", "INBOUND_SERVICE_XMPP_ERROR", "INBOUND_SERVICE_XMPP_MESSAGE",
        /// "INBOUND_SERVICE_XMPP_SUBSCRIBE", "INBOUND_SERVICE_XMPP_PRESENCE", "INBOUND_SERVICE_CHANNEL_PRESENCE",
        /// "INBOUND_SERVICE_WARMUP"]
        pub inbound_services: pulumi_gestalt_rust::Output<Option<Vec<String>>>,
        /// Instance class that is used to run this version. Valid values are AutomaticScaling: F1, F2, F4, F4_1G BasicScaling or
        /// ManualScaling: B1, B2, B4, B4_1G, B8 Defaults to F1 for AutomaticScaling and B2 for ManualScaling and BasicScaling. If
        /// no scaling is specified, AutomaticScaling is chosen.
        pub instance_class: pulumi_gestalt_rust::Output<String>,
        /// Configuration for third-party Python runtime libraries that are required by the application.
        pub libraries: pulumi_gestalt_rust::Output<
            Option<Vec<super::super::types::appengine::StandardAppVersionLibrary>>,
        >,
        /// A service with manual scaling runs continuously, allowing you to perform complex initialization and rely on the state of
        /// its memory over time.
        pub manual_scaling: pulumi_gestalt_rust::Output<
            Option<super::super::types::appengine::StandardAppVersionManualScaling>,
        >,
        /// Full path to the Version resource in the API. Example, "v1".
        pub name: pulumi_gestalt_rust::Output<String>,
        /// If set to 'true', the application version will not be deleted.
        pub noop_on_destroy: pulumi_gestalt_rust::Output<Option<bool>>,
        pub project: pulumi_gestalt_rust::Output<String>,
        /// Desired runtime. Example python27.
        pub runtime: pulumi_gestalt_rust::Output<String>,
        /// The version of the API in the given runtime environment. Please see the app.yaml reference for valid values at
        /// 'https://cloud.google.com/appengine/docs/standard/<language>/config/appref'\ Substitute '<language>' with 'python',
        /// 'java', 'php', 'ruby', 'go' or 'nodejs'.
        pub runtime_api_version: pulumi_gestalt_rust::Output<Option<String>>,
        /// AppEngine service resource
        pub service: pulumi_gestalt_rust::Output<String>,
        /// The identity that the deployed version will run as. Admin API will use the App Engine Appspot service account as default
        /// if this field is neither provided in app.yaml file nor through CLI flag.
        pub service_account: pulumi_gestalt_rust::Output<String>,
        /// Whether multiple requests can be dispatched to this version at once.
        pub threadsafe: pulumi_gestalt_rust::Output<Option<bool>>,
        /// Relative name of the version within the service. For example, 'v1'. Version names can contain only lowercase letters,
        /// numbers, or hyphens. Reserved names,"default", "latest", and any name with the prefix "ah-".
        pub version_id: pulumi_gestalt_rust::Output<Option<String>>,
        /// Enables VPC connectivity for standard apps.
        pub vpc_access_connector: pulumi_gestalt_rust::Output<
            Option<super::super::types::appengine::StandardAppVersionVpcAccessConnector>,
        >,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: StandardAppVersionArgs,
    ) -> StandardAppVersionResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let app_engine_apis_binding = args.app_engine_apis.get_output(context);
        let automatic_scaling_binding = args.automatic_scaling.get_output(context);
        let basic_scaling_binding = args.basic_scaling.get_output(context);
        let delete_service_on_destroy_binding = args
            .delete_service_on_destroy
            .get_output(context);
        let deployment_binding = args.deployment.get_output(context);
        let entrypoint_binding = args.entrypoint.get_output(context);
        let env_variables_binding = args.env_variables.get_output(context);
        let handlers_binding = args.handlers.get_output(context);
        let inbound_services_binding = args.inbound_services.get_output(context);
        let instance_class_binding = args.instance_class.get_output(context);
        let libraries_binding = args.libraries.get_output(context);
        let manual_scaling_binding = args.manual_scaling.get_output(context);
        let noop_on_destroy_binding = args.noop_on_destroy.get_output(context);
        let project_binding = args.project.get_output(context);
        let runtime_binding = args.runtime.get_output(context);
        let runtime_api_version_binding = args.runtime_api_version.get_output(context);
        let service_binding = args.service.get_output(context);
        let service_account_binding = args.service_account.get_output(context);
        let threadsafe_binding = args.threadsafe.get_output(context);
        let version_id_binding = args.version_id.get_output(context);
        let vpc_access_connector_binding = args.vpc_access_connector.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "gcp:appengine/standardAppVersion:StandardAppVersion".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "appEngineApis".into(),
                    value: &app_engine_apis_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "automaticScaling".into(),
                    value: &automatic_scaling_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "basicScaling".into(),
                    value: &basic_scaling_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "deleteServiceOnDestroy".into(),
                    value: &delete_service_on_destroy_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "deployment".into(),
                    value: &deployment_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "entrypoint".into(),
                    value: &entrypoint_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "envVariables".into(),
                    value: &env_variables_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "handlers".into(),
                    value: &handlers_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "inboundServices".into(),
                    value: &inbound_services_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "instanceClass".into(),
                    value: &instance_class_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "libraries".into(),
                    value: &libraries_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "manualScaling".into(),
                    value: &manual_scaling_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "noopOnDestroy".into(),
                    value: &noop_on_destroy_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "project".into(),
                    value: &project_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "runtime".into(),
                    value: &runtime_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "runtimeApiVersion".into(),
                    value: &runtime_api_version_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "service".into(),
                    value: &service_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "serviceAccount".into(),
                    value: &service_account_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "threadsafe".into(),
                    value: &threadsafe_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "versionId".into(),
                    value: &version_id_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "vpcAccessConnector".into(),
                    value: &vpc_access_connector_binding.drop_type(),
                },
            ],
        };
        let o = context.register_resource(request);
        StandardAppVersionResult {
            app_engine_apis: o.get_field("appEngineApis"),
            automatic_scaling: o.get_field("automaticScaling"),
            basic_scaling: o.get_field("basicScaling"),
            delete_service_on_destroy: o.get_field("deleteServiceOnDestroy"),
            deployment: o.get_field("deployment"),
            entrypoint: o.get_field("entrypoint"),
            env_variables: o.get_field("envVariables"),
            handlers: o.get_field("handlers"),
            inbound_services: o.get_field("inboundServices"),
            instance_class: o.get_field("instanceClass"),
            libraries: o.get_field("libraries"),
            manual_scaling: o.get_field("manualScaling"),
            name: o.get_field("name"),
            noop_on_destroy: o.get_field("noopOnDestroy"),
            project: o.get_field("project"),
            runtime: o.get_field("runtime"),
            runtime_api_version: o.get_field("runtimeApiVersion"),
            service: o.get_field("service"),
            service_account: o.get_field("serviceAccount"),
            threadsafe: o.get_field("threadsafe"),
            version_id: o.get_field("versionId"),
            vpc_access_connector: o.get_field("vpcAccessConnector"),
        }
    }
}
