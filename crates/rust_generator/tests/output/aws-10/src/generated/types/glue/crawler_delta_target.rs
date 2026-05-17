#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct CrawlerDeltaTarget {
    /// The name of the connection to use to connect to the Delta table target.
    #[builder(into)]
    #[serde(rename = "connectionName")]
    pub r#connection_name: Option<String>,
    /// Specifies whether the crawler will create native tables, to allow integration with query engines that support querying of the Delta transaction log directly.
    #[builder(into)]
    #[serde(rename = "createNativeDeltaTable")]
    pub r#create_native_delta_table: Option<bool>,
    /// A list of the Amazon S3 paths to the Delta tables.
    #[builder(into)]
    #[serde(rename = "deltaTables")]
    pub r#delta_tables: Vec<String>,
    /// Specifies whether to write the manifest files to the Delta table path.
    #[builder(into)]
    #[serde(rename = "writeManifest")]
    pub r#write_manifest: bool,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for CrawlerDeltaTarget {
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
                "connection_name".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#connection_name,
                )
                .await,
            );
            map.insert(
                "create_native_delta_table".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#create_native_delta_table,
                )
                .await,
            );
            map.insert(
                "delta_tables".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#delta_tables,
                )
                .await,
            );
            map.insert(
                "write_manifest".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#write_manifest,
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

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for CrawlerDeltaTarget {
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
                    r#connection_name: {
                        let field_value = match fields_map.get("connection_name") {
                            Some(value) => value,
                            None => bail!("Missing field 'connection_name' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#create_native_delta_table: {
                        let field_value = match fields_map.get("create_native_delta_table") {
                            Some(value) => value,
                            None => bail!("Missing field 'create_native_delta_table' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#delta_tables: {
                        let field_value = match fields_map.get("delta_tables") {
                            Some(value) => value,
                            None => bail!("Missing field 'delta_tables' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#write_manifest: {
                        let field_value = match fields_map.get("write_manifest") {
                            Some(value) => value,
                            None => bail!("Missing field 'write_manifest' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
