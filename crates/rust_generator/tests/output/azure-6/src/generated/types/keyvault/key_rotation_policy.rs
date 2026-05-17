#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct KeyRotationPolicy {
    /// An `automatic` block as defined below.
    #[builder(into)]
    #[serde(rename = "automatic")]
    pub r#automatic: Option<Box<super::super::types::keyvault::KeyRotationPolicyAutomatic>>,
    /// Expire a Key Vault Key after given duration as an [ISO 8601 duration](https://en.wikipedia.org/wiki/ISO_8601#Durations).
    #[builder(into)]
    #[serde(rename = "expireAfter")]
    pub r#expire_after: Option<String>,
    /// Notify at a given duration before expiry as an [ISO 8601 duration](https://en.wikipedia.org/wiki/ISO_8601#Durations).
    #[builder(into)]
    #[serde(rename = "notifyBeforeExpiry")]
    pub r#notify_before_expiry: Option<String>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for KeyRotationPolicy {
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
                "automatic".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#automatic,
                )
                .await,
            );
            map.insert(
                "expire_after".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#expire_after,
                )
                .await,
            );
            map.insert(
                "notify_before_expiry".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#notify_before_expiry,
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

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for KeyRotationPolicy {
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
                    r#automatic: {
                        let field_value = match fields_map.get("automatic") {
                            Some(value) => value,
                            None => bail!("Missing field 'automatic' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#expire_after: {
                        let field_value = match fields_map.get("expire_after") {
                            Some(value) => value,
                            None => bail!("Missing field 'expire_after' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#notify_before_expiry: {
                        let field_value = match fields_map.get("notify_before_expiry") {
                            Some(value) => value,
                            None => bail!("Missing field 'notify_before_expiry' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
