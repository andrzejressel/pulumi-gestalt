#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct ManagedClusterLbRule {
    /// LB Backend port.
    #[builder(into)]
    #[serde(rename = "backendPort")]
    pub r#backend_port: i32,
    /// LB Frontend port.
    #[builder(into)]
    #[serde(rename = "frontendPort")]
    pub r#frontend_port: i32,
    /// Protocol for the probe. Can be one of `tcp`, `udp`, `http`, or `https`.
    #[builder(into)]
    #[serde(rename = "probeProtocol")]
    pub r#probe_protocol: String,
    /// Path for the probe to check, when probe protocol is set to `http`.
    #[builder(into)]
    #[serde(rename = "probeRequestPath")]
    pub r#probe_request_path: Option<String>,
    /// The transport protocol used in this rule. Can be one of `tcp` or `udp`.
    #[builder(into)]
    #[serde(rename = "protocol")]
    pub r#protocol: String,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for ManagedClusterLbRule {
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
                "backend_port".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#backend_port,
                )
                .await,
            );
            map.insert(
                "frontend_port".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#frontend_port,
                )
                .await,
            );
            map.insert(
                "probe_protocol".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#probe_protocol,
                )
                .await,
            );
            map.insert(
                "probe_request_path".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#probe_request_path,
                )
                .await,
            );
            map.insert(
                "protocol".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#protocol,
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

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for ManagedClusterLbRule {
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
                    r#backend_port: {
                        let field_value = match fields_map.get("backend_port") {
                            Some(value) => value,
                            None => bail!("Missing field 'backend_port' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#frontend_port: {
                        let field_value = match fields_map.get("frontend_port") {
                            Some(value) => value,
                            None => bail!("Missing field 'frontend_port' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#probe_protocol: {
                        let field_value = match fields_map.get("probe_protocol") {
                            Some(value) => value,
                            None => bail!("Missing field 'probe_protocol' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#probe_request_path: {
                        let field_value = match fields_map.get("probe_request_path") {
                            Some(value) => value,
                            None => bail!("Missing field 'probe_request_path' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#protocol: {
                        let field_value = match fields_map.get("protocol") {
                            Some(value) => value,
                            None => bail!("Missing field 'protocol' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
