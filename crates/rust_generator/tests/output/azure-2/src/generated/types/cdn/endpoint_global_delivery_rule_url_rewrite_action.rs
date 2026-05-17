#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct EndpointGlobalDeliveryRuleUrlRewriteAction {
    /// This value must start with a `/` and can't be longer than 260 characters.
    #[builder(into)]
    #[serde(rename = "destination")]
    pub r#destination: String,
    /// Whether preserve an unmatched path. Defaults to `true`.
    #[builder(into)]
    #[serde(rename = "preserveUnmatchedPath")]
    pub r#preserve_unmatched_path: Option<bool>,
    /// This value must start with a `/` and can't be longer than 260 characters.
    #[builder(into)]
    #[serde(rename = "sourcePattern")]
    pub r#source_pattern: String,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for EndpointGlobalDeliveryRuleUrlRewriteAction {
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
                "destination".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#destination,
                )
                .await,
            );
            map.insert(
                "preserve_unmatched_path".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#preserve_unmatched_path,
                )
                .await,
            );
            map.insert(
                "source_pattern".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#source_pattern,
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

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for EndpointGlobalDeliveryRuleUrlRewriteAction {
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
                    r#destination: {
                        let field_value = match fields_map.get("destination") {
                            Some(value) => value,
                            None => bail!("Missing field 'destination' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#preserve_unmatched_path: {
                        let field_value = match fields_map.get("preserve_unmatched_path") {
                            Some(value) => value,
                            None => bail!("Missing field 'preserve_unmatched_path' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#source_pattern: {
                        let field_value = match fields_map.get("source_pattern") {
                            Some(value) => value,
                            None => bail!("Missing field 'source_pattern' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
