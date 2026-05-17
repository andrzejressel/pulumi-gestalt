#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct PreventionInspectTemplateInspectConfigLimitsMaxFindingsPerInfoType {
    /// Type of information the findings limit applies to. Only one limit per infoType should be provided. If InfoTypeLimit does
    /// not have an infoType, the DLP API applies the limit against all infoTypes that are found but not
    /// specified in another InfoTypeLimit.
    /// Structure is documented below.
    #[builder(into)]
    #[serde(rename = "infoType")]
    pub r#info_type: Option<Box<super::super::types::dataloss::PreventionInspectTemplateInspectConfigLimitsMaxFindingsPerInfoTypeInfoType>>,
    /// Max findings limit for the given infoType.
    #[builder(into)]
    #[serde(rename = "maxFindings")]
    pub r#max_findings: i32,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for PreventionInspectTemplateInspectConfigLimitsMaxFindingsPerInfoType {
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
                "info_type".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#info_type,
                )
                .await,
            );
            map.insert(
                "max_findings".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#max_findings,
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

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for PreventionInspectTemplateInspectConfigLimitsMaxFindingsPerInfoType {
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
                    r#info_type: {
                        let field_value = match fields_map.get("info_type") {
                            Some(value) => value,
                            None => bail!("Missing field 'info_type' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#max_findings: {
                        let field_value = match fields_map.get("max_findings") {
                            Some(value) => value,
                            None => bail!("Missing field 'max_findings' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
