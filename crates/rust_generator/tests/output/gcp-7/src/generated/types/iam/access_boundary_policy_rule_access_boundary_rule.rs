#[derive(pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct AccessBoundaryPolicyRuleAccessBoundaryRule {
    /// The availability condition further constrains the access allowed by the access boundary rule.
    /// Structure is documented below.
    #[builder(into)]
    pub r#availability_condition: Option<Box<super::super::types::iam::AccessBoundaryPolicyRuleAccessBoundaryRuleAvailabilityCondition>>,
    /// A list of permissions that may be allowed for use on the specified resource.
    #[builder(into)]
    pub r#available_permissions: Option<Vec<String>>,
    /// The full resource name of a Google Cloud resource entity.
    #[builder(into)]
    pub r#available_resource: Option<String>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for AccessBoundaryPolicyRuleAccessBoundaryRule {
    fn to_pulumi_value(
        &self,
    ) -> impl std::future::Future<
        Output = pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue,
    > + Send {
        use pulumi_gestalt_rust::__private::futures::FutureExt;
        use pulumi_gestalt_rust::__private::pulumi_gestalt_model::__private::to_pulumi_object_concurrent;
        async move {
            use pulumi_gestalt_rust::__private::pulumi_gestalt_model::__private::{
                to_pulumi_object_field, ToPulumiObjectFieldFuture,
            };
            let field_futures: Vec<ToPulumiObjectFieldFuture<'_>> = vec![
                to_pulumi_object_field(
                    "availabilityCondition",
                    &self.r#availability_condition,
                ),
                to_pulumi_object_field(
                    "availablePermissions",
                    &self.r#available_permissions,
                ),
                to_pulumi_object_field(
                    "availableResource",
                    &self.r#available_resource,
                ),
            ];
            to_pulumi_object_concurrent(field_futures).await
        }
        .boxed()
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
                        let field_value = match fields_map.get("availabilityCondition") {
                            Some(value) => value,
                            None => bail!("Missing field 'availabilityCondition' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#available_permissions: {
                        let field_value = match fields_map.get("availablePermissions") {
                            Some(value) => value,
                            None => bail!("Missing field 'availablePermissions' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#available_resource: {
                        let field_value = match fields_map.get("availableResource") {
                            Some(value) => value,
                            None => bail!("Missing field 'availableResource' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
