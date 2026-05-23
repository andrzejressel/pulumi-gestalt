#[derive(pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct BucketWebsite {
    /// An absolute path to the document to return in case of a 4XX error.
    #[builder(into)]
    pub r#error_document: Option<String>,
    /// Amazon S3 returns this index document when requests are made to the root domain or any of the subfolders.
    #[builder(into)]
    pub r#index_document: Option<String>,
    /// A hostname to redirect all website requests for this bucket to. Hostname can optionally be prefixed with a protocol (`http://` or `https://`) to use when redirecting requests. The default is the protocol that is used in the original request.
    #[builder(into)]
    pub r#redirect_all_requests_to: Option<String>,
    /// A json array containing [routing rules](https://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-s3-websiteconfiguration-routingrules.html)
    /// describing redirect behavior and when redirects are applied.
    /// 
    /// The `CORS` object supports the following:
    #[builder(into)]
    pub r#routing_rules: Option<String>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for BucketWebsite {
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
                    "errorDocument",
                    &self.r#error_document,
                ),
                to_pulumi_object_field(
                    "indexDocument",
                    &self.r#index_document,
                ),
                to_pulumi_object_field(
                    "redirectAllRequestsTo",
                    &self.r#redirect_all_requests_to,
                ),
                to_pulumi_object_field(
                    "routingRules",
                    &self.r#routing_rules,
                ),
            ];
            to_pulumi_object_concurrent(field_futures).await
        }
        .boxed()
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for BucketWebsite {
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
                    r#error_document: {
                        let field_value = match fields_map.get("errorDocument") {
                            Some(value) => value,
                            None => bail!("Missing field 'errorDocument' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#index_document: {
                        let field_value = match fields_map.get("indexDocument") {
                            Some(value) => value,
                            None => bail!("Missing field 'indexDocument' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#redirect_all_requests_to: {
                        let field_value = match fields_map.get("redirectAllRequestsTo") {
                            Some(value) => value,
                            None => bail!("Missing field 'redirectAllRequestsTo' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#routing_rules: {
                        let field_value = match fields_map.get("routingRules") {
                            Some(value) => value,
                            None => bail!("Missing field 'routingRules' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
