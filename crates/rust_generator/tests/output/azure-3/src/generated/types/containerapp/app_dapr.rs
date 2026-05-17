#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct AppDapr {
    /// The Dapr Application Identifier.
    #[builder(into)]
    #[serde(rename = "appId")]
    pub r#app_id: String,
    /// The port which the application is listening on. This is the same as the `ingress` port.
    #[builder(into)]
    #[serde(rename = "appPort")]
    pub r#app_port: Option<i32>,
    /// The protocol for the app. Possible values include `http` and `grpc`. Defaults to `http`.
    #[builder(into)]
    #[serde(rename = "appProtocol")]
    pub r#app_protocol: Option<String>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for AppDapr {
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
                "app_id".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#app_id,
                )
                .await,
            );
            map.insert(
                "app_port".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#app_port,
                )
                .await,
            );
            map.insert(
                "app_protocol".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#app_protocol,
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

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for AppDapr {
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
                    r#app_id: {
                        let field_value = match fields_map.get("app_id") {
                            Some(value) => value,
                            None => bail!("Missing field 'app_id' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#app_port: {
                        let field_value = match fields_map.get("app_port") {
                            Some(value) => value,
                            None => bail!("Missing field 'app_port' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#app_protocol: {
                        let field_value = match fields_map.get("app_protocol") {
                            Some(value) => value,
                            None => bail!("Missing field 'app_protocol' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
