#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct StandardWebTestValidationRules {
    /// A `content` block as defined above.
    #[builder(into)]
    #[serde(rename = "content")]
    pub r#content: Option<Box<super::super::types::appinsights::StandardWebTestValidationRulesContent>>,
    /// The expected status code of the response. Default is '200', '0' means 'response code < 400'
    #[builder(into)]
    #[serde(rename = "expectedStatusCode")]
    pub r#expected_status_code: Option<i32>,
    /// The number of days of SSL certificate validity remaining for the checked endpoint. If the certificate has a shorter remaining lifetime left, the test will fail. This number should be between 1 and 365.
    #[builder(into)]
    #[serde(rename = "sslCertRemainingLifetime")]
    pub r#ssl_cert_remaining_lifetime: Option<i32>,
    /// Should the SSL check be enabled?
    #[builder(into)]
    #[serde(rename = "sslCheckEnabled")]
    pub r#ssl_check_enabled: Option<bool>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for StandardWebTestValidationRules {
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
                "content".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#content,
                )
                .await,
            );
            map.insert(
                "expected_status_code".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#expected_status_code,
                )
                .await,
            );
            map.insert(
                "ssl_cert_remaining_lifetime".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#ssl_cert_remaining_lifetime,
                )
                .await,
            );
            map.insert(
                "ssl_check_enabled".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#ssl_check_enabled,
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

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for StandardWebTestValidationRules {
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
                    r#content: {
                        let field_value = match fields_map.get("content") {
                            Some(value) => value,
                            None => bail!("Missing field 'content' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#expected_status_code: {
                        let field_value = match fields_map.get("expected_status_code") {
                            Some(value) => value,
                            None => bail!("Missing field 'expected_status_code' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#ssl_cert_remaining_lifetime: {
                        let field_value = match fields_map.get("ssl_cert_remaining_lifetime") {
                            Some(value) => value,
                            None => bail!("Missing field 'ssl_cert_remaining_lifetime' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#ssl_check_enabled: {
                        let field_value = match fields_map.get("ssl_check_enabled") {
                            Some(value) => value,
                            None => bail!("Missing field 'ssl_check_enabled' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
