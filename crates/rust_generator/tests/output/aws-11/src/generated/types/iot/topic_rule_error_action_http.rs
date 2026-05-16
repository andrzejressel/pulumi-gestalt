#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct TopicRuleErrorActionHttp {
    /// The HTTPS URL used to verify ownership of `url`.
    #[builder(into)]
    #[serde(rename = "confirmationUrl")]
    pub r#confirmation_url: Option<String>,
    /// Custom HTTP header IoT Core should send. It is possible to define more than one custom header.
    #[builder(into)]
    #[serde(rename = "httpHeaders")]
    pub r#http_headers: Option<Vec<super::super::types::iot::TopicRuleErrorActionHttpHttpHeader>>,
    /// The HTTPS URL.
    #[builder(into)]
    #[serde(rename = "url")]
    pub r#url: String,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for TopicRuleErrorActionHttp {
    fn to_pulumi_value(
        &self,
    ) -> impl std::future::Future<
        Output = pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue,
    > {
        async move {
            use std::collections::BTreeMap;
            use pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue;

            let mut map: BTreeMap<String, pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue> = BTreeMap::new();
            map.insert("confirmation_url".to_string(), self.r#confirmation_url.to_pulumi_value().await);
            map.insert("http_headers".to_string(), self.r#http_headers.to_pulumi_value().await);
            map.insert("url".to_string(), self.r#url.to_pulumi_value().await);

            map.to_pulumi_value().await
        }
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for TopicRuleErrorActionHttp {
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
                    r#confirmation_url: {
                        let field_value = match fields_map.get("confirmation_url") {
                            Some(value) => value,
                            None => bail!("Missing field 'confirmation_url' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Option<String> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#http_headers: {
                        let field_value = match fields_map.get("http_headers") {
                            Some(value) => value,
                            None => bail!("Missing field 'http_headers' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Option<Vec<super::super::types::iot::TopicRuleErrorActionHttpHttpHeader>> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#url: {
                        let field_value = match fields_map.get("url") {
                            Some(value) => value,
                            None => bail!("Missing field 'url' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <String as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
