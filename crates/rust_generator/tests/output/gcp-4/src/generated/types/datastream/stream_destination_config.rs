#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct StreamDestinationConfig {
    /// A configuration for how data should be loaded to Google BigQuery.
    /// Structure is documented below.
    #[builder(into)]
    #[serde(rename = "bigqueryDestinationConfig")]
    pub r#bigquery_destination_config: Option<Box<super::super::types::datastream::StreamDestinationConfigBigqueryDestinationConfig>>,
    /// Destination connection profile resource. Format: projects/{project}/locations/{location}/connectionProfiles/{name}
    #[builder(into)]
    #[serde(rename = "destinationConnectionProfile")]
    pub r#destination_connection_profile: String,
    /// A configuration for how data should be loaded to Cloud Storage.
    /// Structure is documented below.
    #[builder(into)]
    #[serde(rename = "gcsDestinationConfig")]
    pub r#gcs_destination_config: Option<Box<super::super::types::datastream::StreamDestinationConfigGcsDestinationConfig>>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for StreamDestinationConfig {
    fn to_pulumi_value(
        &self,
    ) -> impl std::future::Future<
        Output = pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue,
    > {
        use pulumi_gestalt_rust::__private::futures::FutureExt;
        use pulumi_gestalt_rust::__private::pulumi_gestalt_model::__private::to_pulumi_object_concurrent;
        async move {
            use pulumi_gestalt_rust::__private::pulumi_gestalt_model::__private::{
                to_pulumi_object_field, ToPulumiObjectFieldFuture,
            };
            let field_futures: Vec<ToPulumiObjectFieldFuture<'_>> = vec![
                to_pulumi_object_field(
                    "bigquery_destination_config",
                    &self.r#bigquery_destination_config,
                ),
                to_pulumi_object_field(
                    "destination_connection_profile",
                    &self.r#destination_connection_profile,
                ),
                to_pulumi_object_field(
                    "gcs_destination_config",
                    &self.r#gcs_destination_config,
                ),
            ];
            to_pulumi_object_concurrent(field_futures).await
        }
        .boxed_local()
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for StreamDestinationConfig {
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
                    r#bigquery_destination_config: {
                        let field_value = match fields_map.get("bigquery_destination_config") {
                            Some(value) => value,
                            None => bail!("Missing field 'bigquery_destination_config' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#destination_connection_profile: {
                        let field_value = match fields_map.get("destination_connection_profile") {
                            Some(value) => value,
                            None => bail!("Missing field 'destination_connection_profile' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#gcs_destination_config: {
                        let field_value = match fields_map.get("gcs_destination_config") {
                            Some(value) => value,
                            None => bail!("Missing field 'gcs_destination_config' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
