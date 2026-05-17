#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct GetVirtualGatewaySpecListenerConnectionPool {
    #[builder(into)]
    #[serde(rename = "grpcs")]
    pub r#grpcs: Vec<super::super::types::appmesh::GetVirtualGatewaySpecListenerConnectionPoolGrpc>,
    #[builder(into)]
    #[serde(rename = "http2s")]
    pub r#http_2_s: Vec<super::super::types::appmesh::GetVirtualGatewaySpecListenerConnectionPoolHttp2>,
    #[builder(into)]
    #[serde(rename = "https")]
    pub r#https: Vec<super::super::types::appmesh::GetVirtualGatewaySpecListenerConnectionPoolHttp>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for GetVirtualGatewaySpecListenerConnectionPool {
    fn to_pulumi_value(
        &self,
    ) -> impl std::future::Future<
        Output = pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue,
    > {
        async move {
            use std::collections::BTreeMap;
            use pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue;
            use pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue;

            let mut map: BTreeMap<String, PulumiValue> = BTreeMap::new();
            map.insert(
                "grpcs".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#grpcs,
                )
                .await,
            );
            map.insert(
                "http_2_s".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#http_2_s,
                )
                .await,
            );
            map.insert(
                "https".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#https,
                )
                .await,
            );

            ToPulumiValue::to_pulumi_value(
                &map,
            )
            .await
        }
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for GetVirtualGatewaySpecListenerConnectionPool {
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
                    r#grpcs: {
                        let field_value = match fields_map.get("grpcs") {
                            Some(value) => value,
                            None => bail!("Missing field 'grpcs' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#http_2_s: {
                        let field_value = match fields_map.get("http_2_s") {
                            Some(value) => value,
                            None => bail!("Missing field 'http_2_s' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#https: {
                        let field_value = match fields_map.get("https") {
                            Some(value) => value,
                            None => bail!("Missing field 'https' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
