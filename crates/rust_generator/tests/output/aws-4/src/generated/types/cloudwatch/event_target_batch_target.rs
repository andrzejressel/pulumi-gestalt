#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct EventTargetBatchTarget {
    /// The size of the array, if this is an array batch job. Valid values are integers between 2 and 10,000.
    #[builder(into)]
    #[serde(rename = "arraySize")]
    pub r#array_size: Option<i32>,
    /// The number of times to attempt to retry, if the job fails. Valid values are 1 to 10.
    #[builder(into)]
    #[serde(rename = "jobAttempts")]
    pub r#job_attempts: Option<i32>,
    /// The ARN or name of the job definition to use if the event target is an AWS Batch job. This job definition must already exist.
    #[builder(into)]
    #[serde(rename = "jobDefinition")]
    pub r#job_definition: String,
    /// The name to use for this execution of the job, if the target is an AWS Batch job.
    #[builder(into)]
    #[serde(rename = "jobName")]
    pub r#job_name: String,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for EventTargetBatchTarget {
    fn to_pulumi_value(
        &self,
    ) -> impl std::future::Future<
        Output = pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue,
    > {
        async move {
            use std::collections::BTreeMap;
            use pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue;

            let mut map: BTreeMap<String, pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue> = BTreeMap::new();
            map.insert("array_size".to_string(), self.r#array_size.to_pulumi_value().await);
            map.insert("job_attempts".to_string(), self.r#job_attempts.to_pulumi_value().await);
            map.insert("job_definition".to_string(), self.r#job_definition.to_pulumi_value().await);
            map.insert("job_name".to_string(), self.r#job_name.to_pulumi_value().await);

            map.to_pulumi_value().await
        }
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for EventTargetBatchTarget {
    fn from_pulumi_value(
        value: &pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue,
    ) -> pulumi_gestalt_rust::__private::rootcause::Result<Self> {
        use std::collections::BTreeMap;
        use pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValueContent;
        use pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue;
        use pulumi_gestalt_rust::__private::rootcause::bail;

        match value.content {
            PulumiValueContent::Object(ref obj) => {
                let fields_map: BTreeMap<String, pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue> =
                    obj.iter().cloned().collect();

                Ok(Self {
                    r#array_size: {
                        let field_value = match fields_map.get("array_size") {
                            Some(value) => value,
                            None => bail!("Missing field 'array_size' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Option<i32> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#job_attempts: {
                        let field_value = match fields_map.get("job_attempts") {
                            Some(value) => value,
                            None => bail!("Missing field 'job_attempts' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Option<i32> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#job_definition: {
                        let field_value = match fields_map.get("job_definition") {
                            Some(value) => value,
                            None => bail!("Missing field 'job_definition' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <String as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#job_name: {
                        let field_value = match fields_map.get("job_name") {
                            Some(value) => value,
                            None => bail!("Missing field 'job_name' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <String as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
