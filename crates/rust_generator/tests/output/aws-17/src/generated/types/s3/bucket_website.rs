#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct BucketWebsite {
    /// An absolute path to the document to return in case of a 4XX error.
    #[builder(into)]
    #[serde(rename = "errorDocument")]
    pub r#error_document: Option<String>,
    /// Amazon S3 returns this index document when requests are made to the root domain or any of the subfolders.
    #[builder(into)]
    #[serde(rename = "indexDocument")]
    pub r#index_document: Option<String>,
    /// A hostname to redirect all website requests for this bucket to. Hostname can optionally be prefixed with a protocol (`http://` or `https://`) to use when redirecting requests. The default is the protocol that is used in the original request.
    #[builder(into)]
    #[serde(rename = "redirectAllRequestsTo")]
    pub r#redirect_all_requests_to: Option<String>,
    /// A json array containing [routing rules](https://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-s3-websiteconfiguration-routingrules.html)
    /// describing redirect behavior and when redirects are applied.
    /// 
    /// The `CORS` object supports the following:
    #[builder(into)]
    #[serde(rename = "routingRules")]
    pub r#routing_rules: Option<String>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for BucketWebsite {
    fn to_pulumi_value(
        &self,
    ) -> impl std::future::Future<
        Output = pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue,
    > {
        async move {
            use std::collections::BTreeMap;
            use pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue;
            use pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue;

            let mut map: BTreeMap<String, PulumiValue> = BTreeMap::new();
            map.insert(
                "error_document".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#error_document,
                )
                .await,
            );
            map.insert(
                "index_document".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#index_document,
                )
                .await,
            );
            map.insert(
                "redirect_all_requests_to".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#redirect_all_requests_to,
                )
                .await,
            );
            map.insert(
                "routing_rules".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#routing_rules,
                )
                .await,
            );

            ToPulumiValue::to_pulumi_value(
                &map,
            )
            .await
        }
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
                        let field_value = match fields_map.get("error_document") {
                            Some(value) => value,
                            None => bail!("Missing field 'error_document' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#index_document: {
                        let field_value = match fields_map.get("index_document") {
                            Some(value) => value,
                            None => bail!("Missing field 'index_document' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#redirect_all_requests_to: {
                        let field_value = match fields_map.get("redirect_all_requests_to") {
                            Some(value) => value,
                            None => bail!("Missing field 'redirect_all_requests_to' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#routing_rules: {
                        let field_value = match fields_map.get("routing_rules") {
                            Some(value) => value,
                            None => bail!("Missing field 'routing_rules' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
