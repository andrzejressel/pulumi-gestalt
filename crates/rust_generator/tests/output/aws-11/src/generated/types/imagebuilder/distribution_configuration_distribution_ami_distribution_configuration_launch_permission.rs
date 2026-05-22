#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct DistributionConfigurationDistributionAmiDistributionConfigurationLaunchPermission {
    /// Set of AWS Organization ARNs to assign.
    #[builder(into)]
    #[serde(rename = "organizationArns")]
    pub r#organization_arns: Option<Vec<String>>,
    /// Set of AWS Organizational Unit ARNs to assign.
    #[builder(into)]
    #[serde(rename = "organizationalUnitArns")]
    pub r#organizational_unit_arns: Option<Vec<String>>,
    /// Set of EC2 launch permission user groups to assign. Use `all` to distribute a public AMI.
    #[builder(into)]
    #[serde(rename = "userGroups")]
    pub r#user_groups: Option<Vec<String>>,
    /// Set of AWS Account identifiers to assign.
    #[builder(into)]
    #[serde(rename = "userIds")]
    pub r#user_ids: Option<Vec<String>>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for DistributionConfigurationDistributionAmiDistributionConfigurationLaunchPermission {
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
                    "organizationArns",
                    &self.r#organization_arns,
                ),
                to_pulumi_object_field(
                    "organizationalUnitArns",
                    &self.r#organizational_unit_arns,
                ),
                to_pulumi_object_field(
                    "userGroups",
                    &self.r#user_groups,
                ),
                to_pulumi_object_field(
                    "userIds",
                    &self.r#user_ids,
                ),
            ];
            to_pulumi_object_concurrent(field_futures).await
        }
        .boxed()
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for DistributionConfigurationDistributionAmiDistributionConfigurationLaunchPermission {
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
                    r#organization_arns: {
                        let field_value = match fields_map.get("organizationArns") {
                            Some(value) => value,
                            None => bail!("Missing field 'organizationArns' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#organizational_unit_arns: {
                        let field_value = match fields_map.get("organizationalUnitArns") {
                            Some(value) => value,
                            None => bail!("Missing field 'organizationalUnitArns' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#user_groups: {
                        let field_value = match fields_map.get("userGroups") {
                            Some(value) => value,
                            None => bail!("Missing field 'userGroups' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#user_ids: {
                        let field_value = match fields_map.get("userIds") {
                            Some(value) => value,
                            None => bail!("Missing field 'userIds' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
