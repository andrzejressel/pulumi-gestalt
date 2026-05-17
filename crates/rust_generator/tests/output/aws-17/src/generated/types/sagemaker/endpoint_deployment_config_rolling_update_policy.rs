#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct EndpointDeploymentConfigRollingUpdatePolicy {
    /// Batch size for each rolling step to provision capacity and turn on traffic on the new endpoint fleet, and terminate capacity on the old endpoint fleet. Value must be between 5% to 50% of the variant's total instance count. See Maximum Batch Size.
    #[builder(into)]
    #[serde(rename = "maximumBatchSize")]
    pub r#maximum_batch_size: Box<super::super::types::sagemaker::EndpointDeploymentConfigRollingUpdatePolicyMaximumBatchSize>,
    /// The time limit for the total deployment. Exceeding this limit causes a timeout. Valid values are between `600` and `14400`.
    #[builder(into)]
    #[serde(rename = "maximumExecutionTimeoutInSeconds")]
    pub r#maximum_execution_timeout_in_seconds: Option<i32>,
    /// Batch size for rollback to the old endpoint fleet. Each rolling step to provision capacity and turn on traffic on the old endpoint fleet, and terminate capacity on the new endpoint fleet. If this field is absent, the default value will be set to 100% of total capacity which means to bring up the whole capacity of the old fleet at once during rollback. See Rollback Maximum Batch Size.
    #[builder(into)]
    #[serde(rename = "rollbackMaximumBatchSize")]
    pub r#rollback_maximum_batch_size: Option<Box<super::super::types::sagemaker::EndpointDeploymentConfigRollingUpdatePolicyRollbackMaximumBatchSize>>,
    /// The length of the baking period, during which SageMaker monitors alarms for each batch on the new fleet. Valid values are between `0` and `3600`.
    #[builder(into)]
    #[serde(rename = "waitIntervalInSeconds")]
    pub r#wait_interval_in_seconds: i32,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for EndpointDeploymentConfigRollingUpdatePolicy {
    fn to_pulumi_value(
        &self,
    ) -> impl std::future::Future<
        Output = pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue,
    > {
        use pulumi_gestalt_rust::__private::futures::FutureExt;

        async move {
            use std::collections::BTreeMap;
            use pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue;
            use pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue;

            let mut map: BTreeMap<String, PulumiValue> = BTreeMap::new();
            map.insert(
                "maximum_batch_size".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#maximum_batch_size,
                )
                .await,
            );
            map.insert(
                "maximum_execution_timeout_in_seconds".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#maximum_execution_timeout_in_seconds,
                )
                .await,
            );
            map.insert(
                "rollback_maximum_batch_size".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#rollback_maximum_batch_size,
                )
                .await,
            );
            map.insert(
                "wait_interval_in_seconds".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#wait_interval_in_seconds,
                )
                .await,
            );

            ToPulumiValue::to_pulumi_value(
                &map,
            )
            .await
        }
        .boxed_local()
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for EndpointDeploymentConfigRollingUpdatePolicy {
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
                    r#maximum_batch_size: {
                        let field_value = match fields_map.get("maximum_batch_size") {
                            Some(value) => value,
                            None => bail!("Missing field 'maximum_batch_size' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#maximum_execution_timeout_in_seconds: {
                        let field_value = match fields_map.get("maximum_execution_timeout_in_seconds") {
                            Some(value) => value,
                            None => bail!("Missing field 'maximum_execution_timeout_in_seconds' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#rollback_maximum_batch_size: {
                        let field_value = match fields_map.get("rollback_maximum_batch_size") {
                            Some(value) => value,
                            None => bail!("Missing field 'rollback_maximum_batch_size' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#wait_interval_in_seconds: {
                        let field_value = match fields_map.get("wait_interval_in_seconds") {
                            Some(value) => value,
                            None => bail!("Missing field 'wait_interval_in_seconds' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
