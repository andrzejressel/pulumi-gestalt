#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct GetVirtualNodeSpec {
    #[builder(into)]
    #[serde(rename = "backendDefaults")]
    pub r#backend_defaults: Vec<super::super::types::appmesh::GetVirtualNodeSpecBackendDefault>,
    #[builder(into)]
    #[serde(rename = "backends")]
    pub r#backends: Vec<super::super::types::appmesh::GetVirtualNodeSpecBackend>,
    #[builder(into)]
    #[serde(rename = "listeners")]
    pub r#listeners: Vec<super::super::types::appmesh::GetVirtualNodeSpecListener>,
    #[builder(into)]
    #[serde(rename = "loggings")]
    pub r#loggings: Vec<super::super::types::appmesh::GetVirtualNodeSpecLogging>,
    #[builder(into)]
    #[serde(rename = "serviceDiscoveries")]
    pub r#service_discoveries: Vec<super::super::types::appmesh::GetVirtualNodeSpecServiceDiscovery>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for GetVirtualNodeSpec {
    fn to_pulumi_value(
        &self,
    ) -> impl std::future::Future<
        Output = pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue,
    > {
        use pulumi_gestalt_rust::__private::futures::FutureExt;
        use pulumi_gestalt_rust::__private::pulumi_gestalt_model::__private::to_pulumi_object_concurrent;
        async move {
            use pulumi_gestalt_rust::__private::pulumi_gestalt_model::__private::{
                to_pulumi_object_field, ToPulumiObjectFieldFuture,
            };
            let field_futures: Vec<ToPulumiObjectFieldFuture<'_>> = vec![
                to_pulumi_object_field(
                    "backend_defaults",
                    &self.r#backend_defaults,
                ),
                to_pulumi_object_field(
                    "backends",
                    &self.r#backends,
                ),
                to_pulumi_object_field(
                    "listeners",
                    &self.r#listeners,
                ),
                to_pulumi_object_field(
                    "loggings",
                    &self.r#loggings,
                ),
                to_pulumi_object_field(
                    "service_discoveries",
                    &self.r#service_discoveries,
                ),
            ];
            to_pulumi_object_concurrent(field_futures).await
        }
        .boxed_local()
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for GetVirtualNodeSpec {
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
                    r#backend_defaults: {
                        let field_value = match fields_map.get("backend_defaults") {
                            Some(value) => value,
                            None => bail!("Missing field 'backend_defaults' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#backends: {
                        let field_value = match fields_map.get("backends") {
                            Some(value) => value,
                            None => bail!("Missing field 'backends' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#listeners: {
                        let field_value = match fields_map.get("listeners") {
                            Some(value) => value,
                            None => bail!("Missing field 'listeners' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#loggings: {
                        let field_value = match fields_map.get("loggings") {
                            Some(value) => value,
                            None => bail!("Missing field 'loggings' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#service_discoveries: {
                        let field_value = match fields_map.get("service_discoveries") {
                            Some(value) => value,
                            None => bail!("Missing field 'service_discoveries' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
