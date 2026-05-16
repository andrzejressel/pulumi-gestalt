#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct RiskConfigurationAccountTakeoverRiskConfigurationNotifyConfigurationMfaEmail {
    /// The email HTML body.
    #[builder(into)]
    #[serde(rename = "htmlBody")]
    pub r#html_body: String,
    /// The email subject.
    #[builder(into)]
    #[serde(rename = "subject")]
    pub r#subject: String,
    /// The email text body.
    #[builder(into)]
    #[serde(rename = "textBody")]
    pub r#text_body: String,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for RiskConfigurationAccountTakeoverRiskConfigurationNotifyConfigurationMfaEmail {
    fn to_pulumi_value(
        &self,
    ) -> impl std::future::Future<
        Output = pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue,
    > {
        async move {
            use std::collections::BTreeMap;
            use pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue;

            let mut map: BTreeMap<String, pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue> = BTreeMap::new();
            map.insert("html_body".to_string(), self.r#html_body.to_pulumi_value().await);
            map.insert("subject".to_string(), self.r#subject.to_pulumi_value().await);
            map.insert("text_body".to_string(), self.r#text_body.to_pulumi_value().await);

            map.to_pulumi_value().await
        }
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for RiskConfigurationAccountTakeoverRiskConfigurationNotifyConfigurationMfaEmail {
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
                    r#html_body: {
                        let field_value = match fields_map.get("html_body") {
                            Some(value) => value,
                            None => bail!("Missing field 'html_body' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <String as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#subject: {
                        let field_value = match fields_map.get("subject") {
                            Some(value) => value,
                            None => bail!("Missing field 'subject' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <String as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#text_body: {
                        let field_value = match fields_map.get("text_body") {
                            Some(value) => value,
                            None => bail!("Missing field 'text_body' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <String as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
