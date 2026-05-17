#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct GetVirtualNodeSpecListenerOutlierDetection {
    #[builder(into)]
    #[serde(rename = "baseEjectionDurations")]
    pub r#base_ejection_durations: Vec<super::super::types::appmesh::GetVirtualNodeSpecListenerOutlierDetectionBaseEjectionDuration>,
    #[builder(into)]
    #[serde(rename = "intervals")]
    pub r#intervals: Vec<super::super::types::appmesh::GetVirtualNodeSpecListenerOutlierDetectionInterval>,
    #[builder(into)]
    #[serde(rename = "maxEjectionPercent")]
    pub r#max_ejection_percent: i32,
    #[builder(into)]
    #[serde(rename = "maxServerErrors")]
    pub r#max_server_errors: i32,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for GetVirtualNodeSpecListenerOutlierDetection {
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
                "base_ejection_durations".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#base_ejection_durations,
                )
                .await,
            );
            map.insert(
                "intervals".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#intervals,
                )
                .await,
            );
            map.insert(
                "max_ejection_percent".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#max_ejection_percent,
                )
                .await,
            );
            map.insert(
                "max_server_errors".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#max_server_errors,
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

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for GetVirtualNodeSpecListenerOutlierDetection {
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
                    r#base_ejection_durations: {
                        let field_value = match fields_map.get("base_ejection_durations") {
                            Some(value) => value,
                            None => bail!("Missing field 'base_ejection_durations' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#intervals: {
                        let field_value = match fields_map.get("intervals") {
                            Some(value) => value,
                            None => bail!("Missing field 'intervals' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#max_ejection_percent: {
                        let field_value = match fields_map.get("max_ejection_percent") {
                            Some(value) => value,
                            None => bail!("Missing field 'max_ejection_percent' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#max_server_errors: {
                        let field_value = match fields_map.get("max_server_errors") {
                            Some(value) => value,
                            None => bail!("Missing field 'max_server_errors' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
