#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct EventSubscriptionDeliveryProperty {
    /// The name of the header to send on to the destination
    #[builder(into)]
    #[serde(rename = "headerName")]
    pub r#header_name: String,
    /// True if the `value` is a secret and should be protected, otherwise false. If True, then this value won't be returned from Azure API calls
    #[builder(into)]
    #[serde(rename = "secret")]
    pub r#secret: Option<bool>,
    /// If the `type` is `Dynamic`, then provide the payload field to be used as the value. Valid source fields differ by subscription type.
    #[builder(into)]
    #[serde(rename = "sourceField")]
    pub r#source_field: Option<String>,
    /// Either `Static` or `Dynamic`
    #[builder(into)]
    #[serde(rename = "type")]
    pub r#type_: String,
    /// If the `type` is `Static`, then provide the value to use
    #[builder(into)]
    #[serde(rename = "value")]
    pub r#value: Option<String>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for EventSubscriptionDeliveryProperty {
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
                "header_name".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#header_name,
                )
                .await,
            );
            map.insert(
                "secret".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#secret,
                )
                .await,
            );
            map.insert(
                "source_field".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#source_field,
                )
                .await,
            );
            map.insert(
                "type_".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#type_,
                )
                .await,
            );
            map.insert(
                "value".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#value,
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

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for EventSubscriptionDeliveryProperty {
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
                    r#header_name: {
                        let field_value = match fields_map.get("header_name") {
                            Some(value) => value,
                            None => bail!("Missing field 'header_name' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#secret: {
                        let field_value = match fields_map.get("secret") {
                            Some(value) => value,
                            None => bail!("Missing field 'secret' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#source_field: {
                        let field_value = match fields_map.get("source_field") {
                            Some(value) => value,
                            None => bail!("Missing field 'source_field' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#type_: {
                        let field_value = match fields_map.get("type_") {
                            Some(value) => value,
                            None => bail!("Missing field 'type_' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#value: {
                        let field_value = match fields_map.get("value") {
                            Some(value) => value,
                            None => bail!("Missing field 'value' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
