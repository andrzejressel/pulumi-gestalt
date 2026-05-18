#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct ScheduleTargetEcsParametersNetworkConfiguration {
    /// Specifies whether the task's elastic network interface receives a public IP address. This attribute is a boolean type, where `true` maps to `ENABLED` and `false` to `DISABLED`. You can specify `true` only when the `launch_type` is set to `FARGATE`.
    #[builder(into)]
    #[serde(rename = "assignPublicIp")]
    pub r#assign_public_ip: Option<bool>,
    /// Set of 1 to 5 Security Group ID-s to be associated with the task. These security groups must all be in the same VPC.
    #[builder(into)]
    #[serde(rename = "securityGroups")]
    pub r#security_groups: Option<Vec<String>>,
    /// Set of 1 to 16 subnets to be associated with the task. These subnets must all be in the same VPC.
    #[builder(into)]
    #[serde(rename = "subnets")]
    pub r#subnets: Vec<String>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for ScheduleTargetEcsParametersNetworkConfiguration {
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
                    "assign_public_ip",
                    &self.r#assign_public_ip,
                ),
                to_pulumi_object_field(
                    "security_groups",
                    &self.r#security_groups,
                ),
                to_pulumi_object_field(
                    "subnets",
                    &self.r#subnets,
                ),
            ];
            to_pulumi_object_concurrent(field_futures).await
        }
        .boxed()
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for ScheduleTargetEcsParametersNetworkConfiguration {
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
                    r#assign_public_ip: {
                        let field_value = match fields_map.get("assign_public_ip") {
                            Some(value) => value,
                            None => bail!("Missing field 'assign_public_ip' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#security_groups: {
                        let field_value = match fields_map.get("security_groups") {
                            Some(value) => value,
                            None => bail!("Missing field 'security_groups' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#subnets: {
                        let field_value = match fields_map.get("subnets") {
                            Some(value) => value,
                            None => bail!("Missing field 'subnets' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
