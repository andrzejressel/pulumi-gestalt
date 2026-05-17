#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct PreventionInspectTemplateInspectConfigLimits {
    /// Configuration of findings limit given for specified infoTypes.
    /// Structure is documented below.
    #[builder(into)]
    #[serde(rename = "maxFindingsPerInfoTypes")]
    pub r#max_findings_per_info_types: Option<Vec<super::super::types::dataloss::PreventionInspectTemplateInspectConfigLimitsMaxFindingsPerInfoType>>,
    /// Max number of findings that will be returned for each item scanned. The maximum returned is 2000.
    #[builder(into)]
    #[serde(rename = "maxFindingsPerItem")]
    pub r#max_findings_per_item: i32,
    /// Max number of findings that will be returned per request/job. The maximum returned is 2000.
    #[builder(into)]
    #[serde(rename = "maxFindingsPerRequest")]
    pub r#max_findings_per_request: i32,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for PreventionInspectTemplateInspectConfigLimits {
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
                "max_findings_per_info_types".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#max_findings_per_info_types,
                )
                .await,
            );
            map.insert(
                "max_findings_per_item".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#max_findings_per_item,
                )
                .await,
            );
            map.insert(
                "max_findings_per_request".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#max_findings_per_request,
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

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for PreventionInspectTemplateInspectConfigLimits {
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
                    r#max_findings_per_info_types: {
                        let field_value = match fields_map.get("max_findings_per_info_types") {
                            Some(value) => value,
                            None => bail!("Missing field 'max_findings_per_info_types' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#max_findings_per_item: {
                        let field_value = match fields_map.get("max_findings_per_item") {
                            Some(value) => value,
                            None => bail!("Missing field 'max_findings_per_item' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#max_findings_per_request: {
                        let field_value = match fields_map.get("max_findings_per_request") {
                            Some(value) => value,
                            None => bail!("Missing field 'max_findings_per_request' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
