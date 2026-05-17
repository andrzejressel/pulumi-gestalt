#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct InsightsReportConfigObjectMetadataReportOptions {
    /// The metadata fields included in an inventory report.
    #[builder(into)]
    #[serde(rename = "metadataFields")]
    pub r#metadata_fields: Vec<String>,
    /// Options for where the inventory reports are stored.
    /// Structure is documented below.
    #[builder(into)]
    #[serde(rename = "storageDestinationOptions")]
    pub r#storage_destination_options: Box<super::super::types::storage::InsightsReportConfigObjectMetadataReportOptionsStorageDestinationOptions>,
    /// A nested object resource.
    /// Structure is documented below.
    #[builder(into)]
    #[serde(rename = "storageFilters")]
    pub r#storage_filters: Option<Box<super::super::types::storage::InsightsReportConfigObjectMetadataReportOptionsStorageFilters>>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for InsightsReportConfigObjectMetadataReportOptions {
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
                "metadata_fields".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#metadata_fields,
                )
                .await,
            );
            map.insert(
                "storage_destination_options".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#storage_destination_options,
                )
                .await,
            );
            map.insert(
                "storage_filters".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#storage_filters,
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

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for InsightsReportConfigObjectMetadataReportOptions {
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
                    r#metadata_fields: {
                        let field_value = match fields_map.get("metadata_fields") {
                            Some(value) => value,
                            None => bail!("Missing field 'metadata_fields' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#storage_destination_options: {
                        let field_value = match fields_map.get("storage_destination_options") {
                            Some(value) => value,
                            None => bail!("Missing field 'storage_destination_options' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#storage_filters: {
                        let field_value = match fields_map.get("storage_filters") {
                            Some(value) => value,
                            None => bail!("Missing field 'storage_filters' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
