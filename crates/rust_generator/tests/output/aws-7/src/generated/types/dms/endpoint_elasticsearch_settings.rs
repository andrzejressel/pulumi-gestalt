#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct EndpointElasticsearchSettings {
    /// Endpoint for the OpenSearch cluster.
    #[builder(into)]
    #[serde(rename = "endpointUri")]
    pub r#endpoint_uri: String,
    /// Maximum number of seconds for which DMS retries failed API requests to the OpenSearch cluster. Default is `300`.
    #[builder(into)]
    #[serde(rename = "errorRetryDuration")]
    pub r#error_retry_duration: Option<i32>,
    /// Maximum percentage of records that can fail to be written before a full load operation stops. Default is `10`.
    #[builder(into)]
    #[serde(rename = "fullLoadErrorPercentage")]
    pub r#full_load_error_percentage: Option<i32>,
    /// ARN of the IAM Role with permissions to write to the OpenSearch cluster.
    #[builder(into)]
    #[serde(rename = "serviceAccessRoleArn")]
    pub r#service_access_role_arn: String,
    /// Enable to migrate documentation using the documentation type `_doc`. OpenSearch and an Elasticsearch clusters only support the _doc documentation type in versions 7.x and later. The default value is `false`.
    #[builder(into)]
    #[serde(rename = "useNewMappingType")]
    pub r#use_new_mapping_type: Option<bool>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for EndpointElasticsearchSettings {
    fn to_pulumi_value(
        &self,
    ) -> impl std::future::Future<
        Output = pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue,
    > + Send {
        use pulumi_gestalt_rust::__private::futures::FutureExt;
        use pulumi_gestalt_rust::__private::pulumi_gestalt_model::__private::to_pulumi_object_concurrent;
        async move {
            use pulumi_gestalt_rust::__private::pulumi_gestalt_model::__private::{
                to_pulumi_object_field, ToPulumiObjectFieldFuture,
            };
            let field_futures: Vec<ToPulumiObjectFieldFuture<'_>> = vec![
                to_pulumi_object_field(
                    "endpoint_uri",
                    &self.r#endpoint_uri,
                ),
                to_pulumi_object_field(
                    "error_retry_duration",
                    &self.r#error_retry_duration,
                ),
                to_pulumi_object_field(
                    "full_load_error_percentage",
                    &self.r#full_load_error_percentage,
                ),
                to_pulumi_object_field(
                    "service_access_role_arn",
                    &self.r#service_access_role_arn,
                ),
                to_pulumi_object_field(
                    "use_new_mapping_type",
                    &self.r#use_new_mapping_type,
                ),
            ];
            to_pulumi_object_concurrent(field_futures).await
        }
        .boxed()
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for EndpointElasticsearchSettings {
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
                    r#endpoint_uri: {
                        let field_value = match fields_map.get("endpoint_uri") {
                            Some(value) => value,
                            None => bail!("Missing field 'endpoint_uri' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#error_retry_duration: {
                        let field_value = match fields_map.get("error_retry_duration") {
                            Some(value) => value,
                            None => bail!("Missing field 'error_retry_duration' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#full_load_error_percentage: {
                        let field_value = match fields_map.get("full_load_error_percentage") {
                            Some(value) => value,
                            None => bail!("Missing field 'full_load_error_percentage' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#service_access_role_arn: {
                        let field_value = match fields_map.get("service_access_role_arn") {
                            Some(value) => value,
                            None => bail!("Missing field 'service_access_role_arn' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#use_new_mapping_type: {
                        let field_value = match fields_map.get("use_new_mapping_type") {
                            Some(value) => value,
                            None => bail!("Missing field 'use_new_mapping_type' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
