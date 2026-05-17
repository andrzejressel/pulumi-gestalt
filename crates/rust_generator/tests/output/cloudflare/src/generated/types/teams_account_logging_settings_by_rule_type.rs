#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct TeamsAccountLoggingSettingsByRuleType {
    /// Logging configuration for DNS requests.
    #[builder(into)]
    #[serde(rename = "dns")]
    pub r#dns: Box<super::types::TeamsAccountLoggingSettingsByRuleTypeDns>,
    /// Logging configuration for HTTP requests.
    #[builder(into)]
    #[serde(rename = "http")]
    pub r#http: Box<super::types::TeamsAccountLoggingSettingsByRuleTypeHttp>,
    /// Logging configuration for layer 4 requests.
    #[builder(into)]
    #[serde(rename = "l4")]
    pub r#l_4: Box<super::types::TeamsAccountLoggingSettingsByRuleTypeL4>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for TeamsAccountLoggingSettingsByRuleType {
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
                "dns".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#dns,
                )
                .await,
            );
            map.insert(
                "http".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#http,
                )
                .await,
            );
            map.insert(
                "l_4".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#l_4,
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

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for TeamsAccountLoggingSettingsByRuleType {
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
                    r#dns: {
                        let field_value = match fields_map.get("dns") {
                            Some(value) => value,
                            None => bail!("Missing field 'dns' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#http: {
                        let field_value = match fields_map.get("http") {
                            Some(value) => value,
                            None => bail!("Missing field 'http' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#l_4: {
                        let field_value = match fields_map.get("l_4") {
                            Some(value) => value,
                            None => bail!("Missing field 'l_4' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
