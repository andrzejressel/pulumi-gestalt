#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct WorkflowTemplateJob {
    /// Job is a Hadoop job.
    #[builder(into)]
    #[serde(rename = "hadoopJob")]
    pub r#hadoop_job: Option<Box<super::super::types::dataproc::WorkflowTemplateJobHadoopJob>>,
    /// Job is a Hive job.
    #[builder(into)]
    #[serde(rename = "hiveJob")]
    pub r#hive_job: Option<Box<super::super::types::dataproc::WorkflowTemplateJobHiveJob>>,
    /// The labels to associate with this job. Label keys must be between 1 and 63 characters long, and must conform to the following regular expression: {0,63} No more than 32 labels can be associated with a given job.
    #[builder(into)]
    #[serde(rename = "labels")]
    pub r#labels: Option<std::collections::HashMap<String, String>>,
    /// Job is a Pig job.
    #[builder(into)]
    #[serde(rename = "pigJob")]
    pub r#pig_job: Option<Box<super::super::types::dataproc::WorkflowTemplateJobPigJob>>,
    /// The optional list of prerequisite job step_ids. If not specified, the job will start at the beginning of workflow.
    #[builder(into)]
    #[serde(rename = "prerequisiteStepIds")]
    pub r#prerequisite_step_ids: Option<Vec<String>>,
    /// Job is a Presto job.
    #[builder(into)]
    #[serde(rename = "prestoJob")]
    pub r#presto_job: Option<Box<super::super::types::dataproc::WorkflowTemplateJobPrestoJob>>,
    /// Job is a PySpark job.
    #[builder(into)]
    #[serde(rename = "pysparkJob")]
    pub r#pyspark_job: Option<Box<super::super::types::dataproc::WorkflowTemplateJobPysparkJob>>,
    /// Job scheduling configuration.
    #[builder(into)]
    #[serde(rename = "scheduling")]
    pub r#scheduling: Option<Box<super::super::types::dataproc::WorkflowTemplateJobScheduling>>,
    /// Job is a Spark job.
    #[builder(into)]
    #[serde(rename = "sparkJob")]
    pub r#spark_job: Option<Box<super::super::types::dataproc::WorkflowTemplateJobSparkJob>>,
    /// Job is a SparkR job.
    #[builder(into)]
    #[serde(rename = "sparkRJob")]
    pub r#spark_r_job: Option<Box<super::super::types::dataproc::WorkflowTemplateJobSparkRJob>>,
    /// Job is a SparkSql job.
    #[builder(into)]
    #[serde(rename = "sparkSqlJob")]
    pub r#spark_sql_job: Option<Box<super::super::types::dataproc::WorkflowTemplateJobSparkSqlJob>>,
    /// Required. The step id. The id must be unique among all jobs within the template. The step id is used as prefix for job id, as job `goog-dataproc-workflow-step-id` label, and in field from other steps. The id must contain only letters (a-z, A-Z), numbers (0-9), underscores (_), and hyphens (-). Cannot begin or end with underscore or hyphen. Must consist of between 3 and 50 characters.
    #[builder(into)]
    #[serde(rename = "stepId")]
    pub r#step_id: String,
}
