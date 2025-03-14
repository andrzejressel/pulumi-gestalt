/// A Spark application is a single Spark workload run on a GDC cluster.
///
///
/// To get more information about SparkApplication, see:
///
/// * [API documentation](https://cloud.google.com/dataproc-gdc/docs/reference/rest/v1/projects.locations.serviceInstances.sparkApplications)
/// * How-to Guides
///     * [Dataproc Intro](https://cloud.google.com/dataproc/)
///
/// ## Example Usage
///
/// ### Dataprocgdc Sparkapplication Basic
///
///
/// ```yaml
/// resources:
///   spark-application:
///     type: gcp:dataproc:GdcSparkApplication
///     properties:
///       sparkApplicationId: tf-e2e-spark-app-basic
///       serviceinstance: do-not-delete-dataproc-gdc-instance
///       project: my-project
///       location: us-west2
///       namespace: default
///       sparkApplicationConfig:
///         mainClass: org.apache.spark.examples.SparkPi
///         jarFileUris:
///           - file:///usr/lib/spark/examples/jars/spark-examples.jar
///         args:
///           - '10000'
/// ```
/// ### Dataprocgdc Sparkapplication
///
///
/// ```yaml
/// resources:
///   appEnv:
///     type: gcp:dataproc:GdcApplicationEnvironment
///     name: app_env
///     properties:
///       applicationEnvironmentId: tf-e2e-spark-app-env
///       serviceinstance: do-not-delete-dataproc-gdc-instance
///       project: my-project
///       location: us-west2
///       namespace: default
///   spark-application:
///     type: gcp:dataproc:GdcSparkApplication
///     properties:
///       sparkApplicationId: tf-e2e-spark-app
///       serviceinstance: do-not-delete-dataproc-gdc-instance
///       project: my-project
///       location: us-west2
///       namespace: default
///       labels:
///         test-label: label-value
///       annotations:
///         an_annotation: annotation_value
///       properties:
///         spark.executor.instances: '2'
///       applicationEnvironment: ${appEnv.name}
///       version: '1.2'
///       sparkApplicationConfig:
///         mainJarFileUri: file:///usr/lib/spark/examples/jars/spark-examples.jar
///         jarFileUris:
///           - file:///usr/lib/spark/examples/jars/spark-examples.jar
///         archiveUris:
///           - file://usr/lib/spark/examples/spark-examples.jar
///         fileUris:
///           - file:///usr/lib/spark/examples/jars/spark-examples.jar
/// ```
/// ### Dataprocgdc Sparkapplication Pyspark
///
///
/// ```yaml
/// resources:
///   spark-application:
///     type: gcp:dataproc:GdcSparkApplication
///     properties:
///       sparkApplicationId: tf-e2e-pyspark-app
///       serviceinstance: do-not-delete-dataproc-gdc-instance
///       project: my-project
///       location: us-west2
///       namespace: default
///       displayName: A Pyspark application for a Terraform create test
///       dependencyImages:
///         - gcr.io/some/image
///       pysparkApplicationConfig:
///         mainPythonFileUri: gs://goog-dataproc-initialization-actions-us-west2/conda/test_conda.py
///         jarFileUris:
///           - file:///usr/lib/spark/examples/jars/spark-examples.jar
///         pythonFileUris:
///           - gs://goog-dataproc-initialization-actions-us-west2/conda/get-sys-exec.py
///         fileUris:
///           - file://usr/lib/spark/examples/spark-examples.jar
///         archiveUris:
///           - file://usr/lib/spark/examples/spark-examples.jar
///         args:
///           - '10'
/// ```
/// ### Dataprocgdc Sparkapplication Sparkr
///
///
/// ```yaml
/// resources:
///   spark-application:
///     type: gcp:dataproc:GdcSparkApplication
///     properties:
///       sparkApplicationId: tf-e2e-sparkr-app
///       serviceinstance: do-not-delete-dataproc-gdc-instance
///       project: my-project
///       location: us-west2
///       namespace: default
///       displayName: A SparkR application for a Terraform create test
///       sparkRApplicationConfig:
///         mainRFileUri: gs://some-bucket/something.R
///         fileUris:
///           - file://usr/lib/spark/examples/spark-examples.jar
///         archiveUris:
///           - file://usr/lib/spark/examples/spark-examples.jar
///         args:
///           - '10'
/// ```
/// ### Dataprocgdc Sparkapplication Sparksql
///
///
/// ```yaml
/// resources:
///   spark-application:
///     type: gcp:dataproc:GdcSparkApplication
///     properties:
///       sparkApplicationId: tf-e2e-sparksql-app
///       serviceinstance: do-not-delete-dataproc-gdc-instance
///       project: my-project
///       location: us-west2
///       namespace: default
///       displayName: A SparkSql application for a Terraform create test
///       sparkSqlApplicationConfig:
///         jarFileUris:
///           - file:///usr/lib/spark/examples/jars/spark-examples.jar
///         queryList:
///           queries:
///             - show tables;
///         scriptVariables:
///           MY_VAR: '1'
/// ```
/// ### Dataprocgdc Sparkapplication Sparksql Query File
///
///
/// ```yaml
/// resources:
///   spark-application:
///     type: gcp:dataproc:GdcSparkApplication
///     properties:
///       sparkApplicationId: tf-e2e-sparksql-app
///       serviceinstance: do-not-delete-dataproc-gdc-instance
///       project: my-project
///       location: us-west2
///       namespace: default
///       displayName: A SparkSql application for a Terraform create test
///       sparkSqlApplicationConfig:
///         jarFileUris:
///           - file:///usr/lib/spark/examples/jars/spark-examples.jar
///         queryFileUri: gs://some-bucket/something.sql
///         scriptVariables:
///           MY_VAR: '1'
/// ```
///
/// ## Import
///
/// SparkApplication can be imported using any of these accepted formats:
///
/// * `projects/{{project}}/locations/{{location}}/serviceInstances/{{serviceinstance}}/sparkApplications/{{spark_application_id}}`
///
/// * `{{project}}/{{location}}/{{serviceinstance}}/{{spark_application_id}}`
///
/// * `{{location}}/{{serviceinstance}}/{{spark_application_id}}`
///
/// When using the `pulumi import` command, SparkApplication can be imported using one of the formats above. For example:
///
/// ```sh
/// $ pulumi import gcp:dataproc/gdcSparkApplication:GdcSparkApplication default projects/{{project}}/locations/{{location}}/serviceInstances/{{serviceinstance}}/sparkApplications/{{spark_application_id}}
/// ```
///
/// ```sh
/// $ pulumi import gcp:dataproc/gdcSparkApplication:GdcSparkApplication default {{project}}/{{location}}/{{serviceinstance}}/{{spark_application_id}}
/// ```
///
/// ```sh
/// $ pulumi import gcp:dataproc/gdcSparkApplication:GdcSparkApplication default {{location}}/{{serviceinstance}}/{{spark_application_id}}
/// ```
///
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod gdc_spark_application {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GdcSparkApplicationArgs {
        /// The annotations to associate with this application. Annotations may be used to store client information, but are not used by the server.
        /// **Note**: This field is non-authoritative, and will only manage the annotations present in your configuration.
        /// Please refer to the field `effective_annotations` for all of the annotations present on the resource.
        #[builder(into, default)]
        pub annotations: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// An ApplicationEnvironment from which to inherit configuration properties.
        #[builder(into, default)]
        pub application_environment: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// List of container image uris for additional file dependencies. Dependent files are sequentially copied from each image. If a file with the same name exists in 2 images then the file from later image is used.
        #[builder(into, default)]
        pub dependency_images: pulumi_gestalt_rust::InputOrOutput<Option<Vec<String>>>,
        /// User-provided human-readable name to be used in user interfaces.
        #[builder(into, default)]
        pub display_name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The labels to associate with this application. Labels may be used for filtering and billing tracking.
        /// **Note**: This field is non-authoritative, and will only manage the labels present in your configuration.
        /// Please refer to the field `effective_labels` for all of the labels present on the resource.
        #[builder(into, default)]
        pub labels: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// The location of the spark application.
        #[builder(into)]
        pub location: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The Kubernetes namespace in which to create the application. This namespace must already exist on the cluster.
        #[builder(into, default)]
        pub namespace: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The ID of the project in which the resource belongs.
        /// If it is not provided, the provider project is used.
        #[builder(into, default)]
        pub project: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// application-specific properties.
        #[builder(into, default)]
        pub properties: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// Represents the PySparkApplicationConfig.
        /// Structure is documented below.
        #[builder(into, default)]
        pub pyspark_application_config: pulumi_gestalt_rust::InputOrOutput<
            Option<
                super::super::types::dataproc::GdcSparkApplicationPysparkApplicationConfig,
            >,
        >,
        /// The id of the service instance to which this spark application belongs.
        #[builder(into)]
        pub serviceinstance: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Represents the SparkApplicationConfig.
        /// Structure is documented below.
        #[builder(into, default)]
        pub spark_application_config: pulumi_gestalt_rust::InputOrOutput<
            Option<
                super::super::types::dataproc::GdcSparkApplicationSparkApplicationConfig,
            >,
        >,
        /// The id of the application
        ///
        ///
        /// - - -
        #[builder(into)]
        pub spark_application_id: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Represents the SparkRApplicationConfig.
        /// Structure is documented below.
        #[builder(into, default)]
        pub spark_r_application_config: pulumi_gestalt_rust::InputOrOutput<
            Option<
                super::super::types::dataproc::GdcSparkApplicationSparkRApplicationConfig,
            >,
        >,
        /// Represents the SparkRApplicationConfig.
        /// Structure is documented below.
        #[builder(into, default)]
        pub spark_sql_application_config: pulumi_gestalt_rust::InputOrOutput<
            Option<
                super::super::types::dataproc::GdcSparkApplicationSparkSqlApplicationConfig,
            >,
        >,
        /// The Dataproc version of this application.
        #[builder(into, default)]
        pub version: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct GdcSparkApplicationResult {
        /// The annotations to associate with this application. Annotations may be used to store client information, but are not used by the server.
        /// **Note**: This field is non-authoritative, and will only manage the annotations present in your configuration.
        /// Please refer to the field `effective_annotations` for all of the annotations present on the resource.
        pub annotations: pulumi_gestalt_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// An ApplicationEnvironment from which to inherit configuration properties.
        pub application_environment: pulumi_gestalt_rust::Output<Option<String>>,
        /// The timestamp when the resource was created.
        pub create_time: pulumi_gestalt_rust::Output<String>,
        /// List of container image uris for additional file dependencies. Dependent files are sequentially copied from each image. If a file with the same name exists in 2 images then the file from later image is used.
        pub dependency_images: pulumi_gestalt_rust::Output<Option<Vec<String>>>,
        /// User-provided human-readable name to be used in user interfaces.
        pub display_name: pulumi_gestalt_rust::Output<Option<String>>,
        pub effective_annotations: pulumi_gestalt_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        /// All of labels (key/value pairs) present on the resource in GCP, including the labels configured through Pulumi, other clients and services.
        pub effective_labels: pulumi_gestalt_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        /// The labels to associate with this application. Labels may be used for filtering and billing tracking.
        /// **Note**: This field is non-authoritative, and will only manage the labels present in your configuration.
        /// Please refer to the field `effective_labels` for all of the labels present on the resource.
        pub labels: pulumi_gestalt_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// The location of the spark application.
        pub location: pulumi_gestalt_rust::Output<String>,
        /// URL for a monitoring UI for this application (for eventual Spark PHS/UI support) Out of scope for private GA
        pub monitoring_endpoint: pulumi_gestalt_rust::Output<String>,
        /// Identifier. The name of the application. Format: projects/{project}/locations/{location}/serviceInstances/{service_instance}/sparkApplications/{application}
        pub name: pulumi_gestalt_rust::Output<String>,
        /// The Kubernetes namespace in which to create the application. This namespace must already exist on the cluster.
        pub namespace: pulumi_gestalt_rust::Output<Option<String>>,
        /// An HCFS URI pointing to the location of stdout and stdout of the application Mainly useful for Pantheon and gcloud Not in scope for private GA
        pub output_uri: pulumi_gestalt_rust::Output<String>,
        /// The ID of the project in which the resource belongs.
        /// If it is not provided, the provider project is used.
        pub project: pulumi_gestalt_rust::Output<String>,
        /// application-specific properties.
        pub properties: pulumi_gestalt_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// The combination of labels configured directly on the resource
        /// and default labels configured on the provider.
        pub pulumi_labels: pulumi_gestalt_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        /// Represents the PySparkApplicationConfig.
        /// Structure is documented below.
        pub pyspark_application_config: pulumi_gestalt_rust::Output<
            Option<
                super::super::types::dataproc::GdcSparkApplicationPysparkApplicationConfig,
            >,
        >,
        /// Whether the application is currently reconciling. True if the current state of the resource does not match the intended state, and the system is working to reconcile them, whether or not the change was user initiated.
        pub reconciling: pulumi_gestalt_rust::Output<bool>,
        /// The id of the service instance to which this spark application belongs.
        pub serviceinstance: pulumi_gestalt_rust::Output<String>,
        /// Represents the SparkApplicationConfig.
        /// Structure is documented below.
        pub spark_application_config: pulumi_gestalt_rust::Output<
            Option<
                super::super::types::dataproc::GdcSparkApplicationSparkApplicationConfig,
            >,
        >,
        /// The id of the application
        ///
        ///
        /// - - -
        pub spark_application_id: pulumi_gestalt_rust::Output<String>,
        /// Represents the SparkRApplicationConfig.
        /// Structure is documented below.
        pub spark_r_application_config: pulumi_gestalt_rust::Output<
            Option<
                super::super::types::dataproc::GdcSparkApplicationSparkRApplicationConfig,
            >,
        >,
        /// Represents the SparkRApplicationConfig.
        /// Structure is documented below.
        pub spark_sql_application_config: pulumi_gestalt_rust::Output<
            Option<
                super::super::types::dataproc::GdcSparkApplicationSparkSqlApplicationConfig,
            >,
        >,
        /// The current state.
        /// Possible values:
        /// * `STATE_UNSPECIFIED`
        /// * `PENDING`
        /// * `RUNNING`
        /// * `CANCELLING`
        /// * `CANCELLED`
        /// * `SUCCEEDED`
        /// * `FAILED`
        pub state: pulumi_gestalt_rust::Output<String>,
        /// A message explaining the current state.
        pub state_message: pulumi_gestalt_rust::Output<String>,
        /// System generated unique identifier for this application, formatted as UUID4.
        pub uid: pulumi_gestalt_rust::Output<String>,
        /// The timestamp when the resource was most recently updated.
        pub update_time: pulumi_gestalt_rust::Output<String>,
        /// The Dataproc version of this application.
        pub version: pulumi_gestalt_rust::Output<Option<String>>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: GdcSparkApplicationArgs,
    ) -> GdcSparkApplicationResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let annotations_binding = args.annotations.get_output(context);
        let application_environment_binding = args
            .application_environment
            .get_output(context);
        let dependency_images_binding = args.dependency_images.get_output(context);
        let display_name_binding = args.display_name.get_output(context);
        let labels_binding = args.labels.get_output(context);
        let location_binding = args.location.get_output(context);
        let namespace_binding = args.namespace.get_output(context);
        let project_binding = args.project.get_output(context);
        let properties_binding = args.properties.get_output(context);
        let pyspark_application_config_binding = args
            .pyspark_application_config
            .get_output(context);
        let serviceinstance_binding = args.serviceinstance.get_output(context);
        let spark_application_config_binding = args
            .spark_application_config
            .get_output(context);
        let spark_application_id_binding = args.spark_application_id.get_output(context);
        let spark_r_application_config_binding = args
            .spark_r_application_config
            .get_output(context);
        let spark_sql_application_config_binding = args
            .spark_sql_application_config
            .get_output(context);
        let version_binding = args.version.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "gcp:dataproc/gdcSparkApplication:GdcSparkApplication".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "annotations".into(),
                    value: &annotations_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "applicationEnvironment".into(),
                    value: &application_environment_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "dependencyImages".into(),
                    value: &dependency_images_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "displayName".into(),
                    value: &display_name_binding.drop_type(),
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
                    name: "namespace".into(),
                    value: &namespace_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "project".into(),
                    value: &project_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "properties".into(),
                    value: &properties_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "pysparkApplicationConfig".into(),
                    value: &pyspark_application_config_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "serviceinstance".into(),
                    value: &serviceinstance_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "sparkApplicationConfig".into(),
                    value: &spark_application_config_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "sparkApplicationId".into(),
                    value: &spark_application_id_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "sparkRApplicationConfig".into(),
                    value: &spark_r_application_config_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "sparkSqlApplicationConfig".into(),
                    value: &spark_sql_application_config_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "version".into(),
                    value: &version_binding.drop_type(),
                },
            ],
        };
        let o = context.register_resource(request);
        GdcSparkApplicationResult {
            annotations: o.get_field("annotations"),
            application_environment: o.get_field("applicationEnvironment"),
            create_time: o.get_field("createTime"),
            dependency_images: o.get_field("dependencyImages"),
            display_name: o.get_field("displayName"),
            effective_annotations: o.get_field("effectiveAnnotations"),
            effective_labels: o.get_field("effectiveLabels"),
            labels: o.get_field("labels"),
            location: o.get_field("location"),
            monitoring_endpoint: o.get_field("monitoringEndpoint"),
            name: o.get_field("name"),
            namespace: o.get_field("namespace"),
            output_uri: o.get_field("outputUri"),
            project: o.get_field("project"),
            properties: o.get_field("properties"),
            pulumi_labels: o.get_field("pulumiLabels"),
            pyspark_application_config: o.get_field("pysparkApplicationConfig"),
            reconciling: o.get_field("reconciling"),
            serviceinstance: o.get_field("serviceinstance"),
            spark_application_config: o.get_field("sparkApplicationConfig"),
            spark_application_id: o.get_field("sparkApplicationId"),
            spark_r_application_config: o.get_field("sparkRApplicationConfig"),
            spark_sql_application_config: o.get_field("sparkSqlApplicationConfig"),
            state: o.get_field("state"),
            state_message: o.get_field("stateMessage"),
            uid: o.get_field("uid"),
            update_time: o.get_field("updateTime"),
            version: o.get_field("version"),
        }
    }
}
