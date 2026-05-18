#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct GetApplicationAttribute {
    /// Optional. Business team that ensures user needs are met and value is delivered
    #[builder(into)]
    #[serde(rename = "businessOwners")]
    pub r#business_owners: Vec<super::super::types::apphub::GetApplicationAttributeBusinessOwner>,
    /// Criticality of the Application, Service, or Workload
    #[builder(into)]
    #[serde(rename = "criticalities")]
    pub r#criticalities: Vec<super::super::types::apphub::GetApplicationAttributeCriticality>,
    /// Optional. Developer team that owns development and coding.
    #[builder(into)]
    #[serde(rename = "developerOwners")]
    pub r#developer_owners: Vec<super::super::types::apphub::GetApplicationAttributeDeveloperOwner>,
    /// Environment of the Application, Service, or Workload
    #[builder(into)]
    #[serde(rename = "environments")]
    pub r#environments: Vec<super::super::types::apphub::GetApplicationAttributeEnvironment>,
    /// Optional. Operator team that ensures runtime and operations.
    #[builder(into)]
    #[serde(rename = "operatorOwners")]
    pub r#operator_owners: Vec<super::super::types::apphub::GetApplicationAttributeOperatorOwner>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for GetApplicationAttribute {
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
                    "business_owners",
                    &self.r#business_owners,
                ),
                to_pulumi_object_field(
                    "criticalities",
                    &self.r#criticalities,
                ),
                to_pulumi_object_field(
                    "developer_owners",
                    &self.r#developer_owners,
                ),
                to_pulumi_object_field(
                    "environments",
                    &self.r#environments,
                ),
                to_pulumi_object_field(
                    "operator_owners",
                    &self.r#operator_owners,
                ),
            ];
            to_pulumi_object_concurrent(field_futures).await
        }
        .boxed()
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for GetApplicationAttribute {
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
                    r#business_owners: {
                        let field_value = match fields_map.get("business_owners") {
                            Some(value) => value,
                            None => bail!("Missing field 'business_owners' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#criticalities: {
                        let field_value = match fields_map.get("criticalities") {
                            Some(value) => value,
                            None => bail!("Missing field 'criticalities' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#developer_owners: {
                        let field_value = match fields_map.get("developer_owners") {
                            Some(value) => value,
                            None => bail!("Missing field 'developer_owners' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#environments: {
                        let field_value = match fields_map.get("environments") {
                            Some(value) => value,
                            None => bail!("Missing field 'environments' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#operator_owners: {
                        let field_value = match fields_map.get("operator_owners") {
                            Some(value) => value,
                            None => bail!("Missing field 'operator_owners' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
