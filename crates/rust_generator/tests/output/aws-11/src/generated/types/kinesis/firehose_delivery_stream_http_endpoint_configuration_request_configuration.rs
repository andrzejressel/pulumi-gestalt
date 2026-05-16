#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct FirehoseDeliveryStreamHttpEndpointConfigurationRequestConfiguration {
    /// Describes the metadata sent to the HTTP endpoint destination. See `common_attributes` block below for details.
    #[builder(into)]
    #[serde(rename = "commonAttributes")]
    pub r#common_attributes: Option<Vec<super::super::types::kinesis::FirehoseDeliveryStreamHttpEndpointConfigurationRequestConfigurationCommonAttribute>>,
    /// Kinesis Data Firehose uses the content encoding to compress the body of a request before sending the request to the destination. Valid values are `NONE` and `GZIP`.  Default value is `NONE`.
    #[builder(into)]
    #[serde(rename = "contentEncoding")]
    pub r#content_encoding: Option<String>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for FirehoseDeliveryStreamHttpEndpointConfigurationRequestConfiguration {
    fn to_pulumi_value(
        &self,
    ) -> impl std::future::Future<
        Output = pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue,
    > {
        async move {
            use std::collections::BTreeMap;
            use pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue;

            let mut map: BTreeMap<String, pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue> = BTreeMap::new();
            map.insert("common_attributes".to_string(), self.r#common_attributes.to_pulumi_value().await);
            map.insert("content_encoding".to_string(), self.r#content_encoding.to_pulumi_value().await);

            map.to_pulumi_value().await
        }
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for FirehoseDeliveryStreamHttpEndpointConfigurationRequestConfiguration {
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
                    r#common_attributes: {
                        let field_value = match fields_map.get("common_attributes") {
                            Some(value) => value,
                            None => bail!("Missing field 'common_attributes' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Option<Vec<super::super::types::kinesis::FirehoseDeliveryStreamHttpEndpointConfigurationRequestConfigurationCommonAttribute>> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#content_encoding: {
                        let field_value = match fields_map.get("content_encoding") {
                            Some(value) => value,
                            None => bail!("Missing field 'content_encoding' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Option<String> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
