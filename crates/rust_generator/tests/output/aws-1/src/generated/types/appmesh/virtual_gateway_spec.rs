#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct VirtualGatewaySpec {
    /// Defaults for backends.
    #[builder(into)]
    #[serde(rename = "backendDefaults")]
    pub r#backend_defaults: Option<Box<super::super::types::appmesh::VirtualGatewaySpecBackendDefaults>>,
    /// Listeners that the mesh endpoint is expected to receive inbound traffic from. You can specify one listener.
    #[builder(into)]
    #[serde(rename = "listeners")]
    pub r#listeners: Vec<super::super::types::appmesh::VirtualGatewaySpecListener>,
    /// Inbound and outbound access logging information for the virtual gateway.
    #[builder(into)]
    #[serde(rename = "logging")]
    pub r#logging: Option<Box<super::super::types::appmesh::VirtualGatewaySpecLogging>>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for VirtualGatewaySpec {
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
                "backend_defaults".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#backend_defaults,
                )
                .await,
            );
            map.insert(
                "listeners".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#listeners,
                )
                .await,
            );
            map.insert(
                "logging".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#logging,
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

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for VirtualGatewaySpec {
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
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
