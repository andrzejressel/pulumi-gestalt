#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct GetLogDataProtectionPolicyDocumentStatement {
    /// Set of at least 1 sensitive data identifiers that you want to mask. Read more in [Types of data that you can protect](https://docs.aws.amazon.com/AmazonCloudWatch/latest/logs/protect-sensitive-log-data-types.html).
    #[builder(into)]
    #[serde(rename = "dataIdentifiers")]
    pub r#data_identifiers: Vec<String>,
    /// Configures the data protection operation applied by this statement.
    #[builder(into)]
    #[serde(rename = "operation")]
    pub r#operation: Box<super::super::types::cloudwatch::GetLogDataProtectionPolicyDocumentStatementOperation>,
    /// Name of this statement.
    #[builder(into)]
    #[serde(rename = "sid")]
    pub r#sid: Option<String>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for GetLogDataProtectionPolicyDocumentStatement {
    fn to_pulumi_value(
        &self,
    ) -> impl std::future::Future<
        Output = pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue,
    > {
        async move {
            use std::collections::BTreeMap;
            use pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue;

            let mut map: BTreeMap<String, pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue> = BTreeMap::new();
            map.insert("data_identifiers".to_string(), self.r#data_identifiers.to_pulumi_value().await);
            map.insert("operation".to_string(), self.r#operation.to_pulumi_value().await);
            map.insert("sid".to_string(), self.r#sid.to_pulumi_value().await);

            map.to_pulumi_value().await
        }
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for GetLogDataProtectionPolicyDocumentStatement {
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
                    r#data_identifiers: {
                        let field_value = match fields_map.get("data_identifiers") {
                            Some(value) => value,
                            None => bail!("Missing field 'data_identifiers' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Vec<String> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#operation: {
                        let field_value = match fields_map.get("operation") {
                            Some(value) => value,
                            None => bail!("Missing field 'operation' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Box<super::super::types::cloudwatch::GetLogDataProtectionPolicyDocumentStatementOperation> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#sid: {
                        let field_value = match fields_map.get("sid") {
                            Some(value) => value,
                            None => bail!("Missing field 'sid' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Option<String> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
