#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
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

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for WorkflowTemplateJob {
    fn to_pulumi_value(
        &self,
    ) -> impl std::future::Future<
        Output = pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue,
    > {
        use pulumi_gestalt_rust::__private::futures::FutureExt;

        async move {
            use pulumi_gestalt_rust::__private::pulumi_gestalt_model::__private::{
                to_pulumi_object_concurrent, to_pulumi_object_field, ToPulumiObjectFieldFuture,
            };
            let field_futures: Vec<ToPulumiObjectFieldFuture<'_>> = vec![
                to_pulumi_object_field(
                    "hadoop_job",
                    &self.r#hadoop_job,
                ),
                to_pulumi_object_field(
                    "hive_job",
                    &self.r#hive_job,
                ),
                to_pulumi_object_field(
                    "labels",
                    &self.r#labels,
                ),
                to_pulumi_object_field(
                    "pig_job",
                    &self.r#pig_job,
                ),
                to_pulumi_object_field(
                    "prerequisite_step_ids",
                    &self.r#prerequisite_step_ids,
                ),
                to_pulumi_object_field(
                    "presto_job",
                    &self.r#presto_job,
                ),
                to_pulumi_object_field(
                    "pyspark_job",
                    &self.r#pyspark_job,
                ),
                to_pulumi_object_field(
                    "scheduling",
                    &self.r#scheduling,
                ),
                to_pulumi_object_field(
                    "spark_job",
                    &self.r#spark_job,
                ),
                to_pulumi_object_field(
                    "spark_r_job",
                    &self.r#spark_r_job,
                ),
                to_pulumi_object_field(
                    "spark_sql_job",
                    &self.r#spark_sql_job,
                ),
                to_pulumi_object_field(
                    "step_id",
                    &self.r#step_id,
                ),
            ];
            to_pulumi_object_concurrent(field_futures).await
        }
        .boxed_local()
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for WorkflowTemplateJob {
    fn from_pulumi_value(
        value: &pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue,
    ) -> pulumi_gestalt_rust::__private::pulumi_gestalt_model::__private::rootcause::Result<Self> {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValueContent;
        use pulumi_gestalt_rust::__private::pulumi_gestalt_model::__private::rootcause::bail;
        use pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue;
        use pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue;

        match value.content {
            PulumiValueContent::Object(ref _obj) => {
                use std::collections::BTreeMap;
                let fields_map: BTreeMap<String, PulumiValue> =
                    _obj.iter().cloned().collect();

                Ok(Self {
                    r#hadoop_job: {
                        let field_value = match fields_map.get("hadoop_job") {
                            Some(value) => value,
                            None => bail!("Missing field 'hadoop_job' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#hive_job: {
                        let field_value = match fields_map.get("hive_job") {
                            Some(value) => value,
                            None => bail!("Missing field 'hive_job' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#labels: {
                        let field_value = match fields_map.get("labels") {
                            Some(value) => value,
                            None => bail!("Missing field 'labels' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#pig_job: {
                        let field_value = match fields_map.get("pig_job") {
                            Some(value) => value,
                            None => bail!("Missing field 'pig_job' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#prerequisite_step_ids: {
                        let field_value = match fields_map.get("prerequisite_step_ids") {
                            Some(value) => value,
                            None => bail!("Missing field 'prerequisite_step_ids' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#presto_job: {
                        let field_value = match fields_map.get("presto_job") {
                            Some(value) => value,
                            None => bail!("Missing field 'presto_job' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#pyspark_job: {
                        let field_value = match fields_map.get("pyspark_job") {
                            Some(value) => value,
                            None => bail!("Missing field 'pyspark_job' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#scheduling: {
                        let field_value = match fields_map.get("scheduling") {
                            Some(value) => value,
                            None => bail!("Missing field 'scheduling' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#spark_job: {
                        let field_value = match fields_map.get("spark_job") {
                            Some(value) => value,
                            None => bail!("Missing field 'spark_job' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#spark_r_job: {
                        let field_value = match fields_map.get("spark_r_job") {
                            Some(value) => value,
                            None => bail!("Missing field 'spark_r_job' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#spark_sql_job: {
                        let field_value = match fields_map.get("spark_sql_job") {
                            Some(value) => value,
                            None => bail!("Missing field 'spark_sql_job' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#step_id: {
                        let field_value = match fields_map.get("step_id") {
                            Some(value) => value,
                            None => bail!("Missing field 'step_id' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
