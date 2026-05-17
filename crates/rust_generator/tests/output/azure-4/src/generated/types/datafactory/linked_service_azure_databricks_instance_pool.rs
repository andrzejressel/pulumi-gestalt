#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct LinkedServiceAzureDatabricksInstancePool {
    /// Spark version of a the cluster.
    #[builder(into)]
    #[serde(rename = "clusterVersion")]
    pub r#cluster_version: String,
    /// Identifier of the instance pool within the linked ADB instance.
    #[builder(into)]
    #[serde(rename = "instancePoolId")]
    pub r#instance_pool_id: String,
    /// The max number of worker nodes. Set this value if you want to enable autoscaling between the `min_number_of_workers` and this value. Omit this value to use a fixed number of workers defined in the `min_number_of_workers` property.
    #[builder(into)]
    #[serde(rename = "maxNumberOfWorkers")]
    pub r#max_number_of_workers: Option<i32>,
    /// The minimum number of worker nodes. Defaults to `1`.
    #[builder(into)]
    #[serde(rename = "minNumberOfWorkers")]
    pub r#min_number_of_workers: Option<i32>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for LinkedServiceAzureDatabricksInstancePool {
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
                "cluster_version".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#cluster_version,
                )
                .await,
            );
            map.insert(
                "instance_pool_id".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#instance_pool_id,
                )
                .await,
            );
            map.insert(
                "max_number_of_workers".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#max_number_of_workers,
                )
                .await,
            );
            map.insert(
                "min_number_of_workers".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#min_number_of_workers,
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

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for LinkedServiceAzureDatabricksInstancePool {
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
                    r#cluster_version: {
                        let field_value = match fields_map.get("cluster_version") {
                            Some(value) => value,
                            None => bail!("Missing field 'cluster_version' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#instance_pool_id: {
                        let field_value = match fields_map.get("instance_pool_id") {
                            Some(value) => value,
                            None => bail!("Missing field 'instance_pool_id' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#max_number_of_workers: {
                        let field_value = match fields_map.get("max_number_of_workers") {
                            Some(value) => value,
                            None => bail!("Missing field 'max_number_of_workers' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#min_number_of_workers: {
                        let field_value = match fields_map.get("min_number_of_workers") {
                            Some(value) => value,
                            None => bail!("Missing field 'min_number_of_workers' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
