#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct AssetDiscoveryStatusStat {
    /// The count of data items within the referenced resource.
    #[builder(into)]
    #[serde(rename = "dataItems")]
    pub r#data_items: Option<i32>,
    /// The number of stored data bytes within the referenced resource.
    #[builder(into)]
    #[serde(rename = "dataSize")]
    pub r#data_size: Option<i32>,
    /// The count of fileset entities within the referenced resource.
    #[builder(into)]
    #[serde(rename = "filesets")]
    pub r#filesets: Option<i32>,
    /// The count of table entities within the referenced resource.
    #[builder(into)]
    #[serde(rename = "tables")]
    pub r#tables: Option<i32>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for AssetDiscoveryStatusStat {
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
                "data_items".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#data_items,
                )
                .await,
            );
            map.insert(
                "data_size".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#data_size,
                )
                .await,
            );
            map.insert(
                "filesets".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#filesets,
                )
                .await,
            );
            map.insert(
                "tables".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#tables,
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

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for AssetDiscoveryStatusStat {
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
                    r#data_items: {
                        let field_value = match fields_map.get("data_items") {
                            Some(value) => value,
                            None => bail!("Missing field 'data_items' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#data_size: {
                        let field_value = match fields_map.get("data_size") {
                            Some(value) => value,
                            None => bail!("Missing field 'data_size' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#filesets: {
                        let field_value = match fields_map.get("filesets") {
                            Some(value) => value,
                            None => bail!("Missing field 'filesets' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#tables: {
                        let field_value = match fields_map.get("tables") {
                            Some(value) => value,
                            None => bail!("Missing field 'tables' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
