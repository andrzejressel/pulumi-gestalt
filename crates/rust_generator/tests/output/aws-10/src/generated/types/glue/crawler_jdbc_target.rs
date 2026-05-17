#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct CrawlerJdbcTarget {
    /// The name of the connection to use to connect to the JDBC target.
    #[builder(into)]
    #[serde(rename = "connectionName")]
    pub r#connection_name: String,
    /// Specify a value of `RAWTYPES` or `COMMENTS` to enable additional metadata intable responses. `RAWTYPES` provides the native-level datatype. `COMMENTS` provides comments associated with a column or table in the database.
    #[builder(into)]
    #[serde(rename = "enableAdditionalMetadatas")]
    pub r#enable_additional_metadatas: Option<Vec<String>>,
    /// A list of glob patterns used to exclude from the crawl.
    #[builder(into)]
    #[serde(rename = "exclusions")]
    pub r#exclusions: Option<Vec<String>>,
    /// The path of the JDBC target.
    #[builder(into)]
    #[serde(rename = "path")]
    pub r#path: String,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for CrawlerJdbcTarget {
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
                "enable_additional_metadatas".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#enable_additional_metadatas,
                )
                .await,
            );
            map.insert(
                "exclusions".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#exclusions,
                )
                .await,
            );
            map.insert(
                "path".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#path,
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

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for CrawlerJdbcTarget {
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
                    r#enable_additional_metadatas: {
                        let field_value = match fields_map.get("enable_additional_metadatas") {
                            Some(value) => value,
                            None => bail!("Missing field 'enable_additional_metadatas' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#exclusions: {
                        let field_value = match fields_map.get("exclusions") {
                            Some(value) => value,
                            None => bail!("Missing field 'exclusions' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#path: {
                        let field_value = match fields_map.get("path") {
                            Some(value) => value,
                            None => bail!("Missing field 'path' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
