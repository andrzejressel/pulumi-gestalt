#[derive(pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct HttpRouteRuleMatchHeader {
    /// The value of the header should match exactly the content of exactMatch.
    #[builder(into)]
    pub r#exact_match: Option<String>,
    /// The name of the HTTP header to match against.
    #[builder(into)]
    pub r#header: Option<String>,
    /// If specified, the match result will be inverted before checking. Default value is set to false.
    #[builder(into)]
    pub r#invert_match: Option<bool>,
    /// The value of the header must start with the contents of prefixMatch.
    #[builder(into)]
    pub r#prefix_match: Option<String>,
    /// A header with headerName must exist. The match takes place whether or not the header has a value.
    #[builder(into)]
    pub r#present_match: Option<bool>,
    /// If specified, the rule will match if the request header value is within the range.
    /// Structure is documented below.
    #[builder(into)]
    pub r#range_match: Option<Box<super::super::types::networkservices::HttpRouteRuleMatchHeaderRangeMatch>>,
    /// The value of the header must match the regular expression specified in regexMatch.
    #[builder(into)]
    pub r#regex_match: Option<String>,
    /// The value of the header must end with the contents of suffixMatch.
    #[builder(into)]
    pub r#suffix_match: Option<String>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for HttpRouteRuleMatchHeader {
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
                    "exactMatch",
                    &self.r#exact_match,
                ),
                to_pulumi_object_field(
                    "header",
                    &self.r#header,
                ),
                to_pulumi_object_field(
                    "invertMatch",
                    &self.r#invert_match,
                ),
                to_pulumi_object_field(
                    "prefixMatch",
                    &self.r#prefix_match,
                ),
                to_pulumi_object_field(
                    "presentMatch",
                    &self.r#present_match,
                ),
                to_pulumi_object_field(
                    "rangeMatch",
                    &self.r#range_match,
                ),
                to_pulumi_object_field(
                    "regexMatch",
                    &self.r#regex_match,
                ),
                to_pulumi_object_field(
                    "suffixMatch",
                    &self.r#suffix_match,
                ),
            ];
            to_pulumi_object_concurrent(field_futures).await
        }
        .boxed()
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for HttpRouteRuleMatchHeader {
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
                    r#exact_match: {
                        let field_value = match fields_map.get("exactMatch") {
                            Some(value) => value,
                            None => bail!("Missing field 'exactMatch' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#header: {
                        let field_value = match fields_map.get("header") {
                            Some(value) => value,
                            None => bail!("Missing field 'header' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#invert_match: {
                        let field_value = match fields_map.get("invertMatch") {
                            Some(value) => value,
                            None => bail!("Missing field 'invertMatch' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#prefix_match: {
                        let field_value = match fields_map.get("prefixMatch") {
                            Some(value) => value,
                            None => bail!("Missing field 'prefixMatch' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#present_match: {
                        let field_value = match fields_map.get("presentMatch") {
                            Some(value) => value,
                            None => bail!("Missing field 'presentMatch' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#range_match: {
                        let field_value = match fields_map.get("rangeMatch") {
                            Some(value) => value,
                            None => bail!("Missing field 'rangeMatch' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#regex_match: {
                        let field_value = match fields_map.get("regexMatch") {
                            Some(value) => value,
                            None => bail!("Missing field 'regexMatch' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#suffix_match: {
                        let field_value = match fields_map.get("suffixMatch") {
                            Some(value) => value,
                            None => bail!("Missing field 'suffixMatch' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
