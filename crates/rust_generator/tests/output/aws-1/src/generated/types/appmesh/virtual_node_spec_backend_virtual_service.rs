#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct VirtualNodeSpecBackendVirtualService {
    /// Client policy for the backend.
    #[builder(into)]
    #[serde(rename = "clientPolicy")]
    pub r#client_policy: Option<Box<super::super::types::appmesh::VirtualNodeSpecBackendVirtualServiceClientPolicy>>,
    /// Name of the virtual service that is acting as a virtual node backend. Must be between 1 and 255 characters in length.
    #[builder(into)]
    #[serde(rename = "virtualServiceName")]
    pub r#virtual_service_name: String,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for VirtualNodeSpecBackendVirtualService {
    fn to_pulumi_value(
        &self,
    ) -> impl std::future::Future<
        Output = pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue,
    > {
        async move {
            use std::collections::BTreeMap;
            use pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue;

            let mut map: BTreeMap<String, pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue> = BTreeMap::new();
            map.insert("client_policy".to_string(), self.r#client_policy.to_pulumi_value().await);
            map.insert("virtual_service_name".to_string(), self.r#virtual_service_name.to_pulumi_value().await);

            map.to_pulumi_value().await
        }
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for VirtualNodeSpecBackendVirtualService {
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
                    r#client_policy: {
                        let field_value = match fields_map.get("client_policy") {
                            Some(value) => value,
                            None => bail!("Missing field 'client_policy' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Option<Box<super::super::types::appmesh::VirtualNodeSpecBackendVirtualServiceClientPolicy>> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#virtual_service_name: {
                        let field_value = match fields_map.get("virtual_service_name") {
                            Some(value) => value,
                            None => bail!("Missing field 'virtual_service_name' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <String as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
