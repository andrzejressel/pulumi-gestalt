#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct UserName {
    /// The family name of the user.
    #[builder(into)]
    #[serde(rename = "familyName")]
    pub r#family_name: String,
    /// The name that is typically displayed when the name is shown for display.
    #[builder(into)]
    #[serde(rename = "formatted")]
    pub r#formatted: Option<String>,
    /// The given name of the user.
    /// 
    /// The following arguments are optional:
    #[builder(into)]
    #[serde(rename = "givenName")]
    pub r#given_name: String,
    /// The honorific prefix of the user.
    #[builder(into)]
    #[serde(rename = "honorificPrefix")]
    pub r#honorific_prefix: Option<String>,
    /// The honorific suffix of the user.
    #[builder(into)]
    #[serde(rename = "honorificSuffix")]
    pub r#honorific_suffix: Option<String>,
    /// The middle name of the user.
    #[builder(into)]
    #[serde(rename = "middleName")]
    pub r#middle_name: Option<String>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for UserName {
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
                    "family_name",
                    &self.r#family_name,
                ),
                to_pulumi_object_field(
                    "formatted",
                    &self.r#formatted,
                ),
                to_pulumi_object_field(
                    "given_name",
                    &self.r#given_name,
                ),
                to_pulumi_object_field(
                    "honorific_prefix",
                    &self.r#honorific_prefix,
                ),
                to_pulumi_object_field(
                    "honorific_suffix",
                    &self.r#honorific_suffix,
                ),
                to_pulumi_object_field(
                    "middle_name",
                    &self.r#middle_name,
                ),
            ];
            to_pulumi_object_concurrent(field_futures).await
        }
        .boxed_local()
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for UserName {
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
                    r#family_name: {
                        let field_value = match fields_map.get("family_name") {
                            Some(value) => value,
                            None => bail!("Missing field 'family_name' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#formatted: {
                        let field_value = match fields_map.get("formatted") {
                            Some(value) => value,
                            None => bail!("Missing field 'formatted' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#given_name: {
                        let field_value = match fields_map.get("given_name") {
                            Some(value) => value,
                            None => bail!("Missing field 'given_name' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#honorific_prefix: {
                        let field_value = match fields_map.get("honorific_prefix") {
                            Some(value) => value,
                            None => bail!("Missing field 'honorific_prefix' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#honorific_suffix: {
                        let field_value = match fields_map.get("honorific_suffix") {
                            Some(value) => value,
                            None => bail!("Missing field 'honorific_suffix' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#middle_name: {
                        let field_value = match fields_map.get("middle_name") {
                            Some(value) => value,
                            None => bail!("Missing field 'middle_name' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
