#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct ZoneSoaRecord {
    /// The email contact for the SOA record.
    #[builder(into)]
    #[serde(rename = "email")]
    pub r#email: String,
    /// The expire time for the SOA record. Defaults to `2419200`.
    #[builder(into)]
    #[serde(rename = "expireTime")]
    pub r#expire_time: Option<i32>,
    /// The fully qualified domain name of the Record Set.
    #[builder(into)]
    #[serde(rename = "fqdn")]
    pub r#fqdn: Option<String>,
    /// The domain name of the authoritative name server for the SOA record.
    #[builder(into)]
    #[serde(rename = "hostName")]
    pub r#host_name: Option<String>,
    /// The minimum Time To Live for the SOA record. By convention, it is used to determine the negative caching duration. Defaults to `10`.
    #[builder(into)]
    #[serde(rename = "minimumTtl")]
    pub r#minimum_ttl: Option<i32>,
    /// The refresh time for the SOA record. Defaults to `3600`.
    #[builder(into)]
    #[serde(rename = "refreshTime")]
    pub r#refresh_time: Option<i32>,
    /// The retry time for the SOA record. Defaults to `300`.
    #[builder(into)]
    #[serde(rename = "retryTime")]
    pub r#retry_time: Option<i32>,
    /// The serial number for the SOA record.
    #[builder(into)]
    #[serde(rename = "serialNumber")]
    pub r#serial_number: Option<i32>,
    /// A mapping of tags to assign to the Record Set.
    #[builder(into)]
    #[serde(rename = "tags")]
    pub r#tags: Option<std::collections::HashMap<String, String>>,
    /// The Time To Live of the SOA Record in seconds. Defaults to `3600`.
    #[builder(into)]
    #[serde(rename = "ttl")]
    pub r#ttl: Option<i32>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for ZoneSoaRecord {
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
                "email".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#email,
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
                "fqdn".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#fqdn,
                )
                .await,
            );
            map.insert(
                "host_name".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#host_name,
                )
                .await,
            );
            map.insert(
                "minimum_ttl".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#minimum_ttl,
                )
                .await,
            );
            map.insert(
                "refresh_time".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#refresh_time,
                )
                .await,
            );
            map.insert(
                "retry_time".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#retry_time,
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
                "tags".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#tags,
                )
                .await,
            );
            map.insert(
                "ttl".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#ttl,
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

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for ZoneSoaRecord {
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
                    r#email: {
                        let field_value = match fields_map.get("email") {
                            Some(value) => value,
                            None => bail!("Missing field 'email' while converting PulumiValue to {}", std::any::type_name::<Self>()),
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
                    r#fqdn: {
                        let field_value = match fields_map.get("fqdn") {
                            Some(value) => value,
                            None => bail!("Missing field 'fqdn' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#host_name: {
                        let field_value = match fields_map.get("host_name") {
                            Some(value) => value,
                            None => bail!("Missing field 'host_name' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#minimum_ttl: {
                        let field_value = match fields_map.get("minimum_ttl") {
                            Some(value) => value,
                            None => bail!("Missing field 'minimum_ttl' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#refresh_time: {
                        let field_value = match fields_map.get("refresh_time") {
                            Some(value) => value,
                            None => bail!("Missing field 'refresh_time' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#retry_time: {
                        let field_value = match fields_map.get("retry_time") {
                            Some(value) => value,
                            None => bail!("Missing field 'retry_time' while converting PulumiValue to {}", std::any::type_name::<Self>()),
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
                    r#tags: {
                        let field_value = match fields_map.get("tags") {
                            Some(value) => value,
                            None => bail!("Missing field 'tags' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#ttl: {
                        let field_value = match fields_map.get("ttl") {
                            Some(value) => value,
                            None => bail!("Missing field 'ttl' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
