#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct AppMonitorAppMonitorConfiguration {
    /// If you set this to `true`, RUM web client sets two cookies, a session cookie  and a user cookie. The cookies allow the RUM web client to collect data relating to the number of users an application has and the behavior of the application across a sequence of events. Cookies are stored in the top-level domain of the current page.
    #[builder(into)]
    #[serde(rename = "allowCookies")]
    pub r#allow_cookies: Option<bool>,
    /// If you set this to `true`, RUM enables X-Ray tracing for the user sessions  that RUM samples. RUM adds an X-Ray trace header to allowed HTTP requests. It also records an X-Ray segment for allowed HTTP requests.
    #[builder(into)]
    #[serde(rename = "enableXray")]
    pub r#enable_xray: Option<bool>,
    /// A list of URLs in your website or application to exclude from RUM data collection.
    #[builder(into)]
    #[serde(rename = "excludedPages")]
    pub r#excluded_pages: Option<Vec<String>>,
    /// A list of pages in the CloudWatch RUM console that are to be displayed with a "favorite" icon.
    #[builder(into)]
    #[serde(rename = "favoritePages")]
    pub r#favorite_pages: Option<Vec<String>>,
    /// The ARN of the guest IAM role that is attached to the Amazon Cognito identity pool that is used to authorize the sending of data to RUM.
    #[builder(into)]
    #[serde(rename = "guestRoleArn")]
    pub r#guest_role_arn: Option<String>,
    /// The ID of the Amazon Cognito identity pool that is used to authorize the sending of data to RUM.
    #[builder(into)]
    #[serde(rename = "identityPoolId")]
    pub r#identity_pool_id: Option<String>,
    /// If this app monitor is to collect data from only certain pages in your application, this structure lists those pages.
    #[builder(into)]
    #[serde(rename = "includedPages")]
    pub r#included_pages: Option<Vec<String>>,
    /// Specifies the percentage of user sessions to use for RUM data collection. Choosing a higher percentage gives you more data but also incurs more costs. The number you specify is the percentage of user sessions that will be used. Default value is `0.1`.
    #[builder(into)]
    #[serde(rename = "sessionSampleRate")]
    pub r#session_sample_rate: Option<f64>,
    /// An array that lists the types of telemetry data that this app monitor is to collect. Valid values are `errors`, `performance`, and `http`.
    #[builder(into)]
    #[serde(rename = "telemetries")]
    pub r#telemetries: Option<Vec<String>>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for AppMonitorAppMonitorConfiguration {
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
                "allow_cookies".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#allow_cookies,
                )
                .await,
            );
            map.insert(
                "enable_xray".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#enable_xray,
                )
                .await,
            );
            map.insert(
                "excluded_pages".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#excluded_pages,
                )
                .await,
            );
            map.insert(
                "favorite_pages".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#favorite_pages,
                )
                .await,
            );
            map.insert(
                "guest_role_arn".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#guest_role_arn,
                )
                .await,
            );
            map.insert(
                "identity_pool_id".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#identity_pool_id,
                )
                .await,
            );
            map.insert(
                "included_pages".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#included_pages,
                )
                .await,
            );
            map.insert(
                "session_sample_rate".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#session_sample_rate,
                )
                .await,
            );
            map.insert(
                "telemetries".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#telemetries,
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

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for AppMonitorAppMonitorConfiguration {
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
                    r#allow_cookies: {
                        let field_value = match fields_map.get("allow_cookies") {
                            Some(value) => value,
                            None => bail!("Missing field 'allow_cookies' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#enable_xray: {
                        let field_value = match fields_map.get("enable_xray") {
                            Some(value) => value,
                            None => bail!("Missing field 'enable_xray' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#excluded_pages: {
                        let field_value = match fields_map.get("excluded_pages") {
                            Some(value) => value,
                            None => bail!("Missing field 'excluded_pages' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#favorite_pages: {
                        let field_value = match fields_map.get("favorite_pages") {
                            Some(value) => value,
                            None => bail!("Missing field 'favorite_pages' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#guest_role_arn: {
                        let field_value = match fields_map.get("guest_role_arn") {
                            Some(value) => value,
                            None => bail!("Missing field 'guest_role_arn' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#identity_pool_id: {
                        let field_value = match fields_map.get("identity_pool_id") {
                            Some(value) => value,
                            None => bail!("Missing field 'identity_pool_id' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#included_pages: {
                        let field_value = match fields_map.get("included_pages") {
                            Some(value) => value,
                            None => bail!("Missing field 'included_pages' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#session_sample_rate: {
                        let field_value = match fields_map.get("session_sample_rate") {
                            Some(value) => value,
                            None => bail!("Missing field 'session_sample_rate' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#telemetries: {
                        let field_value = match fields_map.get("telemetries") {
                            Some(value) => value,
                            None => bail!("Missing field 'telemetries' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
