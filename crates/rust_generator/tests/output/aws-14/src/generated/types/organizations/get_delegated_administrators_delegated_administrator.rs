#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct GetDelegatedAdministratorsDelegatedAdministrator {
    /// The ARN of the delegated administrator's account.
    #[builder(into)]
    #[serde(rename = "arn")]
    pub r#arn: String,
    /// The date when the account was made a delegated administrator.
    #[builder(into)]
    #[serde(rename = "delegationEnabledDate")]
    pub r#delegation_enabled_date: String,
    /// The email address that is associated with the delegated administrator's AWS account.
    #[builder(into)]
    #[serde(rename = "email")]
    pub r#email: String,
    /// The unique identifier (ID) of the delegated administrator's account.
    #[builder(into)]
    #[serde(rename = "id")]
    pub r#id: String,
    /// The method by which the delegated administrator's account joined the organization.
    #[builder(into)]
    #[serde(rename = "joinedMethod")]
    pub r#joined_method: String,
    /// The date when the delegated administrator's account became a part of the organization.
    #[builder(into)]
    #[serde(rename = "joinedTimestamp")]
    pub r#joined_timestamp: String,
    /// The friendly name of the delegated administrator's account.
    #[builder(into)]
    #[serde(rename = "name")]
    pub r#name: String,
    /// The status of the delegated administrator's account in the organization.
    #[builder(into)]
    #[serde(rename = "status")]
    pub r#status: String,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for GetDelegatedAdministratorsDelegatedAdministrator {
    fn to_pulumi_value(
        &self,
    ) -> impl std::future::Future<
        Output = pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue,
    > {
        async move {
            use std::collections::BTreeMap;
            use pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue;

            let mut map: BTreeMap<String, pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue> = BTreeMap::new();
            map.insert("arn".to_string(), self.r#arn.to_pulumi_value().await);
            map.insert("delegation_enabled_date".to_string(), self.r#delegation_enabled_date.to_pulumi_value().await);
            map.insert("email".to_string(), self.r#email.to_pulumi_value().await);
            map.insert("id".to_string(), self.r#id.to_pulumi_value().await);
            map.insert("joined_method".to_string(), self.r#joined_method.to_pulumi_value().await);
            map.insert("joined_timestamp".to_string(), self.r#joined_timestamp.to_pulumi_value().await);
            map.insert("name".to_string(), self.r#name.to_pulumi_value().await);
            map.insert("status".to_string(), self.r#status.to_pulumi_value().await);

            map.to_pulumi_value().await
        }
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for GetDelegatedAdministratorsDelegatedAdministrator {
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
                    r#arn: {
                        let field_value = match fields_map.get("arn") {
                            Some(value) => value,
                            None => bail!("Missing field 'arn' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <String as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#delegation_enabled_date: {
                        let field_value = match fields_map.get("delegation_enabled_date") {
                            Some(value) => value,
                            None => bail!("Missing field 'delegation_enabled_date' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <String as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#email: {
                        let field_value = match fields_map.get("email") {
                            Some(value) => value,
                            None => bail!("Missing field 'email' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <String as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#id: {
                        let field_value = match fields_map.get("id") {
                            Some(value) => value,
                            None => bail!("Missing field 'id' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <String as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#joined_method: {
                        let field_value = match fields_map.get("joined_method") {
                            Some(value) => value,
                            None => bail!("Missing field 'joined_method' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <String as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#joined_timestamp: {
                        let field_value = match fields_map.get("joined_timestamp") {
                            Some(value) => value,
                            None => bail!("Missing field 'joined_timestamp' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <String as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#name: {
                        let field_value = match fields_map.get("name") {
                            Some(value) => value,
                            None => bail!("Missing field 'name' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <String as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#status: {
                        let field_value = match fields_map.get("status") {
                            Some(value) => value,
                            None => bail!("Missing field 'status' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <String as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
