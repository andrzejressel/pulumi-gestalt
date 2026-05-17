#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct PlanStageTarget {
    /// A configuration block for specifying information about the contact channel that Incident Manager engages. See Channel Target Info for more details.
    #[builder(into)]
    #[serde(rename = "channelTargetInfo")]
    pub r#channel_target_info: Option<Box<super::super::types::ssmcontacts::PlanStageTargetChannelTargetInfo>>,
    /// A configuration block for specifying information about the contact that Incident Manager engages. See Contact Target Info for more details.
    #[builder(into)]
    #[serde(rename = "contactTargetInfo")]
    pub r#contact_target_info: Option<Box<super::super::types::ssmcontacts::PlanStageTargetContactTargetInfo>>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for PlanStageTarget {
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
                "channel_target_info".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#channel_target_info,
                )
                .await,
            );
            map.insert(
                "contact_target_info".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#contact_target_info,
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

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for PlanStageTarget {
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
                    r#channel_target_info: {
                        let field_value = match fields_map.get("channel_target_info") {
                            Some(value) => value,
                            None => bail!("Missing field 'channel_target_info' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#contact_target_info: {
                        let field_value = match fields_map.get("contact_target_info") {
                            Some(value) => value,
                            None => bail!("Missing field 'contact_target_info' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
