#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct LifecyclePolicyPolicyDetailsActionCrossRegionCopy {
    /// The encryption settings for the copied snapshot. See the `encryption_configuration` block. Max of 1 per action.
    #[builder(into)]
    #[serde(rename = "encryptionConfiguration")]
    pub r#encryption_configuration: Box<super::super::types::dlm::LifecyclePolicyPolicyDetailsActionCrossRegionCopyEncryptionConfiguration>,
    #[builder(into)]
    #[serde(rename = "retainRule")]
    pub r#retain_rule: Option<Box<super::super::types::dlm::LifecyclePolicyPolicyDetailsActionCrossRegionCopyRetainRule>>,
    #[builder(into)]
    #[serde(rename = "target")]
    pub r#target: String,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for LifecyclePolicyPolicyDetailsActionCrossRegionCopy {
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
                "encryption_configuration".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#encryption_configuration,
                )
                .await,
            );
            map.insert(
                "retain_rule".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#retain_rule,
                )
                .await,
            );
            map.insert(
                "target".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#target,
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

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for LifecyclePolicyPolicyDetailsActionCrossRegionCopy {
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
                    r#encryption_configuration: {
                        let field_value = match fields_map.get("encryption_configuration") {
                            Some(value) => value,
                            None => bail!("Missing field 'encryption_configuration' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#retain_rule: {
                        let field_value = match fields_map.get("retain_rule") {
                            Some(value) => value,
                            None => bail!("Missing field 'retain_rule' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#target: {
                        let field_value = match fields_map.get("target") {
                            Some(value) => value,
                            None => bail!("Missing field 'target' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
