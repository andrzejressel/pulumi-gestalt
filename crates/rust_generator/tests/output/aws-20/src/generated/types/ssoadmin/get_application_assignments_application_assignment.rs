#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct GetApplicationAssignmentsApplicationAssignment {
    /// ARN of the application.
    #[builder(into)]
    #[serde(rename = "applicationArn")]
    pub r#application_arn: String,
    /// An identifier for an object in IAM Identity Center, such as a user or group.
    #[builder(into)]
    #[serde(rename = "principalId")]
    pub r#principal_id: String,
    /// Entity type for which the assignment will be created. Valid values are `USER` or `GROUP`.
    #[builder(into)]
    #[serde(rename = "principalType")]
    pub r#principal_type: String,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for GetApplicationAssignmentsApplicationAssignment {
    fn to_pulumi_value(
        &self,
    ) -> impl std::future::Future<
        Output = pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue,
    > {
        async move {
            use std::collections::BTreeMap;
            use pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue;

            let mut map: BTreeMap<String, pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue> = BTreeMap::new();
            map.insert("application_arn".to_string(), self.r#application_arn.to_pulumi_value().await);
            map.insert("principal_id".to_string(), self.r#principal_id.to_pulumi_value().await);
            map.insert("principal_type".to_string(), self.r#principal_type.to_pulumi_value().await);

            map.to_pulumi_value().await
        }
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for GetApplicationAssignmentsApplicationAssignment {
    fn from_pulumi_value(
        value: &pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue,
    ) -> pulumi_gestalt_rust::__private::rootcause::Result<Self> {
        use std::collections::BTreeMap;
        use pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValueContent;
        use pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue;
        use pulumi_gestalt_rust::__private::rootcause::bail;

        match value.content {
            PulumiValueContent::Object(ref obj) => {
                let fields_map: BTreeMap<String, pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue> =
                    obj.iter().cloned().collect();

                Ok(Self {
                    r#application_arn: {
                        let field_value = match fields_map.get("application_arn") {
                            Some(value) => value,
                            None => bail!("Missing field 'application_arn' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <String as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#principal_id: {
                        let field_value = match fields_map.get("principal_id") {
                            Some(value) => value,
                            None => bail!("Missing field 'principal_id' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <String as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#principal_type: {
                        let field_value = match fields_map.get("principal_type") {
                            Some(value) => value,
                            None => bail!("Missing field 'principal_type' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <String as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
