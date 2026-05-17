#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct GetListenerMutualAuthentication {
    #[builder(into)]
    #[serde(rename = "advertiseTrustStoreCaNames")]
    pub r#advertise_trust_store_ca_names: String,
    #[builder(into)]
    #[serde(rename = "ignoreClientCertificateExpiry")]
    pub r#ignore_client_certificate_expiry: bool,
    #[builder(into)]
    #[serde(rename = "mode")]
    pub r#mode: String,
    #[builder(into)]
    #[serde(rename = "trustStoreArn")]
    pub r#trust_store_arn: String,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for GetListenerMutualAuthentication {
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
                "advertise_trust_store_ca_names".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#advertise_trust_store_ca_names,
                )
                .await,
            );
            map.insert(
                "ignore_client_certificate_expiry".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#ignore_client_certificate_expiry,
                )
                .await,
            );
            map.insert(
                "mode".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#mode,
                )
                .await,
            );
            map.insert(
                "trust_store_arn".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#trust_store_arn,
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

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for GetListenerMutualAuthentication {
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
                    r#advertise_trust_store_ca_names: {
                        let field_value = match fields_map.get("advertise_trust_store_ca_names") {
                            Some(value) => value,
                            None => bail!("Missing field 'advertise_trust_store_ca_names' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#ignore_client_certificate_expiry: {
                        let field_value = match fields_map.get("ignore_client_certificate_expiry") {
                            Some(value) => value,
                            None => bail!("Missing field 'ignore_client_certificate_expiry' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#mode: {
                        let field_value = match fields_map.get("mode") {
                            Some(value) => value,
                            None => bail!("Missing field 'mode' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#trust_store_arn: {
                        let field_value = match fields_map.get("trust_store_arn") {
                            Some(value) => value,
                            None => bail!("Missing field 'trust_store_arn' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
