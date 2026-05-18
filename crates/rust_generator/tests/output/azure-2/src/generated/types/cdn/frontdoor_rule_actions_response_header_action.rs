#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct FrontdoorRuleActionsResponseHeaderAction {
    /// The action to be taken on the specified `header_name`. Possible values include `Append`, `Overwrite` or `Delete`.
    /// 
    /// > **NOTE:** `Append` causes the specified header to be added to the request with the specified value. If the header is already present, the value is appended to the existing header value using string concatenation. No delimiters are added. `Overwrite` causes specified header to be added to the request with the specified value. If the header is already present, the specified value overwrites the existing value. `Delete` causes the header to be deleted from the request.
    #[builder(into)]
    #[serde(rename = "headerAction")]
    pub r#header_action: String,
    /// The name of the header to modify.
    #[builder(into)]
    #[serde(rename = "headerName")]
    pub r#header_name: String,
    /// The value to append or overwrite.
    /// 
    /// ->**NOTE:** `value` is required if the `header_action` is set to `Append` or `Overwrite`.
    #[builder(into)]
    #[serde(rename = "value")]
    pub r#value: Option<String>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for FrontdoorRuleActionsResponseHeaderAction {
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
                    "header_action",
                    &self.r#header_action,
                ),
                to_pulumi_object_field(
                    "header_name",
                    &self.r#header_name,
                ),
                to_pulumi_object_field(
                    "value",
                    &self.r#value,
                ),
            ];
            to_pulumi_object_concurrent(field_futures).await
        }
        .boxed()
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for FrontdoorRuleActionsResponseHeaderAction {
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
                    r#header_action: {
                        let field_value = match fields_map.get("header_action") {
                            Some(value) => value,
                            None => bail!("Missing field 'header_action' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#header_name: {
                        let field_value = match fields_map.get("header_name") {
                            Some(value) => value,
                            None => bail!("Missing field 'header_name' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#value: {
                        let field_value = match fields_map.get("value") {
                            Some(value) => value,
                            None => bail!("Missing field 'value' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
