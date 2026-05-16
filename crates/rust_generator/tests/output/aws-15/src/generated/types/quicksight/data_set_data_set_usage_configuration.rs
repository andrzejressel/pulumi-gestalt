#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct DataSetDataSetUsageConfiguration {
    /// Controls whether a child dataset of a direct query can use this dataset as a source.
    #[builder(into)]
    #[serde(rename = "disableUseAsDirectQuerySource")]
    pub r#disable_use_as_direct_query_source: Option<bool>,
    /// Controls whether a child dataset that's stored in QuickSight can use this dataset as a source.
    #[builder(into)]
    #[serde(rename = "disableUseAsImportedSource")]
    pub r#disable_use_as_imported_source: Option<bool>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for DataSetDataSetUsageConfiguration {
    fn to_pulumi_value(
        &self,
    ) -> impl std::future::Future<
        Output = pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue,
    > {
        async move {
            use std::collections::BTreeMap;
            use pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue;

            let mut map: BTreeMap<String, pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue> = BTreeMap::new();
            map.insert("disable_use_as_direct_query_source".to_string(), self.r#disable_use_as_direct_query_source.to_pulumi_value().await);
            map.insert("disable_use_as_imported_source".to_string(), self.r#disable_use_as_imported_source.to_pulumi_value().await);

            map.to_pulumi_value().await
        }
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for DataSetDataSetUsageConfiguration {
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
                    r#disable_use_as_direct_query_source: {
                        let field_value = match fields_map.get("disable_use_as_direct_query_source") {
                            Some(value) => value,
                            None => bail!("Missing field 'disable_use_as_direct_query_source' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Option<bool> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#disable_use_as_imported_source: {
                        let field_value = match fields_map.get("disable_use_as_imported_source") {
                            Some(value) => value,
                            None => bail!("Missing field 'disable_use_as_imported_source' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Option<bool> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
