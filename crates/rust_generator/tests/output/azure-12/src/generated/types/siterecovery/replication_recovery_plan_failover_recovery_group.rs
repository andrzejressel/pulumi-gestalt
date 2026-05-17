#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct ReplicationRecoveryPlanFailoverRecoveryGroup {
    /// one or more `action` block as defined below. which will be executed after the group recovery.
    #[builder(into)]
    #[serde(rename = "postActions")]
    pub r#post_actions: Option<Vec<super::super::types::siterecovery::ReplicationRecoveryPlanFailoverRecoveryGroupPostAction>>,
    /// one or more `action` block as defined below. which will be executed before the group recovery.
    #[builder(into)]
    #[serde(rename = "preActions")]
    pub r#pre_actions: Option<Vec<super::super::types::siterecovery::ReplicationRecoveryPlanFailoverRecoveryGroupPreAction>>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for ReplicationRecoveryPlanFailoverRecoveryGroup {
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
                "post_actions".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#post_actions,
                )
                .await,
            );
            map.insert(
                "pre_actions".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#pre_actions,
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

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for ReplicationRecoveryPlanFailoverRecoveryGroup {
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
                    r#post_actions: {
                        let field_value = match fields_map.get("post_actions") {
                            Some(value) => value,
                            None => bail!("Missing field 'post_actions' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#pre_actions: {
                        let field_value = match fields_map.get("pre_actions") {
                            Some(value) => value,
                            None => bail!("Missing field 'pre_actions' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
