#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct DataSourceRelationalDatabaseConfig {
    /// Amazon RDS HTTP endpoint configuration. See `http_endpoint_config` Block for details.
    #[builder(into)]
    #[serde(rename = "httpEndpointConfig")]
    pub r#http_endpoint_config: Option<Box<super::super::types::appsync::DataSourceRelationalDatabaseConfigHttpEndpointConfig>>,
    /// Source type for the relational database. Valid values: `RDS_HTTP_ENDPOINT`.
    #[builder(into)]
    #[serde(rename = "sourceType")]
    pub r#source_type: Option<String>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for DataSourceRelationalDatabaseConfig {
    fn to_pulumi_value(
        &self,
    ) -> impl std::future::Future<
        Output = pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue,
    > {
        async move {
            use std::collections::BTreeMap;
            use pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue;

            let mut map: BTreeMap<String, pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue> = BTreeMap::new();
            map.insert("http_endpoint_config".to_string(), self.r#http_endpoint_config.to_pulumi_value().await);
            map.insert("source_type".to_string(), self.r#source_type.to_pulumi_value().await);

            map.to_pulumi_value().await
        }
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for DataSourceRelationalDatabaseConfig {
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
                    r#http_endpoint_config: {
                        let field_value = match fields_map.get("http_endpoint_config") {
                            Some(value) => value,
                            None => bail!("Missing field 'http_endpoint_config' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Option<Box<super::super::types::appsync::DataSourceRelationalDatabaseConfigHttpEndpointConfig>> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#source_type: {
                        let field_value = match fields_map.get("source_type") {
                            Some(value) => value,
                            None => bail!("Missing field 'source_type' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Option<String> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
