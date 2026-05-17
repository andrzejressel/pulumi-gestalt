#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct GetLoadBalancerListener {
    #[builder(into)]
    #[serde(rename = "instancePort")]
    pub r#instance_port: i32,
    #[builder(into)]
    #[serde(rename = "instanceProtocol")]
    pub r#instance_protocol: String,
    #[builder(into)]
    #[serde(rename = "lbPort")]
    pub r#lb_port: i32,
    #[builder(into)]
    #[serde(rename = "lbProtocol")]
    pub r#lb_protocol: String,
    #[builder(into)]
    #[serde(rename = "sslCertificateId")]
    pub r#ssl_certificate_id: String,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for GetLoadBalancerListener {
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
                "instance_port".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#instance_port,
                )
                .await,
            );
            map.insert(
                "instance_protocol".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#instance_protocol,
                )
                .await,
            );
            map.insert(
                "lb_port".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#lb_port,
                )
                .await,
            );
            map.insert(
                "lb_protocol".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#lb_protocol,
                )
                .await,
            );
            map.insert(
                "ssl_certificate_id".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#ssl_certificate_id,
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

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for GetLoadBalancerListener {
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
                    r#instance_port: {
                        let field_value = match fields_map.get("instance_port") {
                            Some(value) => value,
                            None => bail!("Missing field 'instance_port' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#instance_protocol: {
                        let field_value = match fields_map.get("instance_protocol") {
                            Some(value) => value,
                            None => bail!("Missing field 'instance_protocol' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#lb_port: {
                        let field_value = match fields_map.get("lb_port") {
                            Some(value) => value,
                            None => bail!("Missing field 'lb_port' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#lb_protocol: {
                        let field_value = match fields_map.get("lb_protocol") {
                            Some(value) => value,
                            None => bail!("Missing field 'lb_protocol' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#ssl_certificate_id: {
                        let field_value = match fields_map.get("ssl_certificate_id") {
                            Some(value) => value,
                            None => bail!("Missing field 'ssl_certificate_id' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
