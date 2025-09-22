#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct FlexibleAppVersionHandler {
    /// Actions to take when the user is not logged in.
    /// Possible values are: `AUTH_FAIL_ACTION_REDIRECT`, `AUTH_FAIL_ACTION_UNAUTHORIZED`.
    #[builder(into)]
    #[serde(rename = "authFailAction")]
    pub r#auth_fail_action: Option<String>,
    /// Methods to restrict access to a URL based on login status.
    /// Possible values are: `LOGIN_OPTIONAL`, `LOGIN_ADMIN`, `LOGIN_REQUIRED`.
    #[builder(into)]
    #[serde(rename = "login")]
    pub r#login: Option<String>,
    /// 30x code to use when performing redirects for the secure field.
    /// Possible values are: `REDIRECT_HTTP_RESPONSE_CODE_301`, `REDIRECT_HTTP_RESPONSE_CODE_302`, `REDIRECT_HTTP_RESPONSE_CODE_303`, `REDIRECT_HTTP_RESPONSE_CODE_307`.
    #[builder(into)]
    #[serde(rename = "redirectHttpResponseCode")]
    pub r#redirect_http_response_code: Option<String>,
    /// Executes a script to handle the requests that match this URL pattern.
    /// Only the auto value is supported for Node.js in the App Engine standard environment, for example "script:" "auto".
    /// Structure is documented below.
    #[builder(into)]
    #[serde(rename = "script")]
    pub r#script: Option<Box<super::super::types::appengine::FlexibleAppVersionHandlerScript>>,
    /// Security (HTTPS) enforcement for this URL.
    /// Possible values are: `SECURE_DEFAULT`, `SECURE_NEVER`, `SECURE_OPTIONAL`, `SECURE_ALWAYS`.
    #[builder(into)]
    #[serde(rename = "securityLevel")]
    pub r#security_level: Option<String>,
    /// Files served directly to the user for a given URL, such as images, CSS stylesheets, or JavaScript source files.
    /// Static file handlers describe which files in the application directory are static files, and which URLs serve them.
    /// Structure is documented below.
    #[builder(into)]
    #[serde(rename = "staticFiles")]
    pub r#static_files: Option<Box<super::super::types::appengine::FlexibleAppVersionHandlerStaticFiles>>,
    /// URL prefix. Uses regular expression syntax, which means regexp special characters must be escaped, but should not contain groupings.
    /// All URLs that begin with this prefix are handled by this handler, using the portion of the URL after the prefix as part of the file path.
    #[builder(into)]
    #[serde(rename = "urlRegex")]
    pub r#url_regex: Option<String>,
}
