#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct VirtualNodeSpec {
    /// Defaults for backends.
    #[builder(into)]
    #[serde(rename = "backendDefaults")]
    pub r#backend_defaults: Option<Box<super::super::types::appmesh::VirtualNodeSpecBackendDefaults>>,
    /// Backends to which the virtual node is expected to send outbound traffic.
    #[builder(into)]
    #[serde(rename = "backends")]
    pub r#backends: Option<Vec<super::super::types::appmesh::VirtualNodeSpecBackend>>,
    /// Listeners from which the virtual node is expected to receive inbound traffic.
    #[builder(into)]
    #[serde(rename = "listeners")]
    pub r#listeners: Option<Vec<super::super::types::appmesh::VirtualNodeSpecListener>>,
    /// Inbound and outbound access logging information for the virtual node.
    #[builder(into)]
    #[serde(rename = "logging")]
    pub r#logging: Option<Box<super::super::types::appmesh::VirtualNodeSpecLogging>>,
    /// Service discovery information for the virtual node.
    #[builder(into)]
    #[serde(rename = "serviceDiscovery")]
    pub r#service_discovery: Option<Box<super::super::types::appmesh::VirtualNodeSpecServiceDiscovery>>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for VirtualNodeSpec {
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
                    "logging",
                    &self.r#logging,
                ),
                to_pulumi_object_field(
                    "service_discovery",
                    &self.r#service_discovery,
                ),
            ];
            to_pulumi_object_concurrent(field_futures).await
        }
        .boxed_local()
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for VirtualNodeSpec {
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
                    r#logging: {
                        let field_value = match fields_map.get("logging") {
                            Some(value) => value,
                            None => bail!("Missing field 'logging' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#service_discovery: {
                        let field_value = match fields_map.get("service_discovery") {
                            Some(value) => value,
                            None => bail!("Missing field 'service_discovery' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
