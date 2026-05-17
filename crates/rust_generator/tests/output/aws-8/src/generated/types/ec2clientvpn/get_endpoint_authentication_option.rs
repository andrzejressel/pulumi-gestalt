#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct GetEndpointAuthenticationOption {
    #[builder(into)]
    #[serde(rename = "activeDirectoryId")]
    pub r#active_directory_id: String,
    #[builder(into)]
    #[serde(rename = "rootCertificateChainArn")]
    pub r#root_certificate_chain_arn: String,
    #[builder(into)]
    #[serde(rename = "samlProviderArn")]
    pub r#saml_provider_arn: String,
    #[builder(into)]
    #[serde(rename = "selfServiceSamlProviderArn")]
    pub r#self_service_saml_provider_arn: String,
    #[builder(into)]
    #[serde(rename = "type")]
    pub r#type_: String,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for GetEndpointAuthenticationOption {
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
                "active_directory_id".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#active_directory_id,
                )
                .await,
            );
            map.insert(
                "root_certificate_chain_arn".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#root_certificate_chain_arn,
                )
                .await,
            );
            map.insert(
                "saml_provider_arn".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#saml_provider_arn,
                )
                .await,
            );
            map.insert(
                "self_service_saml_provider_arn".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#self_service_saml_provider_arn,
                )
                .await,
            );
            map.insert(
                "type_".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#type_,
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

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for GetEndpointAuthenticationOption {
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
                    r#active_directory_id: {
                        let field_value = match fields_map.get("active_directory_id") {
                            Some(value) => value,
                            None => bail!("Missing field 'active_directory_id' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#root_certificate_chain_arn: {
                        let field_value = match fields_map.get("root_certificate_chain_arn") {
                            Some(value) => value,
                            None => bail!("Missing field 'root_certificate_chain_arn' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#saml_provider_arn: {
                        let field_value = match fields_map.get("saml_provider_arn") {
                            Some(value) => value,
                            None => bail!("Missing field 'saml_provider_arn' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#self_service_saml_provider_arn: {
                        let field_value = match fields_map.get("self_service_saml_provider_arn") {
                            Some(value) => value,
                            None => bail!("Missing field 'self_service_saml_provider_arn' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#type_: {
                        let field_value = match fields_map.get("type_") {
                            Some(value) => value,
                            None => bail!("Missing field 'type_' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
