#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct PreventionJobTriggerInspectJobStorageConfig {
    /// Options defining BigQuery table and row identifiers.
    /// Structure is documented below.
    #[builder(into)]
    #[serde(rename = "bigQueryOptions")]
    pub r#big_query_options: Option<Box<super::super::types::dataloss::PreventionJobTriggerInspectJobStorageConfigBigQueryOptions>>,
    /// Options defining a file or a set of files within a Google Cloud Storage bucket.
    /// Structure is documented below.
    #[builder(into)]
    #[serde(rename = "cloudStorageOptions")]
    pub r#cloud_storage_options: Option<Box<super::super::types::dataloss::PreventionJobTriggerInspectJobStorageConfigCloudStorageOptions>>,
    /// Options defining a data set within Google Cloud Datastore.
    /// Structure is documented below.
    #[builder(into)]
    #[serde(rename = "datastoreOptions")]
    pub r#datastore_options: Option<Box<super::super::types::dataloss::PreventionJobTriggerInspectJobStorageConfigDatastoreOptions>>,
    /// Configuration to control jobs where the content being inspected is outside of Google Cloud Platform.
    /// Structure is documented below.
    #[builder(into)]
    #[serde(rename = "hybridOptions")]
    pub r#hybrid_options: Option<Box<super::super::types::dataloss::PreventionJobTriggerInspectJobStorageConfigHybridOptions>>,
    /// Configuration of the timespan of the items to include in scanning
    /// Structure is documented below.
    #[builder(into)]
    #[serde(rename = "timespanConfig")]
    pub r#timespan_config: Option<Box<super::super::types::dataloss::PreventionJobTriggerInspectJobStorageConfigTimespanConfig>>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for PreventionJobTriggerInspectJobStorageConfig {
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
                "big_query_options".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#big_query_options,
                )
                .await,
            );
            map.insert(
                "cloud_storage_options".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#cloud_storage_options,
                )
                .await,
            );
            map.insert(
                "datastore_options".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#datastore_options,
                )
                .await,
            );
            map.insert(
                "hybrid_options".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#hybrid_options,
                )
                .await,
            );
            map.insert(
                "timespan_config".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#timespan_config,
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

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for PreventionJobTriggerInspectJobStorageConfig {
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
                    r#big_query_options: {
                        let field_value = match fields_map.get("big_query_options") {
                            Some(value) => value,
                            None => bail!("Missing field 'big_query_options' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#cloud_storage_options: {
                        let field_value = match fields_map.get("cloud_storage_options") {
                            Some(value) => value,
                            None => bail!("Missing field 'cloud_storage_options' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#datastore_options: {
                        let field_value = match fields_map.get("datastore_options") {
                            Some(value) => value,
                            None => bail!("Missing field 'datastore_options' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#hybrid_options: {
                        let field_value = match fields_map.get("hybrid_options") {
                            Some(value) => value,
                            None => bail!("Missing field 'hybrid_options' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#timespan_config: {
                        let field_value = match fields_map.get("timespan_config") {
                            Some(value) => value,
                            None => bail!("Missing field 'timespan_config' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
