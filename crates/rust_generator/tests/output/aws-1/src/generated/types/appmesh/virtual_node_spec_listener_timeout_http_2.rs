#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct VirtualNodeSpecListenerTimeoutHttp2 {
    /// Idle timeout. An idle timeout bounds the amount of time that a connection may be idle.
    #[builder(into)]
    #[serde(rename = "idle")]
    pub r#idle: Option<Box<super::super::types::appmesh::VirtualNodeSpecListenerTimeoutHttp2Idle>>,
    /// Per request timeout.
    #[builder(into)]
    #[serde(rename = "perRequest")]
    pub r#per_request: Option<Box<super::super::types::appmesh::VirtualNodeSpecListenerTimeoutHttp2PerRequest>>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for VirtualNodeSpecListenerTimeoutHttp2 {
    fn to_pulumi_value(
        &self,
    ) -> impl std::future::Future<
        Output = pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue,
    > {
        async move {
            use std::collections::BTreeMap;
            use pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue;

            let mut map: BTreeMap<String, pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue> = BTreeMap::new();
            map.insert("idle".to_string(), self.r#idle.to_pulumi_value().await);
            map.insert("per_request".to_string(), self.r#per_request.to_pulumi_value().await);

            map.to_pulumi_value().await
        }
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for VirtualNodeSpecListenerTimeoutHttp2 {
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
                    r#idle: {
                        let field_value = match fields_map.get("idle") {
                            Some(value) => value,
                            None => bail!("Missing field 'idle' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Option<Box<super::super::types::appmesh::VirtualNodeSpecListenerTimeoutHttp2Idle>> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#per_request: {
                        let field_value = match fields_map.get("per_request") {
                            Some(value) => value,
                            None => bail!("Missing field 'per_request' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Option<Box<super::super::types::appmesh::VirtualNodeSpecListenerTimeoutHttp2PerRequest>> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
