#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct UserPoolSchema {
    /// Attribute data type. Must be one of `Boolean`, `Number`, `String`, `DateTime`.
    #[builder(into)]
    #[serde(rename = "attributeDataType")]
    pub r#attribute_data_type: String,
    /// Whether the attribute type is developer only.
    #[builder(into)]
    #[serde(rename = "developerOnlyAttribute")]
    pub r#developer_only_attribute: Option<bool>,
    /// Whether the attribute can be changed once it has been created.
    #[builder(into)]
    #[serde(rename = "mutable")]
    pub r#mutable: Option<bool>,
    /// Name of the attribute.
    #[builder(into)]
    #[serde(rename = "name")]
    pub r#name: String,
    /// Configuration block for the constraints for an attribute of the number type. Detailed below.
    #[builder(into)]
    #[serde(rename = "numberAttributeConstraints")]
    pub r#number_attribute_constraints: Option<Box<super::super::types::cognito::UserPoolSchemaNumberAttributeConstraints>>,
    /// Whether a user pool attribute is required. If the attribute is required and the user does not provide a value, registration or sign-in will fail.
    #[builder(into)]
    #[serde(rename = "required")]
    pub r#required: Option<bool>,
    /// Constraints for an attribute of the string type. Detailed below.
    #[builder(into)]
    #[serde(rename = "stringAttributeConstraints")]
    pub r#string_attribute_constraints: Option<Box<super::super::types::cognito::UserPoolSchemaStringAttributeConstraints>>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for UserPoolSchema {
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
                "attribute_data_type".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#attribute_data_type,
                )
                .await,
            );
            map.insert(
                "developer_only_attribute".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#developer_only_attribute,
                )
                .await,
            );
            map.insert(
                "mutable".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#mutable,
                )
                .await,
            );
            map.insert(
                "name".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#name,
                )
                .await,
            );
            map.insert(
                "number_attribute_constraints".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#number_attribute_constraints,
                )
                .await,
            );
            map.insert(
                "required".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#required,
                )
                .await,
            );
            map.insert(
                "string_attribute_constraints".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#string_attribute_constraints,
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

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for UserPoolSchema {
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
                    r#attribute_data_type: {
                        let field_value = match fields_map.get("attribute_data_type") {
                            Some(value) => value,
                            None => bail!("Missing field 'attribute_data_type' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#developer_only_attribute: {
                        let field_value = match fields_map.get("developer_only_attribute") {
                            Some(value) => value,
                            None => bail!("Missing field 'developer_only_attribute' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#mutable: {
                        let field_value = match fields_map.get("mutable") {
                            Some(value) => value,
                            None => bail!("Missing field 'mutable' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#name: {
                        let field_value = match fields_map.get("name") {
                            Some(value) => value,
                            None => bail!("Missing field 'name' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#number_attribute_constraints: {
                        let field_value = match fields_map.get("number_attribute_constraints") {
                            Some(value) => value,
                            None => bail!("Missing field 'number_attribute_constraints' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#required: {
                        let field_value = match fields_map.get("required") {
                            Some(value) => value,
                            None => bail!("Missing field 'required' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#string_attribute_constraints: {
                        let field_value = match fields_map.get("string_attribute_constraints") {
                            Some(value) => value,
                            None => bail!("Missing field 'string_attribute_constraints' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
