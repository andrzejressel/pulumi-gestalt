#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct AccessBoundaryPolicyRuleAccessBoundaryRule {
    /// The availability condition further constrains the access allowed by the access boundary rule.
    /// Structure is documented below.
    #[builder(into)]
    #[serde(rename = "availabilityCondition")]
    pub r#availability_condition: Option<Box<super::super::types::iam::AccessBoundaryPolicyRuleAccessBoundaryRuleAvailabilityCondition>>,
    /// A list of permissions that may be allowed for use on the specified resource.
    #[builder(into)]
    #[serde(rename = "availablePermissions")]
    pub r#available_permissions: Option<Vec<String>>,
    /// The full resource name of a Google Cloud resource entity.
    #[builder(into)]
    #[serde(rename = "availableResource")]
    pub r#available_resource: Option<String>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for AccessBoundaryPolicyRuleAccessBoundaryRule {
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
                "availability_condition".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#availability_condition,
                )
                .await,
            );
            map.insert(
                "available_permissions".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#available_permissions,
                )
                .await,
            );
            map.insert(
                "available_resource".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#available_resource,
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

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for AccessBoundaryPolicyRuleAccessBoundaryRule {
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
                    r#availability_condition: {
                        let field_value = match fields_map.get("availability_condition") {
                            Some(value) => value,
                            None => bail!("Missing field 'availability_condition' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#available_permissions: {
                        let field_value = match fields_map.get("available_permissions") {
                            Some(value) => value,
                            None => bail!("Missing field 'available_permissions' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#available_resource: {
                        let field_value = match fields_map.get("available_resource") {
                            Some(value) => value,
                            None => bail!("Missing field 'available_resource' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
