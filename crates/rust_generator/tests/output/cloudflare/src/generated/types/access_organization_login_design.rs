#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct AccessOrganizationLoginDesign {
    /// The background color on the login page.
    #[builder(into)]
    #[serde(rename = "backgroundColor")]
    pub r#background_color: Option<String>,
    /// The text at the bottom of the login page.
    #[builder(into)]
    #[serde(rename = "footerText")]
    pub r#footer_text: Option<String>,
    /// The text at the top of the login page.
    #[builder(into)]
    #[serde(rename = "headerText")]
    pub r#header_text: Option<String>,
    /// The URL of the logo on the login page.
    #[builder(into)]
    #[serde(rename = "logoPath")]
    pub r#logo_path: Option<String>,
    /// The text color on the login page.
    #[builder(into)]
    #[serde(rename = "textColor")]
    pub r#text_color: Option<String>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for AccessOrganizationLoginDesign {
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
                    "background_color",
                    &self.r#background_color,
                ),
                to_pulumi_object_field(
                    "footer_text",
                    &self.r#footer_text,
                ),
                to_pulumi_object_field(
                    "header_text",
                    &self.r#header_text,
                ),
                to_pulumi_object_field(
                    "logo_path",
                    &self.r#logo_path,
                ),
                to_pulumi_object_field(
                    "text_color",
                    &self.r#text_color,
                ),
            ];
            to_pulumi_object_concurrent(field_futures).await
        }
        .boxed_local()
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for AccessOrganizationLoginDesign {
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
                    r#background_color: {
                        let field_value = match fields_map.get("background_color") {
                            Some(value) => value,
                            None => bail!("Missing field 'background_color' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#footer_text: {
                        let field_value = match fields_map.get("footer_text") {
                            Some(value) => value,
                            None => bail!("Missing field 'footer_text' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#header_text: {
                        let field_value = match fields_map.get("header_text") {
                            Some(value) => value,
                            None => bail!("Missing field 'header_text' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#logo_path: {
                        let field_value = match fields_map.get("logo_path") {
                            Some(value) => value,
                            None => bail!("Missing field 'logo_path' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#text_color: {
                        let field_value = match fields_map.get("text_color") {
                            Some(value) => value,
                            None => bail!("Missing field 'text_color' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
