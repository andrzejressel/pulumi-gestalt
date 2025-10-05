/// A user-defined function or a stored procedure that belongs to a Dataset
///
///
/// To get more information about Routine, see:
///
/// * [API documentation](https://cloud.google.com/bigquery/docs/reference/rest/v2/routines)
/// * How-to Guides
///     * [Routines Intro](https://cloud.google.com/bigquery/docs/reference/rest/v2/routines)
///
/// ## Example Usage
///
/// ### Bigquery Routine Basic
///
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let sproc = routine::create(
///         "sproc",
///         RoutineArgs::builder()
///             .dataset_id("${test.datasetId}")
///             .definition_body(
///                 "CREATE FUNCTION Add(x FLOAT64, y FLOAT64) RETURNS FLOAT64 AS (x + y);",
///             )
///             .language("SQL")
///             .routine_id("routine_id")
///             .routine_type("PROCEDURE")
///             .build_struct(),
///     );
///     let test = dataset::create(
///         "test",
///         DatasetArgs::builder().dataset_id("dataset_id").build_struct(),
///     );
/// }
/// ```
/// ### Bigquery Routine Json
///
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let sproc = routine::create(
///         "sproc",
///         RoutineArgs::builder()
///             .arguments(
///                 vec![
///                     RoutineArgument::builder().dataType("{\"typeKind\" :  \"FLOAT64\"}")
///                     .name("x").build_struct(), RoutineArgument::builder()
///                     .dataType("{\"typeKind\" :  \"FLOAT64\"}").name("y").build_struct(),
///                 ],
///             )
///             .dataset_id("${test.datasetId}")
///             .definition_body("CREATE FUNCTION multiplyInputs return x*y;")
///             .language("JAVASCRIPT")
///             .return_type("{\"typeKind\" :  \"FLOAT64\"}")
///             .routine_id("routine_id")
///             .routine_type("SCALAR_FUNCTION")
///             .build_struct(),
///     );
///     let test = dataset::create(
///         "test",
///         DatasetArgs::builder().dataset_id("dataset_id").build_struct(),
///     );
/// }
/// ```
/// ### Bigquery Routine Tvf
///
///
/// ```yaml
/// resources:
///   test:
///     type: gcp:bigquery:Dataset
///     properties:
///       datasetId: dataset_id
///   sproc:
///     type: gcp:bigquery:Routine
///     properties:
///       datasetId: ${test.datasetId}
///       routineId: routine_id
///       routineType: TABLE_VALUED_FUNCTION
///       language: SQL
///       definitionBody: |
///         SELECT 1 + value AS value
///       arguments:
///         - name: value
///           argumentKind: FIXED_TYPE
///           dataType:
///             fn::toJSON:
///               typeKind: INT64
///       returnTableType:
///         fn::toJSON:
///           columns:
///             - name: value
///               type:
///                 typeKind: INT64
/// ```
/// ### Bigquery Routine Pyspark
///
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let pyspark = routine::create(
///         "pyspark",
///         RoutineArgs::builder()
///             .dataset_id("${test.datasetId}")
///             .definition_body(
///                 "from pyspark.sql import SparkSession\n\nspark = SparkSession.builder.appName(\"spark-bigquery-demo\").getOrCreate()\n    \n# Load data from BigQuery.\nwords = spark.read.format(\"bigquery\") \\\n  .option(\"table\", \"bigquery-public-data:samples.shakespeare\") \\\n  .load()\nwords.createOrReplaceTempView(\"words\")\n    \n# Perform word count.\nword_count = words.select('word', 'word_count').groupBy('word').sum('word_count').withColumnRenamed(\"sum(word_count)\", \"sum_word_count\")\nword_count.show()\nword_count.printSchema()\n    \n# Saving the data to BigQuery\nword_count.write.format(\"bigquery\") \\\n  .option(\"writeMethod\", \"direct\") \\\n  .save(\"wordcount_dataset.wordcount_output\")\n",
///             )
///             .language("PYTHON")
///             .routine_id("routine_id")
///             .routine_type("PROCEDURE")
///             .spark_options(
///                 RoutineSparkOptions::builder()
///                     .connection("${testConnection.name}")
///                     .runtimeVersion("2.1")
///                     .build_struct(),
///             )
///             .build_struct(),
///     );
///     let test = dataset::create(
///         "test",
///         DatasetArgs::builder().dataset_id("dataset_id").build_struct(),
///     );
///     let testConnection = connection::create(
///         "testConnection",
///         ConnectionArgs::builder()
///             .connection_id("connection_id")
///             .location("US")
///             .spark(ConnectionSpark::builder().build_struct())
///             .build_struct(),
///     );
/// }
/// ```
/// ### Bigquery Routine Pyspark Mainfile
///
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let pysparkMainfile = routine::create(
///         "pysparkMainfile",
///         RoutineArgs::builder()
///             .dataset_id("${test.datasetId}")
///             .definition_body("")
///             .language("PYTHON")
///             .routine_id("routine_id")
///             .routine_type("PROCEDURE")
///             .spark_options(
///                 RoutineSparkOptions::builder()
///                     .archiveUris(vec!["gs://test-bucket/distribute_in_executor.tar.gz",])
///                     .connection("${testConnection.name}")
///                     .fileUris(vec!["gs://test-bucket/distribute_in_executor.json",])
///                     .mainFileUri("gs://test-bucket/main.py")
///                     .pyFileUris(vec!["gs://test-bucket/lib.py",])
///                     .runtimeVersion("2.1")
///                     .build_struct(),
///             )
///             .build_struct(),
///     );
///     let test = dataset::create(
///         "test",
///         DatasetArgs::builder().dataset_id("dataset_id").build_struct(),
///     );
///     let testConnection = connection::create(
///         "testConnection",
///         ConnectionArgs::builder()
///             .connection_id("connection_id")
///             .location("US")
///             .spark(ConnectionSpark::builder().build_struct())
///             .build_struct(),
///     );
/// }
/// ```
/// ### Bigquery Routine Spark Jar
///
///
/// ```yaml
/// resources:
///   test:
///     type: gcp:bigquery:Dataset
///     properties:
///       datasetId: dataset_id
///   testConnection:
///     type: gcp:bigquery:Connection
///     name: test
///     properties:
///       connectionId: connection_id
///       location: US
///       spark: {}
///   sparkJar:
///     type: gcp:bigquery:Routine
///     name: spark_jar
///     properties:
///       datasetId: ${test.datasetId}
///       routineId: routine_id
///       routineType: PROCEDURE
///       language: SCALA
///       definitionBody: ""
///       sparkOptions:
///         connection: ${testConnection.name}
///         runtimeVersion: '2.1'
///         containerImage: gcr.io/my-project-id/my-spark-image:latest
///         mainClass: com.google.test.jar.MainClass
///         jarUris:
///           - gs://test-bucket/uberjar_spark_spark3.jar
///         properties:
///           spark.dataproc.scaling.version: '2'
///           spark.reducer.fetchMigratedShuffle.enabled: 'true'
/// ```
/// ### Bigquery Routine Data Governance Type
///
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let customMaskingRoutine = routine::create(
///         "customMaskingRoutine",
///         RoutineArgs::builder()
///             .arguments(
///                 vec![
///                     RoutineArgument::builder().dataType("{\"typeKind\" :  \"STRING\"}")
///                     .name("ssn").build_struct(),
///                 ],
///             )
///             .data_governance_type("DATA_MASKING")
///             .dataset_id("${test.datasetId}")
///             .definition_body("SAFE.REGEXP_REPLACE(ssn, '[0-9]', 'X')")
///             .language("SQL")
///             .return_type("{\"typeKind\" :  \"STRING\"}")
///             .routine_id("custom_masking_routine")
///             .routine_type("SCALAR_FUNCTION")
///             .build_struct(),
///     );
///     let test = dataset::create(
///         "test",
///         DatasetArgs::builder().dataset_id("tf_test_dataset_id_15222").build_struct(),
///     );
/// }
/// ```
/// ### Bigquery Routine Remote Function
///
///
/// ```yaml
/// resources:
///   test:
///     type: gcp:bigquery:Dataset
///     properties:
///       datasetId: dataset_id
///   testConnection:
///     type: gcp:bigquery:Connection
///     name: test
///     properties:
///       connectionId: connection_id
///       location: US
///       cloudResource: {}
///   remoteFunction:
///     type: gcp:bigquery:Routine
///     name: remote_function
///     properties:
///       datasetId: ${test.datasetId}
///       routineId: routine_id
///       routineType: SCALAR_FUNCTION
///       definitionBody: ""
///       returnType: '{"typeKind" :  "STRING"}'
///       remoteFunctionOptions:
///         endpoint: https://us-east1-my_gcf_project.cloudfunctions.net/remote_add
///         connection: ${testConnection.name}
///         maxBatchingRows: '10'
///         userDefinedContext:
///           z: '1.5'
/// ```
///
/// ## Import
///
/// Routine can be imported using any of these accepted formats:
///
/// * `projects/{{project}}/datasets/{{dataset_id}}/routines/{{routine_id}}`
///
/// * `{{project}}/{{dataset_id}}/{{routine_id}}`
///
/// * `{{dataset_id}}/{{routine_id}}`
///
/// When using the `pulumi import` command, Routine can be imported using one of the formats above. For example:
///
/// ```sh
/// $ pulumi import gcp:bigquery/routine:Routine default projects/{{project}}/datasets/{{dataset_id}}/routines/{{routine_id}}
/// ```
///
/// ```sh
/// $ pulumi import gcp:bigquery/routine:Routine default {{project}}/{{dataset_id}}/{{routine_id}}
/// ```
///
/// ```sh
/// $ pulumi import gcp:bigquery/routine:Routine default {{dataset_id}}/{{routine_id}}
/// ```
///
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod routine {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct RoutineArgs {
        /// Input/output argument of a function or a stored procedure.
        /// Structure is documented below.
        #[builder(into, default)]
        pub arguments: pulumi_gestalt_rust::InputOrOutput<
            Option<Vec<super::super::types::bigquery::RoutineArgument>>,
        >,
        /// If set to DATA_MASKING, the function is validated and made available as a masking function. For more information, see https://cloud.google.com/bigquery/docs/user-defined-functions#custom-mask
        /// Possible values are: `DATA_MASKING`.
        #[builder(into, default)]
        pub data_governance_type: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The ID of the dataset containing this routine
        #[builder(into)]
        pub dataset_id: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The body of the routine. For functions, this is the expression in the AS clause.
        /// If language=SQL, it is the substring inside (but excluding) the parentheses.
        ///
        ///
        /// - - -
        #[builder(into)]
        pub definition_body: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The description of the routine if defined.
        #[builder(into, default)]
        pub description: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The determinism level of the JavaScript UDF if defined.
        /// Possible values are: `DETERMINISM_LEVEL_UNSPECIFIED`, `DETERMINISTIC`, `NOT_DETERMINISTIC`.
        #[builder(into, default)]
        pub determinism_level: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Optional. If language = "JAVASCRIPT", this field stores the path of the
        /// imported JAVASCRIPT libraries.
        #[builder(into, default)]
        pub imported_libraries: pulumi_gestalt_rust::InputOrOutput<Option<Vec<String>>>,
        /// The language of the routine.
        /// Possible values are: `SQL`, `JAVASCRIPT`, `PYTHON`, `JAVA`, `SCALA`.
        #[builder(into, default)]
        pub language: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The ID of the project in which the resource belongs.
        /// If it is not provided, the provider project is used.
        #[builder(into, default)]
        pub project: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Remote function specific options.
        /// Structure is documented below.
        #[builder(into, default)]
        pub remote_function_options: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::bigquery::RoutineRemoteFunctionOptions>,
        >,
        /// Optional. Can be set only if routineType = "TABLE_VALUED_FUNCTION".
        /// If absent, the return table type is inferred from definitionBody at query time in each query
        /// that references this routine. If present, then the columns in the evaluated table result will
        /// be cast to match the column types specificed in return table type, at query time.
        #[builder(into, default)]
        pub return_table_type: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// A JSON schema for the return type. Optional if language = "SQL"; required otherwise.
        /// If absent, the return type is inferred from definitionBody at query time in each query
        /// that references this routine. If present, then the evaluated result will be cast to
        /// the specified returned type at query time. ~>**NOTE**: Because this field expects a JSON
        /// string, any changes to the string will create a diff, even if the JSON itself hasn't
        /// changed. If the API returns a different value for the same schema, e.g. it switche
        /// d the order of values or replaced STRUCT field type with RECORD field type, we currently
        /// cannot suppress the recurring diff this causes. As a workaround, we recommend using
        /// the schema as returned by the API.
        #[builder(into, default)]
        pub return_type: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The ID of the the routine. The ID must contain only letters (a-z, A-Z), numbers (0-9), or underscores (_). The maximum length is 256 characters.
        #[builder(into)]
        pub routine_id: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The type of routine.
        /// Possible values are: `SCALAR_FUNCTION`, `PROCEDURE`, `TABLE_VALUED_FUNCTION`.
        #[builder(into)]
        pub routine_type: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Optional. If language is one of "PYTHON", "JAVA", "SCALA", this field stores the options for spark stored procedure.
        /// Structure is documented below.
        #[builder(into, default)]
        pub spark_options: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::bigquery::RoutineSparkOptions>,
        >,
    }
    #[allow(dead_code)]
    pub struct RoutineResult {
        /// Pulumi ID is the provider-assigned unique ID for this managed resource.
        /// It is set during deployments and may be missing (unknown) during planning phases.
        pub id: pulumi_gestalt_rust::Output<String>,
        /// Input/output argument of a function or a stored procedure.
        /// Structure is documented below.
        pub arguments: pulumi_gestalt_rust::Output<
            Option<Vec<super::super::types::bigquery::RoutineArgument>>,
        >,
        /// The time when this routine was created, in milliseconds since the
        /// epoch.
        pub creation_time: pulumi_gestalt_rust::Output<i32>,
        /// If set to DATA_MASKING, the function is validated and made available as a masking function. For more information, see https://cloud.google.com/bigquery/docs/user-defined-functions#custom-mask
        /// Possible values are: `DATA_MASKING`.
        pub data_governance_type: pulumi_gestalt_rust::Output<Option<String>>,
        /// The ID of the dataset containing this routine
        pub dataset_id: pulumi_gestalt_rust::Output<String>,
        /// The body of the routine. For functions, this is the expression in the AS clause.
        /// If language=SQL, it is the substring inside (but excluding) the parentheses.
        ///
        ///
        /// - - -
        pub definition_body: pulumi_gestalt_rust::Output<String>,
        /// The description of the routine if defined.
        pub description: pulumi_gestalt_rust::Output<Option<String>>,
        /// The determinism level of the JavaScript UDF if defined.
        /// Possible values are: `DETERMINISM_LEVEL_UNSPECIFIED`, `DETERMINISTIC`, `NOT_DETERMINISTIC`.
        pub determinism_level: pulumi_gestalt_rust::Output<Option<String>>,
        /// Optional. If language = "JAVASCRIPT", this field stores the path of the
        /// imported JAVASCRIPT libraries.
        pub imported_libraries: pulumi_gestalt_rust::Output<Option<Vec<String>>>,
        /// The language of the routine.
        /// Possible values are: `SQL`, `JAVASCRIPT`, `PYTHON`, `JAVA`, `SCALA`.
        pub language: pulumi_gestalt_rust::Output<Option<String>>,
        /// The time when this routine was modified, in milliseconds since the
        /// epoch.
        pub last_modified_time: pulumi_gestalt_rust::Output<i32>,
        /// The ID of the project in which the resource belongs.
        /// If it is not provided, the provider project is used.
        pub project: pulumi_gestalt_rust::Output<String>,
        /// Remote function specific options.
        /// Structure is documented below.
        pub remote_function_options: pulumi_gestalt_rust::Output<
            Option<super::super::types::bigquery::RoutineRemoteFunctionOptions>,
        >,
        /// Optional. Can be set only if routineType = "TABLE_VALUED_FUNCTION".
        /// If absent, the return table type is inferred from definitionBody at query time in each query
        /// that references this routine. If present, then the columns in the evaluated table result will
        /// be cast to match the column types specificed in return table type, at query time.
        pub return_table_type: pulumi_gestalt_rust::Output<Option<String>>,
        /// A JSON schema for the return type. Optional if language = "SQL"; required otherwise.
        /// If absent, the return type is inferred from definitionBody at query time in each query
        /// that references this routine. If present, then the evaluated result will be cast to
        /// the specified returned type at query time. ~>**NOTE**: Because this field expects a JSON
        /// string, any changes to the string will create a diff, even if the JSON itself hasn't
        /// changed. If the API returns a different value for the same schema, e.g. it switche
        /// d the order of values or replaced STRUCT field type with RECORD field type, we currently
        /// cannot suppress the recurring diff this causes. As a workaround, we recommend using
        /// the schema as returned by the API.
        pub return_type: pulumi_gestalt_rust::Output<Option<String>>,
        /// The ID of the the routine. The ID must contain only letters (a-z, A-Z), numbers (0-9), or underscores (_). The maximum length is 256 characters.
        pub routine_id: pulumi_gestalt_rust::Output<String>,
        /// The type of routine.
        /// Possible values are: `SCALAR_FUNCTION`, `PROCEDURE`, `TABLE_VALUED_FUNCTION`.
        pub routine_type: pulumi_gestalt_rust::Output<String>,
        /// Optional. If language is one of "PYTHON", "JAVA", "SCALA", this field stores the options for spark stored procedure.
        /// Structure is documented below.
        pub spark_options: pulumi_gestalt_rust::Output<
            Option<super::super::types::bigquery::RoutineSparkOptions>,
        >,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: RoutineArgs,
    ) -> RoutineResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let arguments_binding = args.arguments.get_output(context);
        let data_governance_type_binding = args.data_governance_type.get_output(context);
        let dataset_id_binding = args.dataset_id.get_output(context);
        let definition_body_binding = args.definition_body.get_output(context);
        let description_binding = args.description.get_output(context);
        let determinism_level_binding = args.determinism_level.get_output(context);
        let imported_libraries_binding = args.imported_libraries.get_output(context);
        let language_binding = args.language.get_output(context);
        let project_binding = args.project.get_output(context);
        let remote_function_options_binding = args
            .remote_function_options
            .get_output(context);
        let return_table_type_binding = args.return_table_type.get_output(context);
        let return_type_binding = args.return_type.get_output(context);
        let routine_id_binding = args.routine_id.get_output(context);
        let routine_type_binding = args.routine_type.get_output(context);
        let spark_options_binding = args.spark_options.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "gcp:bigquery/routine:Routine".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "arguments".into(),
                    value: &arguments_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "dataGovernanceType".into(),
                    value: &data_governance_type_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "datasetId".into(),
                    value: &dataset_id_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "definitionBody".into(),
                    value: &definition_body_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "description".into(),
                    value: &description_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "determinismLevel".into(),
                    value: &determinism_level_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "importedLibraries".into(),
                    value: &imported_libraries_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "language".into(),
                    value: &language_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "project".into(),
                    value: &project_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "remoteFunctionOptions".into(),
                    value: &remote_function_options_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "returnTableType".into(),
                    value: &return_table_type_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "returnType".into(),
                    value: &return_type_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "routineId".into(),
                    value: &routine_id_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "routineType".into(),
                    value: &routine_type_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "sparkOptions".into(),
                    value: &spark_options_binding.drop_type(),
                },
            ],
        };
        let o = context.register_resource(request);
        RoutineResult {
            id: o.get_field("id"),
            arguments: o.get_field("arguments"),
            creation_time: o.get_field("creationTime"),
            data_governance_type: o.get_field("dataGovernanceType"),
            dataset_id: o.get_field("datasetId"),
            definition_body: o.get_field("definitionBody"),
            description: o.get_field("description"),
            determinism_level: o.get_field("determinismLevel"),
            imported_libraries: o.get_field("importedLibraries"),
            language: o.get_field("language"),
            last_modified_time: o.get_field("lastModifiedTime"),
            project: o.get_field("project"),
            remote_function_options: o.get_field("remoteFunctionOptions"),
            return_table_type: o.get_field("returnTableType"),
            return_type: o.get_field("returnType"),
            routine_id: o.get_field("routineId"),
            routine_type: o.get_field("routineType"),
            spark_options: o.get_field("sparkOptions"),
        }
    }
}
