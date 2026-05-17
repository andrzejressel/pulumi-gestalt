#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct ApplicationAttributes {
    /// Optional. Business team that ensures user needs are met and value is delivered
    /// Structure is documented below.
    #[builder(into)]
    #[serde(rename = "businessOwners")]
    pub r#business_owners: Option<Vec<super::super::types::apphub::ApplicationAttributesBusinessOwner>>,
    /// Criticality of the Application, Service, or Workload
    /// Structure is documented below.
    #[builder(into)]
    #[serde(rename = "criticality")]
    pub r#criticality: Option<Box<super::super::types::apphub::ApplicationAttributesCriticality>>,
    /// Optional. Developer team that owns development and coding.
    /// Structure is documented below.
    #[builder(into)]
    #[serde(rename = "developerOwners")]
    pub r#developer_owners: Option<Vec<super::super::types::apphub::ApplicationAttributesDeveloperOwner>>,
    /// Environment of the Application, Service, or Workload
    /// Structure is documented below.
    #[builder(into)]
    #[serde(rename = "environment")]
    pub r#environment: Option<Box<super::super::types::apphub::ApplicationAttributesEnvironment>>,
    /// Optional. Operator team that ensures runtime and operations.
    /// Structure is documented below.
    #[builder(into)]
    #[serde(rename = "operatorOwners")]
    pub r#operator_owners: Option<Vec<super::super::types::apphub::ApplicationAttributesOperatorOwner>>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for ApplicationAttributes {
    fn to_pulumi_value(
        &self,
    ) -> impl std::future::Future<
        Output = pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue,
    > {
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
                    "criticality",
                    &self.r#criticality,
                ),
                to_pulumi_object_field(
                    "developer_owners",
                    &self.r#developer_owners,
                ),
                to_pulumi_object_field(
                    "environment",
                    &self.r#environment,
                ),
                to_pulumi_object_field(
                    "operator_owners",
                    &self.r#operator_owners,
                ),
            ];
            to_pulumi_object_concurrent(field_futures).await
        }
        .boxed_local()
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for ApplicationAttributes {
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
                    r#criticality: {
                        let field_value = match fields_map.get("criticality") {
                            Some(value) => value,
                            None => bail!("Missing field 'criticality' while converting PulumiValue to {}", std::any::type_name::<Self>()),
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
                    r#environment: {
                        let field_value = match fields_map.get("environment") {
                            Some(value) => value,
                            None => bail!("Missing field 'environment' while converting PulumiValue to {}", std::any::type_name::<Self>()),
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
