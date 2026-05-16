#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct SearchResource {
    /// Amazon resource name of resource.
    #[builder(into)]
    #[serde(rename = "arn")]
    pub r#arn: String,
    /// The date and time that the information about this resource property was last updated.
    #[builder(into)]
    #[serde(rename = "lastReportedAt")]
    pub r#last_reported_at: String,
    /// Amazon Web Services account that owns the resource.
    #[builder(into)]
    #[serde(rename = "owningAccountId")]
    pub r#owning_account_id: String,
    /// Structure with additional type-specific details about the resource.  See `properties` below.
    #[builder(into)]
    #[serde(rename = "properties")]
    pub r#properties: Vec<super::super::types::resourceexplorer::SearchResourceProperty>,
    /// Amazon Web Services Region in which the resource was created and exists.
    #[builder(into)]
    #[serde(rename = "region")]
    pub r#region: String,
    /// Type of the resource.
    #[builder(into)]
    #[serde(rename = "resourceType")]
    pub r#resource_type: String,
    /// Amazon Web Service that owns the resource and is responsible for creating and updating it.
    #[builder(into)]
    #[serde(rename = "service")]
    pub r#service: String,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for SearchResource {
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
            map.insert("last_reported_at".to_string(), self.r#last_reported_at.to_pulumi_value().await);
            map.insert("owning_account_id".to_string(), self.r#owning_account_id.to_pulumi_value().await);
            map.insert("properties".to_string(), self.r#properties.to_pulumi_value().await);
            map.insert("region".to_string(), self.r#region.to_pulumi_value().await);
            map.insert("resource_type".to_string(), self.r#resource_type.to_pulumi_value().await);
            map.insert("service".to_string(), self.r#service.to_pulumi_value().await);

            map.to_pulumi_value().await
        }
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for SearchResource {
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
                    r#last_reported_at: {
                        let field_value = match fields_map.get("last_reported_at") {
                            Some(value) => value,
                            None => bail!("Missing field 'last_reported_at' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <String as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#owning_account_id: {
                        let field_value = match fields_map.get("owning_account_id") {
                            Some(value) => value,
                            None => bail!("Missing field 'owning_account_id' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <String as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#properties: {
                        let field_value = match fields_map.get("properties") {
                            Some(value) => value,
                            None => bail!("Missing field 'properties' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Vec<super::super::types::resourceexplorer::SearchResourceProperty> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#region: {
                        let field_value = match fields_map.get("region") {
                            Some(value) => value,
                            None => bail!("Missing field 'region' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <String as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#resource_type: {
                        let field_value = match fields_map.get("resource_type") {
                            Some(value) => value,
                            None => bail!("Missing field 'resource_type' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <String as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#service: {
                        let field_value = match fields_map.get("service") {
                            Some(value) => value,
                            None => bail!("Missing field 'service' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <String as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
