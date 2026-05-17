#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct ControlControlMappingSource {
    /// Description of the source.
    #[builder(into)]
    #[serde(rename = "sourceDescription")]
    pub r#source_description: Option<String>,
    /// Frequency of evidence collection. Valid values are `DAILY`, `WEEKLY`, or `MONTHLY`.
    #[builder(into)]
    #[serde(rename = "sourceFrequency")]
    pub r#source_frequency: Option<String>,
    #[builder(into)]
    #[serde(rename = "sourceId")]
    pub r#source_id: Option<String>,
    /// The keyword to search for in CloudTrail logs, Config rules, Security Hub checks, and Amazon Web Services API names. See `source_keyword` below.
    #[builder(into)]
    #[serde(rename = "sourceKeyword")]
    pub r#source_keyword: Option<Box<super::super::types::auditmanager::ControlControlMappingSourceSourceKeyword>>,
    /// Name of the source.
    #[builder(into)]
    #[serde(rename = "sourceName")]
    pub r#source_name: String,
    /// The setup option for the data source. This option reflects if the evidence collection is automated or manual. Valid values are `System_Controls_Mapping` (automated) and `Procedural_Controls_Mapping` (manual).
    #[builder(into)]
    #[serde(rename = "sourceSetUpOption")]
    pub r#source_set_up_option: String,
    /// Type of data source for evidence collection. If `source_set_up_option` is manual, the only valid value is `MANUAL`. If `source_set_up_option` is automated, valid values are `AWS_Cloudtrail`, `AWS_Config`, `AWS_Security_Hub`, or `AWS_API_Call`.
    /// 
    /// The following arguments are optional:
    #[builder(into)]
    #[serde(rename = "sourceType")]
    pub r#source_type: String,
    /// Instructions for troubleshooting the control.
    #[builder(into)]
    #[serde(rename = "troubleshootingText")]
    pub r#troubleshooting_text: Option<String>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for ControlControlMappingSource {
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
                "source_description".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#source_description,
                )
                .await,
            );
            map.insert(
                "source_frequency".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#source_frequency,
                )
                .await,
            );
            map.insert(
                "source_id".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#source_id,
                )
                .await,
            );
            map.insert(
                "source_keyword".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#source_keyword,
                )
                .await,
            );
            map.insert(
                "source_name".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#source_name,
                )
                .await,
            );
            map.insert(
                "source_set_up_option".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#source_set_up_option,
                )
                .await,
            );
            map.insert(
                "source_type".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#source_type,
                )
                .await,
            );
            map.insert(
                "troubleshooting_text".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#troubleshooting_text,
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

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for ControlControlMappingSource {
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
                    r#source_description: {
                        let field_value = match fields_map.get("source_description") {
                            Some(value) => value,
                            None => bail!("Missing field 'source_description' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#source_frequency: {
                        let field_value = match fields_map.get("source_frequency") {
                            Some(value) => value,
                            None => bail!("Missing field 'source_frequency' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#source_id: {
                        let field_value = match fields_map.get("source_id") {
                            Some(value) => value,
                            None => bail!("Missing field 'source_id' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#source_keyword: {
                        let field_value = match fields_map.get("source_keyword") {
                            Some(value) => value,
                            None => bail!("Missing field 'source_keyword' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#source_name: {
                        let field_value = match fields_map.get("source_name") {
                            Some(value) => value,
                            None => bail!("Missing field 'source_name' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#source_set_up_option: {
                        let field_value = match fields_map.get("source_set_up_option") {
                            Some(value) => value,
                            None => bail!("Missing field 'source_set_up_option' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#source_type: {
                        let field_value = match fields_map.get("source_type") {
                            Some(value) => value,
                            None => bail!("Missing field 'source_type' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#troubleshooting_text: {
                        let field_value = match fields_map.get("troubleshooting_text") {
                            Some(value) => value,
                            None => bail!("Missing field 'troubleshooting_text' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
