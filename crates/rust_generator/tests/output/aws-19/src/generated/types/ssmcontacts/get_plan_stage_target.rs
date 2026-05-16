#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct GetPlanStageTarget {
    #[builder(into)]
    #[serde(rename = "channelTargetInfos")]
    pub r#channel_target_infos: Vec<super::super::types::ssmcontacts::GetPlanStageTargetChannelTargetInfo>,
    #[builder(into)]
    #[serde(rename = "contactTargetInfos")]
    pub r#contact_target_infos: Vec<super::super::types::ssmcontacts::GetPlanStageTargetContactTargetInfo>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for GetPlanStageTarget {
    fn to_pulumi_value(
        &self,
    ) -> impl std::future::Future<
        Output = pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue,
    > {
        async move {
            use std::collections::BTreeMap;
            use pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue;

            let mut map: BTreeMap<String, pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue> = BTreeMap::new();
            map.insert("channel_target_infos".to_string(), self.r#channel_target_infos.to_pulumi_value().await);
            map.insert("contact_target_infos".to_string(), self.r#contact_target_infos.to_pulumi_value().await);

            map.to_pulumi_value().await
        }
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for GetPlanStageTarget {
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
                    r#channel_target_infos: {
                        let field_value = match fields_map.get("channel_target_infos") {
                            Some(value) => value,
                            None => bail!("Missing field 'channel_target_infos' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Vec<super::super::types::ssmcontacts::GetPlanStageTargetChannelTargetInfo> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#contact_target_infos: {
                        let field_value = match fields_map.get("contact_target_infos") {
                            Some(value) => value,
                            None => bail!("Missing field 'contact_target_infos' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Vec<super::super::types::ssmcontacts::GetPlanStageTargetContactTargetInfo> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
