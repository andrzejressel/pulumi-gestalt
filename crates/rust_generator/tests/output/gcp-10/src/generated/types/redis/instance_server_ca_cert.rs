#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct InstanceServerCaCert {
    /// (Output)
    /// The certificate data in PEM format.
    #[builder(into)]
    #[serde(rename = "cert")]
    pub r#cert: Option<String>,
    /// (Output)
    /// The time when the certificate was created.
    #[builder(into)]
    #[serde(rename = "createTime")]
    pub r#create_time: Option<String>,
    /// (Output)
    /// The time when the certificate expires.
    #[builder(into)]
    #[serde(rename = "expireTime")]
    pub r#expire_time: Option<String>,
    /// (Output)
    /// Serial number, as extracted from the certificate.
    #[builder(into)]
    #[serde(rename = "serialNumber")]
    pub r#serial_number: Option<String>,
    /// (Output)
    /// Sha1 Fingerprint of the certificate.
    #[builder(into)]
    #[serde(rename = "sha1Fingerprint")]
    pub r#sha_1_fingerprint: Option<String>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for InstanceServerCaCert {
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
                "cert".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#cert,
                )
                .await,
            );
            map.insert(
                "create_time".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#create_time,
                )
                .await,
            );
            map.insert(
                "expire_time".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#expire_time,
                )
                .await,
            );
            map.insert(
                "serial_number".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#serial_number,
                )
                .await,
            );
            map.insert(
                "sha_1_fingerprint".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#sha_1_fingerprint,
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

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for InstanceServerCaCert {
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
                    r#cert: {
                        let field_value = match fields_map.get("cert") {
                            Some(value) => value,
                            None => bail!("Missing field 'cert' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#create_time: {
                        let field_value = match fields_map.get("create_time") {
                            Some(value) => value,
                            None => bail!("Missing field 'create_time' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#expire_time: {
                        let field_value = match fields_map.get("expire_time") {
                            Some(value) => value,
                            None => bail!("Missing field 'expire_time' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#serial_number: {
                        let field_value = match fields_map.get("serial_number") {
                            Some(value) => value,
                            None => bail!("Missing field 'serial_number' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#sha_1_fingerprint: {
                        let field_value = match fields_map.get("sha_1_fingerprint") {
                            Some(value) => value,
                            None => bail!("Missing field 'sha_1_fingerprint' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
