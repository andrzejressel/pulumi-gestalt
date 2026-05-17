#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct InstanceVerifiedAccessTrustProvider {
    /// A description for the AWS Verified Access Instance.
    #[builder(into)]
    #[serde(rename = "description")]
    pub r#description: Option<String>,
    /// The type of device-based trust provider.
    #[builder(into)]
    #[serde(rename = "deviceTrustProviderType")]
    pub r#device_trust_provider_type: Option<String>,
    /// The type of trust provider (user- or device-based).
    #[builder(into)]
    #[serde(rename = "trustProviderType")]
    pub r#trust_provider_type: Option<String>,
    /// The type of user-based trust provider.
    #[builder(into)]
    #[serde(rename = "userTrustProviderType")]
    pub r#user_trust_provider_type: Option<String>,
    /// The ID of the trust provider.
    #[builder(into)]
    #[serde(rename = "verifiedAccessTrustProviderId")]
    pub r#verified_access_trust_provider_id: Option<String>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for InstanceVerifiedAccessTrustProvider {
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
                "description".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#description,
                )
                .await,
            );
            map.insert(
                "device_trust_provider_type".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#device_trust_provider_type,
                )
                .await,
            );
            map.insert(
                "trust_provider_type".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#trust_provider_type,
                )
                .await,
            );
            map.insert(
                "user_trust_provider_type".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#user_trust_provider_type,
                )
                .await,
            );
            map.insert(
                "verified_access_trust_provider_id".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#verified_access_trust_provider_id,
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

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for InstanceVerifiedAccessTrustProvider {
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
                    r#description: {
                        let field_value = match fields_map.get("description") {
                            Some(value) => value,
                            None => bail!("Missing field 'description' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#device_trust_provider_type: {
                        let field_value = match fields_map.get("device_trust_provider_type") {
                            Some(value) => value,
                            None => bail!("Missing field 'device_trust_provider_type' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#trust_provider_type: {
                        let field_value = match fields_map.get("trust_provider_type") {
                            Some(value) => value,
                            None => bail!("Missing field 'trust_provider_type' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#user_trust_provider_type: {
                        let field_value = match fields_map.get("user_trust_provider_type") {
                            Some(value) => value,
                            None => bail!("Missing field 'user_trust_provider_type' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#verified_access_trust_provider_id: {
                        let field_value = match fields_map.get("verified_access_trust_provider_id") {
                            Some(value) => value,
                            None => bail!("Missing field 'verified_access_trust_provider_id' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
