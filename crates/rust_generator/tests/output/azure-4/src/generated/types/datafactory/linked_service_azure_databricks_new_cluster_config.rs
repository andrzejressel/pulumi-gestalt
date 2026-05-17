#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct LinkedServiceAzureDatabricksNewClusterConfig {
    /// Spark version of a the cluster.
    #[builder(into)]
    #[serde(rename = "clusterVersion")]
    pub r#cluster_version: String,
    /// Tags for the cluster resource.
    #[builder(into)]
    #[serde(rename = "customTags")]
    pub r#custom_tags: Option<std::collections::HashMap<String, String>>,
    /// Driver node type for the cluster.
    #[builder(into)]
    #[serde(rename = "driverNodeType")]
    pub r#driver_node_type: Option<String>,
    /// User defined initialization scripts for the cluster.
    #[builder(into)]
    #[serde(rename = "initScripts")]
    pub r#init_scripts: Option<Vec<String>>,
    /// Location to deliver Spark driver, worker, and event logs.
    #[builder(into)]
    #[serde(rename = "logDestination")]
    pub r#log_destination: Option<String>,
    /// Specifies the maximum number of worker nodes. It should be between 1 and 25000.
    #[builder(into)]
    #[serde(rename = "maxNumberOfWorkers")]
    pub r#max_number_of_workers: Option<i32>,
    /// Specifies the minimum number of worker nodes. It should be between 1 and 25000. It defaults to `1`.
    #[builder(into)]
    #[serde(rename = "minNumberOfWorkers")]
    pub r#min_number_of_workers: Option<i32>,
    /// Node type for the new cluster.
    #[builder(into)]
    #[serde(rename = "nodeType")]
    pub r#node_type: String,
    /// User-specified Spark configuration variables key-value pairs.
    #[builder(into)]
    #[serde(rename = "sparkConfig")]
    pub r#spark_config: Option<std::collections::HashMap<String, String>>,
    /// User-specified Spark environment variables key-value pairs.
    #[builder(into)]
    #[serde(rename = "sparkEnvironmentVariables")]
    pub r#spark_environment_variables: Option<std::collections::HashMap<String, String>>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for LinkedServiceAzureDatabricksNewClusterConfig {
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
                "cluster_version".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#cluster_version,
                )
                .await,
            );
            map.insert(
                "custom_tags".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#custom_tags,
                )
                .await,
            );
            map.insert(
                "driver_node_type".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#driver_node_type,
                )
                .await,
            );
            map.insert(
                "init_scripts".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#init_scripts,
                )
                .await,
            );
            map.insert(
                "log_destination".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#log_destination,
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
            map.insert(
                "node_type".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#node_type,
                )
                .await,
            );
            map.insert(
                "spark_config".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#spark_config,
                )
                .await,
            );
            map.insert(
                "spark_environment_variables".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#spark_environment_variables,
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

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for LinkedServiceAzureDatabricksNewClusterConfig {
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
                    r#custom_tags: {
                        let field_value = match fields_map.get("custom_tags") {
                            Some(value) => value,
                            None => bail!("Missing field 'custom_tags' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#driver_node_type: {
                        let field_value = match fields_map.get("driver_node_type") {
                            Some(value) => value,
                            None => bail!("Missing field 'driver_node_type' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#init_scripts: {
                        let field_value = match fields_map.get("init_scripts") {
                            Some(value) => value,
                            None => bail!("Missing field 'init_scripts' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#log_destination: {
                        let field_value = match fields_map.get("log_destination") {
                            Some(value) => value,
                            None => bail!("Missing field 'log_destination' while converting PulumiValue to {}", std::any::type_name::<Self>()),
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
                    r#node_type: {
                        let field_value = match fields_map.get("node_type") {
                            Some(value) => value,
                            None => bail!("Missing field 'node_type' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#spark_config: {
                        let field_value = match fields_map.get("spark_config") {
                            Some(value) => value,
                            None => bail!("Missing field 'spark_config' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#spark_environment_variables: {
                        let field_value = match fields_map.get("spark_environment_variables") {
                            Some(value) => value,
                            None => bail!("Missing field 'spark_environment_variables' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
