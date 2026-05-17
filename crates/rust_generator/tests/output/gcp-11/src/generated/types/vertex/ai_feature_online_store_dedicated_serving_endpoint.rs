#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct AiFeatureOnlineStoreDedicatedServingEndpoint {
    /// Private service connect config.
    /// Structure is documented below.
    #[builder(into)]
    #[serde(rename = "privateServiceConnectConfig")]
    pub r#private_service_connect_config: Option<Box<super::super::types::vertex::AiFeatureOnlineStoreDedicatedServingEndpointPrivateServiceConnectConfig>>,
    /// (Output)
    /// Domain name to use for this FeatureOnlineStore
    #[builder(into)]
    #[serde(rename = "publicEndpointDomainName")]
    pub r#public_endpoint_domain_name: Option<String>,
    /// (Output)
    /// Name of the service attachment resource. Applicable only if private service connect is enabled and after FeatureViewSync is created.
    #[builder(into)]
    #[serde(rename = "serviceAttachment")]
    pub r#service_attachment: Option<String>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for AiFeatureOnlineStoreDedicatedServingEndpoint {
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
                "private_service_connect_config".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#private_service_connect_config,
                )
                .await,
            );
            map.insert(
                "public_endpoint_domain_name".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#public_endpoint_domain_name,
                )
                .await,
            );
            map.insert(
                "service_attachment".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#service_attachment,
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

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for AiFeatureOnlineStoreDedicatedServingEndpoint {
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
                    r#private_service_connect_config: {
                        let field_value = match fields_map.get("private_service_connect_config") {
                            Some(value) => value,
                            None => bail!("Missing field 'private_service_connect_config' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#public_endpoint_domain_name: {
                        let field_value = match fields_map.get("public_endpoint_domain_name") {
                            Some(value) => value,
                            None => bail!("Missing field 'public_endpoint_domain_name' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#service_attachment: {
                        let field_value = match fields_map.get("service_attachment") {
                            Some(value) => value,
                            None => bail!("Missing field 'service_attachment' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
