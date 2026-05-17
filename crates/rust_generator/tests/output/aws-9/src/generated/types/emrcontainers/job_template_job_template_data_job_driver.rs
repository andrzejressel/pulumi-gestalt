#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct JobTemplateJobTemplateDataJobDriver {
    /// The job driver for job type.
    #[builder(into)]
    #[serde(rename = "sparkSqlJobDriver")]
    pub r#spark_sql_job_driver: Option<Box<super::super::types::emrcontainers::JobTemplateJobTemplateDataJobDriverSparkSqlJobDriver>>,
    /// The job driver parameters specified for spark submit.
    #[builder(into)]
    #[serde(rename = "sparkSubmitJobDriver")]
    pub r#spark_submit_job_driver: Option<Box<super::super::types::emrcontainers::JobTemplateJobTemplateDataJobDriverSparkSubmitJobDriver>>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for JobTemplateJobTemplateDataJobDriver {
    fn to_pulumi_value(
        &self,
    ) -> impl std::future::Future<
        Output = pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue,
    > {
        async move {
            use std::collections::BTreeMap;
            use pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue;
            use pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue;

            let mut map: BTreeMap<String, PulumiValue> = BTreeMap::new();
            map.insert(
                "spark_sql_job_driver".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#spark_sql_job_driver,
                )
                .await,
            );
            map.insert(
                "spark_submit_job_driver".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#spark_submit_job_driver,
                )
                .await,
            );

            ToPulumiValue::to_pulumi_value(
                &map,
            )
            .await
        }
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for JobTemplateJobTemplateDataJobDriver {
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
                    r#spark_sql_job_driver: {
                        let field_value = match fields_map.get("spark_sql_job_driver") {
                            Some(value) => value,
                            None => bail!("Missing field 'spark_sql_job_driver' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#spark_submit_job_driver: {
                        let field_value = match fields_map.get("spark_submit_job_driver") {
                            Some(value) => value,
                            None => bail!("Missing field 'spark_submit_job_driver' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
