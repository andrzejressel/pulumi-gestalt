#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct GetVirtualNodeSpecListenerTimeout {
    #[builder(into)]
    #[serde(rename = "grpcs")]
    pub r#grpcs: Vec<super::super::types::appmesh::GetVirtualNodeSpecListenerTimeoutGrpc>,
    #[builder(into)]
    #[serde(rename = "http2s")]
    pub r#http_2_s: Vec<super::super::types::appmesh::GetVirtualNodeSpecListenerTimeoutHttp2>,
    #[builder(into)]
    #[serde(rename = "https")]
    pub r#https: Vec<super::super::types::appmesh::GetVirtualNodeSpecListenerTimeoutHttp>,
    #[builder(into)]
    #[serde(rename = "tcps")]
    pub r#tcps: Vec<super::super::types::appmesh::GetVirtualNodeSpecListenerTimeoutTcp>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for GetVirtualNodeSpecListenerTimeout {
    fn to_pulumi_value(
        &self,
    ) -> impl std::future::Future<
        Output = pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue,
    > {
        async move {
            use std::collections::BTreeMap;
            use pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue;

            let mut map: BTreeMap<String, pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue> = BTreeMap::new();
            map.insert("grpcs".to_string(), self.r#grpcs.to_pulumi_value().await);
            map.insert("http_2_s".to_string(), self.r#http_2_s.to_pulumi_value().await);
            map.insert("https".to_string(), self.r#https.to_pulumi_value().await);
            map.insert("tcps".to_string(), self.r#tcps.to_pulumi_value().await);

            map.to_pulumi_value().await
        }
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for GetVirtualNodeSpecListenerTimeout {
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
                    r#grpcs: {
                        let field_value = match fields_map.get("grpcs") {
                            Some(value) => value,
                            None => bail!("Missing field 'grpcs' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Vec<super::super::types::appmesh::GetVirtualNodeSpecListenerTimeoutGrpc> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#http_2_s: {
                        let field_value = match fields_map.get("http_2_s") {
                            Some(value) => value,
                            None => bail!("Missing field 'http_2_s' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Vec<super::super::types::appmesh::GetVirtualNodeSpecListenerTimeoutHttp2> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#https: {
                        let field_value = match fields_map.get("https") {
                            Some(value) => value,
                            None => bail!("Missing field 'https' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Vec<super::super::types::appmesh::GetVirtualNodeSpecListenerTimeoutHttp> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#tcps: {
                        let field_value = match fields_map.get("tcps") {
                            Some(value) => value,
                            None => bail!("Missing field 'tcps' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Vec<super::super::types::appmesh::GetVirtualNodeSpecListenerTimeoutTcp> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
