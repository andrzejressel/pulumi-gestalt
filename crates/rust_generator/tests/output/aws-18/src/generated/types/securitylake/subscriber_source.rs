#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct SubscriberSource {
    /// Amazon Security Lake supports log and event collection for natively supported AWS services. See `aws_log_source_resource` Block below.
    #[builder(into)]
    #[serde(rename = "awsLogSourceResource")]
    pub r#aws_log_source_resource: Option<Box<super::super::types::securitylake::SubscriberSourceAwsLogSourceResource>>,
    /// Amazon Security Lake supports custom source types. See `custom_log_source_resource` Block below.
    #[builder(into)]
    #[serde(rename = "customLogSourceResource")]
    pub r#custom_log_source_resource: Option<Box<super::super::types::securitylake::SubscriberSourceCustomLogSourceResource>>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for SubscriberSource {
    fn to_pulumi_value(
        &self,
    ) -> impl std::future::Future<
        Output = pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue,
    > {
        async move {
            use std::collections::BTreeMap;
            use pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue;

            let mut map: BTreeMap<String, pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue> = BTreeMap::new();
            map.insert("aws_log_source_resource".to_string(), self.r#aws_log_source_resource.to_pulumi_value().await);
            map.insert("custom_log_source_resource".to_string(), self.r#custom_log_source_resource.to_pulumi_value().await);

            map.to_pulumi_value().await
        }
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for SubscriberSource {
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
                    r#aws_log_source_resource: {
                        let field_value = match fields_map.get("aws_log_source_resource") {
                            Some(value) => value,
                            None => bail!("Missing field 'aws_log_source_resource' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Option<Box<super::super::types::securitylake::SubscriberSourceAwsLogSourceResource>> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#custom_log_source_resource: {
                        let field_value = match fields_map.get("custom_log_source_resource") {
                            Some(value) => value,
                            None => bail!("Missing field 'custom_log_source_resource' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Option<Box<super::super::types::securitylake::SubscriberSourceCustomLogSourceResource>> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
