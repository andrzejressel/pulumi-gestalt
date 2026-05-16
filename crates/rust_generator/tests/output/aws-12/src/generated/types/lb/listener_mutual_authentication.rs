#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct ListenerMutualAuthentication {
    /// Valid values are `off` and `on`.
    #[builder(into)]
    #[serde(rename = "advertiseTrustStoreCaNames")]
    pub r#advertise_trust_store_ca_names: Option<String>,
    /// Whether client certificate expiry is ignored. Default is `false`.
    #[builder(into)]
    #[serde(rename = "ignoreClientCertificateExpiry")]
    pub r#ignore_client_certificate_expiry: Option<bool>,
    /// Valid values are `off`, `verify` and `passthrough`.
    #[builder(into)]
    #[serde(rename = "mode")]
    pub r#mode: String,
    /// ARN of the elbv2 Trust Store.
    #[builder(into)]
    #[serde(rename = "trustStoreArn")]
    pub r#trust_store_arn: Option<String>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for ListenerMutualAuthentication {
    fn to_pulumi_value(
        &self,
    ) -> impl std::future::Future<
        Output = pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue,
    > {
        async move {
            use std::collections::BTreeMap;
            use pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue;

            let mut map: BTreeMap<String, pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue> = BTreeMap::new();
            map.insert("advertise_trust_store_ca_names".to_string(), self.r#advertise_trust_store_ca_names.to_pulumi_value().await);
            map.insert("ignore_client_certificate_expiry".to_string(), self.r#ignore_client_certificate_expiry.to_pulumi_value().await);
            map.insert("mode".to_string(), self.r#mode.to_pulumi_value().await);
            map.insert("trust_store_arn".to_string(), self.r#trust_store_arn.to_pulumi_value().await);

            map.to_pulumi_value().await
        }
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for ListenerMutualAuthentication {
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
                    r#advertise_trust_store_ca_names: {
                        let field_value = match fields_map.get("advertise_trust_store_ca_names") {
                            Some(value) => value,
                            None => bail!("Missing field 'advertise_trust_store_ca_names' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Option<String> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#ignore_client_certificate_expiry: {
                        let field_value = match fields_map.get("ignore_client_certificate_expiry") {
                            Some(value) => value,
                            None => bail!("Missing field 'ignore_client_certificate_expiry' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Option<bool> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#mode: {
                        let field_value = match fields_map.get("mode") {
                            Some(value) => value,
                            None => bail!("Missing field 'mode' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <String as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#trust_store_arn: {
                        let field_value = match fields_map.get("trust_store_arn") {
                            Some(value) => value,
                            None => bail!("Missing field 'trust_store_arn' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Option<String> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
