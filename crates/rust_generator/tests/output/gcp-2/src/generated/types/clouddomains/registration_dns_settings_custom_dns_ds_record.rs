#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct RegistrationDnsSettingsCustomDnsDsRecord {
    /// The algorithm used to generate the referenced DNSKEY.
    #[builder(into)]
    #[serde(rename = "algorithm")]
    pub r#algorithm: Option<String>,
    /// The digest generated from the referenced DNSKEY.
    #[builder(into)]
    #[serde(rename = "digest")]
    pub r#digest: Option<String>,
    /// The hash function used to generate the digest of the referenced DNSKEY.
    #[builder(into)]
    #[serde(rename = "digestType")]
    pub r#digest_type: Option<String>,
    /// The key tag of the record. Must be set in range 0 -- 65535.
    #[builder(into)]
    #[serde(rename = "keyTag")]
    pub r#key_tag: Option<i32>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for RegistrationDnsSettingsCustomDnsDsRecord {
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
                "algorithm".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#algorithm,
                )
                .await,
            );
            map.insert(
                "digest".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#digest,
                )
                .await,
            );
            map.insert(
                "digest_type".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#digest_type,
                )
                .await,
            );
            map.insert(
                "key_tag".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#key_tag,
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

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for RegistrationDnsSettingsCustomDnsDsRecord {
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
                    r#algorithm: {
                        let field_value = match fields_map.get("algorithm") {
                            Some(value) => value,
                            None => bail!("Missing field 'algorithm' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#digest: {
                        let field_value = match fields_map.get("digest") {
                            Some(value) => value,
                            None => bail!("Missing field 'digest' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#digest_type: {
                        let field_value = match fields_map.get("digest_type") {
                            Some(value) => value,
                            None => bail!("Missing field 'digest_type' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#key_tag: {
                        let field_value = match fields_map.get("key_tag") {
                            Some(value) => value,
                            None => bail!("Missing field 'key_tag' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
