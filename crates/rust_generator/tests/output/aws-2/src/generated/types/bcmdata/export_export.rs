#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct ExportExport {
    /// Data query for this specific data export. See the `data_query` argument reference below.
    #[builder(into)]
    #[serde(rename = "dataQueries")]
    pub r#data_queries: Option<Vec<super::super::types::bcmdata::ExportExportDataQuery>>,
    /// Description for this specific data export.
    #[builder(into)]
    #[serde(rename = "description")]
    pub r#description: Option<String>,
    /// Destination configuration for this specific data export. See the `destination_configurations` argument reference below.
    #[builder(into)]
    #[serde(rename = "destinationConfigurations")]
    pub r#destination_configurations: Option<Vec<super::super::types::bcmdata::ExportExportDestinationConfiguration>>,
    /// Amazon Resource Name (ARN) for this export.
    #[builder(into)]
    #[serde(rename = "exportArn")]
    pub r#export_arn: Option<String>,
    /// Name of this specific data export.
    #[builder(into)]
    #[serde(rename = "name")]
    pub r#name: String,
    /// Cadence for Amazon Web Services to update the export in your S3 bucket. See the `refresh_cadence` argument reference below.
    #[builder(into)]
    #[serde(rename = "refreshCadences")]
    pub r#refresh_cadences: Option<Vec<super::super::types::bcmdata::ExportExportRefreshCadence>>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for ExportExport {
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
                    "data_queries",
                    &self.r#data_queries,
                ),
                to_pulumi_object_field(
                    "description",
                    &self.r#description,
                ),
                to_pulumi_object_field(
                    "destination_configurations",
                    &self.r#destination_configurations,
                ),
                to_pulumi_object_field(
                    "export_arn",
                    &self.r#export_arn,
                ),
                to_pulumi_object_field(
                    "name",
                    &self.r#name,
                ),
                to_pulumi_object_field(
                    "refresh_cadences",
                    &self.r#refresh_cadences,
                ),
            ];
            to_pulumi_object_concurrent(field_futures).await
        }
        .boxed_local()
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for ExportExport {
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
                    r#data_queries: {
                        let field_value = match fields_map.get("data_queries") {
                            Some(value) => value,
                            None => bail!("Missing field 'data_queries' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#description: {
                        let field_value = match fields_map.get("description") {
                            Some(value) => value,
                            None => bail!("Missing field 'description' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#destination_configurations: {
                        let field_value = match fields_map.get("destination_configurations") {
                            Some(value) => value,
                            None => bail!("Missing field 'destination_configurations' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#export_arn: {
                        let field_value = match fields_map.get("export_arn") {
                            Some(value) => value,
                            None => bail!("Missing field 'export_arn' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#name: {
                        let field_value = match fields_map.get("name") {
                            Some(value) => value,
                            None => bail!("Missing field 'name' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#refresh_cadences: {
                        let field_value = match fields_map.get("refresh_cadences") {
                            Some(value) => value,
                            None => bail!("Missing field 'refresh_cadences' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
