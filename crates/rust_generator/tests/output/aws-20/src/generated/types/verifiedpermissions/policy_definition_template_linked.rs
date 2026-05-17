#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct PolicyDefinitionTemplateLinked {
    /// The ID of the template.
    #[builder(into)]
    #[serde(rename = "policyTemplateId")]
    pub r#policy_template_id: String,
    /// The principal of the template linked policy.
    #[builder(into)]
    #[serde(rename = "principal")]
    pub r#principal: Option<Box<super::super::types::verifiedpermissions::PolicyDefinitionTemplateLinkedPrincipal>>,
    /// The resource of the template linked policy.
    #[builder(into)]
    #[serde(rename = "resource")]
    pub r#resource: Option<Box<super::super::types::verifiedpermissions::PolicyDefinitionTemplateLinkedResource>>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for PolicyDefinitionTemplateLinked {
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
                "policy_template_id".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#policy_template_id,
                )
                .await,
            );
            map.insert(
                "principal".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#principal,
                )
                .await,
            );
            map.insert(
                "resource".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#resource,
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

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for PolicyDefinitionTemplateLinked {
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
                    r#policy_template_id: {
                        let field_value = match fields_map.get("policy_template_id") {
                            Some(value) => value,
                            None => bail!("Missing field 'policy_template_id' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#principal: {
                        let field_value = match fields_map.get("principal") {
                            Some(value) => value,
                            None => bail!("Missing field 'principal' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#resource: {
                        let field_value = match fields_map.get("resource") {
                            Some(value) => value,
                            None => bail!("Missing field 'resource' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
