
awsAWS"6.66.2*� 
;
paymentcryptographyKeyaws:paymentcryptography/key:Key�Resource for managing an AWS Payment Cryptography Control Plane Key.

## Example Usage

### Basic Usage

<!--Start PulumiCodeChooser -->
```java
package generated_program;

import com.pulumi.Context;
import com.pulumi.Pulumi;
import com.pulumi.core.Output;
import com.pulumi.aws.paymentcryptography.Key;
import com.pulumi.aws.paymentcryptography.KeyArgs;
import java.util.List;
import java.util.ArrayList;
import java.util.Map;
import java.io.File;
import java.nio.file.Files;
import java.nio.file.Paths;

public class App {
    public static void main(String[] args) {
        Pulumi.run(App::stack);
    }

    public static void stack(Context ctx) {
        var test = new Key("test", KeyArgs.builder()
            .exportable(true)
            .keyAttributes(KeyKeyAttributesArgs.builder()
                .keyAlgorithm("TDES_3KEY")
                .keyClass("SYMMETRIC_KEY")
                .keyUsage("TR31_P0_PIN_ENCRYPTION_KEY")
                .keyModesOfUse(KeyKeyAttributesKeyModesOfUseArgs.builder()
                    .decrypt(true)
                    .encrypt(true)
                    .wrap(true)
                    .unwrap(true)
                    .build())
                .build())
            .build());

    }
}
```
```yaml
resources:
  test:
    type: aws:paymentcryptography:Key
    properties:
      exportable: true
      keyAttributes:
        - keyAlgorithm: TDES_3KEY
          keyClass: SYMMETRIC_KEY
          keyUsage: TR31_P0_PIN_ENCRYPTION_KEY
          keyModesOfUse:
            - decrypt: true
              encrypt: true
              wrap: true
              unwrap: true
```
<!--End PulumiCodeChooser -->

## Import

Using `pulumi import`, import Payment Cryptography Control Plane Key using the `arn:aws:payment-cryptography:us-east-1:123456789012:key/qtbojf64yshyvyzf`. For example:

```sh
$ pulumi import aws:paymentcryptography/key:Key example arn:aws:payment-cryptography:us-east-1:123456789012:key/qtbojf64yshyvyzf
```

deletionWindowInDaysB ,
enabledB
 Whether to enable the key.
B

exportable
 0Whether the key is exportable from the service.
�
keyAttributeshBf:d
b
paymentcryptographyKeyKeyAttributes9aws:paymentcryptography/KeyKeyAttributes:KeyKeyAttributes�Role of the key, the algorithm it supports, and the cryptographic operations allowed with the key.

The following arguments are optional:
u
keyCheckValueAlgorithmB" UAlgorithm that AWS Payment Cryptography uses to calculate the key check value (KCV).
�
tagsB2" �Map of tags assigned to the WorkSpaces Connection Alias. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
e
timeoutsYBW:U
S
paymentcryptographyKeyTimeouts/aws:paymentcryptography/KeyTimeouts:KeyTimeouts"
arn" ARN of the key.
"
deletionWindowInDays "*
enabled
 Whether to enable the key.
"B

exportable
 0Whether the key is exportable from the service.
"�
keyAttributeshBf:d
b
paymentcryptographyKeyKeyAttributes9aws:paymentcryptography/KeyKeyAttributes:KeyKeyAttributes�Role of the key, the algorithm it supports, and the cryptographic operations allowed with the key.

The following arguments are optional:
"�
keyCheckValue" �Key check value (KCV) is used to check if all parties holding a given key have the same key or to detect that a key has changed.
"s
keyCheckValueAlgorithm" UAlgorithm that AWS Payment Cryptography uses to calculate the key check value (KCV).
"-
	keyOrigin" Source of the key material.
"?
keyState" /State of key that is being created or deleted.
"�
tagsB2" �Map of tags assigned to the WorkSpaces Connection Alias. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
"�
tagsAll2" vMap of tags assigned to the resource, including those inherited from the provider `default_tags` configuration block.
"e
timeoutsYBW:U
S
paymentcryptographyKeyTimeouts/aws:paymentcryptography/KeyTimeouts:KeyTimeouts*�
J
paymentcryptographyKeyAlias)aws:paymentcryptography/keyAlias:KeyAlias�Resource for managing an AWS Payment Cryptography Control Plane Key Alias.

## Example Usage

### Basic Usage

<!--Start PulumiCodeChooser -->
```java
package generated_program;

import com.pulumi.Context;
import com.pulumi.Pulumi;
import com.pulumi.core.Output;
import com.pulumi.aws.paymentcryptography.Key;
import com.pulumi.aws.paymentcryptography.KeyArgs;
import com.pulumi.aws.paymentcryptography.KeyAlias;
import com.pulumi.aws.paymentcryptography.KeyAliasArgs;
import java.util.List;
import java.util.ArrayList;
import java.util.Map;
import java.io.File;
import java.nio.file.Files;
import java.nio.file.Paths;

public class App {
    public static void main(String[] args) {
        Pulumi.run(App::stack);
    }

    public static void stack(Context ctx) {
        var test = new Key("test", KeyArgs.builder()
            .exportable(true)
            .keyAttributes(KeyKeyAttributesArgs.builder()
                .keyAlgorithm("TDES_3KEY")
                .keyClass("SYMMETRIC_KEY")
                .keyUsage("TR31_P0_PIN_ENCRYPTION_KEY")
                .keyModesOfUse(KeyKeyAttributesKeyModesOfUseArgs.builder()
                    .decrypt(true)
                    .encrypt(true)
                    .wrap(true)
                    .unwrap(true)
                    .build())
                .build())
            .build());

        var testKeyAlias = new KeyAlias("testKeyAlias", KeyAliasArgs.builder()
            .aliasName("alias/test-alias")
            .keyArn(test.arn())
            .build());

    }
}
```
```yaml
resources:
  test:
    type: aws:paymentcryptography:Key
    properties:
      exportable: true
      keyAttributes:
        - keyAlgorithm: TDES_3KEY
          keyClass: SYMMETRIC_KEY
          keyUsage: TR31_P0_PIN_ENCRYPTION_KEY
          keyModesOfUse:
            - decrypt: true
              encrypt: true
              wrap: true
              unwrap: true
  testKeyAlias:
    type: aws:paymentcryptography:KeyAlias
    name: test
    properties:
      aliasName: alias/test-alias
      keyArn: ${test.arn}
```
<!--End PulumiCodeChooser -->

## Import

Using `pulumi import`, import Payment Cryptography Control Plane Key Alias using the `alias/4681482429376900170`. For example:

```sh
$ pulumi import aws:paymentcryptography/keyAlias:KeyAlias example alias/4681482429376900170
```
O
	aliasName" >Name of the Key Alias.

The following arguments are optional:
 
keyArnB" ARN of the key.
"O
	aliasName" >Name of the Key Alias.

The following arguments are optional:
" 
keyArnB" ARN of the key.
*�
:
pinpoint
AdmChannel"aws:pinpoint/admChannel:AdmChannel�Provides a Pinpoint ADM (Amazon Device Messaging) Channel resource.

> **Note:** All arguments including the Client ID and Client Secret will be stored in the raw state as plain-text.
## Example Usage

<!--Start PulumiCodeChooser -->
```typescript
import * as pulumi from "@pulumi/pulumi";
import * as aws from "@pulumi/aws";

const app = new aws.pinpoint.App("app", {});
const channel = new aws.pinpoint.AdmChannel("channel", {
    applicationId: app.applicationId,
    clientId: "",
    clientSecret: "",
    enabled: true,
});
```
```python
import pulumi
import pulumi_aws as aws

app = aws.pinpoint.App("app")
channel = aws.pinpoint.AdmChannel("channel",
    application_id=app.application_id,
    client_id="",
    client_secret="",
    enabled=True)
```
```csharp
using System.Collections.Generic;
using System.Linq;
using Pulumi;
using Aws = Pulumi.Aws;

return await Deployment.RunAsync(() => 
{
    var app = new Aws.Pinpoint.App("app");

    var channel = new Aws.Pinpoint.AdmChannel("channel", new()
    {
        ApplicationId = app.ApplicationId,
        ClientId = "",
        ClientSecret = "",
        Enabled = true,
    });

});
```
```go
package main

import (
	"github.com/pulumi/pulumi-aws/sdk/v6/go/aws/pinpoint"
	"github.com/pulumi/pulumi/sdk/v3/go/pulumi"
)

func main() {
	pulumi.Run(func(ctx *pulumi.Context) error {
		app, err := pinpoint.NewApp(ctx, "app", nil)
		if err != nil {
			return err
		}
		_, err = pinpoint.NewAdmChannel(ctx, "channel", &pinpoint.AdmChannelArgs{
			ApplicationId: app.ApplicationId,
			ClientId:      pulumi.String(""),
			ClientSecret:  pulumi.String(""),
			Enabled:       pulumi.Bool(true),
		})
		if err != nil {
			return err
		}
		return nil
	})
}
```
```java
package generated_program;

import com.pulumi.Context;
import com.pulumi.Pulumi;
import com.pulumi.core.Output;
import com.pulumi.aws.pinpoint.App;
import com.pulumi.aws.pinpoint.AdmChannel;
import com.pulumi.aws.pinpoint.AdmChannelArgs;
import java.util.List;
import java.util.ArrayList;
import java.util.Map;
import java.io.File;
import java.nio.file.Files;
import java.nio.file.Paths;

public class App {
    public static void main(String[] args) {
        Pulumi.run(App::stack);
    }

    public static void stack(Context ctx) {
        var app = new App("app");

        var channel = new AdmChannel("channel", AdmChannelArgs.builder()
            .applicationId(app.applicationId())
            .clientId("")
            .clientSecret("")
            .enabled(true)
            .build());

    }
}
```
```yaml
resources:
  app:
    type: aws:pinpoint:App
  channel:
    type: aws:pinpoint:AdmChannel
    properties:
      applicationId: ${app.applicationId}
      clientId: ""
      clientSecret: ""
      enabled: true
```
<!--End PulumiCodeChooser -->

## Import

Using `pulumi import`, import Pinpoint ADM Channel using the `application-id`. For example:

```sh
$ pulumi import aws:pinpoint/admChannel:AdmChannel channel application-id
```
)
applicationId" The application ID.
]
clientId" MClient ID (part of OAuth Credentials) obtained via Amazon Developer Account.
e
clientSecret" QClient Secret (part of OAuth Credentials) obtained via Amazon Developer Account.
N
enabledB
 =Specifies whether to enable the channel. Defaults to `true`.
")
applicationId" The application ID.
"]
clientId" MClient ID (part of OAuth Credentials) obtained via Amazon Developer Account.
"e
clientSecret" QClient Secret (part of OAuth Credentials) obtained via Amazon Developer Account.
"N
enabledB
 =Specifies whether to enable the channel. Defaults to `true`.
*�8
=
pinpointApnsChannel$aws:pinpoint/apnsChannel:ApnsChannel� Provides a Pinpoint APNs Channel resource.

> **Note:** All arguments, including certificates and tokens, will be stored in the raw state as plain-text.
## Example Usage

<!--Start PulumiCodeChooser -->
```typescript
import * as pulumi from "@pulumi/pulumi";
import * as aws from "@pulumi/aws";
import * as std from "@pulumi/std";

const app = new aws.pinpoint.App("app", {});
const apns = new aws.pinpoint.ApnsChannel("apns", {
    applicationId: app.applicationId,
    certificate: std.file({
        input: "./certificate.pem",
    }).then(invoke => invoke.result),
    privateKey: std.file({
        input: "./private_key.key",
    }).then(invoke => invoke.result),
});
```
```python
import pulumi
import pulumi_aws as aws
import pulumi_std as std

app = aws.pinpoint.App("app")
apns = aws.pinpoint.ApnsChannel("apns",
    application_id=app.application_id,
    certificate=std.file(input="./certificate.pem").result,
    private_key=std.file(input="./private_key.key").result)
```
```csharp
using System.Collections.Generic;
using System.Linq;
using Pulumi;
using Aws = Pulumi.Aws;
using Std = Pulumi.Std;

return await Deployment.RunAsync(() => 
{
    var app = new Aws.Pinpoint.App("app");

    var apns = new Aws.Pinpoint.ApnsChannel("apns", new()
    {
        ApplicationId = app.ApplicationId,
        Certificate = Std.File.Invoke(new()
        {
            Input = "./certificate.pem",
        }).Apply(invoke => invoke.Result),
        PrivateKey = Std.File.Invoke(new()
        {
            Input = "./private_key.key",
        }).Apply(invoke => invoke.Result),
    });

});
```
```go
package main

import (
	"github.com/pulumi/pulumi-aws/sdk/v6/go/aws/pinpoint"
	"github.com/pulumi/pulumi-std/sdk/go/std"
	"github.com/pulumi/pulumi/sdk/v3/go/pulumi"
)

func main() {
	pulumi.Run(func(ctx *pulumi.Context) error {
		app, err := pinpoint.NewApp(ctx, "app", nil)
		if err != nil {
			return err
		}
		invokeFile, err := std.File(ctx, &std.FileArgs{
			Input: "./certificate.pem",
		}, nil)
		if err != nil {
			return err
		}
		invokeFile1, err := std.File(ctx, &std.FileArgs{
			Input: "./private_key.key",
		}, nil)
		if err != nil {
			return err
		}
		_, err = pinpoint.NewApnsChannel(ctx, "apns", &pinpoint.ApnsChannelArgs{
			ApplicationId: app.ApplicationId,
			Certificate:   pulumi.String(invokeFile.Result),
			PrivateKey:    pulumi.String(invokeFile1.Result),
		})
		if err != nil {
			return err
		}
		return nil
	})
}
```
```java
package generated_program;

import com.pulumi.Context;
import com.pulumi.Pulumi;
import com.pulumi.core.Output;
import com.pulumi.aws.pinpoint.App;
import com.pulumi.aws.pinpoint.ApnsChannel;
import com.pulumi.aws.pinpoint.ApnsChannelArgs;
import java.util.List;
import java.util.ArrayList;
import java.util.Map;
import java.io.File;
import java.nio.file.Files;
import java.nio.file.Paths;

public class App {
    public static void main(String[] args) {
        Pulumi.run(App::stack);
    }

    public static void stack(Context ctx) {
        var app = new App("app");

        var apns = new ApnsChannel("apns", ApnsChannelArgs.builder()
            .applicationId(app.applicationId())
            .certificate(StdFunctions.file(FileArgs.builder()
                .input("./certificate.pem")
                .build()).result())
            .privateKey(StdFunctions.file(FileArgs.builder()
                .input("./private_key.key")
                .build()).result())
            .build());

    }
}
```
```yaml
resources:
  apns:
    type: aws:pinpoint:ApnsChannel
    properties:
      applicationId: ${app.applicationId}
      certificate:
        fn::invoke:
          function: std:file
          arguments:
            input: ./certificate.pem
          return: result
      privateKey:
        fn::invoke:
          function: std:file
          arguments:
            input: ./private_key.key
          return: result
  app:
    type: aws:pinpoint:App
```
<!--End PulumiCodeChooser -->

## Import

Using `pulumi import`, import Pinpoint APNs Channel using the `application-id`. For example:

```sh
$ pulumi import aws:pinpoint/apnsChannel:ApnsChannel apns application-id
```
)
applicationId" The application ID.
�
bundleIdB" �The ID assigned to your iOS app. To find this value, choose Certificates, IDs & Profiles, choose App IDs in the Identifiers section, and choose your app.
A
certificateB" ,The pem encoded TLS Certificate from Apple.
�
defaultAuthenticationMethodB" �The default authentication method used for APNs.
__NOTE__: Amazon Pinpoint uses this default for every APNs push notification that you send using the console.
You can override the default when you send a message programmatically using the Amazon Pinpoint API, the AWS CLI, or an AWS SDK.
If your default authentication type fails, Amazon Pinpoint doesn't attempt to use the other authentication type.

One of the following sets of credentials is also required.

If you choose to use __Certificate credentials__ you will have to provide:
Q
enabledB
 @Whether the channel is enabled or disabled. Defaults to `true`.
�

privateKeyB" xThe Certificate Private Key file (ie. `.key` file).

If you choose to use __Key credentials__ you will have to provide:
u
teamIdB" eThe ID assigned to your Apple developer account team. This value is provided on the Membership page.
|
tokenKeyB" jThe `.p8` file that you download from your Apple developer account when you create an authentication key.
�

tokenKeyIdB" �The ID assigned to your signing key. To find this value, choose Certificates, IDs & Profiles, and choose your key in the Keys section.
")
applicationId" The application ID.
"�
bundleIdB" �The ID assigned to your iOS app. To find this value, choose Certificates, IDs & Profiles, choose App IDs in the Identifiers section, and choose your app.
"A
certificateB" ,The pem encoded TLS Certificate from Apple.
"�
defaultAuthenticationMethodB" �The default authentication method used for APNs.
__NOTE__: Amazon Pinpoint uses this default for every APNs push notification that you send using the console.
You can override the default when you send a message programmatically using the Amazon Pinpoint API, the AWS CLI, or an AWS SDK.
If your default authentication type fails, Amazon Pinpoint doesn't attempt to use the other authentication type.

One of the following sets of credentials is also required.

If you choose to use __Certificate credentials__ you will have to provide:
"Q
enabledB
 @Whether the channel is enabled or disabled. Defaults to `true`.
"�

privateKeyB" xThe Certificate Private Key file (ie. `.key` file).

If you choose to use __Key credentials__ you will have to provide:
"u
teamIdB" eThe ID assigned to your Apple developer account team. This value is provided on the Membership page.
"|
tokenKeyB" jThe `.p8` file that you download from your Apple developer account when you create an authentication key.
"�

tokenKeyIdB" �The ID assigned to your signing key. To find this value, choose Certificates, IDs & Profiles, and choose your key in the Keys section.
*�:
R
pinpointApnsSandboxChannel2aws:pinpoint/apnsSandboxChannel:ApnsSandboxChannel�"Provides a Pinpoint APNs Sandbox Channel resource.

> **Note:** All arguments, including certificates and tokens, will be stored in the raw state as plain-text.
## Example Usage

<!--Start PulumiCodeChooser -->
```typescript
import * as pulumi from "@pulumi/pulumi";
import * as aws from "@pulumi/aws";
import * as std from "@pulumi/std";

const app = new aws.pinpoint.App("app", {});
const apnsSandbox = new aws.pinpoint.ApnsSandboxChannel("apns_sandbox", {
    applicationId: app.applicationId,
    certificate: std.file({
        input: "./certificate.pem",
    }).then(invoke => invoke.result),
    privateKey: std.file({
        input: "./private_key.key",
    }).then(invoke => invoke.result),
});
```
```python
import pulumi
import pulumi_aws as aws
import pulumi_std as std

app = aws.pinpoint.App("app")
apns_sandbox = aws.pinpoint.ApnsSandboxChannel("apns_sandbox",
    application_id=app.application_id,
    certificate=std.file(input="./certificate.pem").result,
    private_key=std.file(input="./private_key.key").result)
```
```csharp
using System.Collections.Generic;
using System.Linq;
using Pulumi;
using Aws = Pulumi.Aws;
using Std = Pulumi.Std;

return await Deployment.RunAsync(() => 
{
    var app = new Aws.Pinpoint.App("app");

    var apnsSandbox = new Aws.Pinpoint.ApnsSandboxChannel("apns_sandbox", new()
    {
        ApplicationId = app.ApplicationId,
        Certificate = Std.File.Invoke(new()
        {
            Input = "./certificate.pem",
        }).Apply(invoke => invoke.Result),
        PrivateKey = Std.File.Invoke(new()
        {
            Input = "./private_key.key",
        }).Apply(invoke => invoke.Result),
    });

});
```
```go
package main

import (
	"github.com/pulumi/pulumi-aws/sdk/v6/go/aws/pinpoint"
	"github.com/pulumi/pulumi-std/sdk/go/std"
	"github.com/pulumi/pulumi/sdk/v3/go/pulumi"
)

func main() {
	pulumi.Run(func(ctx *pulumi.Context) error {
		app, err := pinpoint.NewApp(ctx, "app", nil)
		if err != nil {
			return err
		}
		invokeFile, err := std.File(ctx, &std.FileArgs{
			Input: "./certificate.pem",
		}, nil)
		if err != nil {
			return err
		}
		invokeFile1, err := std.File(ctx, &std.FileArgs{
			Input: "./private_key.key",
		}, nil)
		if err != nil {
			return err
		}
		_, err = pinpoint.NewApnsSandboxChannel(ctx, "apns_sandbox", &pinpoint.ApnsSandboxChannelArgs{
			ApplicationId: app.ApplicationId,
			Certificate:   pulumi.String(invokeFile.Result),
			PrivateKey:    pulumi.String(invokeFile1.Result),
		})
		if err != nil {
			return err
		}
		return nil
	})
}
```
```java
package generated_program;

import com.pulumi.Context;
import com.pulumi.Pulumi;
import com.pulumi.core.Output;
import com.pulumi.aws.pinpoint.App;
import com.pulumi.aws.pinpoint.ApnsSandboxChannel;
import com.pulumi.aws.pinpoint.ApnsSandboxChannelArgs;
import java.util.List;
import java.util.ArrayList;
import java.util.Map;
import java.io.File;
import java.nio.file.Files;
import java.nio.file.Paths;

public class App {
    public static void main(String[] args) {
        Pulumi.run(App::stack);
    }

    public static void stack(Context ctx) {
        var app = new App("app");

        var apnsSandbox = new ApnsSandboxChannel("apnsSandbox", ApnsSandboxChannelArgs.builder()
            .applicationId(app.applicationId())
            .certificate(StdFunctions.file(FileArgs.builder()
                .input("./certificate.pem")
                .build()).result())
            .privateKey(StdFunctions.file(FileArgs.builder()
                .input("./private_key.key")
                .build()).result())
            .build());

    }
}
```
```yaml
resources:
  apnsSandbox:
    type: aws:pinpoint:ApnsSandboxChannel
    name: apns_sandbox
    properties:
      applicationId: ${app.applicationId}
      certificate:
        fn::invoke:
          function: std:file
          arguments:
            input: ./certificate.pem
          return: result
      privateKey:
        fn::invoke:
          function: std:file
          arguments:
            input: ./private_key.key
          return: result
  app:
    type: aws:pinpoint:App
```
<!--End PulumiCodeChooser -->

## Import

Using `pulumi import`, import Pinpoint APNs Sandbox Channel using the `application-id`. For example:

```sh
$ pulumi import aws:pinpoint/apnsSandboxChannel:ApnsSandboxChannel apns_sandbox application-id
```
)
applicationId" The application ID.
�
bundleIdB" �The ID assigned to your iOS app. To find this value, choose Certificates, IDs & Profiles, choose App IDs in the Identifiers section, and choose your app.
A
certificateB" ,The pem encoded TLS Certificate from Apple.
�
defaultAuthenticationMethodB" �The default authentication method used for APNs Sandbox.
__NOTE__: Amazon Pinpoint uses this default for every APNs push notification that you send using the console.
You can override the default when you send a message programmatically using the Amazon Pinpoint API, the AWS CLI, or an AWS SDK.
If your default authentication type fails, Amazon Pinpoint doesn't attempt to use the other authentication type.

One of the following sets of credentials is also required.

If you choose to use __Certificate credentials__ you will have to provide:
Q
enabledB
 @Whether the channel is enabled or disabled. Defaults to `true`.
�

privateKeyB" xThe Certificate Private Key file (ie. `.key` file).

If you choose to use __Key credentials__ you will have to provide:
u
teamIdB" eThe ID assigned to your Apple developer account team. This value is provided on the Membership page.
|
tokenKeyB" jThe `.p8` file that you download from your Apple developer account when you create an authentication key.
�

tokenKeyIdB" �The ID assigned to your signing key. To find this value, choose Certificates, IDs & Profiles, and choose your key in the Keys section.
")
applicationId" The application ID.
"�
bundleIdB" �The ID assigned to your iOS app. To find this value, choose Certificates, IDs & Profiles, choose App IDs in the Identifiers section, and choose your app.
"A
certificateB" ,The pem encoded TLS Certificate from Apple.
"�
defaultAuthenticationMethodB" �The default authentication method used for APNs Sandbox.
__NOTE__: Amazon Pinpoint uses this default for every APNs push notification that you send using the console.
You can override the default when you send a message programmatically using the Amazon Pinpoint API, the AWS CLI, or an AWS SDK.
If your default authentication type fails, Amazon Pinpoint doesn't attempt to use the other authentication type.

One of the following sets of credentials is also required.

If you choose to use __Certificate credentials__ you will have to provide:
"Q
enabledB
 @Whether the channel is enabled or disabled. Defaults to `true`.
"�

privateKeyB" xThe Certificate Private Key file (ie. `.key` file).

If you choose to use __Key credentials__ you will have to provide:
"u
teamIdB" eThe ID assigned to your Apple developer account team. This value is provided on the Membership page.
"|
tokenKeyB" jThe `.p8` file that you download from your Apple developer account when you create an authentication key.
"�

tokenKeyIdB" �The ID assigned to your signing key. To find this value, choose Certificates, IDs & Profiles, and choose your key in the Keys section.
*�9
I
pinpointApnsVoipChannel,aws:pinpoint/apnsVoipChannel:ApnsVoipChannel�!Provides a Pinpoint APNs VoIP Channel resource.

> **Note:** All arguments, including certificates and tokens, will be stored in the raw state as plain-text.
## Example Usage

<!--Start PulumiCodeChooser -->
```typescript
import * as pulumi from "@pulumi/pulumi";
import * as aws from "@pulumi/aws";
import * as std from "@pulumi/std";

const app = new aws.pinpoint.App("app", {});
const apnsVoip = new aws.pinpoint.ApnsVoipChannel("apns_voip", {
    applicationId: app.applicationId,
    certificate: std.file({
        input: "./certificate.pem",
    }).then(invoke => invoke.result),
    privateKey: std.file({
        input: "./private_key.key",
    }).then(invoke => invoke.result),
});
```
```python
import pulumi
import pulumi_aws as aws
import pulumi_std as std

app = aws.pinpoint.App("app")
apns_voip = aws.pinpoint.ApnsVoipChannel("apns_voip",
    application_id=app.application_id,
    certificate=std.file(input="./certificate.pem").result,
    private_key=std.file(input="./private_key.key").result)
```
```csharp
using System.Collections.Generic;
using System.Linq;
using Pulumi;
using Aws = Pulumi.Aws;
using Std = Pulumi.Std;

return await Deployment.RunAsync(() => 
{
    var app = new Aws.Pinpoint.App("app");

    var apnsVoip = new Aws.Pinpoint.ApnsVoipChannel("apns_voip", new()
    {
        ApplicationId = app.ApplicationId,
        Certificate = Std.File.Invoke(new()
        {
            Input = "./certificate.pem",
        }).Apply(invoke => invoke.Result),
        PrivateKey = Std.File.Invoke(new()
        {
            Input = "./private_key.key",
        }).Apply(invoke => invoke.Result),
    });

});
```
```go
package main

import (
	"github.com/pulumi/pulumi-aws/sdk/v6/go/aws/pinpoint"
	"github.com/pulumi/pulumi-std/sdk/go/std"
	"github.com/pulumi/pulumi/sdk/v3/go/pulumi"
)

func main() {
	pulumi.Run(func(ctx *pulumi.Context) error {
		app, err := pinpoint.NewApp(ctx, "app", nil)
		if err != nil {
			return err
		}
		invokeFile, err := std.File(ctx, &std.FileArgs{
			Input: "./certificate.pem",
		}, nil)
		if err != nil {
			return err
		}
		invokeFile1, err := std.File(ctx, &std.FileArgs{
			Input: "./private_key.key",
		}, nil)
		if err != nil {
			return err
		}
		_, err = pinpoint.NewApnsVoipChannel(ctx, "apns_voip", &pinpoint.ApnsVoipChannelArgs{
			ApplicationId: app.ApplicationId,
			Certificate:   pulumi.String(invokeFile.Result),
			PrivateKey:    pulumi.String(invokeFile1.Result),
		})
		if err != nil {
			return err
		}
		return nil
	})
}
```
```java
package generated_program;

import com.pulumi.Context;
import com.pulumi.Pulumi;
import com.pulumi.core.Output;
import com.pulumi.aws.pinpoint.App;
import com.pulumi.aws.pinpoint.ApnsVoipChannel;
import com.pulumi.aws.pinpoint.ApnsVoipChannelArgs;
import java.util.List;
import java.util.ArrayList;
import java.util.Map;
import java.io.File;
import java.nio.file.Files;
import java.nio.file.Paths;

public class App {
    public static void main(String[] args) {
        Pulumi.run(App::stack);
    }

    public static void stack(Context ctx) {
        var app = new App("app");

        var apnsVoip = new ApnsVoipChannel("apnsVoip", ApnsVoipChannelArgs.builder()
            .applicationId(app.applicationId())
            .certificate(StdFunctions.file(FileArgs.builder()
                .input("./certificate.pem")
                .build()).result())
            .privateKey(StdFunctions.file(FileArgs.builder()
                .input("./private_key.key")
                .build()).result())
            .build());

    }
}
```
```yaml
resources:
  apnsVoip:
    type: aws:pinpoint:ApnsVoipChannel
    name: apns_voip
    properties:
      applicationId: ${app.applicationId}
      certificate:
        fn::invoke:
          function: std:file
          arguments:
            input: ./certificate.pem
          return: result
      privateKey:
        fn::invoke:
          function: std:file
          arguments:
            input: ./private_key.key
          return: result
  app:
    type: aws:pinpoint:App
```
<!--End PulumiCodeChooser -->

## Import

Using `pulumi import`, import Pinpoint APNs VoIP Channel using the `application-id`. For example:

```sh
$ pulumi import aws:pinpoint/apnsVoipChannel:ApnsVoipChannel apns_voip application-id
```
)
applicationId" The application ID.
�
bundleIdB" �The ID assigned to your iOS app. To find this value, choose Certificates, IDs & Profiles, choose App IDs in the Identifiers section, and choose your app.
A
certificateB" ,The pem encoded TLS Certificate from Apple.
�
defaultAuthenticationMethodB" �The default authentication method used for APNs.
__NOTE__: Amazon Pinpoint uses this default for every APNs push notification that you send using the console.
You can override the default when you send a message programmatically using the Amazon Pinpoint API, the AWS CLI, or an AWS SDK.
If your default authentication type fails, Amazon Pinpoint doesn't attempt to use the other authentication type.

One of the following sets of credentials is also required.

If you choose to use __Certificate credentials__ you will have to provide:
Q
enabledB
 @Whether the channel is enabled or disabled. Defaults to `true`.
�

privateKeyB" xThe Certificate Private Key file (ie. `.key` file).

If you choose to use __Key credentials__ you will have to provide:
u
teamIdB" eThe ID assigned to your Apple developer account team. This value is provided on the Membership page.
|
tokenKeyB" jThe `.p8` file that you download from your Apple developer account when you create an authentication key.
�

tokenKeyIdB" �The ID assigned to your signing key. To find this value, choose Certificates, IDs & Profiles, and choose your key in the Keys section.
")
applicationId" The application ID.
"�
bundleIdB" �The ID assigned to your iOS app. To find this value, choose Certificates, IDs & Profiles, choose App IDs in the Identifiers section, and choose your app.
"A
certificateB" ,The pem encoded TLS Certificate from Apple.
"�
defaultAuthenticationMethodB" �The default authentication method used for APNs.
__NOTE__: Amazon Pinpoint uses this default for every APNs push notification that you send using the console.
You can override the default when you send a message programmatically using the Amazon Pinpoint API, the AWS CLI, or an AWS SDK.
If your default authentication type fails, Amazon Pinpoint doesn't attempt to use the other authentication type.

One of the following sets of credentials is also required.

If you choose to use __Certificate credentials__ you will have to provide:
"Q
enabledB
 @Whether the channel is enabled or disabled. Defaults to `true`.
"�

privateKeyB" xThe Certificate Private Key file (ie. `.key` file).

If you choose to use __Key credentials__ you will have to provide:
"u
teamIdB" eThe ID assigned to your Apple developer account team. This value is provided on the Membership page.
"|
tokenKeyB" jThe `.p8` file that you download from your Apple developer account when you create an authentication key.
"�

tokenKeyIdB" �The ID assigned to your signing key. To find this value, choose Certificates, IDs & Profiles, and choose your key in the Keys section.
*�;
^
pinpointApnsVoipSandboxChannel:aws:pinpoint/apnsVoipSandboxChannel:ApnsVoipSandboxChannel�"Provides a Pinpoint APNs VoIP Sandbox Channel resource.

> **Note:** All arguments, including certificates and tokens, will be stored in the raw state as plain-text.
## Example Usage

<!--Start PulumiCodeChooser -->
```typescript
import * as pulumi from "@pulumi/pulumi";
import * as aws from "@pulumi/aws";
import * as std from "@pulumi/std";

const app = new aws.pinpoint.App("app", {});
const apnsVoipSandbox = new aws.pinpoint.ApnsVoipSandboxChannel("apns_voip_sandbox", {
    applicationId: app.applicationId,
    certificate: std.file({
        input: "./certificate.pem",
    }).then(invoke => invoke.result),
    privateKey: std.file({
        input: "./private_key.key",
    }).then(invoke => invoke.result),
});
```
```python
import pulumi
import pulumi_aws as aws
import pulumi_std as std

app = aws.pinpoint.App("app")
apns_voip_sandbox = aws.pinpoint.ApnsVoipSandboxChannel("apns_voip_sandbox",
    application_id=app.application_id,
    certificate=std.file(input="./certificate.pem").result,
    private_key=std.file(input="./private_key.key").result)
```
```csharp
using System.Collections.Generic;
using System.Linq;
using Pulumi;
using Aws = Pulumi.Aws;
using Std = Pulumi.Std;

return await Deployment.RunAsync(() => 
{
    var app = new Aws.Pinpoint.App("app");

    var apnsVoipSandbox = new Aws.Pinpoint.ApnsVoipSandboxChannel("apns_voip_sandbox", new()
    {
        ApplicationId = app.ApplicationId,
        Certificate = Std.File.Invoke(new()
        {
            Input = "./certificate.pem",
        }).Apply(invoke => invoke.Result),
        PrivateKey = Std.File.Invoke(new()
        {
            Input = "./private_key.key",
        }).Apply(invoke => invoke.Result),
    });

});
```
```go
package main

import (
	"github.com/pulumi/pulumi-aws/sdk/v6/go/aws/pinpoint"
	"github.com/pulumi/pulumi-std/sdk/go/std"
	"github.com/pulumi/pulumi/sdk/v3/go/pulumi"
)

func main() {
	pulumi.Run(func(ctx *pulumi.Context) error {
		app, err := pinpoint.NewApp(ctx, "app", nil)
		if err != nil {
			return err
		}
		invokeFile, err := std.File(ctx, &std.FileArgs{
			Input: "./certificate.pem",
		}, nil)
		if err != nil {
			return err
		}
		invokeFile1, err := std.File(ctx, &std.FileArgs{
			Input: "./private_key.key",
		}, nil)
		if err != nil {
			return err
		}
		_, err = pinpoint.NewApnsVoipSandboxChannel(ctx, "apns_voip_sandbox", &pinpoint.ApnsVoipSandboxChannelArgs{
			ApplicationId: app.ApplicationId,
			Certificate:   pulumi.String(invokeFile.Result),
			PrivateKey:    pulumi.String(invokeFile1.Result),
		})
		if err != nil {
			return err
		}
		return nil
	})
}
```
```java
package generated_program;

import com.pulumi.Context;
import com.pulumi.Pulumi;
import com.pulumi.core.Output;
import com.pulumi.aws.pinpoint.App;
import com.pulumi.aws.pinpoint.ApnsVoipSandboxChannel;
import com.pulumi.aws.pinpoint.ApnsVoipSandboxChannelArgs;
import java.util.List;
import java.util.ArrayList;
import java.util.Map;
import java.io.File;
import java.nio.file.Files;
import java.nio.file.Paths;

public class App {
    public static void main(String[] args) {
        Pulumi.run(App::stack);
    }

    public static void stack(Context ctx) {
        var app = new App("app");

        var apnsVoipSandbox = new ApnsVoipSandboxChannel("apnsVoipSandbox", ApnsVoipSandboxChannelArgs.builder()
            .applicationId(app.applicationId())
            .certificate(StdFunctions.file(FileArgs.builder()
                .input("./certificate.pem")
                .build()).result())
            .privateKey(StdFunctions.file(FileArgs.builder()
                .input("./private_key.key")
                .build()).result())
            .build());

    }
}
```
```yaml
resources:
  apnsVoipSandbox:
    type: aws:pinpoint:ApnsVoipSandboxChannel
    name: apns_voip_sandbox
    properties:
      applicationId: ${app.applicationId}
      certificate:
        fn::invoke:
          function: std:file
          arguments:
            input: ./certificate.pem
          return: result
      privateKey:
        fn::invoke:
          function: std:file
          arguments:
            input: ./private_key.key
          return: result
  app:
    type: aws:pinpoint:App
```
<!--End PulumiCodeChooser -->

## Import

Using `pulumi import`, import Pinpoint APNs VoIP Sandbox Channel using the `application-id`. For example:

```sh
$ pulumi import aws:pinpoint/apnsVoipSandboxChannel:ApnsVoipSandboxChannel apns_voip_sandbox application-id
```
)
applicationId" The application ID.
�
bundleIdB" �The ID assigned to your iOS app. To find this value, choose Certificates, IDs & Profiles, choose App IDs in the Identifiers section, and choose your app.
A
certificateB" ,The pem encoded TLS Certificate from Apple.
�
defaultAuthenticationMethodB" �The default authentication method used for APNs.
__NOTE__: Amazon Pinpoint uses this default for every APNs push notification that you send using the console.
You can override the default when you send a message programmatically using the Amazon Pinpoint API, the AWS CLI, or an AWS SDK.
If your default authentication type fails, Amazon Pinpoint doesn't attempt to use the other authentication type.

One of the following sets of credentials is also required.

If you choose to use __Certificate credentials__ you will have to provide:
Q
enabledB
 @Whether the channel is enabled or disabled. Defaults to `true`.
�

privateKeyB" xThe Certificate Private Key file (ie. `.key` file).

If you choose to use __Key credentials__ you will have to provide:
u
teamIdB" eThe ID assigned to your Apple developer account team. This value is provided on the Membership page.
|
tokenKeyB" jThe `.p8` file that you download from your Apple developer account when you create an authentication key.
�

tokenKeyIdB" �The ID assigned to your signing key. To find this value, choose Certificates, IDs & Profiles, and choose your key in the Keys section.
")
applicationId" The application ID.
"�
bundleIdB" �The ID assigned to your iOS app. To find this value, choose Certificates, IDs & Profiles, choose App IDs in the Identifiers section, and choose your app.
"A
certificateB" ,The pem encoded TLS Certificate from Apple.
"�
defaultAuthenticationMethodB" �The default authentication method used for APNs.
__NOTE__: Amazon Pinpoint uses this default for every APNs push notification that you send using the console.
You can override the default when you send a message programmatically using the Amazon Pinpoint API, the AWS CLI, or an AWS SDK.
If your default authentication type fails, Amazon Pinpoint doesn't attempt to use the other authentication type.

One of the following sets of credentials is also required.

If you choose to use __Certificate credentials__ you will have to provide:
"Q
enabledB
 @Whether the channel is enabled or disabled. Defaults to `true`.
"�

privateKeyB" xThe Certificate Private Key file (ie. `.key` file).

If you choose to use __Key credentials__ you will have to provide:
"u
teamIdB" eThe ID assigned to your Apple developer account team. This value is provided on the Membership page.
"|
tokenKeyB" jThe `.p8` file that you download from your Apple developer account when you create an authentication key.
"�

tokenKeyIdB" �The ID assigned to your signing key. To find this value, choose Certificates, IDs & Profiles, and choose your key in the Keys section.
*�*
%
pinpointAppaws:pinpoint/app:App�Provides a Pinpoint App resource.

## Example Usage

<!--Start PulumiCodeChooser -->
```typescript
import * as pulumi from "@pulumi/pulumi";
import * as aws from "@pulumi/aws";

const example = new aws.pinpoint.App("example", {
    name: "test-app",
    limits: {
        maximumDuration: 600,
    },
    quietTime: {
        start: "00:00",
        end: "06:00",
    },
});
```
```python
import pulumi
import pulumi_aws as aws

example = aws.pinpoint.App("example",
    name="test-app",
    limits={
        "maximum_duration": 600,
    },
    quiet_time={
        "start": "00:00",
        "end": "06:00",
    })
```
```csharp
using System.Collections.Generic;
using System.Linq;
using Pulumi;
using Aws = Pulumi.Aws;

return await Deployment.RunAsync(() => 
{
    var example = new Aws.Pinpoint.App("example", new()
    {
        Name = "test-app",
        Limits = new Aws.Pinpoint.Inputs.AppLimitsArgs
        {
            MaximumDuration = 600,
        },
        QuietTime = new Aws.Pinpoint.Inputs.AppQuietTimeArgs
        {
            Start = "00:00",
            End = "06:00",
        },
    });

});
```
```go
package main

import (
	"github.com/pulumi/pulumi-aws/sdk/v6/go/aws/pinpoint"
	"github.com/pulumi/pulumi/sdk/v3/go/pulumi"
)

func main() {
	pulumi.Run(func(ctx *pulumi.Context) error {
		_, err := pinpoint.NewApp(ctx, "example", &pinpoint.AppArgs{
			Name: pulumi.String("test-app"),
			Limits: &pinpoint.AppLimitsArgs{
				MaximumDuration: pulumi.Int(600),
			},
			QuietTime: &pinpoint.AppQuietTimeArgs{
				Start: pulumi.String("00:00"),
				End:   pulumi.String("06:00"),
			},
		})
		if err != nil {
			return err
		}
		return nil
	})
}
```
```java
package generated_program;

import com.pulumi.Context;
import com.pulumi.Pulumi;
import com.pulumi.core.Output;
import com.pulumi.aws.pinpoint.App;
import com.pulumi.aws.pinpoint.AppArgs;
import com.pulumi.aws.pinpoint.inputs.AppLimitsArgs;
import com.pulumi.aws.pinpoint.inputs.AppQuietTimeArgs;
import java.util.List;
import java.util.ArrayList;
import java.util.Map;
import java.io.File;
import java.nio.file.Files;
import java.nio.file.Paths;

public class App {
    public static void main(String[] args) {
        Pulumi.run(App::stack);
    }

    public static void stack(Context ctx) {
        var example = new App("example", AppArgs.builder()
            .name("test-app")
            .limits(AppLimitsArgs.builder()
                .maximumDuration(600)
                .build())
            .quietTime(AppQuietTimeArgs.builder()
                .start("00:00")
                .end("06:00")
                .build())
            .build());

    }
}
```
```yaml
resources:
  example:
    type: aws:pinpoint:App
    properties:
      name: test-app
      limits:
        maximumDuration: 600
      quietTime:
        start: 00:00
        end: 06:00
```
<!--End PulumiCodeChooser -->

## Import

Using `pulumi import`, import Pinpoint App using the `application-id`. For example:

```sh
$ pulumi import aws:pinpoint/app:App name application-id
```
�
campaignHookOBM:K
I
pinpointAppCampaignHook,aws:pinpoint/AppCampaignHook:AppCampaignHook`Specifies settings for invoking an AWS Lambda function that customizes a segment for a campaign
�
limits=B;:9
7
pinpoint	AppLimits aws:pinpoint/AppLimits:AppLimits�The default campaign limits for the app. These limits apply to each campaign for the app, unless the campaign overrides the default with limits of its own
C
nameB" 5The application name. By default generated by Pulumi
P

namePrefixB" <The name of the Pinpoint application. Conflicts with `name`
�
	quietTimeFBD:B
@
pinpointAppQuietTime&aws:pinpoint/AppQuietTime:AppQuietTime�The default quiet time for the app. Each campaign for this app sends no messages during this time unless the campaign overrides the default with a quiet time of its own
�
tagsB2" �Key-value map of resource tags. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
"=
applicationId" (The Application ID of the Pinpoint App.
"B
arn" 7Amazon Resource Name (ARN) of the PinPoint Application
"�
campaignHookOBM:K
I
pinpointAppCampaignHook,aws:pinpoint/AppCampaignHook:AppCampaignHook`Specifies settings for invoking an AWS Lambda function that customizes a segment for a campaign
"�
limits=B;:9
7
pinpoint	AppLimits aws:pinpoint/AppLimits:AppLimits�The default campaign limits for the app. These limits apply to each campaign for the app, unless the campaign overrides the default with limits of its own
"A
name" 5The application name. By default generated by Pulumi
"N

namePrefix" <The name of the Pinpoint application. Conflicts with `name`
"�
	quietTimeFBD:B
@
pinpointAppQuietTime&aws:pinpoint/AppQuietTime:AppQuietTime�The default quiet time for the app. Each campaign for this app sends no messages during this time unless the campaign overrides the default with a quiet time of its own
"�
tagsB2" �Key-value map of resource tags. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
"�
tagsAll2" xA map of tags assigned to the resource, including those inherited from the provider `default_tags` configuration block.
*�
@
pinpointBaiduChannel&aws:pinpoint/baiduChannel:BaiduChannel�Provides a Pinpoint Baidu Channel resource.

> **Note:** All arguments including the Api Key and Secret Key will be stored in the raw state as plain-text.
## Example Usage

<!--Start PulumiCodeChooser -->
```typescript
import * as pulumi from "@pulumi/pulumi";
import * as aws from "@pulumi/aws";

const app = new aws.pinpoint.App("app", {});
const channel = new aws.pinpoint.BaiduChannel("channel", {
    applicationId: app.applicationId,
    apiKey: "",
    secretKey: "",
});
```
```python
import pulumi
import pulumi_aws as aws

app = aws.pinpoint.App("app")
channel = aws.pinpoint.BaiduChannel("channel",
    application_id=app.application_id,
    api_key="",
    secret_key="")
```
```csharp
using System.Collections.Generic;
using System.Linq;
using Pulumi;
using Aws = Pulumi.Aws;

return await Deployment.RunAsync(() => 
{
    var app = new Aws.Pinpoint.App("app");

    var channel = new Aws.Pinpoint.BaiduChannel("channel", new()
    {
        ApplicationId = app.ApplicationId,
        ApiKey = "",
        SecretKey = "",
    });

});
```
```go
package main

import (
	"github.com/pulumi/pulumi-aws/sdk/v6/go/aws/pinpoint"
	"github.com/pulumi/pulumi/sdk/v3/go/pulumi"
)

func main() {
	pulumi.Run(func(ctx *pulumi.Context) error {
		app, err := pinpoint.NewApp(ctx, "app", nil)
		if err != nil {
			return err
		}
		_, err = pinpoint.NewBaiduChannel(ctx, "channel", &pinpoint.BaiduChannelArgs{
			ApplicationId: app.ApplicationId,
			ApiKey:        pulumi.String(""),
			SecretKey:     pulumi.String(""),
		})
		if err != nil {
			return err
		}
		return nil
	})
}
```
```java
package generated_program;

import com.pulumi.Context;
import com.pulumi.Pulumi;
import com.pulumi.core.Output;
import com.pulumi.aws.pinpoint.App;
import com.pulumi.aws.pinpoint.BaiduChannel;
import com.pulumi.aws.pinpoint.BaiduChannelArgs;
import java.util.List;
import java.util.ArrayList;
import java.util.Map;
import java.io.File;
import java.nio.file.Files;
import java.nio.file.Paths;

public class App {
    public static void main(String[] args) {
        Pulumi.run(App::stack);
    }

    public static void stack(Context ctx) {
        var app = new App("app");

        var channel = new BaiduChannel("channel", BaiduChannelArgs.builder()
            .applicationId(app.applicationId())
            .apiKey("")
            .secretKey("")
            .build());

    }
}
```
```yaml
resources:
  app:
    type: aws:pinpoint:App
  channel:
    type: aws:pinpoint:BaiduChannel
    properties:
      applicationId: ${app.applicationId}
      apiKey: ""
      secretKey: ""
```
<!--End PulumiCodeChooser -->

## Import

Using `pulumi import`, import Pinpoint Baidu Channel using the `application-id`. For example:

```sh
$ pulumi import aws:pinpoint/baiduChannel:BaiduChannel channel application-id
```
6
apiKey" (Platform credential API key from Baidu.
)
applicationId" The application ID.
N
enabledB
 =Specifies whether to enable the channel. Defaults to `true`.
<
	secretKey" +Platform credential Secret key from Baidu.
"6
apiKey" (Platform credential API key from Baidu.
")
applicationId" The application ID.
"N
enabledB
 =Specifies whether to enable the channel. Defaults to `true`.
"<
	secretKey" +Platform credential Secret key from Baidu.
*�]
@
pinpointEmailChannel&aws:pinpoint/emailChannel:EmailChannel�QProvides a Pinpoint Email Channel resource.

## Example Usage

<!--Start PulumiCodeChooser -->
```typescript
import * as pulumi from "@pulumi/pulumi";
import * as aws from "@pulumi/aws";

const app = new aws.pinpoint.App("app", {});
const assumeRole = aws.iam.getPolicyDocument({
    statements: [{
        effect: "Allow",
        principals: [{
            type: "Service",
            identifiers: ["pinpoint.amazonaws.com"],
        }],
        actions: ["sts:AssumeRole"],
    }],
});
const role = new aws.iam.Role("role", {assumeRolePolicy: assumeRole.then(assumeRole => assumeRole.json)});
const email = new aws.pinpoint.EmailChannel("email", {
    applicationId: app.applicationId,
    fromAddress: "user@example.com",
    roleArn: role.arn,
});
const identity = new aws.ses.DomainIdentity("identity", {domain: "example.com"});
const rolePolicy = aws.iam.getPolicyDocument({
    statements: [{
        effect: "Allow",
        actions: [
            "mobileanalytics:PutEvents",
            "mobileanalytics:PutItems",
        ],
        resources: ["*"],
    }],
});
const rolePolicyRolePolicy = new aws.iam.RolePolicy("role_policy", {
    name: "role_policy",
    role: role.id,
    policy: rolePolicy.then(rolePolicy => rolePolicy.json),
});
```
```python
import pulumi
import pulumi_aws as aws

app = aws.pinpoint.App("app")
assume_role = aws.iam.get_policy_document(statements=[{
    "effect": "Allow",
    "principals": [{
        "type": "Service",
        "identifiers": ["pinpoint.amazonaws.com"],
    }],
    "actions": ["sts:AssumeRole"],
}])
role = aws.iam.Role("role", assume_role_policy=assume_role.json)
email = aws.pinpoint.EmailChannel("email",
    application_id=app.application_id,
    from_address="user@example.com",
    role_arn=role.arn)
identity = aws.ses.DomainIdentity("identity", domain="example.com")
role_policy = aws.iam.get_policy_document(statements=[{
    "effect": "Allow",
    "actions": [
        "mobileanalytics:PutEvents",
        "mobileanalytics:PutItems",
    ],
    "resources": ["*"],
}])
role_policy_role_policy = aws.iam.RolePolicy("role_policy",
    name="role_policy",
    role=role.id,
    policy=role_policy.json)
```
```csharp
using System.Collections.Generic;
using System.Linq;
using Pulumi;
using Aws = Pulumi.Aws;

return await Deployment.RunAsync(() => 
{
    var app = new Aws.Pinpoint.App("app");

    var assumeRole = Aws.Iam.GetPolicyDocument.Invoke(new()
    {
        Statements = new[]
        {
            new Aws.Iam.Inputs.GetPolicyDocumentStatementInputArgs
            {
                Effect = "Allow",
                Principals = new[]
                {
                    new Aws.Iam.Inputs.GetPolicyDocumentStatementPrincipalInputArgs
                    {
                        Type = "Service",
                        Identifiers = new[]
                        {
                            "pinpoint.amazonaws.com",
                        },
                    },
                },
                Actions = new[]
                {
                    "sts:AssumeRole",
                },
            },
        },
    });

    var role = new Aws.Iam.Role("role", new()
    {
        AssumeRolePolicy = assumeRole.Apply(getPolicyDocumentResult => getPolicyDocumentResult.Json),
    });

    var email = new Aws.Pinpoint.EmailChannel("email", new()
    {
        ApplicationId = app.ApplicationId,
        FromAddress = "user@example.com",
        RoleArn = role.Arn,
    });

    var identity = new Aws.Ses.DomainIdentity("identity", new()
    {
        Domain = "example.com",
    });

    var rolePolicy = Aws.Iam.GetPolicyDocument.Invoke(new()
    {
        Statements = new[]
        {
            new Aws.Iam.Inputs.GetPolicyDocumentStatementInputArgs
            {
                Effect = "Allow",
                Actions = new[]
                {
                    "mobileanalytics:PutEvents",
                    "mobileanalytics:PutItems",
                },
                Resources = new[]
                {
                    "*",
                },
            },
        },
    });

    var rolePolicyRolePolicy = new Aws.Iam.RolePolicy("role_policy", new()
    {
        Name = "role_policy",
        Role = role.Id,
        Policy = rolePolicy.Apply(getPolicyDocumentResult => getPolicyDocumentResult.Json),
    });

});
```
```go
package main

import (
	"github.com/pulumi/pulumi-aws/sdk/v6/go/aws/iam"
	"github.com/pulumi/pulumi-aws/sdk/v6/go/aws/pinpoint"
	"github.com/pulumi/pulumi-aws/sdk/v6/go/aws/ses"
	"github.com/pulumi/pulumi/sdk/v3/go/pulumi"
)

func main() {
	pulumi.Run(func(ctx *pulumi.Context) error {
		app, err := pinpoint.NewApp(ctx, "app", nil)
		if err != nil {
			return err
		}
		assumeRole, err := iam.GetPolicyDocument(ctx, &iam.GetPolicyDocumentArgs{
			Statements: []iam.GetPolicyDocumentStatement{
				{
					Effect: pulumi.StringRef("Allow"),
					Principals: []iam.GetPolicyDocumentStatementPrincipal{
						{
							Type: "Service",
							Identifiers: []string{
								"pinpoint.amazonaws.com",
							},
						},
					},
					Actions: []string{
						"sts:AssumeRole",
					},
				},
			},
		}, nil)
		if err != nil {
			return err
		}
		role, err := iam.NewRole(ctx, "role", &iam.RoleArgs{
			AssumeRolePolicy: pulumi.String(assumeRole.Json),
		})
		if err != nil {
			return err
		}
		_, err = pinpoint.NewEmailChannel(ctx, "email", &pinpoint.EmailChannelArgs{
			ApplicationId: app.ApplicationId,
			FromAddress:   pulumi.String("user@example.com"),
			RoleArn:       role.Arn,
		})
		if err != nil {
			return err
		}
		_, err = ses.NewDomainIdentity(ctx, "identity", &ses.DomainIdentityArgs{
			Domain: pulumi.String("example.com"),
		})
		if err != nil {
			return err
		}
		rolePolicy, err := iam.GetPolicyDocument(ctx, &iam.GetPolicyDocumentArgs{
			Statements: []iam.GetPolicyDocumentStatement{
				{
					Effect: pulumi.StringRef("Allow"),
					Actions: []string{
						"mobileanalytics:PutEvents",
						"mobileanalytics:PutItems",
					},
					Resources: []string{
						"*",
					},
				},
			},
		}, nil)
		if err != nil {
			return err
		}
		_, err = iam.NewRolePolicy(ctx, "role_policy", &iam.RolePolicyArgs{
			Name:   pulumi.String("role_policy"),
			Role:   role.ID(),
			Policy: pulumi.String(rolePolicy.Json),
		})
		if err != nil {
			return err
		}
		return nil
	})
}
```
```java
package generated_program;

import com.pulumi.Context;
import com.pulumi.Pulumi;
import com.pulumi.core.Output;
import com.pulumi.aws.pinpoint.App;
import com.pulumi.aws.iam.IamFunctions;
import com.pulumi.aws.iam.inputs.GetPolicyDocumentArgs;
import com.pulumi.aws.iam.Role;
import com.pulumi.aws.iam.RoleArgs;
import com.pulumi.aws.pinpoint.EmailChannel;
import com.pulumi.aws.pinpoint.EmailChannelArgs;
import com.pulumi.aws.ses.DomainIdentity;
import com.pulumi.aws.ses.DomainIdentityArgs;
import com.pulumi.aws.iam.RolePolicy;
import com.pulumi.aws.iam.RolePolicyArgs;
import java.util.List;
import java.util.ArrayList;
import java.util.Map;
import java.io.File;
import java.nio.file.Files;
import java.nio.file.Paths;

public class App {
    public static void main(String[] args) {
        Pulumi.run(App::stack);
    }

    public static void stack(Context ctx) {
        var app = new App("app");

        final var assumeRole = IamFunctions.getPolicyDocument(GetPolicyDocumentArgs.builder()
            .statements(GetPolicyDocumentStatementArgs.builder()
                .effect("Allow")
                .principals(GetPolicyDocumentStatementPrincipalArgs.builder()
                    .type("Service")
                    .identifiers("pinpoint.amazonaws.com")
                    .build())
                .actions("sts:AssumeRole")
                .build())
            .build());

        var role = new Role("role", RoleArgs.builder()
            .assumeRolePolicy(assumeRole.applyValue(getPolicyDocumentResult -> getPolicyDocumentResult.json()))
            .build());

        var email = new EmailChannel("email", EmailChannelArgs.builder()
            .applicationId(app.applicationId())
            .fromAddress("user@example.com")
            .roleArn(role.arn())
            .build());

        var identity = new DomainIdentity("identity", DomainIdentityArgs.builder()
            .domain("example.com")
            .build());

        final var rolePolicy = IamFunctions.getPolicyDocument(GetPolicyDocumentArgs.builder()
            .statements(GetPolicyDocumentStatementArgs.builder()
                .effect("Allow")
                .actions(                
                    "mobileanalytics:PutEvents",
                    "mobileanalytics:PutItems")
                .resources("*")
                .build())
            .build());

        var rolePolicyRolePolicy = new RolePolicy("rolePolicyRolePolicy", RolePolicyArgs.builder()
            .name("role_policy")
            .role(role.id())
            .policy(rolePolicy.applyValue(getPolicyDocumentResult -> getPolicyDocumentResult.json()))
            .build());

    }
}
```
```yaml
resources:
  email:
    type: aws:pinpoint:EmailChannel
    properties:
      applicationId: ${app.applicationId}
      fromAddress: user@example.com
      roleArn: ${role.arn}
  app:
    type: aws:pinpoint:App
  identity:
    type: aws:ses:DomainIdentity
    properties:
      domain: example.com
  role:
    type: aws:iam:Role
    properties:
      assumeRolePolicy: ${assumeRole.json}
  rolePolicyRolePolicy:
    type: aws:iam:RolePolicy
    name: role_policy
    properties:
      name: role_policy
      role: ${role.id}
      policy: ${rolePolicy.json}
variables:
  assumeRole:
    fn::invoke:
      function: aws:iam:getPolicyDocument
      arguments:
        statements:
          - effect: Allow
            principals:
              - type: Service
                identifiers:
                  - pinpoint.amazonaws.com
            actions:
              - sts:AssumeRole
  rolePolicy:
    fn::invoke:
      function: aws:iam:getPolicyDocument
      arguments:
        statements:
          - effect: Allow
            actions:
              - mobileanalytics:PutEvents
              - mobileanalytics:PutItems
            resources:
              - '*'
```
<!--End PulumiCodeChooser -->

## Import

Using `pulumi import`, import Pinpoint Email Channel using the `application-id`. For example:

```sh
$ pulumi import aws:pinpoint/emailChannel:EmailChannel email application-id
```
)
applicationId" The application ID.
�
configurationSetB" rThe ARN of the Amazon SES configuration set that you want to apply to messages that you send through the channel.
Q
enabledB
 @Whether the channel is enabled or disabled. Defaults to `true`.
�
fromAddress" �The email address used to send emails from. You can use email only (`user@example.com`) or friendly address (`User <user@example.com>`). This field comply with [RFC 5322](https://www.ietf.org/rfc/rfc5322.txt).
:
identity" *The ARN of an identity verified with SES.
y
roleArnB" h*Deprecated* The ARN of an IAM Role used to submit events to Mobile Analytics' event ingestion service.
")
applicationId" The application ID.
"�
configurationSetB" rThe ARN of the Amazon SES configuration set that you want to apply to messages that you send through the channel.
"Q
enabledB
 @Whether the channel is enabled or disabled. Defaults to `true`.
"�
fromAddress" �The email address used to send emails from. You can use email only (`user@example.com`) or friendly address (`User <user@example.com>`). This field comply with [RFC 5322](https://www.ietf.org/rfc/rfc5322.txt).
":
identity" *The ARN of an identity verified with SES.
"?
messagesPerSecond &Messages per second that can be sent.
"y
roleArnB" h*Deprecated* The ARN of an IAM Role used to submit events to Mobile Analytics' event ingestion service.
*�*
C
pinpointEmailTemplate(aws:pinpoint/emailTemplate:EmailTemplate� Provides a Pinpoint Email Template resource

## Example Usage

<!--Start PulumiCodeChooser -->
```typescript
import * as pulumi from "@pulumi/pulumi";
import * as aws from "@pulumi/aws";

const test = new aws.pinpoint.EmailTemplate("test", {
    templateName: "testing",
    emailTemplates: [{
        subject: "testing",
        textPart: "we are testing template text part",
        headers: [{
            name: "testingname",
            value: "testingvalue",
        }],
    }],
});
```
```python
import pulumi
import pulumi_aws as aws

test = aws.pinpoint.EmailTemplate("test",
    template_name="testing",
    email_templates=[{
        "subject": "testing",
        "text_part": "we are testing template text part",
        "headers": [{
            "name": "testingname",
            "value": "testingvalue",
        }],
    }])
```
```csharp
using System.Collections.Generic;
using System.Linq;
using Pulumi;
using Aws = Pulumi.Aws;

return await Deployment.RunAsync(() => 
{
    var test = new Aws.Pinpoint.EmailTemplate("test", new()
    {
        TemplateName = "testing",
        EmailTemplates = new[]
        {
            new Aws.Pinpoint.Inputs.EmailTemplateEmailTemplateArgs
            {
                Subject = "testing",
                TextPart = "we are testing template text part",
                Headers = new[]
                {
                    new Aws.Pinpoint.Inputs.EmailTemplateEmailTemplateHeaderArgs
                    {
                        Name = "testingname",
                        Value = "testingvalue",
                    },
                },
            },
        },
    });

});
```
```go
package main

import (
	"github.com/pulumi/pulumi-aws/sdk/v6/go/aws/pinpoint"
	"github.com/pulumi/pulumi/sdk/v3/go/pulumi"
)

func main() {
	pulumi.Run(func(ctx *pulumi.Context) error {
		_, err := pinpoint.NewEmailTemplate(ctx, "test", &pinpoint.EmailTemplateArgs{
			TemplateName: pulumi.String("testing"),
			EmailTemplates: pinpoint.EmailTemplateEmailTemplateArray{
				&pinpoint.EmailTemplateEmailTemplateArgs{
					Subject:  pulumi.String("testing"),
					TextPart: pulumi.String("we are testing template text part"),
					Headers: pinpoint.EmailTemplateEmailTemplateHeaderArray{
						&pinpoint.EmailTemplateEmailTemplateHeaderArgs{
							Name:  pulumi.String("testingname"),
							Value: pulumi.String("testingvalue"),
						},
					},
				},
			},
		})
		if err != nil {
			return err
		}
		return nil
	})
}
```
```java
package generated_program;

import com.pulumi.Context;
import com.pulumi.Pulumi;
import com.pulumi.core.Output;
import com.pulumi.aws.pinpoint.EmailTemplate;
import com.pulumi.aws.pinpoint.EmailTemplateArgs;
import com.pulumi.aws.pinpoint.inputs.EmailTemplateEmailTemplateArgs;
import java.util.List;
import java.util.ArrayList;
import java.util.Map;
import java.io.File;
import java.nio.file.Files;
import java.nio.file.Paths;

public class App {
    public static void main(String[] args) {
        Pulumi.run(App::stack);
    }

    public static void stack(Context ctx) {
        var test = new EmailTemplate("test", EmailTemplateArgs.builder()
            .templateName("testing")
            .emailTemplates(EmailTemplateEmailTemplateArgs.builder()
                .subject("testing")
                .textPart("we are testing template text part")
                .headers(EmailTemplateEmailTemplateHeaderArgs.builder()
                    .name("testingname")
                    .value("testingvalue")
                    .build())
                .build())
            .build());

    }
}
```
```yaml
resources:
  test:
    type: aws:pinpoint:EmailTemplate
    properties:
      templateName: testing
      emailTemplates:
        - subject: testing
          textPart: we are testing template text part
          headers:
            - name: testingname
              value: testingvalue
```
<!--End PulumiCodeChooser -->

## Import

Using `pulumi import`, import Pinpoint Email Template using the `template_name`. For example:

```sh
$ pulumi import aws:pinpoint/emailTemplate:EmailTemplate reset template_name
```
�
emailTemplatesrBp*n:l
j
pinpointEmailTemplateEmailTemplateBaws:pinpoint/EmailTemplateEmailTemplate:EmailTemplateEmailTemplate�Specifies the content and settings for a message template that can be used in messages that are sent through the email channel. See Email Template

tagsB2" �
templateName" �name of the message template. A template name must start with an alphanumeric character and can contain a maximum of 128 characters. The characters can be alphanumeric characters, underscores (_), or hyphens (-). Template names are case sensitive.
"?
arn" 4Amazon Resource Name (ARN) of the message template.
"�
emailTemplatesrBp*n:l
j
pinpointEmailTemplateEmailTemplateBaws:pinpoint/EmailTemplateEmailTemplate:EmailTemplateEmailTemplate�Specifies the content and settings for a message template that can be used in messages that are sent through the email channel. See Email Template
"
tagsB2" "
tagsAll2" "�
templateName" �name of the message template. A template name must start with an alphanumeric character and can contain a maximum of 128 characters. The characters can be alphanumeric characters, underscores (_), or hyphens (-). Template names are case sensitive.
*�[
=
pinpointEventStream$aws:pinpoint/eventStream:EventStream�VProvides a Pinpoint Event Stream resource.

## Example Usage

<!--Start PulumiCodeChooser -->
```typescript
import * as pulumi from "@pulumi/pulumi";
import * as aws from "@pulumi/aws";

const app = new aws.pinpoint.App("app", {});
const testStream = new aws.kinesis.Stream("test_stream", {
    name: "pinpoint-kinesis-test",
    shardCount: 1,
});
const assumeRole = aws.iam.getPolicyDocument({
    statements: [{
        effect: "Allow",
        principals: [{
            type: "Service",
            identifiers: ["pinpoint.us-east-1.amazonaws.com"],
        }],
        actions: ["sts:AssumeRole"],
    }],
});
const testRole = new aws.iam.Role("test_role", {assumeRolePolicy: assumeRole.then(assumeRole => assumeRole.json)});
const stream = new aws.pinpoint.EventStream("stream", {
    applicationId: app.applicationId,
    destinationStreamArn: testStream.arn,
    roleArn: testRole.arn,
});
const testRolePolicy = aws.iam.getPolicyDocument({
    statements: [{
        effect: "Allow",
        actions: [
            "kinesis:PutRecords",
            "kinesis:DescribeStream",
        ],
        resources: ["arn:aws:kinesis:us-east-1:*:*/*"],
    }],
});
const testRolePolicyRolePolicy = new aws.iam.RolePolicy("test_role_policy", {
    name: "test_policy",
    role: testRole.id,
    policy: testRolePolicy.then(testRolePolicy => testRolePolicy.json),
});
```
```python
import pulumi
import pulumi_aws as aws

app = aws.pinpoint.App("app")
test_stream = aws.kinesis.Stream("test_stream",
    name="pinpoint-kinesis-test",
    shard_count=1)
assume_role = aws.iam.get_policy_document(statements=[{
    "effect": "Allow",
    "principals": [{
        "type": "Service",
        "identifiers": ["pinpoint.us-east-1.amazonaws.com"],
    }],
    "actions": ["sts:AssumeRole"],
}])
test_role = aws.iam.Role("test_role", assume_role_policy=assume_role.json)
stream = aws.pinpoint.EventStream("stream",
    application_id=app.application_id,
    destination_stream_arn=test_stream.arn,
    role_arn=test_role.arn)
test_role_policy = aws.iam.get_policy_document(statements=[{
    "effect": "Allow",
    "actions": [
        "kinesis:PutRecords",
        "kinesis:DescribeStream",
    ],
    "resources": ["arn:aws:kinesis:us-east-1:*:*/*"],
}])
test_role_policy_role_policy = aws.iam.RolePolicy("test_role_policy",
    name="test_policy",
    role=test_role.id,
    policy=test_role_policy.json)
```
```csharp
using System.Collections.Generic;
using System.Linq;
using Pulumi;
using Aws = Pulumi.Aws;

return await Deployment.RunAsync(() => 
{
    var app = new Aws.Pinpoint.App("app");

    var testStream = new Aws.Kinesis.Stream("test_stream", new()
    {
        Name = "pinpoint-kinesis-test",
        ShardCount = 1,
    });

    var assumeRole = Aws.Iam.GetPolicyDocument.Invoke(new()
    {
        Statements = new[]
        {
            new Aws.Iam.Inputs.GetPolicyDocumentStatementInputArgs
            {
                Effect = "Allow",
                Principals = new[]
                {
                    new Aws.Iam.Inputs.GetPolicyDocumentStatementPrincipalInputArgs
                    {
                        Type = "Service",
                        Identifiers = new[]
                        {
                            "pinpoint.us-east-1.amazonaws.com",
                        },
                    },
                },
                Actions = new[]
                {
                    "sts:AssumeRole",
                },
            },
        },
    });

    var testRole = new Aws.Iam.Role("test_role", new()
    {
        AssumeRolePolicy = assumeRole.Apply(getPolicyDocumentResult => getPolicyDocumentResult.Json),
    });

    var stream = new Aws.Pinpoint.EventStream("stream", new()
    {
        ApplicationId = app.ApplicationId,
        DestinationStreamArn = testStream.Arn,
        RoleArn = testRole.Arn,
    });

    var testRolePolicy = Aws.Iam.GetPolicyDocument.Invoke(new()
    {
        Statements = new[]
        {
            new Aws.Iam.Inputs.GetPolicyDocumentStatementInputArgs
            {
                Effect = "Allow",
                Actions = new[]
                {
                    "kinesis:PutRecords",
                    "kinesis:DescribeStream",
                },
                Resources = new[]
                {
                    "arn:aws:kinesis:us-east-1:*:*/*",
                },
            },
        },
    });

    var testRolePolicyRolePolicy = new Aws.Iam.RolePolicy("test_role_policy", new()
    {
        Name = "test_policy",
        Role = testRole.Id,
        Policy = testRolePolicy.Apply(getPolicyDocumentResult => getPolicyDocumentResult.Json),
    });

});
```
```go
package main

import (
	"github.com/pulumi/pulumi-aws/sdk/v6/go/aws/iam"
	"github.com/pulumi/pulumi-aws/sdk/v6/go/aws/kinesis"
	"github.com/pulumi/pulumi-aws/sdk/v6/go/aws/pinpoint"
	"github.com/pulumi/pulumi/sdk/v3/go/pulumi"
)

func main() {
	pulumi.Run(func(ctx *pulumi.Context) error {
		app, err := pinpoint.NewApp(ctx, "app", nil)
		if err != nil {
			return err
		}
		testStream, err := kinesis.NewStream(ctx, "test_stream", &kinesis.StreamArgs{
			Name:       pulumi.String("pinpoint-kinesis-test"),
			ShardCount: pulumi.Int(1),
		})
		if err != nil {
			return err
		}
		assumeRole, err := iam.GetPolicyDocument(ctx, &iam.GetPolicyDocumentArgs{
			Statements: []iam.GetPolicyDocumentStatement{
				{
					Effect: pulumi.StringRef("Allow"),
					Principals: []iam.GetPolicyDocumentStatementPrincipal{
						{
							Type: "Service",
							Identifiers: []string{
								"pinpoint.us-east-1.amazonaws.com",
							},
						},
					},
					Actions: []string{
						"sts:AssumeRole",
					},
				},
			},
		}, nil)
		if err != nil {
			return err
		}
		testRole, err := iam.NewRole(ctx, "test_role", &iam.RoleArgs{
			AssumeRolePolicy: pulumi.String(assumeRole.Json),
		})
		if err != nil {
			return err
		}
		_, err = pinpoint.NewEventStream(ctx, "stream", &pinpoint.EventStreamArgs{
			ApplicationId:        app.ApplicationId,
			DestinationStreamArn: testStream.Arn,
			RoleArn:              testRole.Arn,
		})
		if err != nil {
			return err
		}
		testRolePolicy, err := iam.GetPolicyDocument(ctx, &iam.GetPolicyDocumentArgs{
			Statements: []iam.GetPolicyDocumentStatement{
				{
					Effect: pulumi.StringRef("Allow"),
					Actions: []string{
						"kinesis:PutRecords",
						"kinesis:DescribeStream",
					},
					Resources: []string{
						"arn:aws:kinesis:us-east-1:*:*/*",
					},
				},
			},
		}, nil)
		if err != nil {
			return err
		}
		_, err = iam.NewRolePolicy(ctx, "test_role_policy", &iam.RolePolicyArgs{
			Name:   pulumi.String("test_policy"),
			Role:   testRole.ID(),
			Policy: pulumi.String(testRolePolicy.Json),
		})
		if err != nil {
			return err
		}
		return nil
	})
}
```
```java
package generated_program;

import com.pulumi.Context;
import com.pulumi.Pulumi;
import com.pulumi.core.Output;
import com.pulumi.aws.pinpoint.App;
import com.pulumi.aws.kinesis.Stream;
import com.pulumi.aws.kinesis.StreamArgs;
import com.pulumi.aws.iam.IamFunctions;
import com.pulumi.aws.iam.inputs.GetPolicyDocumentArgs;
import com.pulumi.aws.iam.Role;
import com.pulumi.aws.iam.RoleArgs;
import com.pulumi.aws.pinpoint.EventStream;
import com.pulumi.aws.pinpoint.EventStreamArgs;
import com.pulumi.aws.iam.RolePolicy;
import com.pulumi.aws.iam.RolePolicyArgs;
import java.util.List;
import java.util.ArrayList;
import java.util.Map;
import java.io.File;
import java.nio.file.Files;
import java.nio.file.Paths;

public class App {
    public static void main(String[] args) {
        Pulumi.run(App::stack);
    }

    public static void stack(Context ctx) {
        var app = new App("app");

        var testStream = new Stream("testStream", StreamArgs.builder()
            .name("pinpoint-kinesis-test")
            .shardCount(1)
            .build());

        final var assumeRole = IamFunctions.getPolicyDocument(GetPolicyDocumentArgs.builder()
            .statements(GetPolicyDocumentStatementArgs.builder()
                .effect("Allow")
                .principals(GetPolicyDocumentStatementPrincipalArgs.builder()
                    .type("Service")
                    .identifiers("pinpoint.us-east-1.amazonaws.com")
                    .build())
                .actions("sts:AssumeRole")
                .build())
            .build());

        var testRole = new Role("testRole", RoleArgs.builder()
            .assumeRolePolicy(assumeRole.applyValue(getPolicyDocumentResult -> getPolicyDocumentResult.json()))
            .build());

        var stream = new EventStream("stream", EventStreamArgs.builder()
            .applicationId(app.applicationId())
            .destinationStreamArn(testStream.arn())
            .roleArn(testRole.arn())
            .build());

        final var testRolePolicy = IamFunctions.getPolicyDocument(GetPolicyDocumentArgs.builder()
            .statements(GetPolicyDocumentStatementArgs.builder()
                .effect("Allow")
                .actions(                
                    "kinesis:PutRecords",
                    "kinesis:DescribeStream")
                .resources("arn:aws:kinesis:us-east-1:*:*/*")
                .build())
            .build());

        var testRolePolicyRolePolicy = new RolePolicy("testRolePolicyRolePolicy", RolePolicyArgs.builder()
            .name("test_policy")
            .role(testRole.id())
            .policy(testRolePolicy.applyValue(getPolicyDocumentResult -> getPolicyDocumentResult.json()))
            .build());

    }
}
```
```yaml
resources:
  stream:
    type: aws:pinpoint:EventStream
    properties:
      applicationId: ${app.applicationId}
      destinationStreamArn: ${testStream.arn}
      roleArn: ${testRole.arn}
  app:
    type: aws:pinpoint:App
  testStream:
    type: aws:kinesis:Stream
    name: test_stream
    properties:
      name: pinpoint-kinesis-test
      shardCount: 1
  testRole:
    type: aws:iam:Role
    name: test_role
    properties:
      assumeRolePolicy: ${assumeRole.json}
  testRolePolicyRolePolicy:
    type: aws:iam:RolePolicy
    name: test_role_policy
    properties:
      name: test_policy
      role: ${testRole.id}
      policy: ${testRolePolicy.json}
variables:
  assumeRole:
    fn::invoke:
      function: aws:iam:getPolicyDocument
      arguments:
        statements:
          - effect: Allow
            principals:
              - type: Service
                identifiers:
                  - pinpoint.us-east-1.amazonaws.com
            actions:
              - sts:AssumeRole
  testRolePolicy:
    fn::invoke:
      function: aws:iam:getPolicyDocument
      arguments:
        statements:
          - effect: Allow
            actions:
              - kinesis:PutRecords
              - kinesis:DescribeStream
            resources:
              - arn:aws:kinesis:us-east-1:*:*/*
```
<!--End PulumiCodeChooser -->

## Import

Using `pulumi import`, import Pinpoint Event Stream using the `application-id`. For example:

```sh
$ pulumi import aws:pinpoint/eventStream:EventStream stream application-id
```
)
applicationId" The application ID.
�
destinationStreamArn" }The Amazon Resource Name (ARN) of the Amazon Kinesis stream or Firehose delivery stream to which you want to publish events.
m
roleArn" ^The IAM role that authorizes Amazon Pinpoint to publish events to the stream in your account.
")
applicationId" The application ID.
"�
destinationStreamArn" }The Amazon Resource Name (ARN) of the Amazon Kinesis stream or Firehose delivery stream to which you want to publish events.
"m
roleArn" ^The IAM role that authorizes Amazon Pinpoint to publish events to the stream in your account.
*�
:
pinpoint
GcmChannel"aws:pinpoint/gcmChannel:GcmChannel�Provides a Pinpoint GCM Channel resource.

> **Note:** Credentials (Service Account JSON and API Key) will be stored in the raw state as plain-text.
## Import

Using `pulumi import`, import Pinpoint GCM Channel using the `application-id`. For example:

```sh
$ pulumi import aws:pinpoint/gcmChannel:GcmChannel gcm application-id
```
9
apiKeyB" )Platform credential API key from Google.
)
applicationId" The application ID.
#
defaultAuthenticationMethodB" Q
enabledB
 @Whether the channel is enabled or disabled. Defaults to `true`.

serviceJsonB" "9
apiKeyB" )Platform credential API key from Google.
")
applicationId" The application ID.
"#
defaultAuthenticationMethodB" "Q
enabledB
 @Whether the channel is enabled or disabled. Defaults to `true`.
"
serviceJsonB" *�
:
pinpoint
SmsChannel"aws:pinpoint/smsChannel:SmsChannel�Use the `aws.pinpoint.SmsChannel` resource to manage Pinpoint SMS Channels.

## Example Usage

<!--Start PulumiCodeChooser -->
```typescript
import * as pulumi from "@pulumi/pulumi";
import * as aws from "@pulumi/aws";

const app = new aws.pinpoint.App("app", {});
const sms = new aws.pinpoint.SmsChannel("sms", {applicationId: app.applicationId});
```
```python
import pulumi
import pulumi_aws as aws

app = aws.pinpoint.App("app")
sms = aws.pinpoint.SmsChannel("sms", application_id=app.application_id)
```
```csharp
using System.Collections.Generic;
using System.Linq;
using Pulumi;
using Aws = Pulumi.Aws;

return await Deployment.RunAsync(() => 
{
    var app = new Aws.Pinpoint.App("app");

    var sms = new Aws.Pinpoint.SmsChannel("sms", new()
    {
        ApplicationId = app.ApplicationId,
    });

});
```
```go
package main

import (
	"github.com/pulumi/pulumi-aws/sdk/v6/go/aws/pinpoint"
	"github.com/pulumi/pulumi/sdk/v3/go/pulumi"
)

func main() {
	pulumi.Run(func(ctx *pulumi.Context) error {
		app, err := pinpoint.NewApp(ctx, "app", nil)
		if err != nil {
			return err
		}
		_, err = pinpoint.NewSmsChannel(ctx, "sms", &pinpoint.SmsChannelArgs{
			ApplicationId: app.ApplicationId,
		})
		if err != nil {
			return err
		}
		return nil
	})
}
```
```java
package generated_program;

import com.pulumi.Context;
import com.pulumi.Pulumi;
import com.pulumi.core.Output;
import com.pulumi.aws.pinpoint.App;
import com.pulumi.aws.pinpoint.SmsChannel;
import com.pulumi.aws.pinpoint.SmsChannelArgs;
import java.util.List;
import java.util.ArrayList;
import java.util.Map;
import java.io.File;
import java.nio.file.Files;
import java.nio.file.Paths;

public class App {
    public static void main(String[] args) {
        Pulumi.run(App::stack);
    }

    public static void stack(Context ctx) {
        var app = new App("app");

        var sms = new SmsChannel("sms", SmsChannelArgs.builder()
            .applicationId(app.applicationId())
            .build());

    }
}
```
```yaml
resources:
  sms:
    type: aws:pinpoint:SmsChannel
    properties:
      applicationId: ${app.applicationId}
  app:
    type: aws:pinpoint:App
```
<!--End PulumiCodeChooser -->

## Import

Using `pulumi import`, import the Pinpoint SMS Channel using the `application_id`. For example:

```sh
$ pulumi import aws:pinpoint/smsChannel:SmsChannel sms application-id
```
,
applicationId" ID of the application.
^
enabledB
 MWhether the channel is enabled or disabled. By default, it is set to `true`.
>
senderIdB" ,Identifier of the sender for your messages.
B
	shortCodeB" /Short Code registered with the phone provider.
",
applicationId" ID of the application.
"^
enabledB
 MWhether the channel is enabled or disabled. By default, it is set to `true`.
"h
promotionalMessagesPerSecond DMaximum number of promotional messages that can be sent per second.
">
senderIdB" ,Identifier of the sender for your messages.
"B
	shortCodeB" /Short Code registered with the phone provider.
"l
transactionalMessagesPerSecond FMaximum number of transactional messages per second that can be sent.
*�
j
pinpointSmsvoicev2ConfigurationSetBaws:pinpoint/smsvoicev2ConfigurationSet:Smsvoicev2ConfigurationSet�Manages an AWS End User Messaging SMS Configuration Set.

## Example Usage

<!--Start PulumiCodeChooser -->
```typescript
import * as pulumi from "@pulumi/pulumi";
import * as aws from "@pulumi/aws";

const example = new aws.pinpoint.Smsvoicev2ConfigurationSet("example", {
    name: "example-configuration-set",
    defaultSenderId: "example",
    defaultMessageType: "TRANSACTIONAL",
});
```
```python
import pulumi
import pulumi_aws as aws

example = aws.pinpoint.Smsvoicev2ConfigurationSet("example",
    name="example-configuration-set",
    default_sender_id="example",
    default_message_type="TRANSACTIONAL")
```
```csharp
using System.Collections.Generic;
using System.Linq;
using Pulumi;
using Aws = Pulumi.Aws;

return await Deployment.RunAsync(() => 
{
    var example = new Aws.Pinpoint.Smsvoicev2ConfigurationSet("example", new()
    {
        Name = "example-configuration-set",
        DefaultSenderId = "example",
        DefaultMessageType = "TRANSACTIONAL",
    });

});
```
```go
package main

import (
	"github.com/pulumi/pulumi-aws/sdk/v6/go/aws/pinpoint"
	"github.com/pulumi/pulumi/sdk/v3/go/pulumi"
)

func main() {
	pulumi.Run(func(ctx *pulumi.Context) error {
		_, err := pinpoint.NewSmsvoicev2ConfigurationSet(ctx, "example", &pinpoint.Smsvoicev2ConfigurationSetArgs{
			Name:               pulumi.String("example-configuration-set"),
			DefaultSenderId:    pulumi.String("example"),
			DefaultMessageType: pulumi.String("TRANSACTIONAL"),
		})
		if err != nil {
			return err
		}
		return nil
	})
}
```
```java
package generated_program;

import com.pulumi.Context;
import com.pulumi.Pulumi;
import com.pulumi.core.Output;
import com.pulumi.aws.pinpoint.Smsvoicev2ConfigurationSet;
import com.pulumi.aws.pinpoint.Smsvoicev2ConfigurationSetArgs;
import java.util.List;
import java.util.ArrayList;
import java.util.Map;
import java.io.File;
import java.nio.file.Files;
import java.nio.file.Paths;

public class App {
    public static void main(String[] args) {
        Pulumi.run(App::stack);
    }

    public static void stack(Context ctx) {
        var example = new Smsvoicev2ConfigurationSet("example", Smsvoicev2ConfigurationSetArgs.builder()
            .name("example-configuration-set")
            .defaultSenderId("example")
            .defaultMessageType("TRANSACTIONAL")
            .build());

    }
}
```
```yaml
resources:
  example:
    type: aws:pinpoint:Smsvoicev2ConfigurationSet
    properties:
      name: example-configuration-set
      defaultSenderId: example
      defaultMessageType: TRANSACTIONAL
```
<!--End PulumiCodeChooser -->

## Import

Using `pulumi import`, import configuration sets using the `name`. For example:

```sh
$ pulumi import aws:pinpoint/smsvoicev2ConfigurationSet:Smsvoicev2ConfigurationSet example example-configuration-set
```
f
defaultMessageTypeB" JThe default message type. Must either be "TRANSACTIONAL" or "PROMOTIONAL"
R
defaultSenderIdB" 9The default sender ID to use for this configuration set.
1
nameB" #The name of the configuration set.
�
tagsB2" �Key-value map of resource tags. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
")
arn" ARN of the configuration set.
"f
defaultMessageTypeB" JThe default message type. Must either be "TRANSACTIONAL" or "PROMOTIONAL"
"R
defaultSenderIdB" 9The default sender ID to use for this configuration set.
"/
name" #The name of the configuration set.
"�
tagsB2" �Key-value map of resource tags. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
"�
tagsAll2" xA map of tags assigned to the resource, including those inherited from the provider `default_tags` configuration block.
*�
X
pinpointSmsvoicev2OptOutList6aws:pinpoint/smsvoicev2OptOutList:Smsvoicev2OptOutList�Manages an AWS End User Messaging SMS opt-out list.

## Example Usage

<!--Start PulumiCodeChooser -->
```typescript
import * as pulumi from "@pulumi/pulumi";
import * as aws from "@pulumi/aws";

const example = new aws.pinpoint.Smsvoicev2OptOutList("example", {name: "example-opt-out-list"});
```
```python
import pulumi
import pulumi_aws as aws

example = aws.pinpoint.Smsvoicev2OptOutList("example", name="example-opt-out-list")
```
```csharp
using System.Collections.Generic;
using System.Linq;
using Pulumi;
using Aws = Pulumi.Aws;

return await Deployment.RunAsync(() => 
{
    var example = new Aws.Pinpoint.Smsvoicev2OptOutList("example", new()
    {
        Name = "example-opt-out-list",
    });

});
```
```go
package main

import (
	"github.com/pulumi/pulumi-aws/sdk/v6/go/aws/pinpoint"
	"github.com/pulumi/pulumi/sdk/v3/go/pulumi"
)

func main() {
	pulumi.Run(func(ctx *pulumi.Context) error {
		_, err := pinpoint.NewSmsvoicev2OptOutList(ctx, "example", &pinpoint.Smsvoicev2OptOutListArgs{
			Name: pulumi.String("example-opt-out-list"),
		})
		if err != nil {
			return err
		}
		return nil
	})
}
```
```java
package generated_program;

import com.pulumi.Context;
import com.pulumi.Pulumi;
import com.pulumi.core.Output;
import com.pulumi.aws.pinpoint.Smsvoicev2OptOutList;
import com.pulumi.aws.pinpoint.Smsvoicev2OptOutListArgs;
import java.util.List;
import java.util.ArrayList;
import java.util.Map;
import java.io.File;
import java.nio.file.Files;
import java.nio.file.Paths;

public class App {
    public static void main(String[] args) {
        Pulumi.run(App::stack);
    }

    public static void stack(Context ctx) {
        var example = new Smsvoicev2OptOutList("example", Smsvoicev2OptOutListArgs.builder()
            .name("example-opt-out-list")
            .build());

    }
}
```
```yaml
resources:
  example:
    type: aws:pinpoint:Smsvoicev2OptOutList
    properties:
      name: example-opt-out-list
```
<!--End PulumiCodeChooser -->

## Import

Using `pulumi import`, import opt-out lists using the `name`. For example:

```sh
$ pulumi import aws:pinpoint/smsvoicev2OptOutList:Smsvoicev2OptOutList example example-opt-out-list
```
,
nameB" The name of the opt-out list.
�
tagsB2" �Key-value map of resource tags. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
"$
arn" ARN of the opt-out list.
"*
name" The name of the opt-out list.
"�
tagsB2" �Key-value map of resource tags. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
"�
tagsAll2" xA map of tags assigned to the resource, including those inherited from the provider `default_tags` configuration block.
*�5
[
pinpointSmsvoicev2PhoneNumber8aws:pinpoint/smsvoicev2PhoneNumber:Smsvoicev2PhoneNumber�Manages an AWS End User Messaging SMS phone number.

## Example Usage

<!--Start PulumiCodeChooser -->
```typescript
import * as pulumi from "@pulumi/pulumi";
import * as aws from "@pulumi/aws";

const example = new aws.pinpoint.Smsvoicev2PhoneNumber("example", {
    isoCountryCode: "US",
    messageType: "TRANSACTIONAL",
    numberType: "TOLL_FREE",
    numberCapabilities: ["SMS"],
});
```
```python
import pulumi
import pulumi_aws as aws

example = aws.pinpoint.Smsvoicev2PhoneNumber("example",
    iso_country_code="US",
    message_type="TRANSACTIONAL",
    number_type="TOLL_FREE",
    number_capabilities=["SMS"])
```
```csharp
using System.Collections.Generic;
using System.Linq;
using Pulumi;
using Aws = Pulumi.Aws;

return await Deployment.RunAsync(() => 
{
    var example = new Aws.Pinpoint.Smsvoicev2PhoneNumber("example", new()
    {
        IsoCountryCode = "US",
        MessageType = "TRANSACTIONAL",
        NumberType = "TOLL_FREE",
        NumberCapabilities = new[]
        {
            "SMS",
        },
    });

});
```
```go
package main

import (
	"github.com/pulumi/pulumi-aws/sdk/v6/go/aws/pinpoint"
	"github.com/pulumi/pulumi/sdk/v3/go/pulumi"
)

func main() {
	pulumi.Run(func(ctx *pulumi.Context) error {
		_, err := pinpoint.NewSmsvoicev2PhoneNumber(ctx, "example", &pinpoint.Smsvoicev2PhoneNumberArgs{
			IsoCountryCode: pulumi.String("US"),
			MessageType:    pulumi.String("TRANSACTIONAL"),
			NumberType:     pulumi.String("TOLL_FREE"),
			NumberCapabilities: pulumi.StringArray{
				pulumi.String("SMS"),
			},
		})
		if err != nil {
			return err
		}
		return nil
	})
}
```
```java
package generated_program;

import com.pulumi.Context;
import com.pulumi.Pulumi;
import com.pulumi.core.Output;
import com.pulumi.aws.pinpoint.Smsvoicev2PhoneNumber;
import com.pulumi.aws.pinpoint.Smsvoicev2PhoneNumberArgs;
import java.util.List;
import java.util.ArrayList;
import java.util.Map;
import java.io.File;
import java.nio.file.Files;
import java.nio.file.Paths;

public class App {
    public static void main(String[] args) {
        Pulumi.run(App::stack);
    }

    public static void stack(Context ctx) {
        var example = new Smsvoicev2PhoneNumber("example", Smsvoicev2PhoneNumberArgs.builder()
            .isoCountryCode("US")
            .messageType("TRANSACTIONAL")
            .numberType("TOLL_FREE")
            .numberCapabilities("SMS")
            .build());

    }
}
```
```yaml
resources:
  example:
    type: aws:pinpoint:Smsvoicev2PhoneNumber
    properties:
      isoCountryCode: US
      messageType: TRANSACTIONAL
      numberType: TOLL_FREE
      numberCapabilities:
        - SMS
```
<!--End PulumiCodeChooser -->

## Import

Using `pulumi import`, import phone numbers using the `id`. For example:

```sh
$ pulumi import aws:pinpoint/smsvoicev2PhoneNumber:Smsvoicev2PhoneNumber example phone-abcdef0123456789abcdef0123456789
```
|
deletionProtectionEnabledB
 YBy default this is set to `false`. When set to true the phone number can’t be deleted.
g
isoCountryCode" QThe two-character code, in ISO 3166-1 alpha-2 format, for the country or region.
�
messageType" �The type of message. Valid values are `TRANSACTIONAL` for messages that are critical or time-sensitive and `PROMOTIONAL` for messages that aren’t critical or time-sensitive.
�
numberCapabilities*" ~Describes if the origination identity can be used for text messages, voice calls or both. valid values are `SMS` and `VOICE`.
�

numberType" nThe type of phone number to request. Possible values are `LONG_CODE`, `TOLL_FREE`, `TEN_DLC`, or `SIMULATOR`.
Y
optOutListNameB" AThe name of the opt-out list to associate with the phone number.
i
registrationIdB" QUse this field to attach your phone number for an external registration process.
�
selfManagedOptOutsEnabledB
 �When set to `false` an end recipient sends a message that begins with HELP or STOP to one of your dedicated numbers, AWS End User Messaging SMS and Voice automatically replies with a customizable message and adds the end recipient to the opt-out list. When set to true you’re responsible for responding to HELP and STOP requests. You’re also responsible for tracking and honoring opt-out request.

tagsB2" �
timeoutsyBw:u
s
pinpointSmsvoicev2PhoneNumberTimeoutsHaws:pinpoint/Smsvoicev2PhoneNumberTimeouts:Smsvoicev2PhoneNumberTimeoutsQ
twoWayChannelArnB" 7The Amazon Resource Name (ARN) of the two way channel.
�
twoWayChannelEnabledB
 wBy default this is set to `false`. When set to `true` you can receive incoming text messages from your end recipients.
"$
arn" ARN of the phone number.
"z
deletionProtectionEnabled
 YBy default this is set to `false`. When set to true the phone number can’t be deleted.
"g
isoCountryCode" QThe two-character code, in ISO 3166-1 alpha-2 format, for the country or region.
"�
messageType" �The type of message. Valid values are `TRANSACTIONAL` for messages that are critical or time-sensitive and `PROMOTIONAL` for messages that aren’t critical or time-sensitive.
"X
monthlyLeasingPrice" =The monthly price, in US dollars, to lease the phone number.
"�
numberCapabilities*" ~Describes if the origination identity can be used for text messages, voice calls or both. valid values are `SMS` and `VOICE`.
"�

numberType" nThe type of phone number to request. Possible values are `LONG_CODE`, `TOLL_FREE`, `TEN_DLC`, or `SIMULATOR`.
"W
optOutListName" AThe name of the opt-out list to associate with the phone number.
"<
phoneNumber" )The new phone number that was requested.
"i
registrationIdB" QUse this field to attach your phone number for an external registration process.
"�
selfManagedOptOutsEnabled
 �When set to `false` an end recipient sends a message that begins with HELP or STOP to one of your dedicated numbers, AWS End User Messaging SMS and Voice automatically replies with a customizable message and adds the end recipient to the opt-out list. When set to true you’re responsible for responding to HELP and STOP requests. You’re also responsible for tracking and honoring opt-out request.
"
tagsB2" "�
tagsAll2" xA map of tags assigned to the resource, including those inherited from the provider `default_tags` configuration block.
"�
timeoutsyBw:u
s
pinpointSmsvoicev2PhoneNumberTimeoutsHaws:pinpoint/Smsvoicev2PhoneNumberTimeouts:Smsvoicev2PhoneNumberTimeouts"Q
twoWayChannelArnB" 7The Amazon Resource Name (ARN) of the two way channel.
"�
twoWayChannelEnabled
 wBy default this is set to `false`. When set to `true` you can receive incoming text messages from your end recipients.
*��
"
pipesPipeaws:pipes/pipe:Pipe�Resource for managing an AWS EventBridge Pipes Pipe.

You can find out more about EventBridge Pipes in the [User Guide](https://docs.aws.amazon.com/eventbridge/latest/userguide/eb-pipes.html).

EventBridge Pipes are very configurable, and may require IAM permissions to work correctly. More information on the configuration options and IAM permissions can be found in the [User Guide](https://docs.aws.amazon.com/eventbridge/latest/userguide/eb-pipes.html).

> **Note:** EventBridge was formerly known as CloudWatch Events. The functionality is identical.

## Example Usage

### Basic Usage

<!--Start PulumiCodeChooser -->
```typescript
import * as pulumi from "@pulumi/pulumi";
import * as aws from "@pulumi/aws";

const main = aws.getCallerIdentity({});
const example = new aws.iam.Role("example", {assumeRolePolicy: JSON.stringify({
    Version: "2012-10-17",
    Statement: {
        Effect: "Allow",
        Action: "sts:AssumeRole",
        Principal: {
            Service: "pipes.amazonaws.com",
        },
        Condition: {
            StringEquals: {
                "aws:SourceAccount": main.then(main => main.accountId),
            },
        },
    },
})});
const sourceQueue = new aws.sqs.Queue("source", {});
const source = new aws.iam.RolePolicy("source", {
    role: example.id,
    policy: pulumi.jsonStringify({
        Version: "2012-10-17",
        Statement: [{
            Effect: "Allow",
            Action: [
                "sqs:DeleteMessage",
                "sqs:GetQueueAttributes",
                "sqs:ReceiveMessage",
            ],
            Resource: [sourceQueue.arn],
        }],
    }),
});
const targetQueue = new aws.sqs.Queue("target", {});
const target = new aws.iam.RolePolicy("target", {
    role: example.id,
    policy: pulumi.jsonStringify({
        Version: "2012-10-17",
        Statement: [{
            Effect: "Allow",
            Action: ["sqs:SendMessage"],
            Resource: [targetQueue.arn],
        }],
    }),
});
const examplePipe = new aws.pipes.Pipe("example", {
    name: "example-pipe",
    roleArn: example.arn,
    source: sourceQueue.arn,
    target: targetQueue.arn,
}, {
    dependsOn: [
        source,
        target,
    ],
});
```
```python
import pulumi
import json
import pulumi_aws as aws

main = aws.get_caller_identity()
example = aws.iam.Role("example", assume_role_policy=json.dumps({
    "Version": "2012-10-17",
    "Statement": {
        "Effect": "Allow",
        "Action": "sts:AssumeRole",
        "Principal": {
            "Service": "pipes.amazonaws.com",
        },
        "Condition": {
            "StringEquals": {
                "aws:SourceAccount": main.account_id,
            },
        },
    },
}))
source_queue = aws.sqs.Queue("source")
source = aws.iam.RolePolicy("source",
    role=example.id,
    policy=pulumi.Output.json_dumps({
        "Version": "2012-10-17",
        "Statement": [{
            "Effect": "Allow",
            "Action": [
                "sqs:DeleteMessage",
                "sqs:GetQueueAttributes",
                "sqs:ReceiveMessage",
            ],
            "Resource": [source_queue.arn],
        }],
    }))
target_queue = aws.sqs.Queue("target")
target = aws.iam.RolePolicy("target",
    role=example.id,
    policy=pulumi.Output.json_dumps({
        "Version": "2012-10-17",
        "Statement": [{
            "Effect": "Allow",
            "Action": ["sqs:SendMessage"],
            "Resource": [target_queue.arn],
        }],
    }))
example_pipe = aws.pipes.Pipe("example",
    name="example-pipe",
    role_arn=example.arn,
    source=source_queue.arn,
    target=target_queue.arn,
    opts = pulumi.ResourceOptions(depends_on=[
            source,
            target,
        ]))
```
```csharp
using System.Collections.Generic;
using System.Linq;
using System.Text.Json;
using Pulumi;
using Aws = Pulumi.Aws;

return await Deployment.RunAsync(() => 
{
    var main = Aws.GetCallerIdentity.Invoke();

    var example = new Aws.Iam.Role("example", new()
    {
        AssumeRolePolicy = JsonSerializer.Serialize(new Dictionary<string, object?>
        {
            ["Version"] = "2012-10-17",
            ["Statement"] = new Dictionary<string, object?>
            {
                ["Effect"] = "Allow",
                ["Action"] = "sts:AssumeRole",
                ["Principal"] = new Dictionary<string, object?>
                {
                    ["Service"] = "pipes.amazonaws.com",
                },
                ["Condition"] = new Dictionary<string, object?>
                {
                    ["StringEquals"] = new Dictionary<string, object?>
                    {
                        ["aws:SourceAccount"] = main.Apply(getCallerIdentityResult => getCallerIdentityResult.AccountId),
                    },
                },
            },
        }),
    });

    var sourceQueue = new Aws.Sqs.Queue("source");

    var source = new Aws.Iam.RolePolicy("source", new()
    {
        Role = example.Id,
        Policy = Output.JsonSerialize(Output.Create(new Dictionary<string, object?>
        {
            ["Version"] = "2012-10-17",
            ["Statement"] = new[]
            {
                new Dictionary<string, object?>
                {
                    ["Effect"] = "Allow",
                    ["Action"] = new[]
                    {
                        "sqs:DeleteMessage",
                        "sqs:GetQueueAttributes",
                        "sqs:ReceiveMessage",
                    },
                    ["Resource"] = new[]
                    {
                        sourceQueue.Arn,
                    },
                },
            },
        })),
    });

    var targetQueue = new Aws.Sqs.Queue("target");

    var target = new Aws.Iam.RolePolicy("target", new()
    {
        Role = example.Id,
        Policy = Output.JsonSerialize(Output.Create(new Dictionary<string, object?>
        {
            ["Version"] = "2012-10-17",
            ["Statement"] = new[]
            {
                new Dictionary<string, object?>
                {
                    ["Effect"] = "Allow",
                    ["Action"] = new[]
                    {
                        "sqs:SendMessage",
                    },
                    ["Resource"] = new[]
                    {
                        targetQueue.Arn,
                    },
                },
            },
        })),
    });

    var examplePipe = new Aws.Pipes.Pipe("example", new()
    {
        Name = "example-pipe",
        RoleArn = example.Arn,
        Source = sourceQueue.Arn,
        Target = targetQueue.Arn,
    }, new CustomResourceOptions
    {
        DependsOn =
        {
            source,
            target,
        },
    });

});
```
```go
package main

import (
	"encoding/json"

	"github.com/pulumi/pulumi-aws/sdk/v6/go/aws"
	"github.com/pulumi/pulumi-aws/sdk/v6/go/aws/iam"
	"github.com/pulumi/pulumi-aws/sdk/v6/go/aws/pipes"
	"github.com/pulumi/pulumi-aws/sdk/v6/go/aws/sqs"
	"github.com/pulumi/pulumi/sdk/v3/go/pulumi"
)

func main() {
	pulumi.Run(func(ctx *pulumi.Context) error {
		main, err := aws.GetCallerIdentity(ctx, &aws.GetCallerIdentityArgs{}, nil)
		if err != nil {
			return err
		}
		tmpJSON0, err := json.Marshal(map[string]interface{}{
			"Version": "2012-10-17",
			"Statement": map[string]interface{}{
				"Effect": "Allow",
				"Action": "sts:AssumeRole",
				"Principal": map[string]interface{}{
					"Service": "pipes.amazonaws.com",
				},
				"Condition": map[string]interface{}{
					"StringEquals": map[string]interface{}{
						"aws:SourceAccount": main.AccountId,
					},
				},
			},
		})
		if err != nil {
			return err
		}
		json0 := string(tmpJSON0)
		example, err := iam.NewRole(ctx, "example", &iam.RoleArgs{
			AssumeRolePolicy: pulumi.String(json0),
		})
		if err != nil {
			return err
		}
		sourceQueue, err := sqs.NewQueue(ctx, "source", nil)
		if err != nil {
			return err
		}
		source, err := iam.NewRolePolicy(ctx, "source", &iam.RolePolicyArgs{
			Role: example.ID(),
			Policy: sourceQueue.Arn.ApplyT(func(arn string) (pulumi.String, error) {
				var _zero pulumi.String
				tmpJSON1, err := json.Marshal(map[string]interface{}{
					"Version": "2012-10-17",
					"Statement": []map[string]interface{}{
						map[string]interface{}{
							"Effect": "Allow",
							"Action": []string{
								"sqs:DeleteMessage",
								"sqs:GetQueueAttributes",
								"sqs:ReceiveMessage",
							},
							"Resource": []string{
								arn,
							},
						},
					},
				})
				if err != nil {
					return _zero, err
				}
				json1 := string(tmpJSON1)
				return pulumi.String(json1), nil
			}).(pulumi.StringOutput),
		})
		if err != nil {
			return err
		}
		targetQueue, err := sqs.NewQueue(ctx, "target", nil)
		if err != nil {
			return err
		}
		target, err := iam.NewRolePolicy(ctx, "target", &iam.RolePolicyArgs{
			Role: example.ID(),
			Policy: targetQueue.Arn.ApplyT(func(arn string) (pulumi.String, error) {
				var _zero pulumi.String
				tmpJSON2, err := json.Marshal(map[string]interface{}{
					"Version": "2012-10-17",
					"Statement": []map[string]interface{}{
						map[string]interface{}{
							"Effect": "Allow",
							"Action": []string{
								"sqs:SendMessage",
							},
							"Resource": []string{
								arn,
							},
						},
					},
				})
				if err != nil {
					return _zero, err
				}
				json2 := string(tmpJSON2)
				return pulumi.String(json2), nil
			}).(pulumi.StringOutput),
		})
		if err != nil {
			return err
		}
		_, err = pipes.NewPipe(ctx, "example", &pipes.PipeArgs{
			Name:    pulumi.String("example-pipe"),
			RoleArn: example.Arn,
			Source:  sourceQueue.Arn,
			Target:  targetQueue.Arn,
		}, pulumi.DependsOn([]pulumi.Resource{
			source,
			target,
		}))
		if err != nil {
			return err
		}
		return nil
	})
}
```
```java
package generated_program;

import com.pulumi.Context;
import com.pulumi.Pulumi;
import com.pulumi.core.Output;
import com.pulumi.aws.AwsFunctions;
import com.pulumi.aws.inputs.GetCallerIdentityArgs;
import com.pulumi.aws.iam.Role;
import com.pulumi.aws.iam.RoleArgs;
import com.pulumi.aws.sqs.Queue;
import com.pulumi.aws.iam.RolePolicy;
import com.pulumi.aws.iam.RolePolicyArgs;
import com.pulumi.aws.pipes.Pipe;
import com.pulumi.aws.pipes.PipeArgs;
import static com.pulumi.codegen.internal.Serialization.*;
import com.pulumi.resources.CustomResourceOptions;
import java.util.List;
import java.util.ArrayList;
import java.util.Map;
import java.io.File;
import java.nio.file.Files;
import java.nio.file.Paths;

public class App {
    public static void main(String[] args) {
        Pulumi.run(App::stack);
    }

    public static void stack(Context ctx) {
        final var main = AwsFunctions.getCallerIdentity();

        var example = new Role("example", RoleArgs.builder()
            .assumeRolePolicy(serializeJson(
                jsonObject(
                    jsonProperty("Version", "2012-10-17"),
                    jsonProperty("Statement", jsonObject(
                        jsonProperty("Effect", "Allow"),
                        jsonProperty("Action", "sts:AssumeRole"),
                        jsonProperty("Principal", jsonObject(
                            jsonProperty("Service", "pipes.amazonaws.com")
                        )),
                        jsonProperty("Condition", jsonObject(
                            jsonProperty("StringEquals", jsonObject(
                                jsonProperty("aws:SourceAccount", main.applyValue(getCallerIdentityResult -> getCallerIdentityResult.accountId()))
                            ))
                        ))
                    ))
                )))
            .build());

        var sourceQueue = new Queue("sourceQueue");

        var source = new RolePolicy("source", RolePolicyArgs.builder()
            .role(example.id())
            .policy(sourceQueue.arn().applyValue(arn -> serializeJson(
                jsonObject(
                    jsonProperty("Version", "2012-10-17"),
                    jsonProperty("Statement", jsonArray(jsonObject(
                        jsonProperty("Effect", "Allow"),
                        jsonProperty("Action", jsonArray(
                            "sqs:DeleteMessage", 
                            "sqs:GetQueueAttributes", 
                            "sqs:ReceiveMessage"
                        )),
                        jsonProperty("Resource", jsonArray(arn))
                    )))
                ))))
            .build());

        var targetQueue = new Queue("targetQueue");

        var target = new RolePolicy("target", RolePolicyArgs.builder()
            .role(example.id())
            .policy(targetQueue.arn().applyValue(arn -> serializeJson(
                jsonObject(
                    jsonProperty("Version", "2012-10-17"),
                    jsonProperty("Statement", jsonArray(jsonObject(
                        jsonProperty("Effect", "Allow"),
                        jsonProperty("Action", jsonArray("sqs:SendMessage")),
                        jsonProperty("Resource", jsonArray(arn))
                    )))
                ))))
            .build());

        var examplePipe = new Pipe("examplePipe", PipeArgs.builder()
            .name("example-pipe")
            .roleArn(example.arn())
            .source(sourceQueue.arn())
            .target(targetQueue.arn())
            .build(), CustomResourceOptions.builder()
                .dependsOn(                
                    source,
                    target)
                .build());

    }
}
```
```yaml
resources:
  example:
    type: aws:iam:Role
    properties:
      assumeRolePolicy:
        fn::toJSON:
          Version: 2012-10-17
          Statement:
            Effect: Allow
            Action: sts:AssumeRole
            Principal:
              Service: pipes.amazonaws.com
            Condition:
              StringEquals:
                aws:SourceAccount: ${main.accountId}
  source:
    type: aws:iam:RolePolicy
    properties:
      role: ${example.id}
      policy:
        fn::toJSON:
          Version: 2012-10-17
          Statement:
            - Effect: Allow
              Action:
                - sqs:DeleteMessage
                - sqs:GetQueueAttributes
                - sqs:ReceiveMessage
              Resource:
                - ${sourceQueue.arn}
  sourceQueue:
    type: aws:sqs:Queue
    name: source
  target:
    type: aws:iam:RolePolicy
    properties:
      role: ${example.id}
      policy:
        fn::toJSON:
          Version: 2012-10-17
          Statement:
            - Effect: Allow
              Action:
                - sqs:SendMessage
              Resource:
                - ${targetQueue.arn}
  targetQueue:
    type: aws:sqs:Queue
    name: target
  examplePipe:
    type: aws:pipes:Pipe
    name: example
    properties:
      name: example-pipe
      roleArn: ${example.arn}
      source: ${sourceQueue.arn}
      target: ${targetQueue.arn}
    options:
      dependsOn:
        - ${source}
        - ${target}
variables:
  main:
    fn::invoke:
      function: aws:getCallerIdentity
      arguments: {}
```
<!--End PulumiCodeChooser -->

### Enrichment Usage

<!--Start PulumiCodeChooser -->
```typescript
import * as pulumi from "@pulumi/pulumi";
import * as aws from "@pulumi/aws";

const example = new aws.pipes.Pipe("example", {
    name: "example-pipe",
    roleArn: exampleAwsIamRole.arn,
    source: source.arn,
    target: target.arn,
    enrichment: exampleAwsCloudwatchEventApiDestination.arn,
    enrichmentParameters: {
        httpParameters: {
            pathParameterValues: "example-path-param",
            headerParameters: {
                "example-header": "example-value",
                "second-example-header": "second-example-value",
            },
            queryStringParameters: {
                "example-query-string": "example-value",
                "second-example-query-string": "second-example-value",
            },
        },
    },
});
```
```python
import pulumi
import pulumi_aws as aws

example = aws.pipes.Pipe("example",
    name="example-pipe",
    role_arn=example_aws_iam_role["arn"],
    source=source["arn"],
    target=target["arn"],
    enrichment=example_aws_cloudwatch_event_api_destination["arn"],
    enrichment_parameters={
        "http_parameters": {
            "path_parameter_values": "example-path-param",
            "header_parameters": {
                "example-header": "example-value",
                "second-example-header": "second-example-value",
            },
            "query_string_parameters": {
                "example-query-string": "example-value",
                "second-example-query-string": "second-example-value",
            },
        },
    })
```
```csharp
using System.Collections.Generic;
using System.Linq;
using Pulumi;
using Aws = Pulumi.Aws;

return await Deployment.RunAsync(() => 
{
    var example = new Aws.Pipes.Pipe("example", new()
    {
        Name = "example-pipe",
        RoleArn = exampleAwsIamRole.Arn,
        Source = source.Arn,
        Target = target.Arn,
        Enrichment = exampleAwsCloudwatchEventApiDestination.Arn,
        EnrichmentParameters = new Aws.Pipes.Inputs.PipeEnrichmentParametersArgs
        {
            HttpParameters = new Aws.Pipes.Inputs.PipeEnrichmentParametersHttpParametersArgs
            {
                PathParameterValues = "example-path-param",
                HeaderParameters = 
                {
                    { "example-header", "example-value" },
                    { "second-example-header", "second-example-value" },
                },
                QueryStringParameters = 
                {
                    { "example-query-string", "example-value" },
                    { "second-example-query-string", "second-example-value" },
                },
            },
        },
    });

});
```
```go
package main

import (
	"github.com/pulumi/pulumi-aws/sdk/v6/go/aws/pipes"
	"github.com/pulumi/pulumi/sdk/v3/go/pulumi"
)

func main() {
	pulumi.Run(func(ctx *pulumi.Context) error {
		_, err := pipes.NewPipe(ctx, "example", &pipes.PipeArgs{
			Name:       pulumi.String("example-pipe"),
			RoleArn:    pulumi.Any(exampleAwsIamRole.Arn),
			Source:     pulumi.Any(source.Arn),
			Target:     pulumi.Any(target.Arn),
			Enrichment: pulumi.Any(exampleAwsCloudwatchEventApiDestination.Arn),
			EnrichmentParameters: &pipes.PipeEnrichmentParametersArgs{
				HttpParameters: &pipes.PipeEnrichmentParametersHttpParametersArgs{
					PathParameterValues: pulumi.String("example-path-param"),
					HeaderParameters: pulumi.StringMap{
						"example-header":        pulumi.String("example-value"),
						"second-example-header": pulumi.String("second-example-value"),
					},
					QueryStringParameters: pulumi.StringMap{
						"example-query-string":        pulumi.String("example-value"),
						"second-example-query-string": pulumi.String("second-example-value"),
					},
				},
			},
		})
		if err != nil {
			return err
		}
		return nil
	})
}
```
```java
package generated_program;

import com.pulumi.Context;
import com.pulumi.Pulumi;
import com.pulumi.core.Output;
import com.pulumi.aws.pipes.Pipe;
import com.pulumi.aws.pipes.PipeArgs;
import com.pulumi.aws.pipes.inputs.PipeEnrichmentParametersArgs;
import com.pulumi.aws.pipes.inputs.PipeEnrichmentParametersHttpParametersArgs;
import java.util.List;
import java.util.ArrayList;
import java.util.Map;
import java.io.File;
import java.nio.file.Files;
import java.nio.file.Paths;

public class App {
    public static void main(String[] args) {
        Pulumi.run(App::stack);
    }

    public static void stack(Context ctx) {
        var example = new Pipe("example", PipeArgs.builder()
            .name("example-pipe")
            .roleArn(exampleAwsIamRole.arn())
            .source(source.arn())
            .target(target.arn())
            .enrichment(exampleAwsCloudwatchEventApiDestination.arn())
            .enrichmentParameters(PipeEnrichmentParametersArgs.builder()
                .httpParameters(PipeEnrichmentParametersHttpParametersArgs.builder()
                    .pathParameterValues("example-path-param")
                    .headerParameters(Map.ofEntries(
                        Map.entry("example-header", "example-value"),
                        Map.entry("second-example-header", "second-example-value")
                    ))
                    .queryStringParameters(Map.ofEntries(
                        Map.entry("example-query-string", "example-value"),
                        Map.entry("second-example-query-string", "second-example-value")
                    ))
                    .build())
                .build())
            .build());

    }
}
```
```yaml
resources:
  example:
    type: aws:pipes:Pipe
    properties:
      name: example-pipe
      roleArn: ${exampleAwsIamRole.arn}
      source: ${source.arn}
      target: ${target.arn}
      enrichment: ${exampleAwsCloudwatchEventApiDestination.arn}
      enrichmentParameters:
        httpParameters:
          pathParameterValues: example-path-param
          headerParameters:
            example-header: example-value
            second-example-header: second-example-value
          queryStringParameters:
            example-query-string: example-value
            second-example-query-string: second-example-value
```
<!--End PulumiCodeChooser -->

### Filter Usage

<!--Start PulumiCodeChooser -->
```typescript
import * as pulumi from "@pulumi/pulumi";
import * as aws from "@pulumi/aws";

const example = new aws.pipes.Pipe("example", {
    name: "example-pipe",
    roleArn: exampleAwsIamRole.arn,
    source: source.arn,
    target: target.arn,
    sourceParameters: {
        filterCriteria: {
            filters: [{
                pattern: JSON.stringify({
                    source: ["event-source"],
                }),
            }],
        },
    },
});
```
```python
import pulumi
import json
import pulumi_aws as aws

example = aws.pipes.Pipe("example",
    name="example-pipe",
    role_arn=example_aws_iam_role["arn"],
    source=source["arn"],
    target=target["arn"],
    source_parameters={
        "filter_criteria": {
            "filters": [{
                "pattern": json.dumps({
                    "source": ["event-source"],
                }),
            }],
        },
    })
```
```csharp
using System.Collections.Generic;
using System.Linq;
using System.Text.Json;
using Pulumi;
using Aws = Pulumi.Aws;

return await Deployment.RunAsync(() => 
{
    var example = new Aws.Pipes.Pipe("example", new()
    {
        Name = "example-pipe",
        RoleArn = exampleAwsIamRole.Arn,
        Source = source.Arn,
        Target = target.Arn,
        SourceParameters = new Aws.Pipes.Inputs.PipeSourceParametersArgs
        {
            FilterCriteria = new Aws.Pipes.Inputs.PipeSourceParametersFilterCriteriaArgs
            {
                Filters = new[]
                {
                    new Aws.Pipes.Inputs.PipeSourceParametersFilterCriteriaFilterArgs
                    {
                        Pattern = JsonSerializer.Serialize(new Dictionary<string, object?>
                        {
                            ["source"] = new[]
                            {
                                "event-source",
                            },
                        }),
                    },
                },
            },
        },
    });

});
```
```go
package main

import (
	"encoding/json"

	"github.com/pulumi/pulumi-aws/sdk/v6/go/aws/pipes"
	"github.com/pulumi/pulumi/sdk/v3/go/pulumi"
)

func main() {
	pulumi.Run(func(ctx *pulumi.Context) error {
		tmpJSON0, err := json.Marshal(map[string]interface{}{
			"source": []string{
				"event-source",
			},
		})
		if err != nil {
			return err
		}
		json0 := string(tmpJSON0)
		_, err = pipes.NewPipe(ctx, "example", &pipes.PipeArgs{
			Name:    pulumi.String("example-pipe"),
			RoleArn: pulumi.Any(exampleAwsIamRole.Arn),
			Source:  pulumi.Any(source.Arn),
			Target:  pulumi.Any(target.Arn),
			SourceParameters: &pipes.PipeSourceParametersArgs{
				FilterCriteria: &pipes.PipeSourceParametersFilterCriteriaArgs{
					Filters: pipes.PipeSourceParametersFilterCriteriaFilterArray{
						&pipes.PipeSourceParametersFilterCriteriaFilterArgs{
							Pattern: pulumi.String(json0),
						},
					},
				},
			},
		})
		if err != nil {
			return err
		}
		return nil
	})
}
```
```java
package generated_program;

import com.pulumi.Context;
import com.pulumi.Pulumi;
import com.pulumi.core.Output;
import com.pulumi.aws.pipes.Pipe;
import com.pulumi.aws.pipes.PipeArgs;
import com.pulumi.aws.pipes.inputs.PipeSourceParametersArgs;
import com.pulumi.aws.pipes.inputs.PipeSourceParametersFilterCriteriaArgs;
import static com.pulumi.codegen.internal.Serialization.*;
import java.util.List;
import java.util.ArrayList;
import java.util.Map;
import java.io.File;
import java.nio.file.Files;
import java.nio.file.Paths;

public class App {
    public static void main(String[] args) {
        Pulumi.run(App::stack);
    }

    public static void stack(Context ctx) {
        var example = new Pipe("example", PipeArgs.builder()
            .name("example-pipe")
            .roleArn(exampleAwsIamRole.arn())
            .source(source.arn())
            .target(target.arn())
            .sourceParameters(PipeSourceParametersArgs.builder()
                .filterCriteria(PipeSourceParametersFilterCriteriaArgs.builder()
                    .filters(PipeSourceParametersFilterCriteriaFilterArgs.builder()
                        .pattern(serializeJson(
                            jsonObject(
                                jsonProperty("source", jsonArray("event-source"))
                            )))
                        .build())
                    .build())
                .build())
            .build());

    }
}
```
```yaml
resources:
  example:
    type: aws:pipes:Pipe
    properties:
      name: example-pipe
      roleArn: ${exampleAwsIamRole.arn}
      source: ${source.arn}
      target: ${target.arn}
      sourceParameters:
        filterCriteria:
          filters:
            - pattern:
                fn::toJSON:
                  source:
                    - event-source
```
<!--End PulumiCodeChooser -->

### CloudWatch Logs Logging Configuration Usage

<!--Start PulumiCodeChooser -->
```typescript
import * as pulumi from "@pulumi/pulumi";
import * as aws from "@pulumi/aws";

const example = new aws.cloudwatch.LogGroup("example", {name: "example-pipe-target"});
const examplePipe = new aws.pipes.Pipe("example", {
    name: "example-pipe",
    roleArn: exampleAwsIamRole.arn,
    source: sourceAwsSqsQueue.arn,
    target: targetAwsSqsQueue.arn,
    logConfiguration: {
        includeExecutionDatas: ["ALL"],
        level: "INFO",
        cloudwatchLogsLogDestination: {
            logGroupArn: targetAwsCloudwatchLogGroup.arn,
        },
    },
}, {
    dependsOn: [
        source,
        target,
    ],
});
```
```python
import pulumi
import pulumi_aws as aws

example = aws.cloudwatch.LogGroup("example", name="example-pipe-target")
example_pipe = aws.pipes.Pipe("example",
    name="example-pipe",
    role_arn=example_aws_iam_role["arn"],
    source=source_aws_sqs_queue["arn"],
    target=target_aws_sqs_queue["arn"],
    log_configuration={
        "include_execution_datas": ["ALL"],
        "level": "INFO",
        "cloudwatch_logs_log_destination": {
            "log_group_arn": target_aws_cloudwatch_log_group["arn"],
        },
    },
    opts = pulumi.ResourceOptions(depends_on=[
            source,
            target,
        ]))
```
```csharp
using System.Collections.Generic;
using System.Linq;
using Pulumi;
using Aws = Pulumi.Aws;

return await Deployment.RunAsync(() => 
{
    var example = new Aws.CloudWatch.LogGroup("example", new()
    {
        Name = "example-pipe-target",
    });

    var examplePipe = new Aws.Pipes.Pipe("example", new()
    {
        Name = "example-pipe",
        RoleArn = exampleAwsIamRole.Arn,
        Source = sourceAwsSqsQueue.Arn,
        Target = targetAwsSqsQueue.Arn,
        LogConfiguration = new Aws.Pipes.Inputs.PipeLogConfigurationArgs
        {
            IncludeExecutionDatas = new[]
            {
                "ALL",
            },
            Level = "INFO",
            CloudwatchLogsLogDestination = new Aws.Pipes.Inputs.PipeLogConfigurationCloudwatchLogsLogDestinationArgs
            {
                LogGroupArn = targetAwsCloudwatchLogGroup.Arn,
            },
        },
    }, new CustomResourceOptions
    {
        DependsOn =
        {
            source,
            target,
        },
    });

});
```
```go
package main

import (
	"github.com/pulumi/pulumi-aws/sdk/v6/go/aws/cloudwatch"
	"github.com/pulumi/pulumi-aws/sdk/v6/go/aws/pipes"
	"github.com/pulumi/pulumi/sdk/v3/go/pulumi"
)

func main() {
	pulumi.Run(func(ctx *pulumi.Context) error {
		_, err := cloudwatch.NewLogGroup(ctx, "example", &cloudwatch.LogGroupArgs{
			Name: pulumi.String("example-pipe-target"),
		})
		if err != nil {
			return err
		}
		_, err = pipes.NewPipe(ctx, "example", &pipes.PipeArgs{
			Name:    pulumi.String("example-pipe"),
			RoleArn: pulumi.Any(exampleAwsIamRole.Arn),
			Source:  pulumi.Any(sourceAwsSqsQueue.Arn),
			Target:  pulumi.Any(targetAwsSqsQueue.Arn),
			LogConfiguration: &pipes.PipeLogConfigurationArgs{
				IncludeExecutionDatas: pulumi.StringArray{
					pulumi.String("ALL"),
				},
				Level: pulumi.String("INFO"),
				CloudwatchLogsLogDestination: &pipes.PipeLogConfigurationCloudwatchLogsLogDestinationArgs{
					LogGroupArn: pulumi.Any(targetAwsCloudwatchLogGroup.Arn),
				},
			},
		}, pulumi.DependsOn([]pulumi.Resource{
			source,
			target,
		}))
		if err != nil {
			return err
		}
		return nil
	})
}
```
```java
package generated_program;

import com.pulumi.Context;
import com.pulumi.Pulumi;
import com.pulumi.core.Output;
import com.pulumi.aws.cloudwatch.LogGroup;
import com.pulumi.aws.cloudwatch.LogGroupArgs;
import com.pulumi.aws.pipes.Pipe;
import com.pulumi.aws.pipes.PipeArgs;
import com.pulumi.aws.pipes.inputs.PipeLogConfigurationArgs;
import com.pulumi.aws.pipes.inputs.PipeLogConfigurationCloudwatchLogsLogDestinationArgs;
import com.pulumi.resources.CustomResourceOptions;
import java.util.List;
import java.util.ArrayList;
import java.util.Map;
import java.io.File;
import java.nio.file.Files;
import java.nio.file.Paths;

public class App {
    public static void main(String[] args) {
        Pulumi.run(App::stack);
    }

    public static void stack(Context ctx) {
        var example = new LogGroup("example", LogGroupArgs.builder()
            .name("example-pipe-target")
            .build());

        var examplePipe = new Pipe("examplePipe", PipeArgs.builder()
            .name("example-pipe")
            .roleArn(exampleAwsIamRole.arn())
            .source(sourceAwsSqsQueue.arn())
            .target(targetAwsSqsQueue.arn())
            .logConfiguration(PipeLogConfigurationArgs.builder()
                .includeExecutionDatas("ALL")
                .level("INFO")
                .cloudwatchLogsLogDestination(PipeLogConfigurationCloudwatchLogsLogDestinationArgs.builder()
                    .logGroupArn(targetAwsCloudwatchLogGroup.arn())
                    .build())
                .build())
            .build(), CustomResourceOptions.builder()
                .dependsOn(                
                    source,
                    target)
                .build());

    }
}
```
```yaml
resources:
  example:
    type: aws:cloudwatch:LogGroup
    properties:
      name: example-pipe-target
  examplePipe:
    type: aws:pipes:Pipe
    name: example
    properties:
      name: example-pipe
      roleArn: ${exampleAwsIamRole.arn}
      source: ${sourceAwsSqsQueue.arn}
      target: ${targetAwsSqsQueue.arn}
      logConfiguration:
        includeExecutionDatas:
          - ALL
        level: INFO
        cloudwatchLogsLogDestination:
          logGroupArn: ${targetAwsCloudwatchLogGroup.arn}
    options:
      dependsOn:
        - ${source}
        - ${target}
```
<!--End PulumiCodeChooser -->

### SQS Source and Target Configuration Usage

<!--Start PulumiCodeChooser -->
```typescript
import * as pulumi from "@pulumi/pulumi";
import * as aws from "@pulumi/aws";

const example = new aws.pipes.Pipe("example", {
    name: "example-pipe",
    roleArn: exampleAwsIamRole.arn,
    source: source.arn,
    target: target.arn,
    sourceParameters: {
        sqsQueueParameters: {
            batchSize: 1,
            maximumBatchingWindowInSeconds: 2,
        },
    },
    targetParameters: {
        sqsQueueParameters: {
            messageDeduplicationId: "example-dedupe",
            messageGroupId: "example-group",
        },
    },
});
```
```python
import pulumi
import pulumi_aws as aws

example = aws.pipes.Pipe("example",
    name="example-pipe",
    role_arn=example_aws_iam_role["arn"],
    source=source["arn"],
    target=target["arn"],
    source_parameters={
        "sqs_queue_parameters": {
            "batch_size": 1,
            "maximum_batching_window_in_seconds": 2,
        },
    },
    target_parameters={
        "sqs_queue_parameters": {
            "message_deduplication_id": "example-dedupe",
            "message_group_id": "example-group",
        },
    })
```
```csharp
using System.Collections.Generic;
using System.Linq;
using Pulumi;
using Aws = Pulumi.Aws;

return await Deployment.RunAsync(() => 
{
    var example = new Aws.Pipes.Pipe("example", new()
    {
        Name = "example-pipe",
        RoleArn = exampleAwsIamRole.Arn,
        Source = source.Arn,
        Target = target.Arn,
        SourceParameters = new Aws.Pipes.Inputs.PipeSourceParametersArgs
        {
            SqsQueueParameters = new Aws.Pipes.Inputs.PipeSourceParametersSqsQueueParametersArgs
            {
                BatchSize = 1,
                MaximumBatchingWindowInSeconds = 2,
            },
        },
        TargetParameters = new Aws.Pipes.Inputs.PipeTargetParametersArgs
        {
            SqsQueueParameters = new Aws.Pipes.Inputs.PipeTargetParametersSqsQueueParametersArgs
            {
                MessageDeduplicationId = "example-dedupe",
                MessageGroupId = "example-group",
            },
        },
    });

});
```
```go
package main

import (
	"github.com/pulumi/pulumi-aws/sdk/v6/go/aws/pipes"
	"github.com/pulumi/pulumi/sdk/v3/go/pulumi"
)

func main() {
	pulumi.Run(func(ctx *pulumi.Context) error {
		_, err := pipes.NewPipe(ctx, "example", &pipes.PipeArgs{
			Name:    pulumi.String("example-pipe"),
			RoleArn: pulumi.Any(exampleAwsIamRole.Arn),
			Source:  pulumi.Any(source.Arn),
			Target:  pulumi.Any(target.Arn),
			SourceParameters: &pipes.PipeSourceParametersArgs{
				SqsQueueParameters: &pipes.PipeSourceParametersSqsQueueParametersArgs{
					BatchSize:                      pulumi.Int(1),
					MaximumBatchingWindowInSeconds: pulumi.Int(2),
				},
			},
			TargetParameters: &pipes.PipeTargetParametersArgs{
				SqsQueueParameters: &pipes.PipeTargetParametersSqsQueueParametersArgs{
					MessageDeduplicationId: pulumi.String("example-dedupe"),
					MessageGroupId:         pulumi.String("example-group"),
				},
			},
		})
		if err != nil {
			return err
		}
		return nil
	})
}
```
```java
package generated_program;

import com.pulumi.Context;
import com.pulumi.Pulumi;
import com.pulumi.core.Output;
import com.pulumi.aws.pipes.Pipe;
import com.pulumi.aws.pipes.PipeArgs;
import com.pulumi.aws.pipes.inputs.PipeSourceParametersArgs;
import com.pulumi.aws.pipes.inputs.PipeSourceParametersSqsQueueParametersArgs;
import com.pulumi.aws.pipes.inputs.PipeTargetParametersArgs;
import com.pulumi.aws.pipes.inputs.PipeTargetParametersSqsQueueParametersArgs;
import java.util.List;
import java.util.ArrayList;
import java.util.Map;
import java.io.File;
import java.nio.file.Files;
import java.nio.file.Paths;

public class App {
    public static void main(String[] args) {
        Pulumi.run(App::stack);
    }

    public static void stack(Context ctx) {
        var example = new Pipe("example", PipeArgs.builder()
            .name("example-pipe")
            .roleArn(exampleAwsIamRole.arn())
            .source(source.arn())
            .target(target.arn())
            .sourceParameters(PipeSourceParametersArgs.builder()
                .sqsQueueParameters(PipeSourceParametersSqsQueueParametersArgs.builder()
                    .batchSize(1)
                    .maximumBatchingWindowInSeconds(2)
                    .build())
                .build())
            .targetParameters(PipeTargetParametersArgs.builder()
                .sqsQueueParameters(PipeTargetParametersSqsQueueParametersArgs.builder()
                    .messageDeduplicationId("example-dedupe")
                    .messageGroupId("example-group")
                    .build())
                .build())
            .build());

    }
}
```
```yaml
resources:
  example:
    type: aws:pipes:Pipe
    properties:
      name: example-pipe
      roleArn: ${exampleAwsIamRole.arn}
      source: ${source.arn}
      target: ${target.arn}
      sourceParameters:
        sqsQueueParameters:
          batchSize: 1
          maximumBatchingWindowInSeconds: 2
      targetParameters:
        sqsQueueParameters:
          messageDeduplicationId: example-dedupe
          messageGroupId: example-group
```
<!--End PulumiCodeChooser -->

## Import

Using `pulumi import`, import pipes using the `name`. For example:

```sh
$ pulumi import aws:pipes/pipe:Pipe example my-pipe
```
H
descriptionB" 3A description of the pipe. At most 512 characters.
U
desiredStateB" ?The state the pipe should be in. One of: `RUNNING`, `STOPPED`.
�

enrichmentB" �Enrichment resource of the pipe (typically an ARN). Read more about enrichment in the [User Guide](https://docs.aws.amazon.com/eventbridge/latest/userguide/eb-pipes.html#pipes-enrichment).
�
enrichmentParametersdBb:`
^
pipesPipeEnrichmentParameters;aws:pipes/PipeEnrichmentParameters:PipeEnrichmentParametersBParameters to configure enrichment for your pipe. Detailed below.
�
logConfigurationXBV:T
R
pipesPipeLogConfiguration3aws:pipes/PipeLogConfiguration:PipeLogConfiguration=Logging configuration settings for the pipe. Detailed below.
z
nameB" lName of the pipe. If omitted, the provider will assign a random, unique name. Conflicts with `name_prefix`.
f

namePrefixB" RCreates a unique name beginning with the specified prefix. Conflicts with `name`.
P
roleArn" AARN of the role that allows the pipe to send data to the target.
�
source" �Source resource of the pipe. This field typically requires an ARN (Amazon Resource Name). However, when using a self-managed Kafka cluster, you should use a different format. Instead of an ARN, use 'smk://' followed by the bootstrap server's address.
�
sourceParametersXBV:T
R
pipesPipeSourceParameters3aws:pipes/PipeSourceParameters:PipeSourceParameters?Parameters to configure a source for the pipe. Detailed below.
�
tagsB2" �Key-value mapping of resource tags. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
e
target" WTarget resource of the pipe (typically an ARN).

The following arguments are optional:
�
targetParametersXBV:T
R
pipesPipeTargetParameters3aws:pipes/PipeTargetParameters:PipeTargetParameters@Parameters to configure a target for your pipe. Detailed below.
"
arn" ARN of this pipe.
"H
descriptionB" 3A description of the pipe. At most 512 characters.
"U
desiredStateB" ?The state the pipe should be in. One of: `RUNNING`, `STOPPED`.
"�

enrichmentB" �Enrichment resource of the pipe (typically an ARN). Read more about enrichment in the [User Guide](https://docs.aws.amazon.com/eventbridge/latest/userguide/eb-pipes.html#pipes-enrichment).
"�
enrichmentParametersdBb:`
^
pipesPipeEnrichmentParameters;aws:pipes/PipeEnrichmentParameters:PipeEnrichmentParametersBParameters to configure enrichment for your pipe. Detailed below.
"�
logConfigurationXBV:T
R
pipesPipeLogConfiguration3aws:pipes/PipeLogConfiguration:PipeLogConfiguration=Logging configuration settings for the pipe. Detailed below.
"x
name" lName of the pipe. If omitted, the provider will assign a random, unique name. Conflicts with `name_prefix`.
"d

namePrefix" RCreates a unique name beginning with the specified prefix. Conflicts with `name`.
"P
roleArn" AARN of the role that allows the pipe to send data to the target.
"�
source" �Source resource of the pipe. This field typically requires an ARN (Amazon Resource Name). However, when using a self-managed Kafka cluster, you should use a different format. Instead of an ARN, use 'smk://' followed by the bootstrap server's address.
"�
sourceParametersV:T
R
pipesPipeSourceParameters3aws:pipes/PipeSourceParameters:PipeSourceParameters?Parameters to configure a source for the pipe. Detailed below.
"�
tagsB2" �Key-value mapping of resource tags. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
"�
tagsAll2" vMap of tags assigned to the resource, including those inherited from the provider `default_tags` configuration block.
"e
target" WTarget resource of the pipe (typically an ARN).

The following arguments are optional:
"�
targetParametersXBV:T
R
pipesPipeTargetParameters3aws:pipes/PipeTargetParameters:PipeTargetParameters@Parameters to configure a target for your pipe. Detailed below.
*�%
&
qldbLedgeraws:qldb/ledger:Ledger�Provides an AWS Quantum Ledger Database (QLDB) resource

> **NOTE:** Deletion protection is enabled by default. To successfully delete this resource via this provider, `deletion_protection = false` must be applied before attempting deletion.

## Example Usage

<!--Start PulumiCodeChooser -->
```typescript
import * as pulumi from "@pulumi/pulumi";
import * as aws from "@pulumi/aws";

const sample_ledger = new aws.qldb.Ledger("sample-ledger", {
    name: "sample-ledger",
    permissionsMode: "STANDARD",
});
```
```python
import pulumi
import pulumi_aws as aws

sample_ledger = aws.qldb.Ledger("sample-ledger",
    name="sample-ledger",
    permissions_mode="STANDARD")
```
```csharp
using System.Collections.Generic;
using System.Linq;
using Pulumi;
using Aws = Pulumi.Aws;

return await Deployment.RunAsync(() => 
{
    var sample_ledger = new Aws.Qldb.Ledger("sample-ledger", new()
    {
        Name = "sample-ledger",
        PermissionsMode = "STANDARD",
    });

});
```
```go
package main

import (
	"github.com/pulumi/pulumi-aws/sdk/v6/go/aws/qldb"
	"github.com/pulumi/pulumi/sdk/v3/go/pulumi"
)

func main() {
	pulumi.Run(func(ctx *pulumi.Context) error {
		_, err := qldb.NewLedger(ctx, "sample-ledger", &qldb.LedgerArgs{
			Name:            pulumi.String("sample-ledger"),
			PermissionsMode: pulumi.String("STANDARD"),
		})
		if err != nil {
			return err
		}
		return nil
	})
}
```
```java
package generated_program;

import com.pulumi.Context;
import com.pulumi.Pulumi;
import com.pulumi.core.Output;
import com.pulumi.aws.qldb.Ledger;
import com.pulumi.aws.qldb.LedgerArgs;
import java.util.List;
import java.util.ArrayList;
import java.util.Map;
import java.io.File;
import java.nio.file.Files;
import java.nio.file.Paths;

public class App {
    public static void main(String[] args) {
        Pulumi.run(App::stack);
    }

    public static void stack(Context ctx) {
        var sample_ledger = new Ledger("sample-ledger", LedgerArgs.builder()
            .name("sample-ledger")
            .permissionsMode("STANDARD")
            .build());

    }
}
```
```yaml
resources:
  sample-ledger:
    type: aws:qldb:Ledger
    properties:
      name: sample-ledger
      permissionsMode: STANDARD
```
<!--End PulumiCodeChooser -->

## Import

Using `pulumi import`, import QLDB Ledgers using the `name`. For example:

```sh
$ pulumi import aws:qldb/ledger:Ledger sample-ledger sample-ledger
```
�
deletionProtectionB
 �The deletion protection for the QLDB Ledger instance. By default it is `true`. To delete this resource via the provider, this value must be configured to `false` and applied first before attempting deletion.
�
kmsKeyB" �The key in AWS Key Management Service (AWS KMS) to use for encryption of data at rest in the ledger. For more information, see the [AWS documentation](https://docs.aws.amazon.com/qldb/latest/developerguide/encryption-at-rest.html). Valid values are `"AWS_OWNED_KMS_KEY"` to use an AWS KMS key that is owned and managed by AWS on your behalf, or the ARN of a valid symmetric customer managed KMS key.
d
nameB" VThe friendly name for the QLDB Ledger instance. By default generated by the provider.
t
permissionsMode" ]The permissions mode for the QLDB ledger instance. Specify either `ALLOW_ALL` or `STANDARD`.
�
tagsB2" �Key-value map of resource tags. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
"&
arn" The ARN of the QLDB Ledger
"�
deletionProtectionB
 �The deletion protection for the QLDB Ledger instance. By default it is `true`. To delete this resource via the provider, this value must be configured to `false` and applied first before attempting deletion.
"�
kmsKey" �The key in AWS Key Management Service (AWS KMS) to use for encryption of data at rest in the ledger. For more information, see the [AWS documentation](https://docs.aws.amazon.com/qldb/latest/developerguide/encryption-at-rest.html). Valid values are `"AWS_OWNED_KMS_KEY"` to use an AWS KMS key that is owned and managed by AWS on your behalf, or the ARN of a valid symmetric customer managed KMS key.
"b
name" VThe friendly name for the QLDB Ledger instance. By default generated by the provider.
"t
permissionsMode" ]The permissions mode for the QLDB ledger instance. Specify either `ALLOW_ALL` or `STANDARD`.
"�
tagsB2" �Key-value map of resource tags. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
"�
tagsAll2" xA map of tags assigned to the resource, including those inherited from the provider `default_tags` configuration block.
*�?
&
qldbStreamaws:qldb/stream:Stream�!Provides an AWS Quantum Ledger Database (QLDB) Stream resource

## Example Usage

<!--Start PulumiCodeChooser -->
```typescript
import * as pulumi from "@pulumi/pulumi";
import * as aws from "@pulumi/aws";

const example = new aws.qldb.Stream("example", {
    ledgerName: "existing-ledger-name",
    streamName: "sample-ledger-stream",
    roleArn: "sample-role-arn",
    inclusiveStartTime: "2021-01-01T00:00:00Z",
    kinesisConfiguration: {
        aggregationEnabled: false,
        streamArn: "arn:aws:kinesis:us-east-1:xxxxxxxxxxxx:stream/example-kinesis-stream",
    },
    tags: {
        example: "tag",
    },
});
```
```python
import pulumi
import pulumi_aws as aws

example = aws.qldb.Stream("example",
    ledger_name="existing-ledger-name",
    stream_name="sample-ledger-stream",
    role_arn="sample-role-arn",
    inclusive_start_time="2021-01-01T00:00:00Z",
    kinesis_configuration={
        "aggregation_enabled": False,
        "stream_arn": "arn:aws:kinesis:us-east-1:xxxxxxxxxxxx:stream/example-kinesis-stream",
    },
    tags={
        "example": "tag",
    })
```
```csharp
using System.Collections.Generic;
using System.Linq;
using Pulumi;
using Aws = Pulumi.Aws;

return await Deployment.RunAsync(() => 
{
    var example = new Aws.Qldb.Stream("example", new()
    {
        LedgerName = "existing-ledger-name",
        StreamName = "sample-ledger-stream",
        RoleArn = "sample-role-arn",
        InclusiveStartTime = "2021-01-01T00:00:00Z",
        KinesisConfiguration = new Aws.Qldb.Inputs.StreamKinesisConfigurationArgs
        {
            AggregationEnabled = false,
            StreamArn = "arn:aws:kinesis:us-east-1:xxxxxxxxxxxx:stream/example-kinesis-stream",
        },
        Tags = 
        {
            { "example", "tag" },
        },
    });

});
```
```go
package main

import (
	"github.com/pulumi/pulumi-aws/sdk/v6/go/aws/qldb"
	"github.com/pulumi/pulumi/sdk/v3/go/pulumi"
)

func main() {
	pulumi.Run(func(ctx *pulumi.Context) error {
		_, err := qldb.NewStream(ctx, "example", &qldb.StreamArgs{
			LedgerName:         pulumi.String("existing-ledger-name"),
			StreamName:         pulumi.String("sample-ledger-stream"),
			RoleArn:            pulumi.String("sample-role-arn"),
			InclusiveStartTime: pulumi.String("2021-01-01T00:00:00Z"),
			KinesisConfiguration: &qldb.StreamKinesisConfigurationArgs{
				AggregationEnabled: pulumi.Bool(false),
				StreamArn:          pulumi.String("arn:aws:kinesis:us-east-1:xxxxxxxxxxxx:stream/example-kinesis-stream"),
			},
			Tags: pulumi.StringMap{
				"example": pulumi.String("tag"),
			},
		})
		if err != nil {
			return err
		}
		return nil
	})
}
```
```java
package generated_program;

import com.pulumi.Context;
import com.pulumi.Pulumi;
import com.pulumi.core.Output;
import com.pulumi.aws.qldb.Stream;
import com.pulumi.aws.qldb.StreamArgs;
import com.pulumi.aws.qldb.inputs.StreamKinesisConfigurationArgs;
import java.util.List;
import java.util.ArrayList;
import java.util.Map;
import java.io.File;
import java.nio.file.Files;
import java.nio.file.Paths;

public class App {
    public static void main(String[] args) {
        Pulumi.run(App::stack);
    }

    public static void stack(Context ctx) {
        var example = new Stream("example", StreamArgs.builder()
            .ledgerName("existing-ledger-name")
            .streamName("sample-ledger-stream")
            .roleArn("sample-role-arn")
            .inclusiveStartTime("2021-01-01T00:00:00Z")
            .kinesisConfiguration(StreamKinesisConfigurationArgs.builder()
                .aggregationEnabled(false)
                .streamArn("arn:aws:kinesis:us-east-1:xxxxxxxxxxxx:stream/example-kinesis-stream")
                .build())
            .tags(Map.of("example", "tag"))
            .build());

    }
}
```
```yaml
resources:
  example:
    type: aws:qldb:Stream
    properties:
      ledgerName: existing-ledger-name
      streamName: sample-ledger-stream
      roleArn: sample-role-arn
      inclusiveStartTime: 2021-01-01T00:00:00Z
      kinesisConfiguration:
        aggregationEnabled: false
        streamArn: arn:aws:kinesis:us-east-1:xxxxxxxxxxxx:stream/example-kinesis-stream
      tags:
        example: tag
```
<!--End PulumiCodeChooser -->
�
exclusiveEndTimeB" �The exclusive date and time that specifies when the stream ends. If you don't define this parameter, the stream runs indefinitely until you cancel it. It must be in ISO 8601 date and time format and in Universal Coordinated Time (UTC). For example: `"2019-06-13T21:36:34Z"`.
�
inclusiveStartTime" �The inclusive start date and time from which to start streaming journal data. This parameter must be in ISO 8601 date and time format and in Universal Coordinated Time (UTC). For example: `"2019-06-13T21:36:34Z"`.  This cannot be in the future and must be before `exclusive_end_time`.  If you provide a value that is before the ledger's `CreationDateTime`, QLDB effectively defaults it to the ledger's `CreationDateTime`.
�
kinesisConfigurationf:d
b
qldbStreamKinesisConfiguration>aws:qldb/StreamKinesisConfiguration:StreamKinesisConfigurationnThe configuration settings of the Kinesis Data Streams destination for your stream request. Documented below.
/

ledgerName" The name of the QLDB ledger.
�
roleArn" �The Amazon Resource Name (ARN) of the IAM role that grants QLDB permissions for a journal stream to write data records to a Kinesis Data Streams resource.
�

streamName" �The name that you want to assign to the QLDB journal stream. User-defined names can help identify and indicate the purpose of a stream.  Your stream name must be unique among other active streams for a given ledger. Stream names have the same naming constraints as ledger names, as defined in the [Amazon QLDB Developer Guide](https://docs.aws.amazon.com/qldb/latest/developerguide/limits.html#limits.naming).
�
tagsB2" �Key-value map of resource tags. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
"'
arn" The ARN of the QLDB Stream.
"�
exclusiveEndTimeB" �The exclusive date and time that specifies when the stream ends. If you don't define this parameter, the stream runs indefinitely until you cancel it. It must be in ISO 8601 date and time format and in Universal Coordinated Time (UTC). For example: `"2019-06-13T21:36:34Z"`.
"�
inclusiveStartTime" �The inclusive start date and time from which to start streaming journal data. This parameter must be in ISO 8601 date and time format and in Universal Coordinated Time (UTC). For example: `"2019-06-13T21:36:34Z"`.  This cannot be in the future and must be before `exclusive_end_time`.  If you provide a value that is before the ledger's `CreationDateTime`, QLDB effectively defaults it to the ledger's `CreationDateTime`.
"�
kinesisConfigurationf:d
b
qldbStreamKinesisConfiguration>aws:qldb/StreamKinesisConfiguration:StreamKinesisConfigurationnThe configuration settings of the Kinesis Data Streams destination for your stream request. Documented below.
"/

ledgerName" The name of the QLDB ledger.
"�
roleArn" �The Amazon Resource Name (ARN) of the IAM role that grants QLDB permissions for a journal stream to write data records to a Kinesis Data Streams resource.
"�

streamName" �The name that you want to assign to the QLDB journal stream. User-defined names can help identify and indicate the purpose of a stream.  Your stream name must be unique among other active streams for a given ledger. Stream names have the same naming constraints as ledger names, as defined in the [Amazon QLDB Developer Guide](https://docs.aws.amazon.com/qldb/latest/developerguide/limits.html#limits.naming).
"�
tagsB2" �Key-value map of resource tags. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
"�
tagsAll2" xA map of tags assigned to the resource, including those inherited from the provider `default_tags` configuration block.
*�A
Y

quicksightAccountSubscription6aws:quicksight/accountSubscription:AccountSubscription�Resource for managing an AWS QuickSight Account Subscription.

## Example Usage

<!--Start PulumiCodeChooser -->
```typescript
import * as pulumi from "@pulumi/pulumi";
import * as aws from "@pulumi/aws";

const subscription = new aws.quicksight.AccountSubscription("subscription", {
    accountName: "quicksight-pulumi",
    authenticationMethod: "IAM_AND_QUICKSIGHT",
    edition: "ENTERPRISE",
    notificationEmail: "notification@email.com",
});
```
```python
import pulumi
import pulumi_aws as aws

subscription = aws.quicksight.AccountSubscription("subscription",
    account_name="quicksight-pulumi",
    authentication_method="IAM_AND_QUICKSIGHT",
    edition="ENTERPRISE",
    notification_email="notification@email.com")
```
```csharp
using System.Collections.Generic;
using System.Linq;
using Pulumi;
using Aws = Pulumi.Aws;

return await Deployment.RunAsync(() => 
{
    var subscription = new Aws.Quicksight.AccountSubscription("subscription", new()
    {
        AccountName = "quicksight-pulumi",
        AuthenticationMethod = "IAM_AND_QUICKSIGHT",
        Edition = "ENTERPRISE",
        NotificationEmail = "notification@email.com",
    });

});
```
```go
package main

import (
	"github.com/pulumi/pulumi-aws/sdk/v6/go/aws/quicksight"
	"github.com/pulumi/pulumi/sdk/v3/go/pulumi"
)

func main() {
	pulumi.Run(func(ctx *pulumi.Context) error {
		_, err := quicksight.NewAccountSubscription(ctx, "subscription", &quicksight.AccountSubscriptionArgs{
			AccountName:          pulumi.String("quicksight-pulumi"),
			AuthenticationMethod: pulumi.String("IAM_AND_QUICKSIGHT"),
			Edition:              pulumi.String("ENTERPRISE"),
			NotificationEmail:    pulumi.String("notification@email.com"),
		})
		if err != nil {
			return err
		}
		return nil
	})
}
```
```java
package generated_program;

import com.pulumi.Context;
import com.pulumi.Pulumi;
import com.pulumi.core.Output;
import com.pulumi.aws.quicksight.AccountSubscription;
import com.pulumi.aws.quicksight.AccountSubscriptionArgs;
import java.util.List;
import java.util.ArrayList;
import java.util.Map;
import java.io.File;
import java.nio.file.Files;
import java.nio.file.Paths;

public class App {
    public static void main(String[] args) {
        Pulumi.run(App::stack);
    }

    public static void stack(Context ctx) {
        var subscription = new AccountSubscription("subscription", AccountSubscriptionArgs.builder()
            .accountName("quicksight-pulumi")
            .authenticationMethod("IAM_AND_QUICKSIGHT")
            .edition("ENTERPRISE")
            .notificationEmail("notification@email.com")
            .build());

    }
}
```
```yaml
resources:
  subscription:
    type: aws:quicksight:AccountSubscription
    properties:
      accountName: quicksight-pulumi
      authenticationMethod: IAM_AND_QUICKSIGHT
      edition: ENTERPRISE
      notificationEmail: notification@email.com
```
<!--End PulumiCodeChooser -->

## Import

You cannot import this resource.

�
accountName" uName of your Amazon QuickSight account. This name is unique over all of AWS, and it appears only when users sign in.
�
activeDirectoryNameB" �Name of your Active Directory. This field is required if `ACTIVE_DIRECTORY` is the selected authentication method of the new Amazon QuickSight account.
�
adminGroupsB*" �Admin group associated with your Active Directory. This field is required if `ACTIVE_DIRECTORY` is the selected authentication method of the new Amazon QuickSight account.
�
authenticationMethod" �Method that you want to use to authenticate your Amazon QuickSight account. Currently, the valid values for this parameter are `IAM_AND_QUICKSIGHT`, `IAM_ONLY`, `IAM_IDENTITY_CENTER`, and `ACTIVE_DIRECTORY`.
L
authorGroupsB*" 4Author group associated with your Active Directory.
b
awsAccountIdB" LAWS account ID hosting the QuickSight account. Default to provider account.
�
contactNumberB" �A 10-digit phone number for the author of the Amazon QuickSight account to use for future communications. This field is required if `ENTERPPRISE_AND_Q` is the selected edition of the new Amazon QuickSight account.
a
directoryIdB" LActive Directory ID that is associated with your Amazon QuickSight account.
�
edition" �Edition of Amazon QuickSight that you want your account to have. Currently, you can choose from `STANDARD`, `ENTERPRISE` or `ENTERPRISE_AND_Q`.
�
emailAddressB" �Email address of the author of the Amazon QuickSight account to use for future communications. This field is required if `ENTERPPRISE_AND_Q` is the selected edition of the new Amazon QuickSight account.
�
	firstNameB" �First name of the author of the Amazon QuickSight account to use for future communications. This field is required if `ENTERPPRISE_AND_Q` is the selected edition of the new Amazon QuickSight account.
k
iamIdentityCenterInstanceArnB" EThe Amazon Resource Name (ARN) for the IAM Identity Center instance.
�
lastNameB" �Last name of the author of the Amazon QuickSight account to use for future communications. This field is required if `ENTERPPRISE_AND_Q` is the selected edition of the new Amazon QuickSight account.
�
notificationEmail" �Email address that you want Amazon QuickSight to send notifications to regarding your Amazon QuickSight account or Amazon QuickSight subscription.

The following arguments are optional:
M
readerGroupsB*" 5Reader group associated with your Active Direcrtory.
e
realmB" VRealm of the Active Directory that is associated with your Amazon QuickSight account.
"�
accountName" uName of your Amazon QuickSight account. This name is unique over all of AWS, and it appears only when users sign in.
"Y
accountSubscriptionStatus" 8Status of the Amazon QuickSight account's subscription.
"�
activeDirectoryNameB" �Name of your Active Directory. This field is required if `ACTIVE_DIRECTORY` is the selected authentication method of the new Amazon QuickSight account.
"�
adminGroupsB*" �Admin group associated with your Active Directory. This field is required if `ACTIVE_DIRECTORY` is the selected authentication method of the new Amazon QuickSight account.
"�
authenticationMethod" �Method that you want to use to authenticate your Amazon QuickSight account. Currently, the valid values for this parameter are `IAM_AND_QUICKSIGHT`, `IAM_ONLY`, `IAM_IDENTITY_CENTER`, and `ACTIVE_DIRECTORY`.
"L
authorGroupsB*" 4Author group associated with your Active Directory.
"`
awsAccountId" LAWS account ID hosting the QuickSight account. Default to provider account.
"�
contactNumberB" �A 10-digit phone number for the author of the Amazon QuickSight account to use for future communications. This field is required if `ENTERPPRISE_AND_Q` is the selected edition of the new Amazon QuickSight account.
"a
directoryIdB" LActive Directory ID that is associated with your Amazon QuickSight account.
"�
edition" �Edition of Amazon QuickSight that you want your account to have. Currently, you can choose from `STANDARD`, `ENTERPRISE` or `ENTERPRISE_AND_Q`.
"�
emailAddressB" �Email address of the author of the Amazon QuickSight account to use for future communications. This field is required if `ENTERPPRISE_AND_Q` is the selected edition of the new Amazon QuickSight account.
"�
	firstNameB" �First name of the author of the Amazon QuickSight account to use for future communications. This field is required if `ENTERPPRISE_AND_Q` is the selected edition of the new Amazon QuickSight account.
"k
iamIdentityCenterInstanceArnB" EThe Amazon Resource Name (ARN) for the IAM Identity Center instance.
"�
lastNameB" �Last name of the author of the Amazon QuickSight account to use for future communications. This field is required if `ENTERPPRISE_AND_Q` is the selected edition of the new Amazon QuickSight account.
"�
notificationEmail" �Email address that you want Amazon QuickSight to send notifications to regarding your Amazon QuickSight account or Amazon QuickSight subscription.

The following arguments are optional:
"M
readerGroupsB*" 5Reader group associated with your Active Direcrtory.
"e
realmB" VRealm of the Active Directory that is associated with your Amazon QuickSight account.
*�K
8

quicksightAnalysis aws:quicksight/analysis:Analysis�.Resource for managing a QuickSight Analysis.

## Example Usage

### From Source Template

<!--Start PulumiCodeChooser -->
```typescript
import * as pulumi from "@pulumi/pulumi";
import * as aws from "@pulumi/aws";

const example = new aws.quicksight.Analysis("example", {
    analysisId: "example-id",
    name: "example-name",
    sourceEntity: {
        sourceTemplate: {
            arn: source.arn,
            dataSetReferences: [{
                dataSetArn: dataset.arn,
                dataSetPlaceholder: "1",
            }],
        },
    },
});
```
```python
import pulumi
import pulumi_aws as aws

example = aws.quicksight.Analysis("example",
    analysis_id="example-id",
    name="example-name",
    source_entity={
        "source_template": {
            "arn": source["arn"],
            "data_set_references": [{
                "data_set_arn": dataset["arn"],
                "data_set_placeholder": "1",
            }],
        },
    })
```
```csharp
using System.Collections.Generic;
using System.Linq;
using Pulumi;
using Aws = Pulumi.Aws;

return await Deployment.RunAsync(() => 
{
    var example = new Aws.Quicksight.Analysis("example", new()
    {
        AnalysisId = "example-id",
        Name = "example-name",
        SourceEntity = new Aws.Quicksight.Inputs.AnalysisSourceEntityArgs
        {
            SourceTemplate = new Aws.Quicksight.Inputs.AnalysisSourceEntitySourceTemplateArgs
            {
                Arn = source.Arn,
                DataSetReferences = new[]
                {
                    new Aws.Quicksight.Inputs.AnalysisSourceEntitySourceTemplateDataSetReferenceArgs
                    {
                        DataSetArn = dataset.Arn,
                        DataSetPlaceholder = "1",
                    },
                },
            },
        },
    });

});
```
```go
package main

import (
	"github.com/pulumi/pulumi-aws/sdk/v6/go/aws/quicksight"
	"github.com/pulumi/pulumi/sdk/v3/go/pulumi"
)

func main() {
	pulumi.Run(func(ctx *pulumi.Context) error {
		_, err := quicksight.NewAnalysis(ctx, "example", &quicksight.AnalysisArgs{
			AnalysisId: pulumi.String("example-id"),
			Name:       pulumi.String("example-name"),
			SourceEntity: &quicksight.AnalysisSourceEntityArgs{
				SourceTemplate: &quicksight.AnalysisSourceEntitySourceTemplateArgs{
					Arn: pulumi.Any(source.Arn),
					DataSetReferences: quicksight.AnalysisSourceEntitySourceTemplateDataSetReferenceArray{
						&quicksight.AnalysisSourceEntitySourceTemplateDataSetReferenceArgs{
							DataSetArn:         pulumi.Any(dataset.Arn),
							DataSetPlaceholder: pulumi.String("1"),
						},
					},
				},
			},
		})
		if err != nil {
			return err
		}
		return nil
	})
}
```
```java
package generated_program;

import com.pulumi.Context;
import com.pulumi.Pulumi;
import com.pulumi.core.Output;
import com.pulumi.aws.quicksight.Analysis;
import com.pulumi.aws.quicksight.AnalysisArgs;
import com.pulumi.aws.quicksight.inputs.AnalysisSourceEntityArgs;
import com.pulumi.aws.quicksight.inputs.AnalysisSourceEntitySourceTemplateArgs;
import java.util.List;
import java.util.ArrayList;
import java.util.Map;
import java.io.File;
import java.nio.file.Files;
import java.nio.file.Paths;

public class App {
    public static void main(String[] args) {
        Pulumi.run(App::stack);
    }

    public static void stack(Context ctx) {
        var example = new Analysis("example", AnalysisArgs.builder()
            .analysisId("example-id")
            .name("example-name")
            .sourceEntity(AnalysisSourceEntityArgs.builder()
                .sourceTemplate(AnalysisSourceEntitySourceTemplateArgs.builder()
                    .arn(source.arn())
                    .dataSetReferences(AnalysisSourceEntitySourceTemplateDataSetReferenceArgs.builder()
                        .dataSetArn(dataset.arn())
                        .dataSetPlaceholder("1")
                        .build())
                    .build())
                .build())
            .build());

    }
}
```
```yaml
resources:
  example:
    type: aws:quicksight:Analysis
    properties:
      analysisId: example-id
      name: example-name
      sourceEntity:
        sourceTemplate:
          arn: ${source.arn}
          dataSetReferences:
            - dataSetArn: ${dataset.arn}
              dataSetPlaceholder: '1'
```
<!--End PulumiCodeChooser -->

### With Definition

<!--Start PulumiCodeChooser -->
```yaml
resources:
  example:
    type: aws:quicksight:Analysis
    properties:
      analysisId: example-id
      name: example-name
      definition:
        dataSetIdentifiersDeclarations:
          - dataSetArn: ${dataset.arn}
            identifier: '1'
        sheets:
          - title: Example
            sheetId: Example1
            visuals:
              - lineChartVisual:
                  visualId: LineChart
                  title:
                    formatText:
                      plainText: Line Chart Example
                  chartConfiguration:
                    fieldWells:
                      lineChartAggregatedFieldWells:
                        categories:
                          - categoricalDimensionField:
                              fieldId: '1'
                              column:
                                dataSetIdentifier: '1'
                                columnName: Column1
                        values:
                          - categoricalMeasureField:
                              fieldId: '2'
                              column:
                                dataSetIdentifier: '1'
                                columnName: Column1
                              aggregationFunction: COUNT
```
<!--End PulumiCodeChooser -->

## Import

Using `pulumi import`, import a QuickSight Analysis using the AWS account ID and analysis ID separated by a comma (`,`). For example:

```sh
$ pulumi import aws:quicksight/analysis:Analysis example 123456789012,example-id
```
/

analysisId" Identifier for the analysis.
&
awsAccountIdB" AWS account ID.
T
nameB" FDisplay name for the analysis.

The following arguments are optional:
�

parameters\BZ:X
V

quicksightAnalysisParameters4aws:quicksight/AnalysisParameters:AnalysisParameters�The parameters for the creation of the analysis, which you want to use to override the default settings. An analysis can have any type of parameters, and some parameters might accept multiple values. See parameters.
�
permissions^B\*Z:X
V

quicksightAnalysisPermission4aws:quicksight/AnalysisPermission:AnalysisPermissionUA set of resource permissions on the analysis. Maximum of 64 items. See permissions.
�
recoveryWindowInDaysB �A value that specifies the number of days that Amazon QuickSight waits before it deletes the analysis. Use `0` to force deletion without recovery. Minimum value of `7`. Maximum value of `30`. Default to `30`.
�
sourceEntitybB`:^
\

quicksightAnalysisSourceEntity8aws:quicksight/AnalysisSourceEntity:AnalysisSourceEntity�The entity that you are using as a source when you create the analysis (template). Only one of `definition` or `source_entity` should be configured. See source_entity.
�
tagsB2" �Key-value map of resource tags. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
�
themeArnB" �The Amazon Resource Name (ARN) of the theme that is being used for this analysis. The theme ARN must exist in the same AWS account where you create the analysis.
"/

analysisId" Identifier for the analysis.
" 
arn" ARN of the analysis.
"$
awsAccountId" AWS account ID.
";
createdTime" (The time that the analysis was created.
"
lastPublishedTime" "D
lastUpdatedTime" -The time that the analysis was last updated.
"R
name" FDisplay name for the analysis.

The following arguments are optional:
"�

parametersZ:X
V

quicksightAnalysisParameters4aws:quicksight/AnalysisParameters:AnalysisParameters�The parameters for the creation of the analysis, which you want to use to override the default settings. An analysis can have any type of parameters, and some parameters might accept multiple values. See parameters.
"�
permissions^B\*Z:X
V

quicksightAnalysisPermission4aws:quicksight/AnalysisPermission:AnalysisPermissionUA set of resource permissions on the analysis. Maximum of 64 items. See permissions.
"�
recoveryWindowInDaysB �A value that specifies the number of days that Amazon QuickSight waits before it deletes the analysis. Use `0` to force deletion without recovery. Minimum value of `7`. Maximum value of `30`. Default to `30`.
"�
sourceEntitybB`:^
\

quicksightAnalysisSourceEntity8aws:quicksight/AnalysisSourceEntity:AnalysisSourceEntity�The entity that you are using as a source when you create the analysis (template). Only one of `definition` or `source_entity` should be configured. See source_entity.
",
status" The analysis creation status.
"�
tagsB2" �Key-value map of resource tags. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
"�
tagsAll2" xA map of tags assigned to the resource, including those inherited from the provider `default_tags` configuration block.
"�
themeArnB" �The Amazon Resource Name (ARN) of the theme that is being used for this analysis. The theme ARN must exist in the same AWS account where you create the analysis.
*�P
;

quicksight	Dashboard"aws:quicksight/dashboard:Dashboard�1Resource for managing a QuickSight Dashboard.

## Example Usage

### From Source Template

<!--Start PulumiCodeChooser -->
```typescript
import * as pulumi from "@pulumi/pulumi";
import * as aws from "@pulumi/aws";

const example = new aws.quicksight.Dashboard("example", {
    dashboardId: "example-id",
    name: "example-name",
    versionDescription: "version",
    sourceEntity: {
        sourceTemplate: {
            arn: source.arn,
            dataSetReferences: [{
                dataSetArn: dataset.arn,
                dataSetPlaceholder: "1",
            }],
        },
    },
});
```
```python
import pulumi
import pulumi_aws as aws

example = aws.quicksight.Dashboard("example",
    dashboard_id="example-id",
    name="example-name",
    version_description="version",
    source_entity={
        "source_template": {
            "arn": source["arn"],
            "data_set_references": [{
                "data_set_arn": dataset["arn"],
                "data_set_placeholder": "1",
            }],
        },
    })
```
```csharp
using System.Collections.Generic;
using System.Linq;
using Pulumi;
using Aws = Pulumi.Aws;

return await Deployment.RunAsync(() => 
{
    var example = new Aws.Quicksight.Dashboard("example", new()
    {
        DashboardId = "example-id",
        Name = "example-name",
        VersionDescription = "version",
        SourceEntity = new Aws.Quicksight.Inputs.DashboardSourceEntityArgs
        {
            SourceTemplate = new Aws.Quicksight.Inputs.DashboardSourceEntitySourceTemplateArgs
            {
                Arn = source.Arn,
                DataSetReferences = new[]
                {
                    new Aws.Quicksight.Inputs.DashboardSourceEntitySourceTemplateDataSetReferenceArgs
                    {
                        DataSetArn = dataset.Arn,
                        DataSetPlaceholder = "1",
                    },
                },
            },
        },
    });

});
```
```go
package main

import (
	"github.com/pulumi/pulumi-aws/sdk/v6/go/aws/quicksight"
	"github.com/pulumi/pulumi/sdk/v3/go/pulumi"
)

func main() {
	pulumi.Run(func(ctx *pulumi.Context) error {
		_, err := quicksight.NewDashboard(ctx, "example", &quicksight.DashboardArgs{
			DashboardId:        pulumi.String("example-id"),
			Name:               pulumi.String("example-name"),
			VersionDescription: pulumi.String("version"),
			SourceEntity: &quicksight.DashboardSourceEntityArgs{
				SourceTemplate: &quicksight.DashboardSourceEntitySourceTemplateArgs{
					Arn: pulumi.Any(source.Arn),
					DataSetReferences: quicksight.DashboardSourceEntitySourceTemplateDataSetReferenceArray{
						&quicksight.DashboardSourceEntitySourceTemplateDataSetReferenceArgs{
							DataSetArn:         pulumi.Any(dataset.Arn),
							DataSetPlaceholder: pulumi.String("1"),
						},
					},
				},
			},
		})
		if err != nil {
			return err
		}
		return nil
	})
}
```
```java
package generated_program;

import com.pulumi.Context;
import com.pulumi.Pulumi;
import com.pulumi.core.Output;
import com.pulumi.aws.quicksight.Dashboard;
import com.pulumi.aws.quicksight.DashboardArgs;
import com.pulumi.aws.quicksight.inputs.DashboardSourceEntityArgs;
import com.pulumi.aws.quicksight.inputs.DashboardSourceEntitySourceTemplateArgs;
import java.util.List;
import java.util.ArrayList;
import java.util.Map;
import java.io.File;
import java.nio.file.Files;
import java.nio.file.Paths;

public class App {
    public static void main(String[] args) {
        Pulumi.run(App::stack);
    }

    public static void stack(Context ctx) {
        var example = new Dashboard("example", DashboardArgs.builder()
            .dashboardId("example-id")
            .name("example-name")
            .versionDescription("version")
            .sourceEntity(DashboardSourceEntityArgs.builder()
                .sourceTemplate(DashboardSourceEntitySourceTemplateArgs.builder()
                    .arn(source.arn())
                    .dataSetReferences(DashboardSourceEntitySourceTemplateDataSetReferenceArgs.builder()
                        .dataSetArn(dataset.arn())
                        .dataSetPlaceholder("1")
                        .build())
                    .build())
                .build())
            .build());

    }
}
```
```yaml
resources:
  example:
    type: aws:quicksight:Dashboard
    properties:
      dashboardId: example-id
      name: example-name
      versionDescription: version
      sourceEntity:
        sourceTemplate:
          arn: ${source.arn}
          dataSetReferences:
            - dataSetArn: ${dataset.arn}
              dataSetPlaceholder: '1'
```
<!--End PulumiCodeChooser -->

### With Definition

<!--Start PulumiCodeChooser -->
```yaml
resources:
  example:
    type: aws:quicksight:Dashboard
    properties:
      dashboardId: example-id
      name: example-name
      versionDescription: version
      definition:
        dataSetIdentifiersDeclarations:
          - dataSetArn: ${dataset.arn}
            identifier: '1'
        sheets:
          - title: Example
            sheetId: Example1
            visuals:
              - lineChartVisual:
                  visualId: LineChart
                  title:
                    formatText:
                      plainText: Line Chart Example
                  chartConfiguration:
                    fieldWells:
                      lineChartAggregatedFieldWells:
                        categories:
                          - categoricalDimensionField:
                              fieldId: '1'
                              column:
                                dataSetIdentifier: '1'
                                columnName: Column1
                        values:
                          - categoricalMeasureField:
                              fieldId: '2'
                              column:
                                dataSetIdentifier: '1'
                                columnName: Column1
                              aggregationFunction: COUNT
```
<!--End PulumiCodeChooser -->

## Import

Using `pulumi import`, import a QuickSight Dashboard using the AWS account ID and dashboard ID separated by a comma (`,`). For example:

```sh
$ pulumi import aws:quicksight/dashboard:Dashboard example 123456789012,example-id
```
&
awsAccountIdB" AWS account ID.
1
dashboardId" Identifier for the dashboard.
�
dashboardPublishOptions�B�:�
�

quicksight DashboardDashboardPublishOptionsPaws:quicksight/DashboardDashboardPublishOptions:DashboardDashboardPublishOptionsEOptions for publishing the dashboard. See dashboard_publish_options.
.
nameB"  Display name for the dashboard.
�

parameters_B]:[
Y

quicksightDashboardParameters6aws:quicksight/DashboardParameters:DashboardParameters�The parameters for the creation of the dashboard, which you want to use to override the default settings. A dashboard can have any type of parameters, and some parameters might accept multiple values. See parameters.
�
permissionsaB_*]:[
Y

quicksightDashboardPermission6aws:quicksight/DashboardPermission:DashboardPermissionVA set of resource permissions on the dashboard. Maximum of 64 items. See permissions.
�
sourceEntityeBc:a
_

quicksightDashboardSourceEntity:aws:quicksight/DashboardSourceEntity:DashboardSourceEntity�The entity that you are using as a source when you create the dashboard (template). Only one of `definition` or `source_entity` should be configured. See source_entity.
�
tagsB2" �Key-value map of resource tags. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
�
themeArnB" �The Amazon Resource Name (ARN) of the theme that is being used for this dashboard. The theme ARN must exist in the same AWS account where you create the dashboard.
�
versionDescription" mA description of the current dashboard version being created/updated.

The following arguments are optional:
"!
arn" ARN of the dashboard.
"$
awsAccountId" AWS account ID.
"<
createdTime" )The time that the dashboard was created.
"1
dashboardId" Identifier for the dashboard.
"�
dashboardPublishOptions�:�
�

quicksight DashboardDashboardPublishOptionsPaws:quicksight/DashboardDashboardPublishOptions:DashboardDashboardPublishOptionsEOptions for publishing the dashboard. See dashboard_publish_options.
"
lastPublishedTime" "E
lastUpdatedTime" .The time that the dashboard was last updated.
",
name"  Display name for the dashboard.
"�

parameters]:[
Y

quicksightDashboardParameters6aws:quicksight/DashboardParameters:DashboardParameters�The parameters for the creation of the dashboard, which you want to use to override the default settings. A dashboard can have any type of parameters, and some parameters might accept multiple values. See parameters.
"�
permissionsaB_*]:[
Y

quicksightDashboardPermission6aws:quicksight/DashboardPermission:DashboardPermissionVA set of resource permissions on the dashboard. Maximum of 64 items. See permissions.
"�
sourceEntityeBc:a
_

quicksightDashboardSourceEntity:aws:quicksight/DashboardSourceEntity:DashboardSourceEntity�The entity that you are using as a source when you create the dashboard (template). Only one of `definition` or `source_entity` should be configured. See source_entity.
"h
sourceEntityArn" QAmazon Resource Name (ARN) of a template that was used to create this dashboard.
"-
status" The dashboard creation status.
"�
tagsB2" �Key-value map of resource tags. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
"�
tagsAll2" xA map of tags assigned to the resource, including those inherited from the provider `default_tags` configuration block.
"�
themeArnB" �The Amazon Resource Name (ARN) of the theme that is being used for this dashboard. The theme ARN must exist in the same AWS account where you create the dashboard.
"�
versionDescription" mA description of the current dashboard version being created/updated.

The following arguments are optional:
"B
versionNumber -The version number of the dashboard version.
*��
5

quicksightDataSetaws:quicksight/dataSet:DataSet��Resource for managing a QuickSight Data Set.

## Example Usage

### Basic Usage

<!--Start PulumiCodeChooser -->
```typescript
import * as pulumi from "@pulumi/pulumi";
import * as aws from "@pulumi/aws";

const example = new aws.quicksight.DataSet("example", {
    dataSetId: "example-id",
    name: "example-name",
    importMode: "SPICE",
    physicalTableMaps: [{
        physicalTableMapId: "example-id",
        s3Source: {
            dataSourceArn: exampleAwsQuicksightDataSource.arn,
            inputColumns: [{
                name: "Column1",
                type: "STRING",
            }],
            uploadSettings: {
                format: "JSON",
            },
        },
    }],
});
```
```python
import pulumi
import pulumi_aws as aws

example = aws.quicksight.DataSet("example",
    data_set_id="example-id",
    name="example-name",
    import_mode="SPICE",
    physical_table_maps=[{
        "physical_table_map_id": "example-id",
        "s3_source": {
            "data_source_arn": example_aws_quicksight_data_source["arn"],
            "input_columns": [{
                "name": "Column1",
                "type": "STRING",
            }],
            "upload_settings": {
                "format": "JSON",
            },
        },
    }])
```
```csharp
using System.Collections.Generic;
using System.Linq;
using Pulumi;
using Aws = Pulumi.Aws;

return await Deployment.RunAsync(() => 
{
    var example = new Aws.Quicksight.DataSet("example", new()
    {
        DataSetId = "example-id",
        Name = "example-name",
        ImportMode = "SPICE",
        PhysicalTableMaps = new[]
        {
            new Aws.Quicksight.Inputs.DataSetPhysicalTableMapArgs
            {
                PhysicalTableMapId = "example-id",
                S3Source = new Aws.Quicksight.Inputs.DataSetPhysicalTableMapS3SourceArgs
                {
                    DataSourceArn = exampleAwsQuicksightDataSource.Arn,
                    InputColumns = new[]
                    {
                        new Aws.Quicksight.Inputs.DataSetPhysicalTableMapS3SourceInputColumnArgs
                        {
                            Name = "Column1",
                            Type = "STRING",
                        },
                    },
                    UploadSettings = new Aws.Quicksight.Inputs.DataSetPhysicalTableMapS3SourceUploadSettingsArgs
                    {
                        Format = "JSON",
                    },
                },
            },
        },
    });

});
```
```go
package main

import (
	"github.com/pulumi/pulumi-aws/sdk/v6/go/aws/quicksight"
	"github.com/pulumi/pulumi/sdk/v3/go/pulumi"
)

func main() {
	pulumi.Run(func(ctx *pulumi.Context) error {
		_, err := quicksight.NewDataSet(ctx, "example", &quicksight.DataSetArgs{
			DataSetId:  pulumi.String("example-id"),
			Name:       pulumi.String("example-name"),
			ImportMode: pulumi.String("SPICE"),
			PhysicalTableMaps: quicksight.DataSetPhysicalTableMapArray{
				&quicksight.DataSetPhysicalTableMapArgs{
					PhysicalTableMapId: pulumi.String("example-id"),
					S3Source: &quicksight.DataSetPhysicalTableMapS3SourceArgs{
						DataSourceArn: pulumi.Any(exampleAwsQuicksightDataSource.Arn),
						InputColumns: quicksight.DataSetPhysicalTableMapS3SourceInputColumnArray{
							&quicksight.DataSetPhysicalTableMapS3SourceInputColumnArgs{
								Name: pulumi.String("Column1"),
								Type: pulumi.String("STRING"),
							},
						},
						UploadSettings: &quicksight.DataSetPhysicalTableMapS3SourceUploadSettingsArgs{
							Format: pulumi.String("JSON"),
						},
					},
				},
			},
		})
		if err != nil {
			return err
		}
		return nil
	})
}
```
```java
package generated_program;

import com.pulumi.Context;
import com.pulumi.Pulumi;
import com.pulumi.core.Output;
import com.pulumi.aws.quicksight.DataSet;
import com.pulumi.aws.quicksight.DataSetArgs;
import com.pulumi.aws.quicksight.inputs.DataSetPhysicalTableMapArgs;
import com.pulumi.aws.quicksight.inputs.DataSetPhysicalTableMapS3SourceArgs;
import com.pulumi.aws.quicksight.inputs.DataSetPhysicalTableMapS3SourceUploadSettingsArgs;
import java.util.List;
import java.util.ArrayList;
import java.util.Map;
import java.io.File;
import java.nio.file.Files;
import java.nio.file.Paths;

public class App {
    public static void main(String[] args) {
        Pulumi.run(App::stack);
    }

    public static void stack(Context ctx) {
        var example = new DataSet("example", DataSetArgs.builder()
            .dataSetId("example-id")
            .name("example-name")
            .importMode("SPICE")
            .physicalTableMaps(DataSetPhysicalTableMapArgs.builder()
                .physicalTableMapId("example-id")
                .s3Source(DataSetPhysicalTableMapS3SourceArgs.builder()
                    .dataSourceArn(exampleAwsQuicksightDataSource.arn())
                    .inputColumns(DataSetPhysicalTableMapS3SourceInputColumnArgs.builder()
                        .name("Column1")
                        .type("STRING")
                        .build())
                    .uploadSettings(DataSetPhysicalTableMapS3SourceUploadSettingsArgs.builder()
                        .format("JSON")
                        .build())
                    .build())
                .build())
            .build());

    }
}
```
```yaml
resources:
  example:
    type: aws:quicksight:DataSet
    properties:
      dataSetId: example-id
      name: example-name
      importMode: SPICE
      physicalTableMaps:
        - physicalTableMapId: example-id
          s3Source:
            dataSourceArn: ${exampleAwsQuicksightDataSource.arn}
            inputColumns:
              - name: Column1
                type: STRING
            uploadSettings:
              format: JSON
```
<!--End PulumiCodeChooser -->

### With Column Level Permission Rules

<!--Start PulumiCodeChooser -->
```typescript
import * as pulumi from "@pulumi/pulumi";
import * as aws from "@pulumi/aws";

const example = new aws.quicksight.DataSet("example", {
    dataSetId: "example-id",
    name: "example-name",
    importMode: "SPICE",
    physicalTableMaps: [{
        physicalTableMapId: "example-id",
        s3Source: {
            dataSourceArn: exampleAwsQuicksightDataSource.arn,
            inputColumns: [{
                name: "Column1",
                type: "STRING",
            }],
            uploadSettings: {
                format: "JSON",
            },
        },
    }],
    columnLevelPermissionRules: [{
        columnNames: ["Column1"],
        principals: [exampleAwsQuicksightUser.arn],
    }],
});
```
```python
import pulumi
import pulumi_aws as aws

example = aws.quicksight.DataSet("example",
    data_set_id="example-id",
    name="example-name",
    import_mode="SPICE",
    physical_table_maps=[{
        "physical_table_map_id": "example-id",
        "s3_source": {
            "data_source_arn": example_aws_quicksight_data_source["arn"],
            "input_columns": [{
                "name": "Column1",
                "type": "STRING",
            }],
            "upload_settings": {
                "format": "JSON",
            },
        },
    }],
    column_level_permission_rules=[{
        "column_names": ["Column1"],
        "principals": [example_aws_quicksight_user["arn"]],
    }])
```
```csharp
using System.Collections.Generic;
using System.Linq;
using Pulumi;
using Aws = Pulumi.Aws;

return await Deployment.RunAsync(() => 
{
    var example = new Aws.Quicksight.DataSet("example", new()
    {
        DataSetId = "example-id",
        Name = "example-name",
        ImportMode = "SPICE",
        PhysicalTableMaps = new[]
        {
            new Aws.Quicksight.Inputs.DataSetPhysicalTableMapArgs
            {
                PhysicalTableMapId = "example-id",
                S3Source = new Aws.Quicksight.Inputs.DataSetPhysicalTableMapS3SourceArgs
                {
                    DataSourceArn = exampleAwsQuicksightDataSource.Arn,
                    InputColumns = new[]
                    {
                        new Aws.Quicksight.Inputs.DataSetPhysicalTableMapS3SourceInputColumnArgs
                        {
                            Name = "Column1",
                            Type = "STRING",
                        },
                    },
                    UploadSettings = new Aws.Quicksight.Inputs.DataSetPhysicalTableMapS3SourceUploadSettingsArgs
                    {
                        Format = "JSON",
                    },
                },
            },
        },
        ColumnLevelPermissionRules = new[]
        {
            new Aws.Quicksight.Inputs.DataSetColumnLevelPermissionRuleArgs
            {
                ColumnNames = new[]
                {
                    "Column1",
                },
                Principals = new[]
                {
                    exampleAwsQuicksightUser.Arn,
                },
            },
        },
    });

});
```
```go
package main

import (
	"github.com/pulumi/pulumi-aws/sdk/v6/go/aws/quicksight"
	"github.com/pulumi/pulumi/sdk/v3/go/pulumi"
)

func main() {
	pulumi.Run(func(ctx *pulumi.Context) error {
		_, err := quicksight.NewDataSet(ctx, "example", &quicksight.DataSetArgs{
			DataSetId:  pulumi.String("example-id"),
			Name:       pulumi.String("example-name"),
			ImportMode: pulumi.String("SPICE"),
			PhysicalTableMaps: quicksight.DataSetPhysicalTableMapArray{
				&quicksight.DataSetPhysicalTableMapArgs{
					PhysicalTableMapId: pulumi.String("example-id"),
					S3Source: &quicksight.DataSetPhysicalTableMapS3SourceArgs{
						DataSourceArn: pulumi.Any(exampleAwsQuicksightDataSource.Arn),
						InputColumns: quicksight.DataSetPhysicalTableMapS3SourceInputColumnArray{
							&quicksight.DataSetPhysicalTableMapS3SourceInputColumnArgs{
								Name: pulumi.String("Column1"),
								Type: pulumi.String("STRING"),
							},
						},
						UploadSettings: &quicksight.DataSetPhysicalTableMapS3SourceUploadSettingsArgs{
							Format: pulumi.String("JSON"),
						},
					},
				},
			},
			ColumnLevelPermissionRules: quicksight.DataSetColumnLevelPermissionRuleArray{
				&quicksight.DataSetColumnLevelPermissionRuleArgs{
					ColumnNames: pulumi.StringArray{
						pulumi.String("Column1"),
					},
					Principals: pulumi.StringArray{
						exampleAwsQuicksightUser.Arn,
					},
				},
			},
		})
		if err != nil {
			return err
		}
		return nil
	})
}
```
```java
package generated_program;

import com.pulumi.Context;
import com.pulumi.Pulumi;
import com.pulumi.core.Output;
import com.pulumi.aws.quicksight.DataSet;
import com.pulumi.aws.quicksight.DataSetArgs;
import com.pulumi.aws.quicksight.inputs.DataSetPhysicalTableMapArgs;
import com.pulumi.aws.quicksight.inputs.DataSetPhysicalTableMapS3SourceArgs;
import com.pulumi.aws.quicksight.inputs.DataSetPhysicalTableMapS3SourceUploadSettingsArgs;
import com.pulumi.aws.quicksight.inputs.DataSetColumnLevelPermissionRuleArgs;
import java.util.List;
import java.util.ArrayList;
import java.util.Map;
import java.io.File;
import java.nio.file.Files;
import java.nio.file.Paths;

public class App {
    public static void main(String[] args) {
        Pulumi.run(App::stack);
    }

    public static void stack(Context ctx) {
        var example = new DataSet("example", DataSetArgs.builder()
            .dataSetId("example-id")
            .name("example-name")
            .importMode("SPICE")
            .physicalTableMaps(DataSetPhysicalTableMapArgs.builder()
                .physicalTableMapId("example-id")
                .s3Source(DataSetPhysicalTableMapS3SourceArgs.builder()
                    .dataSourceArn(exampleAwsQuicksightDataSource.arn())
                    .inputColumns(DataSetPhysicalTableMapS3SourceInputColumnArgs.builder()
                        .name("Column1")
                        .type("STRING")
                        .build())
                    .uploadSettings(DataSetPhysicalTableMapS3SourceUploadSettingsArgs.builder()
                        .format("JSON")
                        .build())
                    .build())
                .build())
            .columnLevelPermissionRules(DataSetColumnLevelPermissionRuleArgs.builder()
                .columnNames("Column1")
                .principals(exampleAwsQuicksightUser.arn())
                .build())
            .build());

    }
}
```
```yaml
resources:
  example:
    type: aws:quicksight:DataSet
    properties:
      dataSetId: example-id
      name: example-name
      importMode: SPICE
      physicalTableMaps:
        - physicalTableMapId: example-id
          s3Source:
            dataSourceArn: ${exampleAwsQuicksightDataSource.arn}
            inputColumns:
              - name: Column1
                type: STRING
            uploadSettings:
              format: JSON
      columnLevelPermissionRules:
        - columnNames:
            - Column1
          principals:
            - ${exampleAwsQuicksightUser.arn}
```
<!--End PulumiCodeChooser -->

### With Field Folders

<!--Start PulumiCodeChooser -->
```typescript
import * as pulumi from "@pulumi/pulumi";
import * as aws from "@pulumi/aws";

const example = new aws.quicksight.DataSet("example", {
    dataSetId: "example-id",
    name: "example-name",
    importMode: "SPICE",
    physicalTableMaps: [{
        physicalTableMapId: "example-id",
        s3Source: {
            dataSourceArn: exampleAwsQuicksightDataSource.arn,
            inputColumns: [{
                name: "Column1",
                type: "STRING",
            }],
            uploadSettings: {
                format: "JSON",
            },
        },
    }],
    fieldFolders: [{
        fieldFoldersId: "example-id",
        columns: ["Column1"],
        description: "example description",
    }],
});
```
```python
import pulumi
import pulumi_aws as aws

example = aws.quicksight.DataSet("example",
    data_set_id="example-id",
    name="example-name",
    import_mode="SPICE",
    physical_table_maps=[{
        "physical_table_map_id": "example-id",
        "s3_source": {
            "data_source_arn": example_aws_quicksight_data_source["arn"],
            "input_columns": [{
                "name": "Column1",
                "type": "STRING",
            }],
            "upload_settings": {
                "format": "JSON",
            },
        },
    }],
    field_folders=[{
        "field_folders_id": "example-id",
        "columns": ["Column1"],
        "description": "example description",
    }])
```
```csharp
using System.Collections.Generic;
using System.Linq;
using Pulumi;
using Aws = Pulumi.Aws;

return await Deployment.RunAsync(() => 
{
    var example = new Aws.Quicksight.DataSet("example", new()
    {
        DataSetId = "example-id",
        Name = "example-name",
        ImportMode = "SPICE",
        PhysicalTableMaps = new[]
        {
            new Aws.Quicksight.Inputs.DataSetPhysicalTableMapArgs
            {
                PhysicalTableMapId = "example-id",
                S3Source = new Aws.Quicksight.Inputs.DataSetPhysicalTableMapS3SourceArgs
                {
                    DataSourceArn = exampleAwsQuicksightDataSource.Arn,
                    InputColumns = new[]
                    {
                        new Aws.Quicksight.Inputs.DataSetPhysicalTableMapS3SourceInputColumnArgs
                        {
                            Name = "Column1",
                            Type = "STRING",
                        },
                    },
                    UploadSettings = new Aws.Quicksight.Inputs.DataSetPhysicalTableMapS3SourceUploadSettingsArgs
                    {
                        Format = "JSON",
                    },
                },
            },
        },
        FieldFolders = new[]
        {
            new Aws.Quicksight.Inputs.DataSetFieldFolderArgs
            {
                FieldFoldersId = "example-id",
                Columns = new[]
                {
                    "Column1",
                },
                Description = "example description",
            },
        },
    });

});
```
```go
package main

import (
	"github.com/pulumi/pulumi-aws/sdk/v6/go/aws/quicksight"
	"github.com/pulumi/pulumi/sdk/v3/go/pulumi"
)

func main() {
	pulumi.Run(func(ctx *pulumi.Context) error {
		_, err := quicksight.NewDataSet(ctx, "example", &quicksight.DataSetArgs{
			DataSetId:  pulumi.String("example-id"),
			Name:       pulumi.String("example-name"),
			ImportMode: pulumi.String("SPICE"),
			PhysicalTableMaps: quicksight.DataSetPhysicalTableMapArray{
				&quicksight.DataSetPhysicalTableMapArgs{
					PhysicalTableMapId: pulumi.String("example-id"),
					S3Source: &quicksight.DataSetPhysicalTableMapS3SourceArgs{
						DataSourceArn: pulumi.Any(exampleAwsQuicksightDataSource.Arn),
						InputColumns: quicksight.DataSetPhysicalTableMapS3SourceInputColumnArray{
							&quicksight.DataSetPhysicalTableMapS3SourceInputColumnArgs{
								Name: pulumi.String("Column1"),
								Type: pulumi.String("STRING"),
							},
						},
						UploadSettings: &quicksight.DataSetPhysicalTableMapS3SourceUploadSettingsArgs{
							Format: pulumi.String("JSON"),
						},
					},
				},
			},
			FieldFolders: quicksight.DataSetFieldFolderArray{
				&quicksight.DataSetFieldFolderArgs{
					FieldFoldersId: pulumi.String("example-id"),
					Columns: pulumi.StringArray{
						pulumi.String("Column1"),
					},
					Description: pulumi.String("example description"),
				},
			},
		})
		if err != nil {
			return err
		}
		return nil
	})
}
```
```java
package generated_program;

import com.pulumi.Context;
import com.pulumi.Pulumi;
import com.pulumi.core.Output;
import com.pulumi.aws.quicksight.DataSet;
import com.pulumi.aws.quicksight.DataSetArgs;
import com.pulumi.aws.quicksight.inputs.DataSetPhysicalTableMapArgs;
import com.pulumi.aws.quicksight.inputs.DataSetPhysicalTableMapS3SourceArgs;
import com.pulumi.aws.quicksight.inputs.DataSetPhysicalTableMapS3SourceUploadSettingsArgs;
import com.pulumi.aws.quicksight.inputs.DataSetFieldFolderArgs;
import java.util.List;
import java.util.ArrayList;
import java.util.Map;
import java.io.File;
import java.nio.file.Files;
import java.nio.file.Paths;

public class App {
    public static void main(String[] args) {
        Pulumi.run(App::stack);
    }

    public static void stack(Context ctx) {
        var example = new DataSet("example", DataSetArgs.builder()
            .dataSetId("example-id")
            .name("example-name")
            .importMode("SPICE")
            .physicalTableMaps(DataSetPhysicalTableMapArgs.builder()
                .physicalTableMapId("example-id")
                .s3Source(DataSetPhysicalTableMapS3SourceArgs.builder()
                    .dataSourceArn(exampleAwsQuicksightDataSource.arn())
                    .inputColumns(DataSetPhysicalTableMapS3SourceInputColumnArgs.builder()
                        .name("Column1")
                        .type("STRING")
                        .build())
                    .uploadSettings(DataSetPhysicalTableMapS3SourceUploadSettingsArgs.builder()
                        .format("JSON")
                        .build())
                    .build())
                .build())
            .fieldFolders(DataSetFieldFolderArgs.builder()
                .fieldFoldersId("example-id")
                .columns("Column1")
                .description("example description")
                .build())
            .build());

    }
}
```
```yaml
resources:
  example:
    type: aws:quicksight:DataSet
    properties:
      dataSetId: example-id
      name: example-name
      importMode: SPICE
      physicalTableMaps:
        - physicalTableMapId: example-id
          s3Source:
            dataSourceArn: ${exampleAwsQuicksightDataSource.arn}
            inputColumns:
              - name: Column1
                type: STRING
            uploadSettings:
              format: JSON
      fieldFolders:
        - fieldFoldersId: example-id
          columns:
            - Column1
          description: example description
```
<!--End PulumiCodeChooser -->

### With Permissions

<!--Start PulumiCodeChooser -->
```typescript
import * as pulumi from "@pulumi/pulumi";
import * as aws from "@pulumi/aws";

const example = new aws.quicksight.DataSet("example", {
    dataSetId: "example-id",
    name: "example-name",
    importMode: "SPICE",
    physicalTableMaps: [{
        physicalTableMapId: "example-id",
        s3Source: {
            dataSourceArn: exampleAwsQuicksightDataSource.arn,
            inputColumns: [{
                name: "Column1",
                type: "STRING",
            }],
            uploadSettings: {
                format: "JSON",
            },
        },
    }],
    permissions: [{
        actions: [
            "quicksight:DescribeDataSet",
            "quicksight:DescribeDataSetPermissions",
            "quicksight:PassDataSet",
            "quicksight:DescribeIngestion",
            "quicksight:ListIngestions",
        ],
        principal: exampleAwsQuicksightUser.arn,
    }],
});
```
```python
import pulumi
import pulumi_aws as aws

example = aws.quicksight.DataSet("example",
    data_set_id="example-id",
    name="example-name",
    import_mode="SPICE",
    physical_table_maps=[{
        "physical_table_map_id": "example-id",
        "s3_source": {
            "data_source_arn": example_aws_quicksight_data_source["arn"],
            "input_columns": [{
                "name": "Column1",
                "type": "STRING",
            }],
            "upload_settings": {
                "format": "JSON",
            },
        },
    }],
    permissions=[{
        "actions": [
            "quicksight:DescribeDataSet",
            "quicksight:DescribeDataSetPermissions",
            "quicksight:PassDataSet",
            "quicksight:DescribeIngestion",
            "quicksight:ListIngestions",
        ],
        "principal": example_aws_quicksight_user["arn"],
    }])
```
```csharp
using System.Collections.Generic;
using System.Linq;
using Pulumi;
using Aws = Pulumi.Aws;

return await Deployment.RunAsync(() => 
{
    var example = new Aws.Quicksight.DataSet("example", new()
    {
        DataSetId = "example-id",
        Name = "example-name",
        ImportMode = "SPICE",
        PhysicalTableMaps = new[]
        {
            new Aws.Quicksight.Inputs.DataSetPhysicalTableMapArgs
            {
                PhysicalTableMapId = "example-id",
                S3Source = new Aws.Quicksight.Inputs.DataSetPhysicalTableMapS3SourceArgs
                {
                    DataSourceArn = exampleAwsQuicksightDataSource.Arn,
                    InputColumns = new[]
                    {
                        new Aws.Quicksight.Inputs.DataSetPhysicalTableMapS3SourceInputColumnArgs
                        {
                            Name = "Column1",
                            Type = "STRING",
                        },
                    },
                    UploadSettings = new Aws.Quicksight.Inputs.DataSetPhysicalTableMapS3SourceUploadSettingsArgs
                    {
                        Format = "JSON",
                    },
                },
            },
        },
        Permissions = new[]
        {
            new Aws.Quicksight.Inputs.DataSetPermissionArgs
            {
                Actions = new[]
                {
                    "quicksight:DescribeDataSet",
                    "quicksight:DescribeDataSetPermissions",
                    "quicksight:PassDataSet",
                    "quicksight:DescribeIngestion",
                    "quicksight:ListIngestions",
                },
                Principal = exampleAwsQuicksightUser.Arn,
            },
        },
    });

});
```
```go
package main

import (
	"github.com/pulumi/pulumi-aws/sdk/v6/go/aws/quicksight"
	"github.com/pulumi/pulumi/sdk/v3/go/pulumi"
)

func main() {
	pulumi.Run(func(ctx *pulumi.Context) error {
		_, err := quicksight.NewDataSet(ctx, "example", &quicksight.DataSetArgs{
			DataSetId:  pulumi.String("example-id"),
			Name:       pulumi.String("example-name"),
			ImportMode: pulumi.String("SPICE"),
			PhysicalTableMaps: quicksight.DataSetPhysicalTableMapArray{
				&quicksight.DataSetPhysicalTableMapArgs{
					PhysicalTableMapId: pulumi.String("example-id"),
					S3Source: &quicksight.DataSetPhysicalTableMapS3SourceArgs{
						DataSourceArn: pulumi.Any(exampleAwsQuicksightDataSource.Arn),
						InputColumns: quicksight.DataSetPhysicalTableMapS3SourceInputColumnArray{
							&quicksight.DataSetPhysicalTableMapS3SourceInputColumnArgs{
								Name: pulumi.String("Column1"),
								Type: pulumi.String("STRING"),
							},
						},
						UploadSettings: &quicksight.DataSetPhysicalTableMapS3SourceUploadSettingsArgs{
							Format: pulumi.String("JSON"),
						},
					},
				},
			},
			Permissions: quicksight.DataSetPermissionArray{
				&quicksight.DataSetPermissionArgs{
					Actions: pulumi.StringArray{
						pulumi.String("quicksight:DescribeDataSet"),
						pulumi.String("quicksight:DescribeDataSetPermissions"),
						pulumi.String("quicksight:PassDataSet"),
						pulumi.String("quicksight:DescribeIngestion"),
						pulumi.String("quicksight:ListIngestions"),
					},
					Principal: pulumi.Any(exampleAwsQuicksightUser.Arn),
				},
			},
		})
		if err != nil {
			return err
		}
		return nil
	})
}
```
```java
package generated_program;

import com.pulumi.Context;
import com.pulumi.Pulumi;
import com.pulumi.core.Output;
import com.pulumi.aws.quicksight.DataSet;
import com.pulumi.aws.quicksight.DataSetArgs;
import com.pulumi.aws.quicksight.inputs.DataSetPhysicalTableMapArgs;
import com.pulumi.aws.quicksight.inputs.DataSetPhysicalTableMapS3SourceArgs;
import com.pulumi.aws.quicksight.inputs.DataSetPhysicalTableMapS3SourceUploadSettingsArgs;
import com.pulumi.aws.quicksight.inputs.DataSetPermissionArgs;
import java.util.List;
import java.util.ArrayList;
import java.util.Map;
import java.io.File;
import java.nio.file.Files;
import java.nio.file.Paths;

public class App {
    public static void main(String[] args) {
        Pulumi.run(App::stack);
    }

    public static void stack(Context ctx) {
        var example = new DataSet("example", DataSetArgs.builder()
            .dataSetId("example-id")
            .name("example-name")
            .importMode("SPICE")
            .physicalTableMaps(DataSetPhysicalTableMapArgs.builder()
                .physicalTableMapId("example-id")
                .s3Source(DataSetPhysicalTableMapS3SourceArgs.builder()
                    .dataSourceArn(exampleAwsQuicksightDataSource.arn())
                    .inputColumns(DataSetPhysicalTableMapS3SourceInputColumnArgs.builder()
                        .name("Column1")
                        .type("STRING")
                        .build())
                    .uploadSettings(DataSetPhysicalTableMapS3SourceUploadSettingsArgs.builder()
                        .format("JSON")
                        .build())
                    .build())
                .build())
            .permissions(DataSetPermissionArgs.builder()
                .actions(                
                    "quicksight:DescribeDataSet",
                    "quicksight:DescribeDataSetPermissions",
                    "quicksight:PassDataSet",
                    "quicksight:DescribeIngestion",
                    "quicksight:ListIngestions")
                .principal(exampleAwsQuicksightUser.arn())
                .build())
            .build());

    }
}
```
```yaml
resources:
  example:
    type: aws:quicksight:DataSet
    properties:
      dataSetId: example-id
      name: example-name
      importMode: SPICE
      physicalTableMaps:
        - physicalTableMapId: example-id
          s3Source:
            dataSourceArn: ${exampleAwsQuicksightDataSource.arn}
            inputColumns:
              - name: Column1
                type: STRING
            uploadSettings:
              format: JSON
      permissions:
        - actions:
            - quicksight:DescribeDataSet
            - quicksight:DescribeDataSetPermissions
            - quicksight:PassDataSet
            - quicksight:DescribeIngestion
            - quicksight:ListIngestions
          principal: ${exampleAwsQuicksightUser.arn}
```
<!--End PulumiCodeChooser -->

### With Row Level Permission Tag Configuration

<!--Start PulumiCodeChooser -->
```typescript
import * as pulumi from "@pulumi/pulumi";
import * as aws from "@pulumi/aws";

const example = new aws.quicksight.DataSet("example", {
    dataSetId: "example-id",
    name: "example-name",
    importMode: "SPICE",
    physicalTableMaps: [{
        physicalTableMapId: "example-id",
        s3Source: {
            dataSourceArn: exampleAwsQuicksightDataSource.arn,
            inputColumns: [{
                name: "Column1",
                type: "STRING",
            }],
            uploadSettings: {
                format: "JSON",
            },
        },
    }],
    rowLevelPermissionTagConfiguration: {
        status: "ENABLED",
        tagRules: [{
            columnName: "Column1",
            tagKey: "tagkey",
            matchAllValue: "*",
            tagMultiValueDelimiter: ",",
        }],
    },
});
```
```python
import pulumi
import pulumi_aws as aws

example = aws.quicksight.DataSet("example",
    data_set_id="example-id",
    name="example-name",
    import_mode="SPICE",
    physical_table_maps=[{
        "physical_table_map_id": "example-id",
        "s3_source": {
            "data_source_arn": example_aws_quicksight_data_source["arn"],
            "input_columns": [{
                "name": "Column1",
                "type": "STRING",
            }],
            "upload_settings": {
                "format": "JSON",
            },
        },
    }],
    row_level_permission_tag_configuration={
        "status": "ENABLED",
        "tag_rules": [{
            "column_name": "Column1",
            "tag_key": "tagkey",
            "match_all_value": "*",
            "tag_multi_value_delimiter": ",",
        }],
    })
```
```csharp
using System.Collections.Generic;
using System.Linq;
using Pulumi;
using Aws = Pulumi.Aws;

return await Deployment.RunAsync(() => 
{
    var example = new Aws.Quicksight.DataSet("example", new()
    {
        DataSetId = "example-id",
        Name = "example-name",
        ImportMode = "SPICE",
        PhysicalTableMaps = new[]
        {
            new Aws.Quicksight.Inputs.DataSetPhysicalTableMapArgs
            {
                PhysicalTableMapId = "example-id",
                S3Source = new Aws.Quicksight.Inputs.DataSetPhysicalTableMapS3SourceArgs
                {
                    DataSourceArn = exampleAwsQuicksightDataSource.Arn,
                    InputColumns = new[]
                    {
                        new Aws.Quicksight.Inputs.DataSetPhysicalTableMapS3SourceInputColumnArgs
                        {
                            Name = "Column1",
                            Type = "STRING",
                        },
                    },
                    UploadSettings = new Aws.Quicksight.Inputs.DataSetPhysicalTableMapS3SourceUploadSettingsArgs
                    {
                        Format = "JSON",
                    },
                },
            },
        },
        RowLevelPermissionTagConfiguration = new Aws.Quicksight.Inputs.DataSetRowLevelPermissionTagConfigurationArgs
        {
            Status = "ENABLED",
            TagRules = new[]
            {
                new Aws.Quicksight.Inputs.DataSetRowLevelPermissionTagConfigurationTagRuleArgs
                {
                    ColumnName = "Column1",
                    TagKey = "tagkey",
                    MatchAllValue = "*",
                    TagMultiValueDelimiter = ",",
                },
            },
        },
    });

});
```
```go
package main

import (
	"github.com/pulumi/pulumi-aws/sdk/v6/go/aws/quicksight"
	"github.com/pulumi/pulumi/sdk/v3/go/pulumi"
)

func main() {
	pulumi.Run(func(ctx *pulumi.Context) error {
		_, err := quicksight.NewDataSet(ctx, "example", &quicksight.DataSetArgs{
			DataSetId:  pulumi.String("example-id"),
			Name:       pulumi.String("example-name"),
			ImportMode: pulumi.String("SPICE"),
			PhysicalTableMaps: quicksight.DataSetPhysicalTableMapArray{
				&quicksight.DataSetPhysicalTableMapArgs{
					PhysicalTableMapId: pulumi.String("example-id"),
					S3Source: &quicksight.DataSetPhysicalTableMapS3SourceArgs{
						DataSourceArn: pulumi.Any(exampleAwsQuicksightDataSource.Arn),
						InputColumns: quicksight.DataSetPhysicalTableMapS3SourceInputColumnArray{
							&quicksight.DataSetPhysicalTableMapS3SourceInputColumnArgs{
								Name: pulumi.String("Column1"),
								Type: pulumi.String("STRING"),
							},
						},
						UploadSettings: &quicksight.DataSetPhysicalTableMapS3SourceUploadSettingsArgs{
							Format: pulumi.String("JSON"),
						},
					},
				},
			},
			RowLevelPermissionTagConfiguration: &quicksight.DataSetRowLevelPermissionTagConfigurationArgs{
				Status: pulumi.String("ENABLED"),
				TagRules: quicksight.DataSetRowLevelPermissionTagConfigurationTagRuleArray{
					&quicksight.DataSetRowLevelPermissionTagConfigurationTagRuleArgs{
						ColumnName:             pulumi.String("Column1"),
						TagKey:                 pulumi.String("tagkey"),
						MatchAllValue:          pulumi.String("*"),
						TagMultiValueDelimiter: pulumi.String(","),
					},
				},
			},
		})
		if err != nil {
			return err
		}
		return nil
	})
}
```
```java
package generated_program;

import com.pulumi.Context;
import com.pulumi.Pulumi;
import com.pulumi.core.Output;
import com.pulumi.aws.quicksight.DataSet;
import com.pulumi.aws.quicksight.DataSetArgs;
import com.pulumi.aws.quicksight.inputs.DataSetPhysicalTableMapArgs;
import com.pulumi.aws.quicksight.inputs.DataSetPhysicalTableMapS3SourceArgs;
import com.pulumi.aws.quicksight.inputs.DataSetPhysicalTableMapS3SourceUploadSettingsArgs;
import com.pulumi.aws.quicksight.inputs.DataSetRowLevelPermissionTagConfigurationArgs;
import java.util.List;
import java.util.ArrayList;
import java.util.Map;
import java.io.File;
import java.nio.file.Files;
import java.nio.file.Paths;

public class App {
    public static void main(String[] args) {
        Pulumi.run(App::stack);
    }

    public static void stack(Context ctx) {
        var example = new DataSet("example", DataSetArgs.builder()
            .dataSetId("example-id")
            .name("example-name")
            .importMode("SPICE")
            .physicalTableMaps(DataSetPhysicalTableMapArgs.builder()
                .physicalTableMapId("example-id")
                .s3Source(DataSetPhysicalTableMapS3SourceArgs.builder()
                    .dataSourceArn(exampleAwsQuicksightDataSource.arn())
                    .inputColumns(DataSetPhysicalTableMapS3SourceInputColumnArgs.builder()
                        .name("Column1")
                        .type("STRING")
                        .build())
                    .uploadSettings(DataSetPhysicalTableMapS3SourceUploadSettingsArgs.builder()
                        .format("JSON")
                        .build())
                    .build())
                .build())
            .rowLevelPermissionTagConfiguration(DataSetRowLevelPermissionTagConfigurationArgs.builder()
                .status("ENABLED")
                .tagRules(DataSetRowLevelPermissionTagConfigurationTagRuleArgs.builder()
                    .columnName("Column1")
                    .tagKey("tagkey")
                    .matchAllValue("*")
                    .tagMultiValueDelimiter(",")
                    .build())
                .build())
            .build());

    }
}
```
```yaml
resources:
  example:
    type: aws:quicksight:DataSet
    properties:
      dataSetId: example-id
      name: example-name
      importMode: SPICE
      physicalTableMaps:
        - physicalTableMapId: example-id
          s3Source:
            dataSourceArn: ${exampleAwsQuicksightDataSource.arn}
            inputColumns:
              - name: Column1
                type: STRING
            uploadSettings:
              format: JSON
      rowLevelPermissionTagConfiguration:
        status: ENABLED
        tagRules:
          - columnName: Column1
            tagKey: tagkey
            matchAllValue: '*'
            tagMultiValueDelimiter: ','
```
<!--End PulumiCodeChooser -->

## Import

Using `pulumi import`, import a QuickSight Data Set using the AWS account ID and data set ID separated by a comma (`,`). For example:

```sh
$ pulumi import aws:quicksight/dataSet:DataSet example 123456789012,example-id
```
&
awsAccountIdB" AWS account ID.
�
columnGroups^B\*Z:X
V

quicksightDataSetColumnGroup4aws:quicksight/DataSetColumnGroup:DataSetColumnGroup�Groupings of columns that work together in certain Amazon QuickSight features. Currently, only geospatial hierarchy is supported. See column_groups.
�
columnLevelPermissionRules�B�*�:�
�

quicksight DataSetColumnLevelPermissionRulePaws:quicksight/DataSetColumnLevelPermissionRule:DataSetColumnLevelPermissionRule�A set of 1 or more definitions of a [ColumnLevelPermissionRule](https://docs.aws.amazon.com/quicksight/latest/APIReference/API_ColumnLevelPermissionRule.html). See column_level_permission_rules.
.
	dataSetId" Identifier for the data set.
�
dataSetUsageConfiguration�B�:�
�

quicksight DataSetDataSetUsageConfigurationPaws:quicksight/DataSetDataSetUsageConfiguration:DataSetDataSetUsageConfiguration~The usage configuration to apply to child datasets that reference this dataset as a source. See data_set_usage_configuration.
�
fieldFolders^B\*Z:X
V

quicksightDataSetFieldFolder4aws:quicksight/DataSetFieldFolder:DataSetFieldFolder[The folder that contains fields and nested subfolders for your dataset. See field_folders.
y

importMode" gIndicates whether you want to import the data into SPICE. Valid values are `SPICE` and `DIRECT_QUERY`.
�
logicalTableMapsjBh*f:d
b

quicksightDataSetLogicalTableMap<aws:quicksight/DataSetLogicalTableMap:DataSetLogicalTableMapConfigures the combination and transformation of the data from the physical tables. Maximum of 1 entry. See logical_table_map.
,
nameB" Display name for the dataset.
�
permissions[BY*W:U
S

quicksightDataSetPermission2aws:quicksight/DataSetPermission:DataSetPermissionXA set of resource permissions on the data source. Maximum of 64 items. See permissions.
�
physicalTableMapsmBk*i:g
e

quicksightDataSetPhysicalTableMap>aws:quicksight/DataSetPhysicalTableMap:DataSetPhysicalTableMap�Declares the physical tables that are available in the underlying data sources. See physical_table_map.

The following arguments are optional:
�
refreshPropertiesnBl:j
h

quicksightDataSetRefreshProperties@aws:quicksight/DataSetRefreshProperties:DataSetRefreshProperties|The refresh properties for the data set. **NOTE**: Only valid when `import_mode` is set to `SPICE`. See refresh_properties.
�
rowLevelPermissionDataSet�B�:�
�

quicksight DataSetRowLevelPermissionDataSetPaws:quicksight/DataSetRowLevelPermissionDataSet:DataSetRowLevelPermissionDataSetnThe row-level security configuration for the data that you want to create. See row_level_permission_data_set.
�
"rowLevelPermissionTagConfiguration�B�:�
�

quicksight)DataSetRowLevelPermissionTagConfigurationbaws:quicksight/DataSetRowLevelPermissionTagConfiguration:DataSetRowLevelPermissionTagConfiguration�The configuration of tags on a dataset to set row-level security. Row-level security tags are currently supported for anonymous embedding only. See row_level_permission_tag_configuration.
�
tagsB2" �Key-value map of resource tags. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
"7
arn" ,Amazon Resource Name (ARN) of the data set.
"$
awsAccountId" AWS account ID.
"�
columnGroups^B\*Z:X
V

quicksightDataSetColumnGroup4aws:quicksight/DataSetColumnGroup:DataSetColumnGroup�Groupings of columns that work together in certain Amazon QuickSight features. Currently, only geospatial hierarchy is supported. See column_groups.
"�
columnLevelPermissionRules�B�*�:�
�

quicksight DataSetColumnLevelPermissionRulePaws:quicksight/DataSetColumnLevelPermissionRule:DataSetColumnLevelPermissionRule�A set of 1 or more definitions of a [ColumnLevelPermissionRule](https://docs.aws.amazon.com/quicksight/latest/APIReference/API_ColumnLevelPermissionRule.html). See column_level_permission_rules.
".
	dataSetId" Identifier for the data set.
"�
dataSetUsageConfiguration�:�
�

quicksight DataSetDataSetUsageConfigurationPaws:quicksight/DataSetDataSetUsageConfiguration:DataSetDataSetUsageConfiguration~The usage configuration to apply to child datasets that reference this dataset as a source. See data_set_usage_configuration.
"�
fieldFolders^B\*Z:X
V

quicksightDataSetFieldFolder4aws:quicksight/DataSetFieldFolder:DataSetFieldFolder[The folder that contains fields and nested subfolders for your dataset. See field_folders.
"y

importMode" gIndicates whether you want to import the data into SPICE. Valid values are `SPICE` and `DIRECT_QUERY`.
"�
logicalTableMapsh*f:d
b

quicksightDataSetLogicalTableMap<aws:quicksight/DataSetLogicalTableMap:DataSetLogicalTableMapConfigures the combination and transformation of the data from the physical tables. Maximum of 1 entry. See logical_table_map.
"*
name" Display name for the dataset.
"p
outputColumns_*]:[
Y

quicksightDataSetOutputColumn6aws:quicksight/DataSetOutputColumn:DataSetOutputColumn"�
permissions[BY*W:U
S

quicksightDataSetPermission2aws:quicksight/DataSetPermission:DataSetPermissionXA set of resource permissions on the data source. Maximum of 64 items. See permissions.
"�
physicalTableMapsmBk*i:g
e

quicksightDataSetPhysicalTableMap>aws:quicksight/DataSetPhysicalTableMap:DataSetPhysicalTableMap�Declares the physical tables that are available in the underlying data sources. See physical_table_map.

The following arguments are optional:
"�
refreshPropertiesnBl:j
h

quicksightDataSetRefreshProperties@aws:quicksight/DataSetRefreshProperties:DataSetRefreshProperties|The refresh properties for the data set. **NOTE**: Only valid when `import_mode` is set to `SPICE`. See refresh_properties.
"�
rowLevelPermissionDataSet�B�:�
�

quicksight DataSetRowLevelPermissionDataSetPaws:quicksight/DataSetRowLevelPermissionDataSet:DataSetRowLevelPermissionDataSetnThe row-level security configuration for the data that you want to create. See row_level_permission_data_set.
"�
"rowLevelPermissionTagConfiguration�B�:�
�

quicksight)DataSetRowLevelPermissionTagConfigurationbaws:quicksight/DataSetRowLevelPermissionTagConfiguration:DataSetRowLevelPermissionTagConfiguration�The configuration of tags on a dataset to set row-level security. Row-level security tags are currently supported for anonymous embedding only. See row_level_permission_tag_configuration.
"�
tagsB2" �Key-value map of resource tags. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
"�
tagsAll2" xA map of tags assigned to the resource, including those inherited from the provider `default_tags` configuration block.
*�D
>

quicksight
DataSource$aws:quicksight/dataSource:DataSource�"Resource for managing QuickSight Data Source

## Example Usage

<!--Start PulumiCodeChooser -->
```typescript
import * as pulumi from "@pulumi/pulumi";
import * as aws from "@pulumi/aws";

const _default = new aws.quicksight.DataSource("default", {
    dataSourceId: "example-id",
    name: "My Cool Data in S3",
    parameters: {
        s3: {
            manifestFileLocation: {
                bucket: "my-bucket",
                key: "path/to/manifest.json",
            },
        },
    },
    type: "S3",
});
```
```python
import pulumi
import pulumi_aws as aws

default = aws.quicksight.DataSource("default",
    data_source_id="example-id",
    name="My Cool Data in S3",
    parameters={
        "s3": {
            "manifest_file_location": {
                "bucket": "my-bucket",
                "key": "path/to/manifest.json",
            },
        },
    },
    type="S3")
```
```csharp
using System.Collections.Generic;
using System.Linq;
using Pulumi;
using Aws = Pulumi.Aws;

return await Deployment.RunAsync(() => 
{
    var @default = new Aws.Quicksight.DataSource("default", new()
    {
        DataSourceId = "example-id",
        Name = "My Cool Data in S3",
        Parameters = new Aws.Quicksight.Inputs.DataSourceParametersArgs
        {
            S3 = new Aws.Quicksight.Inputs.DataSourceParametersS3Args
            {
                ManifestFileLocation = new Aws.Quicksight.Inputs.DataSourceParametersS3ManifestFileLocationArgs
                {
                    Bucket = "my-bucket",
                    Key = "path/to/manifest.json",
                },
            },
        },
        Type = "S3",
    });

});
```
```go
package main

import (
	"github.com/pulumi/pulumi-aws/sdk/v6/go/aws/quicksight"
	"github.com/pulumi/pulumi/sdk/v3/go/pulumi"
)

func main() {
	pulumi.Run(func(ctx *pulumi.Context) error {
		_, err := quicksight.NewDataSource(ctx, "default", &quicksight.DataSourceArgs{
			DataSourceId: pulumi.String("example-id"),
			Name:         pulumi.String("My Cool Data in S3"),
			Parameters: &quicksight.DataSourceParametersArgs{
				S3: &quicksight.DataSourceParametersS3Args{
					ManifestFileLocation: &quicksight.DataSourceParametersS3ManifestFileLocationArgs{
						Bucket: pulumi.String("my-bucket"),
						Key:    pulumi.String("path/to/manifest.json"),
					},
				},
			},
			Type: pulumi.String("S3"),
		})
		if err != nil {
			return err
		}
		return nil
	})
}
```
```java
package generated_program;

import com.pulumi.Context;
import com.pulumi.Pulumi;
import com.pulumi.core.Output;
import com.pulumi.aws.quicksight.DataSource;
import com.pulumi.aws.quicksight.DataSourceArgs;
import com.pulumi.aws.quicksight.inputs.DataSourceParametersArgs;
import com.pulumi.aws.quicksight.inputs.DataSourceParametersS3Args;
import com.pulumi.aws.quicksight.inputs.DataSourceParametersS3ManifestFileLocationArgs;
import java.util.List;
import java.util.ArrayList;
import java.util.Map;
import java.io.File;
import java.nio.file.Files;
import java.nio.file.Paths;

public class App {
    public static void main(String[] args) {
        Pulumi.run(App::stack);
    }

    public static void stack(Context ctx) {
        var default_ = new DataSource("default", DataSourceArgs.builder()
            .dataSourceId("example-id")
            .name("My Cool Data in S3")
            .parameters(DataSourceParametersArgs.builder()
                .s3(DataSourceParametersS3Args.builder()
                    .manifestFileLocation(DataSourceParametersS3ManifestFileLocationArgs.builder()
                        .bucket("my-bucket")
                        .key("path/to/manifest.json")
                        .build())
                    .build())
                .build())
            .type("S3")
            .build());

    }
}
```
```yaml
resources:
  default:
    type: aws:quicksight:DataSource
    properties:
      dataSourceId: example-id
      name: My Cool Data in S3
      parameters:
        s3:
          manifestFileLocation:
            bucket: my-bucket
            key: path/to/manifest.json
      type: S3
```
<!--End PulumiCodeChooser -->

## Import

Using `pulumi import`, import a QuickSight data source using the AWS account ID, and data source ID separated by a slash (`/`). For example:

```sh
$ pulumi import aws:quicksight/dataSource:DataSource example 123456789123/my-data-source-id
```
�
awsAccountIdB" �The ID for the AWS account that the data source is in. Currently, you use the ID for the AWS account that contains your Amazon QuickSight account.
�
credentialseBc:a
_

quicksightDataSourceCredentials:aws:quicksight/DataSourceCredentials:DataSourceCredentialsuThe credentials Amazon QuickSight uses to connect to your underlying source. See Credentials below for more details.
7
dataSourceId" #An identifier for the data source.
E
nameB" 7A name for the data source, maximum of 128 characters.
�

parameters`:^
\

quicksightDataSourceParameters8aws:quicksight/DataSourceParameters:DataSourceParametersBThe parameters used to connect to this data source (exactly one).
�
permissionsdBb*`:^
\

quicksightDataSourcePermission8aws:quicksight/DataSourcePermission:DataSourcePermissionnA set of resource permissions on the data source. Maximum of 64 items. See Permission below for more details.
�
sslPropertieskBi:g
e

quicksightDataSourceSslProperties>aws:quicksight/DataSourceSslProperties:DataSourceSslProperties�Secure Socket Layer (SSL) properties that apply when Amazon QuickSight connects to your underlying source. See SSL Properties below for more details.
�
tagsB2" �Key-value map of resource tags. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
�
type" �The type of the data source. See the [AWS Documentation](https://docs.aws.amazon.com/quicksight/latest/APIReference/API_CreateDataSource.html#QS-CreateDataSource-request-Type) for the complete list of valid values.

The following arguments are optional:
�
vpcConnectionProperties�B�:�
�

quicksight!DataSourceVpcConnectionPropertiesRaws:quicksight/DataSourceVpcConnectionProperties:DataSourceVpcConnectionProperties�Use this parameter only when you want Amazon QuickSight to use a VPC connection when connecting to your underlying source. See VPC Connection Properties below for more details.
"9
arn" .Amazon Resource Name (ARN) of the data source
"�
awsAccountId" �The ID for the AWS account that the data source is in. Currently, you use the ID for the AWS account that contains your Amazon QuickSight account.
"�
credentialseBc:a
_

quicksightDataSourceCredentials:aws:quicksight/DataSourceCredentials:DataSourceCredentialsuThe credentials Amazon QuickSight uses to connect to your underlying source. See Credentials below for more details.
"7
dataSourceId" #An identifier for the data source.
"C
name" 7A name for the data source, maximum of 128 characters.
"�

parameters`:^
\

quicksightDataSourceParameters8aws:quicksight/DataSourceParameters:DataSourceParametersBThe parameters used to connect to this data source (exactly one).
"�
permissionsdBb*`:^
\

quicksightDataSourcePermission8aws:quicksight/DataSourcePermission:DataSourcePermissionnA set of resource permissions on the data source. Maximum of 64 items. See Permission below for more details.
"�
sslPropertiesi:g
e

quicksightDataSourceSslProperties>aws:quicksight/DataSourceSslProperties:DataSourceSslProperties�Secure Socket Layer (SSL) properties that apply when Amazon QuickSight connects to your underlying source. See SSL Properties below for more details.
"�
tagsB2" �Key-value map of resource tags. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
"�
tagsAll2" xA map of tags assigned to the resource, including those inherited from the provider `default_tags` configuration block.
"�
type" �The type of the data source. See the [AWS Documentation](https://docs.aws.amazon.com/quicksight/latest/APIReference/API_CreateDataSource.html#QS-CreateDataSource-request-Type) for the complete list of valid values.

The following arguments are optional:
"�
vpcConnectionProperties�B�:�
�

quicksight!DataSourceVpcConnectionPropertiesRaws:quicksight/DataSourceVpcConnectionProperties:DataSourceVpcConnectionProperties�Use this parameter only when you want Amazon QuickSight to use a VPC connection when connecting to your underlying source. See VPC Connection Properties below for more details.
*�b
2

quicksightFolderaws:quicksight/folder:Folder�RResource for managing a QuickSight Folder.

## Example Usage

### Basic Usage

<!--Start PulumiCodeChooser -->
```typescript
import * as pulumi from "@pulumi/pulumi";
import * as aws from "@pulumi/aws";

const example = new aws.quicksight.Folder("example", {
    folderId: "example-id",
    name: "example-name",
});
```
```python
import pulumi
import pulumi_aws as aws

example = aws.quicksight.Folder("example",
    folder_id="example-id",
    name="example-name")
```
```csharp
using System.Collections.Generic;
using System.Linq;
using Pulumi;
using Aws = Pulumi.Aws;

return await Deployment.RunAsync(() => 
{
    var example = new Aws.Quicksight.Folder("example", new()
    {
        FolderId = "example-id",
        Name = "example-name",
    });

});
```
```go
package main

import (
	"github.com/pulumi/pulumi-aws/sdk/v6/go/aws/quicksight"
	"github.com/pulumi/pulumi/sdk/v3/go/pulumi"
)

func main() {
	pulumi.Run(func(ctx *pulumi.Context) error {
		_, err := quicksight.NewFolder(ctx, "example", &quicksight.FolderArgs{
			FolderId: pulumi.String("example-id"),
			Name:     pulumi.String("example-name"),
		})
		if err != nil {
			return err
		}
		return nil
	})
}
```
```java
package generated_program;

import com.pulumi.Context;
import com.pulumi.Pulumi;
import com.pulumi.core.Output;
import com.pulumi.aws.quicksight.Folder;
import com.pulumi.aws.quicksight.FolderArgs;
import java.util.List;
import java.util.ArrayList;
import java.util.Map;
import java.io.File;
import java.nio.file.Files;
import java.nio.file.Paths;

public class App {
    public static void main(String[] args) {
        Pulumi.run(App::stack);
    }

    public static void stack(Context ctx) {
        var example = new Folder("example", FolderArgs.builder()
            .folderId("example-id")
            .name("example-name")
            .build());

    }
}
```
```yaml
resources:
  example:
    type: aws:quicksight:Folder
    properties:
      folderId: example-id
      name: example-name
```
<!--End PulumiCodeChooser -->

### With Permissions

<!--Start PulumiCodeChooser -->
```typescript
import * as pulumi from "@pulumi/pulumi";
import * as aws from "@pulumi/aws";

const example = new aws.quicksight.Folder("example", {
    folderId: "example-id",
    name: "example-name",
    permissions: [{
        actions: [
            "quicksight:CreateFolder",
            "quicksight:DescribeFolder",
            "quicksight:UpdateFolder",
            "quicksight:DeleteFolder",
            "quicksight:CreateFolderMembership",
            "quicksight:DeleteFolderMembership",
            "quicksight:DescribeFolderPermissions",
            "quicksight:UpdateFolderPermissions",
        ],
        principal: exampleAwsQuicksightUser.arn,
    }],
});
```
```python
import pulumi
import pulumi_aws as aws

example = aws.quicksight.Folder("example",
    folder_id="example-id",
    name="example-name",
    permissions=[{
        "actions": [
            "quicksight:CreateFolder",
            "quicksight:DescribeFolder",
            "quicksight:UpdateFolder",
            "quicksight:DeleteFolder",
            "quicksight:CreateFolderMembership",
            "quicksight:DeleteFolderMembership",
            "quicksight:DescribeFolderPermissions",
            "quicksight:UpdateFolderPermissions",
        ],
        "principal": example_aws_quicksight_user["arn"],
    }])
```
```csharp
using System.Collections.Generic;
using System.Linq;
using Pulumi;
using Aws = Pulumi.Aws;

return await Deployment.RunAsync(() => 
{
    var example = new Aws.Quicksight.Folder("example", new()
    {
        FolderId = "example-id",
        Name = "example-name",
        Permissions = new[]
        {
            new Aws.Quicksight.Inputs.FolderPermissionArgs
            {
                Actions = new[]
                {
                    "quicksight:CreateFolder",
                    "quicksight:DescribeFolder",
                    "quicksight:UpdateFolder",
                    "quicksight:DeleteFolder",
                    "quicksight:CreateFolderMembership",
                    "quicksight:DeleteFolderMembership",
                    "quicksight:DescribeFolderPermissions",
                    "quicksight:UpdateFolderPermissions",
                },
                Principal = exampleAwsQuicksightUser.Arn,
            },
        },
    });

});
```
```go
package main

import (
	"github.com/pulumi/pulumi-aws/sdk/v6/go/aws/quicksight"
	"github.com/pulumi/pulumi/sdk/v3/go/pulumi"
)

func main() {
	pulumi.Run(func(ctx *pulumi.Context) error {
		_, err := quicksight.NewFolder(ctx, "example", &quicksight.FolderArgs{
			FolderId: pulumi.String("example-id"),
			Name:     pulumi.String("example-name"),
			Permissions: quicksight.FolderPermissionArray{
				&quicksight.FolderPermissionArgs{
					Actions: pulumi.StringArray{
						pulumi.String("quicksight:CreateFolder"),
						pulumi.String("quicksight:DescribeFolder"),
						pulumi.String("quicksight:UpdateFolder"),
						pulumi.String("quicksight:DeleteFolder"),
						pulumi.String("quicksight:CreateFolderMembership"),
						pulumi.String("quicksight:DeleteFolderMembership"),
						pulumi.String("quicksight:DescribeFolderPermissions"),
						pulumi.String("quicksight:UpdateFolderPermissions"),
					},
					Principal: pulumi.Any(exampleAwsQuicksightUser.Arn),
				},
			},
		})
		if err != nil {
			return err
		}
		return nil
	})
}
```
```java
package generated_program;

import com.pulumi.Context;
import com.pulumi.Pulumi;
import com.pulumi.core.Output;
import com.pulumi.aws.quicksight.Folder;
import com.pulumi.aws.quicksight.FolderArgs;
import com.pulumi.aws.quicksight.inputs.FolderPermissionArgs;
import java.util.List;
import java.util.ArrayList;
import java.util.Map;
import java.io.File;
import java.nio.file.Files;
import java.nio.file.Paths;

public class App {
    public static void main(String[] args) {
        Pulumi.run(App::stack);
    }

    public static void stack(Context ctx) {
        var example = new Folder("example", FolderArgs.builder()
            .folderId("example-id")
            .name("example-name")
            .permissions(FolderPermissionArgs.builder()
                .actions(                
                    "quicksight:CreateFolder",
                    "quicksight:DescribeFolder",
                    "quicksight:UpdateFolder",
                    "quicksight:DeleteFolder",
                    "quicksight:CreateFolderMembership",
                    "quicksight:DeleteFolderMembership",
                    "quicksight:DescribeFolderPermissions",
                    "quicksight:UpdateFolderPermissions")
                .principal(exampleAwsQuicksightUser.arn())
                .build())
            .build());

    }
}
```
```yaml
resources:
  example:
    type: aws:quicksight:Folder
    properties:
      folderId: example-id
      name: example-name
      permissions:
        - actions:
            - quicksight:CreateFolder
            - quicksight:DescribeFolder
            - quicksight:UpdateFolder
            - quicksight:DeleteFolder
            - quicksight:CreateFolderMembership
            - quicksight:DeleteFolderMembership
            - quicksight:DescribeFolderPermissions
            - quicksight:UpdateFolderPermissions
          principal: ${exampleAwsQuicksightUser.arn}
```
<!--End PulumiCodeChooser -->

### With Parent Folder

<!--Start PulumiCodeChooser -->
```typescript
import * as pulumi from "@pulumi/pulumi";
import * as aws from "@pulumi/aws";

const parent = new aws.quicksight.Folder("parent", {
    folderId: "parent-id",
    name: "parent-name",
});
const example = new aws.quicksight.Folder("example", {
    folderId: "example-id",
    name: "example-name",
    parentFolderArn: parent.arn,
});
```
```python
import pulumi
import pulumi_aws as aws

parent = aws.quicksight.Folder("parent",
    folder_id="parent-id",
    name="parent-name")
example = aws.quicksight.Folder("example",
    folder_id="example-id",
    name="example-name",
    parent_folder_arn=parent.arn)
```
```csharp
using System.Collections.Generic;
using System.Linq;
using Pulumi;
using Aws = Pulumi.Aws;

return await Deployment.RunAsync(() => 
{
    var parent = new Aws.Quicksight.Folder("parent", new()
    {
        FolderId = "parent-id",
        Name = "parent-name",
    });

    var example = new Aws.Quicksight.Folder("example", new()
    {
        FolderId = "example-id",
        Name = "example-name",
        ParentFolderArn = parent.Arn,
    });

});
```
```go
package main

import (
	"github.com/pulumi/pulumi-aws/sdk/v6/go/aws/quicksight"
	"github.com/pulumi/pulumi/sdk/v3/go/pulumi"
)

func main() {
	pulumi.Run(func(ctx *pulumi.Context) error {
		parent, err := quicksight.NewFolder(ctx, "parent", &quicksight.FolderArgs{
			FolderId: pulumi.String("parent-id"),
			Name:     pulumi.String("parent-name"),
		})
		if err != nil {
			return err
		}
		_, err = quicksight.NewFolder(ctx, "example", &quicksight.FolderArgs{
			FolderId:        pulumi.String("example-id"),
			Name:            pulumi.String("example-name"),
			ParentFolderArn: parent.Arn,
		})
		if err != nil {
			return err
		}
		return nil
	})
}
```
```java
package generated_program;

import com.pulumi.Context;
import com.pulumi.Pulumi;
import com.pulumi.core.Output;
import com.pulumi.aws.quicksight.Folder;
import com.pulumi.aws.quicksight.FolderArgs;
import java.util.List;
import java.util.ArrayList;
import java.util.Map;
import java.io.File;
import java.nio.file.Files;
import java.nio.file.Paths;

public class App {
    public static void main(String[] args) {
        Pulumi.run(App::stack);
    }

    public static void stack(Context ctx) {
        var parent = new Folder("parent", FolderArgs.builder()
            .folderId("parent-id")
            .name("parent-name")
            .build());

        var example = new Folder("example", FolderArgs.builder()
            .folderId("example-id")
            .name("example-name")
            .parentFolderArn(parent.arn())
            .build());

    }
}
```
```yaml
resources:
  parent:
    type: aws:quicksight:Folder
    properties:
      folderId: parent-id
      name: parent-name
  example:
    type: aws:quicksight:Folder
    properties:
      folderId: example-id
      name: example-name
      parentFolderArn: ${parent.arn}
```
<!--End PulumiCodeChooser -->

## Import

Using `pulumi import`, import a QuickSight folder using the AWS account ID and folder ID name separated by a comma (`,`). For example:

```sh
$ pulumi import aws:quicksight/folder:Folder example 123456789012,example-id
```
&
awsAccountIdB" AWS account ID.
+
folderId" Identifier for the folder.
`

folderTypeB" LThe type of folder. By default, it is `SHARED`. Valid values are: `SHARED`.
R
nameB" DDisplay name for the folder.

The following arguments are optional:
x
parentFolderArnB" _The Amazon Resource Name (ARN) for the parent folder. If not set, creates a root-level folder.
�
permissionsXBV*T:R
P

quicksightFolderPermission0aws:quicksight/FolderPermission:FolderPermissionSA set of resource permissions on the folder. Maximum of 64 items. See permissions.
�
tagsB2" �Key-value map of resource tags. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
"
arn" ARN of the folder.
"$
awsAccountId" AWS account ID.
"9
createdTime" &The time that the folder was created.
"+
folderId" Identifier for the folder.
"d
folderPaths*" OAn array of ancestor ARN strings for the folder. Empty for root-level folders.
"`

folderTypeB" LThe type of folder. By default, it is `SHARED`. Valid values are: `SHARED`.
"B
lastUpdatedTime" +The time that the folder was last updated.
"P
name" DDisplay name for the folder.

The following arguments are optional:
"x
parentFolderArnB" _The Amazon Resource Name (ARN) for the parent folder. If not set, creates a root-level folder.
"�
permissionsXBV*T:R
P

quicksightFolderPermission0aws:quicksight/FolderPermission:FolderPermissionSA set of resource permissions on the folder. Maximum of 64 items. See permissions.
"�
tagsB2" �Key-value map of resource tags. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
"�
tagsAll2" xA map of tags assigned to the resource, including those inherited from the provider `default_tags` configuration block.
*�
P

quicksightFolderMembership0aws:quicksight/folderMembership:FolderMembership�Resource for managing an AWS QuickSight Folder Membership.

## Example Usage

### Basic Usage

<!--Start PulumiCodeChooser -->
```typescript
import * as pulumi from "@pulumi/pulumi";
import * as aws from "@pulumi/aws";

const example = new aws.quicksight.FolderMembership("example", {
    folderId: exampleAwsQuicksightFolder.folderId,
    memberType: "DATASET",
    memberId: exampleAwsQuicksightDataSet.dataSetId,
});
```
```python
import pulumi
import pulumi_aws as aws

example = aws.quicksight.FolderMembership("example",
    folder_id=example_aws_quicksight_folder["folderId"],
    member_type="DATASET",
    member_id=example_aws_quicksight_data_set["dataSetId"])
```
```csharp
using System.Collections.Generic;
using System.Linq;
using Pulumi;
using Aws = Pulumi.Aws;

return await Deployment.RunAsync(() => 
{
    var example = new Aws.Quicksight.FolderMembership("example", new()
    {
        FolderId = exampleAwsQuicksightFolder.FolderId,
        MemberType = "DATASET",
        MemberId = exampleAwsQuicksightDataSet.DataSetId,
    });

});
```
```go
package main

import (
	"github.com/pulumi/pulumi-aws/sdk/v6/go/aws/quicksight"
	"github.com/pulumi/pulumi/sdk/v3/go/pulumi"
)

func main() {
	pulumi.Run(func(ctx *pulumi.Context) error {
		_, err := quicksight.NewFolderMembership(ctx, "example", &quicksight.FolderMembershipArgs{
			FolderId:   pulumi.Any(exampleAwsQuicksightFolder.FolderId),
			MemberType: pulumi.String("DATASET"),
			MemberId:   pulumi.Any(exampleAwsQuicksightDataSet.DataSetId),
		})
		if err != nil {
			return err
		}
		return nil
	})
}
```
```java
package generated_program;

import com.pulumi.Context;
import com.pulumi.Pulumi;
import com.pulumi.core.Output;
import com.pulumi.aws.quicksight.FolderMembership;
import com.pulumi.aws.quicksight.FolderMembershipArgs;
import java.util.List;
import java.util.ArrayList;
import java.util.Map;
import java.io.File;
import java.nio.file.Files;
import java.nio.file.Paths;

public class App {
    public static void main(String[] args) {
        Pulumi.run(App::stack);
    }

    public static void stack(Context ctx) {
        var example = new FolderMembership("example", FolderMembershipArgs.builder()
            .folderId(exampleAwsQuicksightFolder.folderId())
            .memberType("DATASET")
            .memberId(exampleAwsQuicksightDataSet.dataSetId())
            .build());

    }
}
```
```yaml
resources:
  example:
    type: aws:quicksight:FolderMembership
    properties:
      folderId: ${exampleAwsQuicksightFolder.folderId}
      memberType: DATASET
      memberId: ${exampleAwsQuicksightDataSet.dataSetId}
```
<!--End PulumiCodeChooser -->

## Import

Using `pulumi import`, import QuickSight Folder Membership using the AWS account ID, folder ID, member type, and member ID separated by commas (`,`). For example:

```sh
$ pulumi import aws:quicksight/folderMembership:FolderMembership example 123456789012,example-folder,DATASET,example-dataset
```
&
awsAccountIdB" AWS account ID.
+
folderId" Identifier for the folder.
G
memberId" 7ID of the asset (the dashboard, analysis, or dataset).
�

memberType" tType of the member. Valid values are `ANALYSIS`, `DASHBOARD`, and `DATASET`.

The following arguments are optional:
"$
awsAccountId" AWS account ID.
"+
folderId" Identifier for the folder.
"G
memberId" 7ID of the asset (the dashboard, analysis, or dataset).
"�

memberType" tType of the member. Valid values are `ANALYSIS`, `DASHBOARD`, and `DATASET`.

The following arguments are optional:
*�
/

quicksightGroupaws:quicksight/group:Group�Resource for managing QuickSight Group

## Example Usage

<!--Start PulumiCodeChooser -->
```typescript
import * as pulumi from "@pulumi/pulumi";
import * as aws from "@pulumi/aws";

const example = new aws.quicksight.Group("example", {groupName: "tf-example"});
```
```python
import pulumi
import pulumi_aws as aws

example = aws.quicksight.Group("example", group_name="tf-example")
```
```csharp
using System.Collections.Generic;
using System.Linq;
using Pulumi;
using Aws = Pulumi.Aws;

return await Deployment.RunAsync(() => 
{
    var example = new Aws.Quicksight.Group("example", new()
    {
        GroupName = "tf-example",
    });

});
```
```go
package main

import (
	"github.com/pulumi/pulumi-aws/sdk/v6/go/aws/quicksight"
	"github.com/pulumi/pulumi/sdk/v3/go/pulumi"
)

func main() {
	pulumi.Run(func(ctx *pulumi.Context) error {
		_, err := quicksight.NewGroup(ctx, "example", &quicksight.GroupArgs{
			GroupName: pulumi.String("tf-example"),
		})
		if err != nil {
			return err
		}
		return nil
	})
}
```
```java
package generated_program;

import com.pulumi.Context;
import com.pulumi.Pulumi;
import com.pulumi.core.Output;
import com.pulumi.aws.quicksight.Group;
import com.pulumi.aws.quicksight.GroupArgs;
import java.util.List;
import java.util.ArrayList;
import java.util.Map;
import java.io.File;
import java.nio.file.Files;
import java.nio.file.Paths;

public class App {
    public static void main(String[] args) {
        Pulumi.run(App::stack);
    }

    public static void stack(Context ctx) {
        var example = new Group("example", GroupArgs.builder()
            .groupName("tf-example")
            .build());

    }
}
```
```yaml
resources:
  example:
    type: aws:quicksight:Group
    properties:
      groupName: tf-example
```
<!--End PulumiCodeChooser -->

## Import

Using `pulumi import`, import QuickSight Group using the aws account id, namespace and group name separated by `/`. For example:

```sh
$ pulumi import aws:quicksight/group:Group example 123456789123/default/tf-example
```
�
awsAccountIdB" �The ID for the AWS account that the group is in. Currently, you use the ID for the AWS account that contains your Amazon QuickSight account.
2
descriptionB" A description for the group.
'
	groupName" A name for the group.
O
	namespaceB" <The namespace. Currently, you should set this to `default`.
"/
arn" $Amazon Resource Name (ARN) of group
"�
awsAccountId" �The ID for the AWS account that the group is in. Currently, you use the ID for the AWS account that contains your Amazon QuickSight account.
"2
descriptionB" A description for the group.
"'
	groupName" A name for the group.
"O
	namespaceB" <The namespace. Currently, you should set this to `default`.
*�
M

quicksightGroupMembership.aws:quicksight/groupMembership:GroupMembership�Resource for managing QuickSight Group Membership

## Example Usage

<!--Start PulumiCodeChooser -->
```typescript
import * as pulumi from "@pulumi/pulumi";
import * as aws from "@pulumi/aws";

const example = new aws.quicksight.GroupMembership("example", {
    groupName: "all-access-users",
    memberName: "john_smith",
});
```
```python
import pulumi
import pulumi_aws as aws

example = aws.quicksight.GroupMembership("example",
    group_name="all-access-users",
    member_name="john_smith")
```
```csharp
using System.Collections.Generic;
using System.Linq;
using Pulumi;
using Aws = Pulumi.Aws;

return await Deployment.RunAsync(() => 
{
    var example = new Aws.Quicksight.GroupMembership("example", new()
    {
        GroupName = "all-access-users",
        MemberName = "john_smith",
    });

});
```
```go
package main

import (
	"github.com/pulumi/pulumi-aws/sdk/v6/go/aws/quicksight"
	"github.com/pulumi/pulumi/sdk/v3/go/pulumi"
)

func main() {
	pulumi.Run(func(ctx *pulumi.Context) error {
		_, err := quicksight.NewGroupMembership(ctx, "example", &quicksight.GroupMembershipArgs{
			GroupName:  pulumi.String("all-access-users"),
			MemberName: pulumi.String("john_smith"),
		})
		if err != nil {
			return err
		}
		return nil
	})
}
```
```java
package generated_program;

import com.pulumi.Context;
import com.pulumi.Pulumi;
import com.pulumi.core.Output;
import com.pulumi.aws.quicksight.GroupMembership;
import com.pulumi.aws.quicksight.GroupMembershipArgs;
import java.util.List;
import java.util.ArrayList;
import java.util.Map;
import java.io.File;
import java.nio.file.Files;
import java.nio.file.Paths;

public class App {
    public static void main(String[] args) {
        Pulumi.run(App::stack);
    }

    public static void stack(Context ctx) {
        var example = new GroupMembership("example", GroupMembershipArgs.builder()
            .groupName("all-access-users")
            .memberName("john_smith")
            .build());

    }
}
```
```yaml
resources:
  example:
    type: aws:quicksight:GroupMembership
    properties:
      groupName: all-access-users
      memberName: john_smith
```
<!--End PulumiCodeChooser -->

## Import

Using `pulumi import`, import QuickSight Group membership using the AWS account ID, namespace, group name and member name separated by `/`. For example:

```sh
$ pulumi import aws:quicksight/groupMembership:GroupMembership example 123456789123/default/all-access-users/john_smith
```
�
awsAccountIdB" �The ID for the AWS account that the group is in. Currently, you use the ID for the AWS account that contains your Amazon QuickSight account.
J
	groupName" 9The name of the group in which the member will be added.
>

memberName" ,The name of the member to add to the group.
`
	namespaceB" MThe namespace that you want the user to be a part of. Defaults to `default`.
"	
arn" "�
awsAccountId" �The ID for the AWS account that the group is in. Currently, you use the ID for the AWS account that contains your Amazon QuickSight account.
"J
	groupName" 9The name of the group in which the member will be added.
">

memberName" ,The name of the member to add to the group.
"`
	namespaceB" MThe namespace that you want the user to be a part of. Defaults to `default`.
*�'
Y

quicksightIamPolicyAssignment6aws:quicksight/iamPolicyAssignment:IamPolicyAssignment�Resource for managing an AWS QuickSight IAM Policy Assignment.

## Example Usage

### Basic Usage

<!--Start PulumiCodeChooser -->
```typescript
import * as pulumi from "@pulumi/pulumi";
import * as aws from "@pulumi/aws";

const example = new aws.quicksight.IamPolicyAssignment("example", {
    assignmentName: "example",
    assignmentStatus: "ENABLED",
    policyArn: exampleAwsIamPolicy.arn,
    identities: {
        users: [exampleAwsQuicksightUser.userName],
    },
});
```
```python
import pulumi
import pulumi_aws as aws

example = aws.quicksight.IamPolicyAssignment("example",
    assignment_name="example",
    assignment_status="ENABLED",
    policy_arn=example_aws_iam_policy["arn"],
    identities={
        "users": [example_aws_quicksight_user["userName"]],
    })
```
```csharp
using System.Collections.Generic;
using System.Linq;
using Pulumi;
using Aws = Pulumi.Aws;

return await Deployment.RunAsync(() => 
{
    var example = new Aws.Quicksight.IamPolicyAssignment("example", new()
    {
        AssignmentName = "example",
        AssignmentStatus = "ENABLED",
        PolicyArn = exampleAwsIamPolicy.Arn,
        Identities = new Aws.Quicksight.Inputs.IamPolicyAssignmentIdentitiesArgs
        {
            Users = new[]
            {
                exampleAwsQuicksightUser.UserName,
            },
        },
    });

});
```
```go
package main

import (
	"github.com/pulumi/pulumi-aws/sdk/v6/go/aws/quicksight"
	"github.com/pulumi/pulumi/sdk/v3/go/pulumi"
)

func main() {
	pulumi.Run(func(ctx *pulumi.Context) error {
		_, err := quicksight.NewIamPolicyAssignment(ctx, "example", &quicksight.IamPolicyAssignmentArgs{
			AssignmentName:   pulumi.String("example"),
			AssignmentStatus: pulumi.String("ENABLED"),
			PolicyArn:        pulumi.Any(exampleAwsIamPolicy.Arn),
			Identities: &quicksight.IamPolicyAssignmentIdentitiesArgs{
				Users: pulumi.StringArray{
					exampleAwsQuicksightUser.UserName,
				},
			},
		})
		if err != nil {
			return err
		}
		return nil
	})
}
```
```java
package generated_program;

import com.pulumi.Context;
import com.pulumi.Pulumi;
import com.pulumi.core.Output;
import com.pulumi.aws.quicksight.IamPolicyAssignment;
import com.pulumi.aws.quicksight.IamPolicyAssignmentArgs;
import com.pulumi.aws.quicksight.inputs.IamPolicyAssignmentIdentitiesArgs;
import java.util.List;
import java.util.ArrayList;
import java.util.Map;
import java.io.File;
import java.nio.file.Files;
import java.nio.file.Paths;

public class App {
    public static void main(String[] args) {
        Pulumi.run(App::stack);
    }

    public static void stack(Context ctx) {
        var example = new IamPolicyAssignment("example", IamPolicyAssignmentArgs.builder()
            .assignmentName("example")
            .assignmentStatus("ENABLED")
            .policyArn(exampleAwsIamPolicy.arn())
            .identities(IamPolicyAssignmentIdentitiesArgs.builder()
                .users(exampleAwsQuicksightUser.userName())
                .build())
            .build());

    }
}
```
```yaml
resources:
  example:
    type: aws:quicksight:IamPolicyAssignment
    properties:
      assignmentName: example
      assignmentStatus: ENABLED
      policyArn: ${exampleAwsIamPolicy.arn}
      identities:
        users:
          - ${exampleAwsQuicksightUser.userName}
```
<!--End PulumiCodeChooser -->

## Import

Using `pulumi import`, import QuickSight IAM Policy Assignment using the AWS account ID, namespace, and assignment name separated by commas (`,`). For example:

```sh
$ pulumi import aws:quicksight/iamPolicyAssignment:IamPolicyAssignment example 123456789012,default,example
```
.
assignmentName" Name of the assignment.
�
assignmentStatus" vStatus of the assignment. Valid values are `ENABLED`, `DISABLED`, and `DRAFT`.

The following arguments are optional:
&
awsAccountIdB" AWS account ID.
�

identities}B{:y
w

quicksightIamPolicyAssignmentIdentitiesJaws:quicksight/IamPolicyAssignmentIdentities:IamPolicyAssignmentIdentitiesZAmazon QuickSight users, groups, or both to assign the policy to. See `identities` block.
R
	namespaceB" ?Namespace that contains the assignment. Defaults to `default`.
z
	policyArnB" gARN of the IAM policy to apply to the Amazon QuickSight users and groups specified in this assignment.
"#
assignmentId" Assignment ID.
".
assignmentName" Name of the assignment.
"�
assignmentStatus" vStatus of the assignment. Valid values are `ENABLED`, `DISABLED`, and `DRAFT`.

The following arguments are optional:
"$
awsAccountId" AWS account ID.
"�

identities}B{:y
w

quicksightIamPolicyAssignmentIdentitiesJaws:quicksight/IamPolicyAssignmentIdentities:IamPolicyAssignmentIdentitiesZAmazon QuickSight users, groups, or both to assign the policy to. See `identities` block.
"P
	namespace" ?Namespace that contains the assignment. Defaults to `default`.
"z
	policyArnB" gARN of the IAM policy to apply to the Amazon QuickSight users and groups specified in this assignment.
*�
;

quicksight	Ingestion"aws:quicksight/ingestion:Ingestion�Resource for managing an AWS QuickSight Ingestion.

## Example Usage

### Basic Usage

<!--Start PulumiCodeChooser -->
```typescript
import * as pulumi from "@pulumi/pulumi";
import * as aws from "@pulumi/aws";

const example = new aws.quicksight.Ingestion("example", {
    dataSetId: exampleAwsQuicksightDataSet.dataSetId,
    ingestionId: "example-id",
    ingestionType: "FULL_REFRESH",
});
```
```python
import pulumi
import pulumi_aws as aws

example = aws.quicksight.Ingestion("example",
    data_set_id=example_aws_quicksight_data_set["dataSetId"],
    ingestion_id="example-id",
    ingestion_type="FULL_REFRESH")
```
```csharp
using System.Collections.Generic;
using System.Linq;
using Pulumi;
using Aws = Pulumi.Aws;

return await Deployment.RunAsync(() => 
{
    var example = new Aws.Quicksight.Ingestion("example", new()
    {
        DataSetId = exampleAwsQuicksightDataSet.DataSetId,
        IngestionId = "example-id",
        IngestionType = "FULL_REFRESH",
    });

});
```
```go
package main

import (
	"github.com/pulumi/pulumi-aws/sdk/v6/go/aws/quicksight"
	"github.com/pulumi/pulumi/sdk/v3/go/pulumi"
)

func main() {
	pulumi.Run(func(ctx *pulumi.Context) error {
		_, err := quicksight.NewIngestion(ctx, "example", &quicksight.IngestionArgs{
			DataSetId:     pulumi.Any(exampleAwsQuicksightDataSet.DataSetId),
			IngestionId:   pulumi.String("example-id"),
			IngestionType: pulumi.String("FULL_REFRESH"),
		})
		if err != nil {
			return err
		}
		return nil
	})
}
```
```java
package generated_program;

import com.pulumi.Context;
import com.pulumi.Pulumi;
import com.pulumi.core.Output;
import com.pulumi.aws.quicksight.Ingestion;
import com.pulumi.aws.quicksight.IngestionArgs;
import java.util.List;
import java.util.ArrayList;
import java.util.Map;
import java.io.File;
import java.nio.file.Files;
import java.nio.file.Paths;

public class App {
    public static void main(String[] args) {
        Pulumi.run(App::stack);
    }

    public static void stack(Context ctx) {
        var example = new Ingestion("example", IngestionArgs.builder()
            .dataSetId(exampleAwsQuicksightDataSet.dataSetId())
            .ingestionId("example-id")
            .ingestionType("FULL_REFRESH")
            .build());

    }
}
```
```yaml
resources:
  example:
    type: aws:quicksight:Ingestion
    properties:
      dataSetId: ${exampleAwsQuicksightDataSet.dataSetId}
      ingestionId: example-id
      ingestionType: FULL_REFRESH
```
<!--End PulumiCodeChooser -->

## Import

Using `pulumi import`, import QuickSight Ingestion using the AWS account ID, data set ID, and ingestion ID separated by commas (`,`). For example:

```sh
$ pulumi import aws:quicksight/ingestion:Ingestion example 123456789012,example-dataset-id,example-ingestion-id
```
&
awsAccountIdB" AWS account ID.
:
	dataSetId" )ID of the dataset used in the ingestion.
)
ingestionId" ID for the ingestion.
�
ingestionType" �Type of ingestion to be created. Valid values are `INCREMENTAL_REFRESH` and `FULL_REFRESH`.

The following arguments are optional:
"!
arn" ARN of the Ingestion.
"$
awsAccountId" AWS account ID.
":
	dataSetId" )ID of the dataset used in the ingestion.
")
ingestionId" ID for the ingestion.
")
ingestionStatus" Ingestion status.
"�
ingestionType" �Type of ingestion to be created. Valid values are `INCREMENTAL_REFRESH` and `FULL_REFRESH`.

The following arguments are optional:
*�
;

quicksight	Namespace"aws:quicksight/namespace:Namespace�Resource for managing an AWS QuickSight Namespace.

## Example Usage

### Basic Usage

<!--Start PulumiCodeChooser -->
```typescript
import * as pulumi from "@pulumi/pulumi";
import * as aws from "@pulumi/aws";

const example = new aws.quicksight.Namespace("example", {namespace: "example"});
```
```python
import pulumi
import pulumi_aws as aws

example = aws.quicksight.Namespace("example", namespace="example")
```
```csharp
using System.Collections.Generic;
using System.Linq;
using Pulumi;
using Aws = Pulumi.Aws;

return await Deployment.RunAsync(() => 
{
    var example = new Aws.Quicksight.Namespace("example", new()
    {
        NameSpace = "example",
    });

});
```
```go
package main

import (
	"github.com/pulumi/pulumi-aws/sdk/v6/go/aws/quicksight"
	"github.com/pulumi/pulumi/sdk/v3/go/pulumi"
)

func main() {
	pulumi.Run(func(ctx *pulumi.Context) error {
		_, err := quicksight.NewNamespace(ctx, "example", &quicksight.NamespaceArgs{
			Namespace: pulumi.String("example"),
		})
		if err != nil {
			return err
		}
		return nil
	})
}
```
```java
package generated_program;

import com.pulumi.Context;
import com.pulumi.Pulumi;
import com.pulumi.core.Output;
import com.pulumi.aws.quicksight.Namespace;
import com.pulumi.aws.quicksight.NamespaceArgs;
import java.util.List;
import java.util.ArrayList;
import java.util.Map;
import java.io.File;
import java.nio.file.Files;
import java.nio.file.Paths;

public class App {
    public static void main(String[] args) {
        Pulumi.run(App::stack);
    }

    public static void stack(Context ctx) {
        var example = new Namespace("example", NamespaceArgs.builder()
            .namespace("example")
            .build());

    }
}
```
```yaml
resources:
  example:
    type: aws:quicksight:Namespace
    properties:
      namespace: example
```
<!--End PulumiCodeChooser -->

## Import

Using `pulumi import`, import QuickSight Namespace using the AWS account ID and namespace separated by commas (`,`). For example:

```sh
$ pulumi import aws:quicksight/namespace:Namespace example 123456789012,example
```
&
awsAccountIdB" AWS account ID.
m
identityStoreB" VUser identity directory type. Defaults to `QUICKSIGHT`, the only current valid value.
O
	namespace" >Name of the namespace.

The following arguments are optional:
�
tagsB2" �Key-value map of resource tags. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
e
timeoutsYBW:U
S

quicksightNamespaceTimeouts2aws:quicksight/NamespaceTimeouts:NamespaceTimeouts"!
arn" ARN of the Namespace.
"$
awsAccountId" AWS account ID.
",
capacityRegion" Namespace AWS Region.
"8
creationStatus" "Creation status of the namespace.
"k
identityStore" VUser identity directory type. Defaults to `QUICKSIGHT`, the only current valid value.
"O
	namespace" >Name of the namespace.

The following arguments are optional:
"�
tagsB2" �Key-value map of resource tags. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
"�
tagsAll2" xA map of tags assigned to the resource, including those inherited from the provider `default_tags` configuration block.
"e
timeoutsYBW:U
S

quicksightNamespaceTimeouts2aws:quicksight/NamespaceTimeouts:NamespaceTimeouts*�p
M

quicksightRefreshSchedule.aws:quicksight/refreshSchedule:RefreshSchedule�hResource for managing a QuickSight Refresh Schedule.

## Example Usage

### Basic Usage

<!--Start PulumiCodeChooser -->
```typescript
import * as pulumi from "@pulumi/pulumi";
import * as aws from "@pulumi/aws";

const example = new aws.quicksight.RefreshSchedule("example", {
    dataSetId: "dataset-id",
    scheduleId: "schedule-id",
    schedule: {
        refreshType: "FULL_REFRESH",
        scheduleFrequency: {
            interval: "HOURLY",
        },
    },
});
```
```python
import pulumi
import pulumi_aws as aws

example = aws.quicksight.RefreshSchedule("example",
    data_set_id="dataset-id",
    schedule_id="schedule-id",
    schedule={
        "refresh_type": "FULL_REFRESH",
        "schedule_frequency": {
            "interval": "HOURLY",
        },
    })
```
```csharp
using System.Collections.Generic;
using System.Linq;
using Pulumi;
using Aws = Pulumi.Aws;

return await Deployment.RunAsync(() => 
{
    var example = new Aws.Quicksight.RefreshSchedule("example", new()
    {
        DataSetId = "dataset-id",
        ScheduleId = "schedule-id",
        Schedule = new Aws.Quicksight.Inputs.RefreshScheduleScheduleArgs
        {
            RefreshType = "FULL_REFRESH",
            ScheduleFrequency = new Aws.Quicksight.Inputs.RefreshScheduleScheduleScheduleFrequencyArgs
            {
                Interval = "HOURLY",
            },
        },
    });

});
```
```go
package main

import (
	"github.com/pulumi/pulumi-aws/sdk/v6/go/aws/quicksight"
	"github.com/pulumi/pulumi/sdk/v3/go/pulumi"
)

func main() {
	pulumi.Run(func(ctx *pulumi.Context) error {
		_, err := quicksight.NewRefreshSchedule(ctx, "example", &quicksight.RefreshScheduleArgs{
			DataSetId:  pulumi.String("dataset-id"),
			ScheduleId: pulumi.String("schedule-id"),
			Schedule: &quicksight.RefreshScheduleScheduleArgs{
				RefreshType: pulumi.String("FULL_REFRESH"),
				ScheduleFrequency: &quicksight.RefreshScheduleScheduleScheduleFrequencyArgs{
					Interval: pulumi.String("HOURLY"),
				},
			},
		})
		if err != nil {
			return err
		}
		return nil
	})
}
```
```java
package generated_program;

import com.pulumi.Context;
import com.pulumi.Pulumi;
import com.pulumi.core.Output;
import com.pulumi.aws.quicksight.RefreshSchedule;
import com.pulumi.aws.quicksight.RefreshScheduleArgs;
import com.pulumi.aws.quicksight.inputs.RefreshScheduleScheduleArgs;
import com.pulumi.aws.quicksight.inputs.RefreshScheduleScheduleScheduleFrequencyArgs;
import java.util.List;
import java.util.ArrayList;
import java.util.Map;
import java.io.File;
import java.nio.file.Files;
import java.nio.file.Paths;

public class App {
    public static void main(String[] args) {
        Pulumi.run(App::stack);
    }

    public static void stack(Context ctx) {
        var example = new RefreshSchedule("example", RefreshScheduleArgs.builder()
            .dataSetId("dataset-id")
            .scheduleId("schedule-id")
            .schedule(RefreshScheduleScheduleArgs.builder()
                .refreshType("FULL_REFRESH")
                .scheduleFrequency(RefreshScheduleScheduleScheduleFrequencyArgs.builder()
                    .interval("HOURLY")
                    .build())
                .build())
            .build());

    }
}
```
```yaml
resources:
  example:
    type: aws:quicksight:RefreshSchedule
    properties:
      dataSetId: dataset-id
      scheduleId: schedule-id
      schedule:
        refreshType: FULL_REFRESH
        scheduleFrequency:
          interval: HOURLY
```
<!--End PulumiCodeChooser -->

### With Weekly Refresh

<!--Start PulumiCodeChooser -->
```typescript
import * as pulumi from "@pulumi/pulumi";
import * as aws from "@pulumi/aws";

const example = new aws.quicksight.RefreshSchedule("example", {
    dataSetId: "dataset-id",
    scheduleId: "schedule-id",
    schedule: {
        refreshType: "INCREMENTAL_REFRESH",
        scheduleFrequency: {
            interval: "WEEKLY",
            timeOfTheDay: "01:00",
            timezone: "Europe/London",
            refreshOnDay: {
                dayOfWeek: "MONDAY",
            },
        },
    },
});
```
```python
import pulumi
import pulumi_aws as aws

example = aws.quicksight.RefreshSchedule("example",
    data_set_id="dataset-id",
    schedule_id="schedule-id",
    schedule={
        "refresh_type": "INCREMENTAL_REFRESH",
        "schedule_frequency": {
            "interval": "WEEKLY",
            "time_of_the_day": "01:00",
            "timezone": "Europe/London",
            "refresh_on_day": {
                "day_of_week": "MONDAY",
            },
        },
    })
```
```csharp
using System.Collections.Generic;
using System.Linq;
using Pulumi;
using Aws = Pulumi.Aws;

return await Deployment.RunAsync(() => 
{
    var example = new Aws.Quicksight.RefreshSchedule("example", new()
    {
        DataSetId = "dataset-id",
        ScheduleId = "schedule-id",
        Schedule = new Aws.Quicksight.Inputs.RefreshScheduleScheduleArgs
        {
            RefreshType = "INCREMENTAL_REFRESH",
            ScheduleFrequency = new Aws.Quicksight.Inputs.RefreshScheduleScheduleScheduleFrequencyArgs
            {
                Interval = "WEEKLY",
                TimeOfTheDay = "01:00",
                Timezone = "Europe/London",
                RefreshOnDay = new Aws.Quicksight.Inputs.RefreshScheduleScheduleScheduleFrequencyRefreshOnDayArgs
                {
                    DayOfWeek = "MONDAY",
                },
            },
        },
    });

});
```
```go
package main

import (
	"github.com/pulumi/pulumi-aws/sdk/v6/go/aws/quicksight"
	"github.com/pulumi/pulumi/sdk/v3/go/pulumi"
)

func main() {
	pulumi.Run(func(ctx *pulumi.Context) error {
		_, err := quicksight.NewRefreshSchedule(ctx, "example", &quicksight.RefreshScheduleArgs{
			DataSetId:  pulumi.String("dataset-id"),
			ScheduleId: pulumi.String("schedule-id"),
			Schedule: &quicksight.RefreshScheduleScheduleArgs{
				RefreshType: pulumi.String("INCREMENTAL_REFRESH"),
				ScheduleFrequency: &quicksight.RefreshScheduleScheduleScheduleFrequencyArgs{
					Interval:     pulumi.String("WEEKLY"),
					TimeOfTheDay: pulumi.String("01:00"),
					Timezone:     pulumi.String("Europe/London"),
					RefreshOnDay: &quicksight.RefreshScheduleScheduleScheduleFrequencyRefreshOnDayArgs{
						DayOfWeek: pulumi.String("MONDAY"),
					},
				},
			},
		})
		if err != nil {
			return err
		}
		return nil
	})
}
```
```java
package generated_program;

import com.pulumi.Context;
import com.pulumi.Pulumi;
import com.pulumi.core.Output;
import com.pulumi.aws.quicksight.RefreshSchedule;
import com.pulumi.aws.quicksight.RefreshScheduleArgs;
import com.pulumi.aws.quicksight.inputs.RefreshScheduleScheduleArgs;
import com.pulumi.aws.quicksight.inputs.RefreshScheduleScheduleScheduleFrequencyArgs;
import com.pulumi.aws.quicksight.inputs.RefreshScheduleScheduleScheduleFrequencyRefreshOnDayArgs;
import java.util.List;
import java.util.ArrayList;
import java.util.Map;
import java.io.File;
import java.nio.file.Files;
import java.nio.file.Paths;

public class App {
    public static void main(String[] args) {
        Pulumi.run(App::stack);
    }

    public static void stack(Context ctx) {
        var example = new RefreshSchedule("example", RefreshScheduleArgs.builder()
            .dataSetId("dataset-id")
            .scheduleId("schedule-id")
            .schedule(RefreshScheduleScheduleArgs.builder()
                .refreshType("INCREMENTAL_REFRESH")
                .scheduleFrequency(RefreshScheduleScheduleScheduleFrequencyArgs.builder()
                    .interval("WEEKLY")
                    .timeOfTheDay("01:00")
                    .timezone("Europe/London")
                    .refreshOnDay(RefreshScheduleScheduleScheduleFrequencyRefreshOnDayArgs.builder()
                        .dayOfWeek("MONDAY")
                        .build())
                    .build())
                .build())
            .build());

    }
}
```
```yaml
resources:
  example:
    type: aws:quicksight:RefreshSchedule
    properties:
      dataSetId: dataset-id
      scheduleId: schedule-id
      schedule:
        refreshType: INCREMENTAL_REFRESH
        scheduleFrequency:
          interval: WEEKLY
          timeOfTheDay: 01:00
          timezone: Europe/London
          refreshOnDay:
            dayOfWeek: MONDAY
```
<!--End PulumiCodeChooser -->

### With Monthly Refresh

<!--Start PulumiCodeChooser -->
```typescript
import * as pulumi from "@pulumi/pulumi";
import * as aws from "@pulumi/aws";

const example = new aws.quicksight.RefreshSchedule("example", {
    dataSetId: "dataset-id",
    scheduleId: "schedule-id",
    schedule: {
        refreshType: "INCREMENTAL_REFRESH",
        scheduleFrequency: {
            interval: "MONTHLY",
            timeOfTheDay: "01:00",
            timezone: "Europe/London",
            refreshOnDay: {
                dayOfMonth: "1",
            },
        },
    },
});
```
```python
import pulumi
import pulumi_aws as aws

example = aws.quicksight.RefreshSchedule("example",
    data_set_id="dataset-id",
    schedule_id="schedule-id",
    schedule={
        "refresh_type": "INCREMENTAL_REFRESH",
        "schedule_frequency": {
            "interval": "MONTHLY",
            "time_of_the_day": "01:00",
            "timezone": "Europe/London",
            "refresh_on_day": {
                "day_of_month": "1",
            },
        },
    })
```
```csharp
using System.Collections.Generic;
using System.Linq;
using Pulumi;
using Aws = Pulumi.Aws;

return await Deployment.RunAsync(() => 
{
    var example = new Aws.Quicksight.RefreshSchedule("example", new()
    {
        DataSetId = "dataset-id",
        ScheduleId = "schedule-id",
        Schedule = new Aws.Quicksight.Inputs.RefreshScheduleScheduleArgs
        {
            RefreshType = "INCREMENTAL_REFRESH",
            ScheduleFrequency = new Aws.Quicksight.Inputs.RefreshScheduleScheduleScheduleFrequencyArgs
            {
                Interval = "MONTHLY",
                TimeOfTheDay = "01:00",
                Timezone = "Europe/London",
                RefreshOnDay = new Aws.Quicksight.Inputs.RefreshScheduleScheduleScheduleFrequencyRefreshOnDayArgs
                {
                    DayOfMonth = "1",
                },
            },
        },
    });

});
```
```go
package main

import (
	"github.com/pulumi/pulumi-aws/sdk/v6/go/aws/quicksight"
	"github.com/pulumi/pulumi/sdk/v3/go/pulumi"
)

func main() {
	pulumi.Run(func(ctx *pulumi.Context) error {
		_, err := quicksight.NewRefreshSchedule(ctx, "example", &quicksight.RefreshScheduleArgs{
			DataSetId:  pulumi.String("dataset-id"),
			ScheduleId: pulumi.String("schedule-id"),
			Schedule: &quicksight.RefreshScheduleScheduleArgs{
				RefreshType: pulumi.String("INCREMENTAL_REFRESH"),
				ScheduleFrequency: &quicksight.RefreshScheduleScheduleScheduleFrequencyArgs{
					Interval:     pulumi.String("MONTHLY"),
					TimeOfTheDay: pulumi.String("01:00"),
					Timezone:     pulumi.String("Europe/London"),
					RefreshOnDay: &quicksight.RefreshScheduleScheduleScheduleFrequencyRefreshOnDayArgs{
						DayOfMonth: pulumi.String("1"),
					},
				},
			},
		})
		if err != nil {
			return err
		}
		return nil
	})
}
```
```java
package generated_program;

import com.pulumi.Context;
import com.pulumi.Pulumi;
import com.pulumi.core.Output;
import com.pulumi.aws.quicksight.RefreshSchedule;
import com.pulumi.aws.quicksight.RefreshScheduleArgs;
import com.pulumi.aws.quicksight.inputs.RefreshScheduleScheduleArgs;
import com.pulumi.aws.quicksight.inputs.RefreshScheduleScheduleScheduleFrequencyArgs;
import com.pulumi.aws.quicksight.inputs.RefreshScheduleScheduleScheduleFrequencyRefreshOnDayArgs;
import java.util.List;
import java.util.ArrayList;
import java.util.Map;
import java.io.File;
import java.nio.file.Files;
import java.nio.file.Paths;

public class App {
    public static void main(String[] args) {
        Pulumi.run(App::stack);
    }

    public static void stack(Context ctx) {
        var example = new RefreshSchedule("example", RefreshScheduleArgs.builder()
            .dataSetId("dataset-id")
            .scheduleId("schedule-id")
            .schedule(RefreshScheduleScheduleArgs.builder()
                .refreshType("INCREMENTAL_REFRESH")
                .scheduleFrequency(RefreshScheduleScheduleScheduleFrequencyArgs.builder()
                    .interval("MONTHLY")
                    .timeOfTheDay("01:00")
                    .timezone("Europe/London")
                    .refreshOnDay(RefreshScheduleScheduleScheduleFrequencyRefreshOnDayArgs.builder()
                        .dayOfMonth("1")
                        .build())
                    .build())
                .build())
            .build());

    }
}
```
```yaml
resources:
  example:
    type: aws:quicksight:RefreshSchedule
    properties:
      dataSetId: dataset-id
      scheduleId: schedule-id
      schedule:
        refreshType: INCREMENTAL_REFRESH
        scheduleFrequency:
          interval: MONTHLY
          timeOfTheDay: 01:00
          timezone: Europe/London
          refreshOnDay:
            dayOfMonth: '1'
```
<!--End PulumiCodeChooser -->

## Import

Using `pulumi import`, import a QuickSight Refresh Schedule using the AWS account ID, data set ID and schedule ID separated by commas (`,`). For example:

```sh
$ pulumi import aws:quicksight/refreshSchedule:RefreshSchedule example 123456789012,dataset-id,schedule-id
```
&
awsAccountIdB" AWS account ID.
(
	dataSetId" The ID of the dataset.
�
schedulekBi:g
e

quicksightRefreshScheduleSchedule>aws:quicksight/RefreshScheduleSchedule:RefreshScheduleSchedule�The [refresh schedule](https://docs.aws.amazon.com/quicksight/latest/APIReference/API_RefreshSchedule.html). See schedule

The following arguments are optional:
2

scheduleId"  The ID of the refresh schedule.
"?
arn" 4Amazon Resource Name (ARN) of the refresh schedule.
"$
awsAccountId" AWS account ID.
"(
	dataSetId" The ID of the dataset.
"�
schedulekBi:g
e

quicksightRefreshScheduleSchedule>aws:quicksight/RefreshScheduleSchedule:RefreshScheduleSchedule�The [refresh schedule](https://docs.aws.amazon.com/quicksight/latest/APIReference/API_RefreshSchedule.html). See schedule

The following arguments are optional:
"2

scheduleId"  The ID of the refresh schedule.
*�;
8

quicksightTemplate aws:quicksight/template:Template�'Resource for managing a QuickSight Template.

## Example Usage

### From Source Template

<!--Start PulumiCodeChooser -->
```typescript
import * as pulumi from "@pulumi/pulumi";
import * as aws from "@pulumi/aws";

const example = new aws.quicksight.Template("example", {
    templateId: "example-id",
    name: "example-name",
    versionDescription: "version",
    sourceEntity: {
        sourceTemplate: {
            arn: source.arn,
        },
    },
});
```
```python
import pulumi
import pulumi_aws as aws

example = aws.quicksight.Template("example",
    template_id="example-id",
    name="example-name",
    version_description="version",
    source_entity={
        "source_template": {
            "arn": source["arn"],
        },
    })
```
```csharp
using System.Collections.Generic;
using System.Linq;
using Pulumi;
using Aws = Pulumi.Aws;

return await Deployment.RunAsync(() => 
{
    var example = new Aws.Quicksight.Template("example", new()
    {
        TemplateId = "example-id",
        Name = "example-name",
        VersionDescription = "version",
        SourceEntity = new Aws.Quicksight.Inputs.TemplateSourceEntityArgs
        {
            SourceTemplate = new Aws.Quicksight.Inputs.TemplateSourceEntitySourceTemplateArgs
            {
                Arn = source.Arn,
            },
        },
    });

});
```
```go
package main

import (
	"github.com/pulumi/pulumi-aws/sdk/v6/go/aws/quicksight"
	"github.com/pulumi/pulumi/sdk/v3/go/pulumi"
)

func main() {
	pulumi.Run(func(ctx *pulumi.Context) error {
		_, err := quicksight.NewTemplate(ctx, "example", &quicksight.TemplateArgs{
			TemplateId:         pulumi.String("example-id"),
			Name:               pulumi.String("example-name"),
			VersionDescription: pulumi.String("version"),
			SourceEntity: &quicksight.TemplateSourceEntityArgs{
				SourceTemplate: &quicksight.TemplateSourceEntitySourceTemplateArgs{
					Arn: pulumi.Any(source.Arn),
				},
			},
		})
		if err != nil {
			return err
		}
		return nil
	})
}
```
```java
package generated_program;

import com.pulumi.Context;
import com.pulumi.Pulumi;
import com.pulumi.core.Output;
import com.pulumi.aws.quicksight.Template;
import com.pulumi.aws.quicksight.TemplateArgs;
import com.pulumi.aws.quicksight.inputs.TemplateSourceEntityArgs;
import com.pulumi.aws.quicksight.inputs.TemplateSourceEntitySourceTemplateArgs;
import java.util.List;
import java.util.ArrayList;
import java.util.Map;
import java.io.File;
import java.nio.file.Files;
import java.nio.file.Paths;

public class App {
    public static void main(String[] args) {
        Pulumi.run(App::stack);
    }

    public static void stack(Context ctx) {
        var example = new Template("example", TemplateArgs.builder()
            .templateId("example-id")
            .name("example-name")
            .versionDescription("version")
            .sourceEntity(TemplateSourceEntityArgs.builder()
                .sourceTemplate(TemplateSourceEntitySourceTemplateArgs.builder()
                    .arn(source.arn())
                    .build())
                .build())
            .build());

    }
}
```
```yaml
resources:
  example:
    type: aws:quicksight:Template
    properties:
      templateId: example-id
      name: example-name
      versionDescription: version
      sourceEntity:
        sourceTemplate:
          arn: ${source.arn}
```
<!--End PulumiCodeChooser -->

### With Definition

<!--Start PulumiCodeChooser -->
```yaml
resources:
  example:
    type: aws:quicksight:Template
    properties:
      templateId: example-id
      name: example-name
      versionDescription: version
      definition:
        dataSetConfigurations:
          - dataSetSchema:
              columnSchemaLists:
                - name: Column1
                  dataType: STRING
                - name: Column2
                  dataType: INTEGER
            placeholder: '1'
        sheets:
          - title: Test
            sheetId: Test1
            visuals:
              - barChartVisual:
                  visualId: BarChart
                  chartConfiguration:
                    fieldWells:
                      barChartAggregatedFieldWells:
                        categories:
                          - categoricalDimensionField:
                              fieldId: '1'
                              column:
                                columnName: Column1
                                dataSetIdentifier: '1'
                        values:
                          - numericalMeasureField:
                              fieldId: '2'
                              column:
                                columnName: Column2
                                dataSetIdentifier: '1'
                              aggregationFunction:
                                simpleNumericalAggregation: SUM
```
<!--End PulumiCodeChooser -->

## Import

Using `pulumi import`, import a QuickSight Template using the AWS account ID and template ID separated by a comma (`,`). For example:

```sh
$ pulumi import aws:quicksight/template:Template example 123456789012,example-id
```
&
awsAccountIdB" AWS account ID.
-
nameB" Display name for the template.
�
permissions^B\*Z:X
V

quicksightTemplatePermission4aws:quicksight/TemplatePermission:TemplatePermissionUA set of resource permissions on the template. Maximum of 64 items. See permissions.
�
sourceEntitybB`:^
\

quicksightTemplateSourceEntity8aws:quicksight/TemplateSourceEntity:TemplateSourceEntity�The entity that you are using as a source when you create the template (analysis or template). Only one of `definition` or `source_entity` should be configured. See source_entity.
�
tagsB2" �Key-value map of resource tags. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
/

templateId" Identifier for the template.
�
versionDescription" lA description of the current template version being created/updated.

The following arguments are optional:
" 
arn" ARN of the template.
"$
awsAccountId" AWS account ID.
";
createdTime" (The time that the template was created.
"D
lastUpdatedTime" -The time that the template was last updated.
"+
name" Display name for the template.
"�
permissions^B\*Z:X
V

quicksightTemplatePermission4aws:quicksight/TemplatePermission:TemplatePermissionUA set of resource permissions on the template. Maximum of 64 items. See permissions.
"�
sourceEntitybB`:^
\

quicksightTemplateSourceEntity8aws:quicksight/TemplateSourceEntity:TemplateSourceEntity�The entity that you are using as a source when you create the template (analysis or template). Only one of `definition` or `source_entity` should be configured. See source_entity.
"t
sourceEntityArn" ]Amazon Resource Name (ARN) of an analysis or template that was used to create this template.
",
status" The template creation status.
"�
tagsB2" �Key-value map of resource tags. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
"�
tagsAll2" xA map of tags assigned to the resource, including those inherited from the provider `default_tags` configuration block.
"/

templateId" Identifier for the template.
"�
versionDescription" lA description of the current template version being created/updated.

The following arguments are optional:
"A
versionNumber ,The version number of the template version.
*�
G

quicksightTemplateAlias*aws:quicksight/templateAlias:TemplateAlias�Resource for managing an AWS QuickSight Template Alias.

## Example Usage

### Basic Usage

<!--Start PulumiCodeChooser -->
```typescript
import * as pulumi from "@pulumi/pulumi";
import * as aws from "@pulumi/aws";

const example = new aws.quicksight.TemplateAlias("example", {
    aliasName: "example-alias",
    templateId: test.templateId,
    templateVersionNumber: test.versionNumber,
});
```
```python
import pulumi
import pulumi_aws as aws

example = aws.quicksight.TemplateAlias("example",
    alias_name="example-alias",
    template_id=test["templateId"],
    template_version_number=test["versionNumber"])
```
```csharp
using System.Collections.Generic;
using System.Linq;
using Pulumi;
using Aws = Pulumi.Aws;

return await Deployment.RunAsync(() => 
{
    var example = new Aws.Quicksight.TemplateAlias("example", new()
    {
        AliasName = "example-alias",
        TemplateId = test.TemplateId,
        TemplateVersionNumber = test.VersionNumber,
    });

});
```
```go
package main

import (
	"github.com/pulumi/pulumi-aws/sdk/v6/go/aws/quicksight"
	"github.com/pulumi/pulumi/sdk/v3/go/pulumi"
)

func main() {
	pulumi.Run(func(ctx *pulumi.Context) error {
		_, err := quicksight.NewTemplateAlias(ctx, "example", &quicksight.TemplateAliasArgs{
			AliasName:             pulumi.String("example-alias"),
			TemplateId:            pulumi.Any(test.TemplateId),
			TemplateVersionNumber: pulumi.Any(test.VersionNumber),
		})
		if err != nil {
			return err
		}
		return nil
	})
}
```
```java
package generated_program;

import com.pulumi.Context;
import com.pulumi.Pulumi;
import com.pulumi.core.Output;
import com.pulumi.aws.quicksight.TemplateAlias;
import com.pulumi.aws.quicksight.TemplateAliasArgs;
import java.util.List;
import java.util.ArrayList;
import java.util.Map;
import java.io.File;
import java.nio.file.Files;
import java.nio.file.Paths;

public class App {
    public static void main(String[] args) {
        Pulumi.run(App::stack);
    }

    public static void stack(Context ctx) {
        var example = new TemplateAlias("example", TemplateAliasArgs.builder()
            .aliasName("example-alias")
            .templateId(test.templateId())
            .templateVersionNumber(test.versionNumber())
            .build());

    }
}
```
```yaml
resources:
  example:
    type: aws:quicksight:TemplateAlias
    properties:
      aliasName: example-alias
      templateId: ${test.templateId}
      templateVersionNumber: ${test.versionNumber}
```
<!--End PulumiCodeChooser -->

## Import

Using `pulumi import`, import QuickSight Template Alias using the AWS account ID, template ID, and alias name separated by a comma (`,`). For example:

```sh
$ pulumi import aws:quicksight/templateAlias:TemplateAlias example 123456789012,example-id,example-alias
```
5
	aliasName" $Display name of the template alias.
&
awsAccountIdB" AWS account ID.
&

templateId" ID of the template.
d
templateVersionNumber GVersion number of the template.

The following arguments are optional:
"5
	aliasName" $Display name of the template alias.
"=
arn" 2Amazon Resource Name (ARN) of the template alias.
"$
awsAccountId" AWS account ID.
"&

templateId" ID of the template.
"d
templateVersionNumber GVersion number of the template.

The following arguments are optional:
*�E
/

quicksightThemeaws:quicksight/theme:Theme�0Resource for managing a QuickSight Theme.

## Example Usage

### Basic Usage

<!--Start PulumiCodeChooser -->
```typescript
import * as pulumi from "@pulumi/pulumi";
import * as aws from "@pulumi/aws";

const example = new aws.quicksight.Theme("example", {
    themeId: "example",
    name: "example",
    baseThemeId: "MIDNIGHT",
    configuration: {
        dataColorPalette: {
            colors: [
                "#FFFFFF",
                "#111111",
                "#222222",
                "#333333",
                "#444444",
                "#555555",
                "#666666",
                "#777777",
                "#888888",
                "#999999",
            ],
            emptyFillColor: "#FFFFFF",
            minMaxGradients: [
                "#FFFFFF",
                "#111111",
            ],
        },
    },
});
```
```python
import pulumi
import pulumi_aws as aws

example = aws.quicksight.Theme("example",
    theme_id="example",
    name="example",
    base_theme_id="MIDNIGHT",
    configuration={
        "data_color_palette": {
            "colors": [
                "#FFFFFF",
                "#111111",
                "#222222",
                "#333333",
                "#444444",
                "#555555",
                "#666666",
                "#777777",
                "#888888",
                "#999999",
            ],
            "empty_fill_color": "#FFFFFF",
            "min_max_gradients": [
                "#FFFFFF",
                "#111111",
            ],
        },
    })
```
```csharp
using System.Collections.Generic;
using System.Linq;
using Pulumi;
using Aws = Pulumi.Aws;

return await Deployment.RunAsync(() => 
{
    var example = new Aws.Quicksight.Theme("example", new()
    {
        ThemeId = "example",
        Name = "example",
        BaseThemeId = "MIDNIGHT",
        Configuration = new Aws.Quicksight.Inputs.ThemeConfigurationArgs
        {
            DataColorPalette = new Aws.Quicksight.Inputs.ThemeConfigurationDataColorPaletteArgs
            {
                Colors = new[]
                {
                    "#FFFFFF",
                    "#111111",
                    "#222222",
                    "#333333",
                    "#444444",
                    "#555555",
                    "#666666",
                    "#777777",
                    "#888888",
                    "#999999",
                },
                EmptyFillColor = "#FFFFFF",
                MinMaxGradients = new[]
                {
                    "#FFFFFF",
                    "#111111",
                },
            },
        },
    });

});
```
```go
package main

import (
	"github.com/pulumi/pulumi-aws/sdk/v6/go/aws/quicksight"
	"github.com/pulumi/pulumi/sdk/v3/go/pulumi"
)

func main() {
	pulumi.Run(func(ctx *pulumi.Context) error {
		_, err := quicksight.NewTheme(ctx, "example", &quicksight.ThemeArgs{
			ThemeId:     pulumi.String("example"),
			Name:        pulumi.String("example"),
			BaseThemeId: pulumi.String("MIDNIGHT"),
			Configuration: &quicksight.ThemeConfigurationArgs{
				DataColorPalette: &quicksight.ThemeConfigurationDataColorPaletteArgs{
					Colors: pulumi.StringArray{
						pulumi.String("#FFFFFF"),
						pulumi.String("#111111"),
						pulumi.String("#222222"),
						pulumi.String("#333333"),
						pulumi.String("#444444"),
						pulumi.String("#555555"),
						pulumi.String("#666666"),
						pulumi.String("#777777"),
						pulumi.String("#888888"),
						pulumi.String("#999999"),
					},
					EmptyFillColor: pulumi.String("#FFFFFF"),
					MinMaxGradients: pulumi.StringArray{
						pulumi.String("#FFFFFF"),
						pulumi.String("#111111"),
					},
				},
			},
		})
		if err != nil {
			return err
		}
		return nil
	})
}
```
```java
package generated_program;

import com.pulumi.Context;
import com.pulumi.Pulumi;
import com.pulumi.core.Output;
import com.pulumi.aws.quicksight.Theme;
import com.pulumi.aws.quicksight.ThemeArgs;
import com.pulumi.aws.quicksight.inputs.ThemeConfigurationArgs;
import com.pulumi.aws.quicksight.inputs.ThemeConfigurationDataColorPaletteArgs;
import java.util.List;
import java.util.ArrayList;
import java.util.Map;
import java.io.File;
import java.nio.file.Files;
import java.nio.file.Paths;

public class App {
    public static void main(String[] args) {
        Pulumi.run(App::stack);
    }

    public static void stack(Context ctx) {
        var example = new Theme("example", ThemeArgs.builder()
            .themeId("example")
            .name("example")
            .baseThemeId("MIDNIGHT")
            .configuration(ThemeConfigurationArgs.builder()
                .dataColorPalette(ThemeConfigurationDataColorPaletteArgs.builder()
                    .colors(                    
                        "#FFFFFF",
                        "#111111",
                        "#222222",
                        "#333333",
                        "#444444",
                        "#555555",
                        "#666666",
                        "#777777",
                        "#888888",
                        "#999999")
                    .emptyFillColor("#FFFFFF")
                    .minMaxGradients(                    
                        "#FFFFFF",
                        "#111111")
                    .build())
                .build())
            .build());

    }
}
```
```yaml
resources:
  example:
    type: aws:quicksight:Theme
    properties:
      themeId: example
      name: example
      baseThemeId: MIDNIGHT
      configuration:
        dataColorPalette:
          colors:
            - '#FFFFFF'
            - '#111111'
            - '#222222'
            - '#333333'
            - '#444444'
            - '#555555'
            - '#666666'
            - '#777777'
            - '#888888'
            - '#999999'
          emptyFillColor: '#FFFFFF'
          minMaxGradients:
            - '#FFFFFF'
            - '#111111'
```
<!--End PulumiCodeChooser -->

## Import

Using `pulumi import`, import a QuickSight Theme using the AWS account ID and theme ID separated by a comma (`,`). For example:

```sh
$ pulumi import aws:quicksight/theme:Theme example 123456789012,example-id
```
&
awsAccountIdB" AWS account ID.
�
baseThemeId" �The ID of the theme that a custom theme will inherit from. All themes inherit from one of the starting themes defined by Amazon QuickSight. For a list of the starting themes, use ListThemes or choose Themes from within an analysis.
�
configuration\BZ:X
V

quicksightThemeConfiguration4aws:quicksight/ThemeConfiguration:ThemeConfiguration�The theme configuration, which contains the theme display properties. See configuration.

The following arguments are optional:
)
nameB" Display name of the theme.
�
permissionsUBS*Q:O
M

quicksightThemePermission.aws:quicksight/ThemePermission:ThemePermissionRA set of resource permissions on the theme. Maximum of 64 items. See permissions.
�
tagsB2" �Key-value map of resource tags. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
(
themeId" Identifier of the theme.
^
versionDescriptionB" BA description of the current theme version being created/updated.
"
arn" ARN of the theme.
"$
awsAccountId" AWS account ID.
"�
baseThemeId" �The ID of the theme that a custom theme will inherit from. All themes inherit from one of the starting themes defined by Amazon QuickSight. For a list of the starting themes, use ListThemes or choose Themes from within an analysis.
"�
configuration\BZ:X
V

quicksightThemeConfiguration4aws:quicksight/ThemeConfiguration:ThemeConfiguration�The theme configuration, which contains the theme display properties. See configuration.

The following arguments are optional:
"8
createdTime" %The time that the theme was created.
"A
lastUpdatedTime" *The time that the theme was last updated.
"'
name" Display name of the theme.
"�
permissionsUBS*Q:O
M

quicksightThemePermission.aws:quicksight/ThemePermission:ThemePermissionRA set of resource permissions on the theme. Maximum of 64 items. See permissions.
")
status" The theme creation status.
"�
tagsB2" �Key-value map of resource tags. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
"�
tagsAll2" xA map of tags assigned to the resource, including those inherited from the provider `default_tags` configuration block.
"(
themeId" Identifier of the theme.
"^
versionDescriptionB" BA description of the current theme version being created/updated.
">
versionNumber )The version number of the theme version.
*�,
,

quicksightUseraws:quicksight/user:User�Resource for managing QuickSight User

## Example Usage

<!--Start PulumiCodeChooser -->
```typescript
import * as pulumi from "@pulumi/pulumi";
import * as aws from "@pulumi/aws";

const example = new aws.quicksight.User("example", {
    sessionName: "an-author",
    email: "author@example.com",
    namespace: "foo",
    identityType: "IAM",
    iamArn: "arn:aws:iam::123456789012:user/Example",
    userRole: "AUTHOR",
});
```
```python
import pulumi
import pulumi_aws as aws

example = aws.quicksight.User("example",
    session_name="an-author",
    email="author@example.com",
    namespace="foo",
    identity_type="IAM",
    iam_arn="arn:aws:iam::123456789012:user/Example",
    user_role="AUTHOR")
```
```csharp
using System.Collections.Generic;
using System.Linq;
using Pulumi;
using Aws = Pulumi.Aws;

return await Deployment.RunAsync(() => 
{
    var example = new Aws.Quicksight.User("example", new()
    {
        SessionName = "an-author",
        Email = "author@example.com",
        Namespace = "foo",
        IdentityType = "IAM",
        IamArn = "arn:aws:iam::123456789012:user/Example",
        UserRole = "AUTHOR",
    });

});
```
```go
package main

import (
	"github.com/pulumi/pulumi-aws/sdk/v6/go/aws/quicksight"
	"github.com/pulumi/pulumi/sdk/v3/go/pulumi"
)

func main() {
	pulumi.Run(func(ctx *pulumi.Context) error {
		_, err := quicksight.NewUser(ctx, "example", &quicksight.UserArgs{
			SessionName:  pulumi.String("an-author"),
			Email:        pulumi.String("author@example.com"),
			Namespace:    pulumi.String("foo"),
			IdentityType: pulumi.String("IAM"),
			IamArn:       pulumi.String("arn:aws:iam::123456789012:user/Example"),
			UserRole:     pulumi.String("AUTHOR"),
		})
		if err != nil {
			return err
		}
		return nil
	})
}
```
```java
package generated_program;

import com.pulumi.Context;
import com.pulumi.Pulumi;
import com.pulumi.core.Output;
import com.pulumi.aws.quicksight.User;
import com.pulumi.aws.quicksight.UserArgs;
import java.util.List;
import java.util.ArrayList;
import java.util.Map;
import java.io.File;
import java.nio.file.Files;
import java.nio.file.Paths;

public class App {
    public static void main(String[] args) {
        Pulumi.run(App::stack);
    }

    public static void stack(Context ctx) {
        var example = new User("example", UserArgs.builder()
            .sessionName("an-author")
            .email("author@example.com")
            .namespace("foo")
            .identityType("IAM")
            .iamArn("arn:aws:iam::123456789012:user/Example")
            .userRole("AUTHOR")
            .build());

    }
}
```
```yaml
resources:
  example:
    type: aws:quicksight:User
    properties:
      sessionName: an-author
      email: author@example.com
      namespace: foo
      identityType: IAM
      iamArn: arn:aws:iam::123456789012:user/Example
      userRole: AUTHOR
```
<!--End PulumiCodeChooser -->

## Import

You cannot import this resource.

�
awsAccountIdB" �The ID for the AWS account that the user is in. Currently, you use the ID for the AWS account that contains your Amazon QuickSight account.
F
email" 9The email address of the user that you want to register.
a
iamArnB" QThe ARN of the IAM user or role that you are registering with Amazon QuickSight.
�
identityType" �Amazon QuickSight supports several ways of managing the identity of users. This parameter accepts either  `IAM` or `QUICKSIGHT`. If `IAM` is specified, the `iam_arn` must also be specified.
a
	namespaceB" NThe Amazon Quicksight namespace to create the user in. Defaults to `default`.
�
sessionNameB" �The name of the IAM session to use when assuming roles that can embed QuickSight dashboards. Only valid for registering users using an assumed IAM role. Additionally, if registering multiple users using the same IAM role, each user needs to have a unique session name.
�
userNameB" �The Amazon QuickSight user name that you want to create for the user you are registering. Only valid for registering a user with `identity_type` set to `QUICKSIGHT`.
�
userRole" �The Amazon QuickSight role of the user. The user role can be one of the following: `READER`, `AUTHOR`, `ADMIN`, `READER_PRO`, `AUTHOR_PRO` or `ADMIN_PRO`.
"2
arn" 'Amazon Resource Name (ARN) of the user
"�
awsAccountId" �The ID for the AWS account that the user is in. Currently, you use the ID for the AWS account that contains your Amazon QuickSight account.
"F
email" 9The email address of the user that you want to register.
"a
iamArnB" QThe ARN of the IAM user or role that you are registering with Amazon QuickSight.
"�
identityType" �Amazon QuickSight supports several ways of managing the identity of users. This parameter accepts either  `IAM` or `QUICKSIGHT`. If `IAM` is specified, the `iam_arn` must also be specified.
"a
	namespaceB" NThe Amazon Quicksight namespace to create the user in. Defaults to `default`.
"�
sessionNameB" �The name of the IAM session to use when assuming roles that can embed QuickSight dashboards. Only valid for registering users using an assumed IAM role. Additionally, if registering multiple users using the same IAM role, each user needs to have a unique session name.
"�
userNameB" �The Amazon QuickSight user name that you want to create for the user you are registering. Only valid for registering a user with `identity_type` set to `QUICKSIGHT`.
"�
userRole" �The Amazon QuickSight role of the user. The user role can be one of the following: `READER`, `AUTHOR`, `ADMIN`, `READER_PRO`, `AUTHOR_PRO` or `ADMIN_PRO`.
*�i
G

quicksightVpcConnection*aws:quicksight/vpcConnection:VpcConnection�YResource for managing an AWS QuickSight VPC Connection.

## Example Usage

### Basic Usage

<!--Start PulumiCodeChooser -->
```typescript
import * as pulumi from "@pulumi/pulumi";
import * as aws from "@pulumi/aws";

const vpcConnectionRole = new aws.iam.Role("vpc_connection_role", {
    assumeRolePolicy: JSON.stringify({
        Version: "2012-10-17",
        Statement: [{
            Effect: "Allow",
            Action: "sts:AssumeRole",
            Principal: {
                Service: "quicksight.amazonaws.com",
            },
        }],
    }),
    inlinePolicies: [{
        name: "QuickSightVPCConnectionRolePolicy",
        policy: JSON.stringify({
            Version: "2012-10-17",
            Statement: [{
                Effect: "Allow",
                Action: [
                    "ec2:CreateNetworkInterface",
                    "ec2:ModifyNetworkInterfaceAttribute",
                    "ec2:DeleteNetworkInterface",
                    "ec2:DescribeSubnets",
                    "ec2:DescribeSecurityGroups",
                ],
                Resource: ["*"],
            }],
        }),
    }],
});
const example = new aws.quicksight.VpcConnection("example", {
    vpcConnectionId: "example-connection-id",
    name: "Example Connection",
    roleArn: vpcConnectionRole.arn,
    securityGroupIds: ["sg-00000000000000000"],
    subnetIds: [
        "subnet-00000000000000000",
        "subnet-00000000000000001",
    ],
});
```
```python
import pulumi
import json
import pulumi_aws as aws

vpc_connection_role = aws.iam.Role("vpc_connection_role",
    assume_role_policy=json.dumps({
        "Version": "2012-10-17",
        "Statement": [{
            "Effect": "Allow",
            "Action": "sts:AssumeRole",
            "Principal": {
                "Service": "quicksight.amazonaws.com",
            },
        }],
    }),
    inline_policies=[{
        "name": "QuickSightVPCConnectionRolePolicy",
        "policy": json.dumps({
            "Version": "2012-10-17",
            "Statement": [{
                "Effect": "Allow",
                "Action": [
                    "ec2:CreateNetworkInterface",
                    "ec2:ModifyNetworkInterfaceAttribute",
                    "ec2:DeleteNetworkInterface",
                    "ec2:DescribeSubnets",
                    "ec2:DescribeSecurityGroups",
                ],
                "Resource": ["*"],
            }],
        }),
    }])
example = aws.quicksight.VpcConnection("example",
    vpc_connection_id="example-connection-id",
    name="Example Connection",
    role_arn=vpc_connection_role.arn,
    security_group_ids=["sg-00000000000000000"],
    subnet_ids=[
        "subnet-00000000000000000",
        "subnet-00000000000000001",
    ])
```
```csharp
using System.Collections.Generic;
using System.Linq;
using System.Text.Json;
using Pulumi;
using Aws = Pulumi.Aws;

return await Deployment.RunAsync(() => 
{
    var vpcConnectionRole = new Aws.Iam.Role("vpc_connection_role", new()
    {
        AssumeRolePolicy = JsonSerializer.Serialize(new Dictionary<string, object?>
        {
            ["Version"] = "2012-10-17",
            ["Statement"] = new[]
            {
                new Dictionary<string, object?>
                {
                    ["Effect"] = "Allow",
                    ["Action"] = "sts:AssumeRole",
                    ["Principal"] = new Dictionary<string, object?>
                    {
                        ["Service"] = "quicksight.amazonaws.com",
                    },
                },
            },
        }),
        InlinePolicies = new[]
        {
            new Aws.Iam.Inputs.RoleInlinePolicyArgs
            {
                Name = "QuickSightVPCConnectionRolePolicy",
                Policy = JsonSerializer.Serialize(new Dictionary<string, object?>
                {
                    ["Version"] = "2012-10-17",
                    ["Statement"] = new[]
                    {
                        new Dictionary<string, object?>
                        {
                            ["Effect"] = "Allow",
                            ["Action"] = new[]
                            {
                                "ec2:CreateNetworkInterface",
                                "ec2:ModifyNetworkInterfaceAttribute",
                                "ec2:DeleteNetworkInterface",
                                "ec2:DescribeSubnets",
                                "ec2:DescribeSecurityGroups",
                            },
                            ["Resource"] = new[]
                            {
                                "*",
                            },
                        },
                    },
                }),
            },
        },
    });

    var example = new Aws.Quicksight.VpcConnection("example", new()
    {
        VpcConnectionId = "example-connection-id",
        Name = "Example Connection",
        RoleArn = vpcConnectionRole.Arn,
        SecurityGroupIds = new[]
        {
            "sg-00000000000000000",
        },
        SubnetIds = new[]
        {
            "subnet-00000000000000000",
            "subnet-00000000000000001",
        },
    });

});
```
```go
package main

import (
	"encoding/json"

	"github.com/pulumi/pulumi-aws/sdk/v6/go/aws/iam"
	"github.com/pulumi/pulumi-aws/sdk/v6/go/aws/quicksight"
	"github.com/pulumi/pulumi/sdk/v3/go/pulumi"
)

func main() {
	pulumi.Run(func(ctx *pulumi.Context) error {
		tmpJSON0, err := json.Marshal(map[string]interface{}{
			"Version": "2012-10-17",
			"Statement": []map[string]interface{}{
				map[string]interface{}{
					"Effect": "Allow",
					"Action": "sts:AssumeRole",
					"Principal": map[string]interface{}{
						"Service": "quicksight.amazonaws.com",
					},
				},
			},
		})
		if err != nil {
			return err
		}
		json0 := string(tmpJSON0)
		tmpJSON1, err := json.Marshal(map[string]interface{}{
			"Version": "2012-10-17",
			"Statement": []map[string]interface{}{
				map[string]interface{}{
					"Effect": "Allow",
					"Action": []string{
						"ec2:CreateNetworkInterface",
						"ec2:ModifyNetworkInterfaceAttribute",
						"ec2:DeleteNetworkInterface",
						"ec2:DescribeSubnets",
						"ec2:DescribeSecurityGroups",
					},
					"Resource": []string{
						"*",
					},
				},
			},
		})
		if err != nil {
			return err
		}
		json1 := string(tmpJSON1)
		vpcConnectionRole, err := iam.NewRole(ctx, "vpc_connection_role", &iam.RoleArgs{
			AssumeRolePolicy: pulumi.String(json0),
			InlinePolicies: iam.RoleInlinePolicyArray{
				&iam.RoleInlinePolicyArgs{
					Name:   pulumi.String("QuickSightVPCConnectionRolePolicy"),
					Policy: pulumi.String(json1),
				},
			},
		})
		if err != nil {
			return err
		}
		_, err = quicksight.NewVpcConnection(ctx, "example", &quicksight.VpcConnectionArgs{
			VpcConnectionId: pulumi.String("example-connection-id"),
			Name:            pulumi.String("Example Connection"),
			RoleArn:         vpcConnectionRole.Arn,
			SecurityGroupIds: pulumi.StringArray{
				pulumi.String("sg-00000000000000000"),
			},
			SubnetIds: pulumi.StringArray{
				pulumi.String("subnet-00000000000000000"),
				pulumi.String("subnet-00000000000000001"),
			},
		})
		if err != nil {
			return err
		}
		return nil
	})
}
```
```java
package generated_program;

import com.pulumi.Context;
import com.pulumi.Pulumi;
import com.pulumi.core.Output;
import com.pulumi.aws.iam.Role;
import com.pulumi.aws.iam.RoleArgs;
import com.pulumi.aws.iam.inputs.RoleInlinePolicyArgs;
import com.pulumi.aws.quicksight.VpcConnection;
import com.pulumi.aws.quicksight.VpcConnectionArgs;
import static com.pulumi.codegen.internal.Serialization.*;
import java.util.List;
import java.util.ArrayList;
import java.util.Map;
import java.io.File;
import java.nio.file.Files;
import java.nio.file.Paths;

public class App {
    public static void main(String[] args) {
        Pulumi.run(App::stack);
    }

    public static void stack(Context ctx) {
        var vpcConnectionRole = new Role("vpcConnectionRole", RoleArgs.builder()
            .assumeRolePolicy(serializeJson(
                jsonObject(
                    jsonProperty("Version", "2012-10-17"),
                    jsonProperty("Statement", jsonArray(jsonObject(
                        jsonProperty("Effect", "Allow"),
                        jsonProperty("Action", "sts:AssumeRole"),
                        jsonProperty("Principal", jsonObject(
                            jsonProperty("Service", "quicksight.amazonaws.com")
                        ))
                    )))
                )))
            .inlinePolicies(RoleInlinePolicyArgs.builder()
                .name("QuickSightVPCConnectionRolePolicy")
                .policy(serializeJson(
                    jsonObject(
                        jsonProperty("Version", "2012-10-17"),
                        jsonProperty("Statement", jsonArray(jsonObject(
                            jsonProperty("Effect", "Allow"),
                            jsonProperty("Action", jsonArray(
                                "ec2:CreateNetworkInterface", 
                                "ec2:ModifyNetworkInterfaceAttribute", 
                                "ec2:DeleteNetworkInterface", 
                                "ec2:DescribeSubnets", 
                                "ec2:DescribeSecurityGroups"
                            )),
                            jsonProperty("Resource", jsonArray("*"))
                        )))
                    )))
                .build())
            .build());

        var example = new VpcConnection("example", VpcConnectionArgs.builder()
            .vpcConnectionId("example-connection-id")
            .name("Example Connection")
            .roleArn(vpcConnectionRole.arn())
            .securityGroupIds("sg-00000000000000000")
            .subnetIds(            
                "subnet-00000000000000000",
                "subnet-00000000000000001")
            .build());

    }
}
```
```yaml
resources:
  vpcConnectionRole:
    type: aws:iam:Role
    name: vpc_connection_role
    properties:
      assumeRolePolicy:
        fn::toJSON:
          Version: 2012-10-17
          Statement:
            - Effect: Allow
              Action: sts:AssumeRole
              Principal:
                Service: quicksight.amazonaws.com
      inlinePolicies:
        - name: QuickSightVPCConnectionRolePolicy
          policy:
            fn::toJSON:
              Version: 2012-10-17
              Statement:
                - Effect: Allow
                  Action:
                    - ec2:CreateNetworkInterface
                    - ec2:ModifyNetworkInterfaceAttribute
                    - ec2:DeleteNetworkInterface
                    - ec2:DescribeSubnets
                    - ec2:DescribeSecurityGroups
                  Resource:
                    - '*'
  example:
    type: aws:quicksight:VpcConnection
    properties:
      vpcConnectionId: example-connection-id
      name: Example Connection
      roleArn: ${vpcConnectionRole.arn}
      securityGroupIds:
        - sg-00000000000000000
      subnetIds:
        - subnet-00000000000000000
        - subnet-00000000000000001
```
<!--End PulumiCodeChooser -->

## Import

Using `pulumi import`, import QuickSight VPC connection using the AWS account ID and VPC connection ID separated by commas (`,`). For example:

```sh
$ pulumi import aws:quicksight/vpcConnection:VpcConnection example 123456789012,example
```
&
awsAccountIdB" AWS account ID.
a
dnsResolversB*" IA list of IP addresses of DNS resolver endpoints for the VPC connection.
7
nameB" )The display name for the VPC connection.
B
roleArn" 3The IAM role to associate with the VPC connection.
O
securityGroupIds*" 5A list of security group IDs for the VPC connection.
g
	subnetIds*" TA list of subnet IDs for the VPC connection.

The following arguments are optional:
�
tagsB2" �Key-value map of resource tags. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
q
timeoutseBc:a
_

quicksightVpcConnectionTimeouts:aws:quicksight/VpcConnectionTimeouts:VpcConnectionTimeouts5
vpcConnectionId" The ID of the VPC connection.
"&
arn" ARN of the VPC connection.
"�
availabilityStatus" uThe availability status of the VPC connection. Valid values are `AVAILABLE`, `UNAVAILABLE` or `PARTIALLY_AVAILABLE`.
"$
awsAccountId" AWS account ID.
"a
dnsResolversB*" IA list of IP addresses of DNS resolver endpoints for the VPC connection.
"5
name" )The display name for the VPC connection.
"B
roleArn" 3The IAM role to associate with the VPC connection.
"O
securityGroupIds*" 5A list of security group IDs for the VPC connection.
"g
	subnetIds*" TA list of subnet IDs for the VPC connection.

The following arguments are optional:
"�
tagsB2" �Key-value map of resource tags. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
"�
tagsAll2" xA map of tags assigned to the resource, including those inherited from the provider `default_tags` configuration block.
"q
timeoutseBc:a
_

quicksightVpcConnectionTimeouts:aws:quicksight/VpcConnectionTimeouts:VpcConnectionTimeouts"5
vpcConnectionId" The ID of the VPC connection.
*�:
N
ramPrincipalAssociation1aws:ram/principalAssociation:PrincipalAssociation�5Provides a Resource Access Manager (RAM) principal association. Depending if [RAM Sharing with AWS Organizations is enabled](https://docs.aws.amazon.com/ram/latest/userguide/getting-started-sharing.html#getting-started-sharing-orgs), the RAM behavior with different principal types changes.

When RAM Sharing with AWS Organizations is enabled:

- For AWS Account ID, Organization, and Organizational Unit principals within the same AWS Organization, no resource share invitation is sent and resources become available automatically after creating the association.
- For AWS Account ID principals outside the AWS Organization, a resource share invitation is sent and must be accepted before resources become available. See the `aws.ram.ResourceShareAccepter` resource to accept these invitations.

When RAM Sharing with AWS Organizations is not enabled:

- Organization and Organizational Unit principals cannot be used.
- For AWS Account ID principals, a resource share invitation is sent and must be accepted before resources become available. See the `aws.ram.ResourceShareAccepter` resource to accept these invitations.

## Example Usage

### AWS Account ID

<!--Start PulumiCodeChooser -->
```typescript
import * as pulumi from "@pulumi/pulumi";
import * as aws from "@pulumi/aws";

const example = new aws.ram.ResourceShare("example", {allowExternalPrincipals: true});
const examplePrincipalAssociation = new aws.ram.PrincipalAssociation("example", {
    principal: "111111111111",
    resourceShareArn: example.arn,
});
```
```python
import pulumi
import pulumi_aws as aws

example = aws.ram.ResourceShare("example", allow_external_principals=True)
example_principal_association = aws.ram.PrincipalAssociation("example",
    principal="111111111111",
    resource_share_arn=example.arn)
```
```csharp
using System.Collections.Generic;
using System.Linq;
using Pulumi;
using Aws = Pulumi.Aws;

return await Deployment.RunAsync(() => 
{
    var example = new Aws.Ram.ResourceShare("example", new()
    {
        AllowExternalPrincipals = true,
    });

    var examplePrincipalAssociation = new Aws.Ram.PrincipalAssociation("example", new()
    {
        Principal = "111111111111",
        ResourceShareArn = example.Arn,
    });

});
```
```go
package main

import (
	"github.com/pulumi/pulumi-aws/sdk/v6/go/aws/ram"
	"github.com/pulumi/pulumi/sdk/v3/go/pulumi"
)

func main() {
	pulumi.Run(func(ctx *pulumi.Context) error {
		example, err := ram.NewResourceShare(ctx, "example", &ram.ResourceShareArgs{
			AllowExternalPrincipals: pulumi.Bool(true),
		})
		if err != nil {
			return err
		}
		_, err = ram.NewPrincipalAssociation(ctx, "example", &ram.PrincipalAssociationArgs{
			Principal:        pulumi.String("111111111111"),
			ResourceShareArn: example.Arn,
		})
		if err != nil {
			return err
		}
		return nil
	})
}
```
```java
package generated_program;

import com.pulumi.Context;
import com.pulumi.Pulumi;
import com.pulumi.core.Output;
import com.pulumi.aws.ram.ResourceShare;
import com.pulumi.aws.ram.ResourceShareArgs;
import com.pulumi.aws.ram.PrincipalAssociation;
import com.pulumi.aws.ram.PrincipalAssociationArgs;
import java.util.List;
import java.util.ArrayList;
import java.util.Map;
import java.io.File;
import java.nio.file.Files;
import java.nio.file.Paths;

public class App {
    public static void main(String[] args) {
        Pulumi.run(App::stack);
    }

    public static void stack(Context ctx) {
        var example = new ResourceShare("example", ResourceShareArgs.builder()
            .allowExternalPrincipals(true)
            .build());

        var examplePrincipalAssociation = new PrincipalAssociation("examplePrincipalAssociation", PrincipalAssociationArgs.builder()
            .principal("111111111111")
            .resourceShareArn(example.arn())
            .build());

    }
}
```
```yaml
resources:
  example:
    type: aws:ram:ResourceShare
    properties:
      allowExternalPrincipals: true
  examplePrincipalAssociation:
    type: aws:ram:PrincipalAssociation
    name: example
    properties:
      principal: '111111111111'
      resourceShareArn: ${example.arn}
```
<!--End PulumiCodeChooser -->

### AWS Organization

<!--Start PulumiCodeChooser -->
```typescript
import * as pulumi from "@pulumi/pulumi";
import * as aws from "@pulumi/aws";

const example = new aws.ram.PrincipalAssociation("example", {
    principal: exampleAwsOrganizationsOrganization.arn,
    resourceShareArn: exampleAwsRamResourceShare.arn,
});
```
```python
import pulumi
import pulumi_aws as aws

example = aws.ram.PrincipalAssociation("example",
    principal=example_aws_organizations_organization["arn"],
    resource_share_arn=example_aws_ram_resource_share["arn"])
```
```csharp
using System.Collections.Generic;
using System.Linq;
using Pulumi;
using Aws = Pulumi.Aws;

return await Deployment.RunAsync(() => 
{
    var example = new Aws.Ram.PrincipalAssociation("example", new()
    {
        Principal = exampleAwsOrganizationsOrganization.Arn,
        ResourceShareArn = exampleAwsRamResourceShare.Arn,
    });

});
```
```go
package main

import (
	"github.com/pulumi/pulumi-aws/sdk/v6/go/aws/ram"
	"github.com/pulumi/pulumi/sdk/v3/go/pulumi"
)

func main() {
	pulumi.Run(func(ctx *pulumi.Context) error {
		_, err := ram.NewPrincipalAssociation(ctx, "example", &ram.PrincipalAssociationArgs{
			Principal:        pulumi.Any(exampleAwsOrganizationsOrganization.Arn),
			ResourceShareArn: pulumi.Any(exampleAwsRamResourceShare.Arn),
		})
		if err != nil {
			return err
		}
		return nil
	})
}
```
```java
package generated_program;

import com.pulumi.Context;
import com.pulumi.Pulumi;
import com.pulumi.core.Output;
import com.pulumi.aws.ram.PrincipalAssociation;
import com.pulumi.aws.ram.PrincipalAssociationArgs;
import java.util.List;
import java.util.ArrayList;
import java.util.Map;
import java.io.File;
import java.nio.file.Files;
import java.nio.file.Paths;

public class App {
    public static void main(String[] args) {
        Pulumi.run(App::stack);
    }

    public static void stack(Context ctx) {
        var example = new PrincipalAssociation("example", PrincipalAssociationArgs.builder()
            .principal(exampleAwsOrganizationsOrganization.arn())
            .resourceShareArn(exampleAwsRamResourceShare.arn())
            .build());

    }
}
```
```yaml
resources:
  example:
    type: aws:ram:PrincipalAssociation
    properties:
      principal: ${exampleAwsOrganizationsOrganization.arn}
      resourceShareArn: ${exampleAwsRamResourceShare.arn}
```
<!--End PulumiCodeChooser -->

## Import

Using `pulumi import`, import RAM Principal Associations using their Resource Share ARN and the `principal` separated by a comma. For example:

```sh
$ pulumi import aws:ram/principalAssociation:PrincipalAssociation example arn:aws:ram:eu-west-1:123456789012:resource-share/73da1ab9-b94a-4ba3-8eb4-45917f7f4b12,123456789012
```
�
	principal" �The principal to associate with the resource share. Possible values are an AWS account ID, an AWS Organizations Organization ARN, or an AWS Organizations Organization Unit ARN.
N
resourceShareArn" 6The Amazon Resource Name (ARN) of the resource share.
"�
	principal" �The principal to associate with the resource share. Possible values are an AWS account ID, an AWS Organizations Organization ARN, or an AWS Organizations Organization Unit ARN.
"N
resourceShareArn" 6The Amazon Resource Name (ARN) of the resource share.
*�
K
ramResourceAssociation/aws:ram/resourceAssociation:ResourceAssociation�Manages a Resource Access Manager (RAM) Resource Association.

> *NOTE:* Certain AWS resources (e.g., EC2 Subnets) can only be shared in an AWS account that is a member of an AWS Organizations organization with organization-wide Resource Access Manager functionality enabled. See the [Resource Access Manager User Guide](https://docs.aws.amazon.com/ram/latest/userguide/what-is.html) and AWS service specific documentation for additional information.

## Example Usage

<!--Start PulumiCodeChooser -->
```typescript
import * as pulumi from "@pulumi/pulumi";
import * as aws from "@pulumi/aws";

const example = new aws.ram.ResourceAssociation("example", {
    resourceArn: exampleAwsSubnet.arn,
    resourceShareArn: exampleAwsRamResourceShare.arn,
});
```
```python
import pulumi
import pulumi_aws as aws

example = aws.ram.ResourceAssociation("example",
    resource_arn=example_aws_subnet["arn"],
    resource_share_arn=example_aws_ram_resource_share["arn"])
```
```csharp
using System.Collections.Generic;
using System.Linq;
using Pulumi;
using Aws = Pulumi.Aws;

return await Deployment.RunAsync(() => 
{
    var example = new Aws.Ram.ResourceAssociation("example", new()
    {
        ResourceArn = exampleAwsSubnet.Arn,
        ResourceShareArn = exampleAwsRamResourceShare.Arn,
    });

});
```
```go
package main

import (
	"github.com/pulumi/pulumi-aws/sdk/v6/go/aws/ram"
	"github.com/pulumi/pulumi/sdk/v3/go/pulumi"
)

func main() {
	pulumi.Run(func(ctx *pulumi.Context) error {
		_, err := ram.NewResourceAssociation(ctx, "example", &ram.ResourceAssociationArgs{
			ResourceArn:      pulumi.Any(exampleAwsSubnet.Arn),
			ResourceShareArn: pulumi.Any(exampleAwsRamResourceShare.Arn),
		})
		if err != nil {
			return err
		}
		return nil
	})
}
```
```java
package generated_program;

import com.pulumi.Context;
import com.pulumi.Pulumi;
import com.pulumi.core.Output;
import com.pulumi.aws.ram.ResourceAssociation;
import com.pulumi.aws.ram.ResourceAssociationArgs;
import java.util.List;
import java.util.ArrayList;
import java.util.Map;
import java.io.File;
import java.nio.file.Files;
import java.nio.file.Paths;

public class App {
    public static void main(String[] args) {
        Pulumi.run(App::stack);
    }

    public static void stack(Context ctx) {
        var example = new ResourceAssociation("example", ResourceAssociationArgs.builder()
            .resourceArn(exampleAwsSubnet.arn())
            .resourceShareArn(exampleAwsRamResourceShare.arn())
            .build());

    }
}
```
```yaml
resources:
  example:
    type: aws:ram:ResourceAssociation
    properties:
      resourceArn: ${exampleAwsSubnet.arn}
      resourceShareArn: ${exampleAwsRamResourceShare.arn}
```
<!--End PulumiCodeChooser -->

## Import

Using `pulumi import`, import RAM Resource Associations using their Resource Share ARN and Resource ARN separated by a comma. For example:

```sh
$ pulumi import aws:ram/resourceAssociation:ResourceAssociation example arn:aws:ram:eu-west-1:123456789012:resource-share/73da1ab9-b94a-4ba3-8eb4-45917f7f4b12,arn:aws:ec2:eu-west-1:123456789012:subnet/subnet-12345678
```
h
resourceArn" UAmazon Resource Name (ARN) of the resource to associate with the RAM Resource Share.
N
resourceShareArn" 6Amazon Resource Name (ARN) of the RAM Resource Share.
"h
resourceArn" UAmazon Resource Name (ARN) of the resource to associate with the RAM Resource Share.
"N
resourceShareArn" 6Amazon Resource Name (ARN) of the RAM Resource Share.
*�$
9
ramResourceShare#aws:ram/resourceShare:ResourceShare�Manages a Resource Access Manager (RAM) Resource Share. To associate principals with the share, see the `aws.ram.PrincipalAssociation` resource. To associate resources with the share, see the `aws.ram.ResourceAssociation` resource.

## Example Usage

<!--Start PulumiCodeChooser -->
```typescript
import * as pulumi from "@pulumi/pulumi";
import * as aws from "@pulumi/aws";

const example = new aws.ram.ResourceShare("example", {
    name: "example",
    allowExternalPrincipals: true,
    tags: {
        Environment: "Production",
    },
});
```
```python
import pulumi
import pulumi_aws as aws

example = aws.ram.ResourceShare("example",
    name="example",
    allow_external_principals=True,
    tags={
        "Environment": "Production",
    })
```
```csharp
using System.Collections.Generic;
using System.Linq;
using Pulumi;
using Aws = Pulumi.Aws;

return await Deployment.RunAsync(() => 
{
    var example = new Aws.Ram.ResourceShare("example", new()
    {
        Name = "example",
        AllowExternalPrincipals = true,
        Tags = 
        {
            { "Environment", "Production" },
        },
    });

});
```
```go
package main

import (
	"github.com/pulumi/pulumi-aws/sdk/v6/go/aws/ram"
	"github.com/pulumi/pulumi/sdk/v3/go/pulumi"
)

func main() {
	pulumi.Run(func(ctx *pulumi.Context) error {
		_, err := ram.NewResourceShare(ctx, "example", &ram.ResourceShareArgs{
			Name:                    pulumi.String("example"),
			AllowExternalPrincipals: pulumi.Bool(true),
			Tags: pulumi.StringMap{
				"Environment": pulumi.String("Production"),
			},
		})
		if err != nil {
			return err
		}
		return nil
	})
}
```
```java
package generated_program;

import com.pulumi.Context;
import com.pulumi.Pulumi;
import com.pulumi.core.Output;
import com.pulumi.aws.ram.ResourceShare;
import com.pulumi.aws.ram.ResourceShareArgs;
import java.util.List;
import java.util.ArrayList;
import java.util.Map;
import java.io.File;
import java.nio.file.Files;
import java.nio.file.Paths;

public class App {
    public static void main(String[] args) {
        Pulumi.run(App::stack);
    }

    public static void stack(Context ctx) {
        var example = new ResourceShare("example", ResourceShareArgs.builder()
            .name("example")
            .allowExternalPrincipals(true)
            .tags(Map.of("Environment", "Production"))
            .build());

    }
}
```
```yaml
resources:
  example:
    type: aws:ram:ResourceShare
    properties:
      name: example
      allowExternalPrincipals: true
      tags:
        Environment: Production
```
<!--End PulumiCodeChooser -->

## Import

Using `pulumi import`, import resource shares using the `arn` of the resource share. For example:

```sh
$ pulumi import aws:ram/resourceShare:ResourceShare example arn:aws:ram:eu-west-1:123456789012:resource-share/73da1ab9-b94a-4ba3-8eb4-45917f7f4b12
```
�
allowExternalPrincipalsB
 `Indicates whether principals outside your organization can be associated with a resource share.
.
nameB"  The name of the resource share.
�
permissionArnsB*" �Specifies the Amazon Resource Names (ARNs) of the RAM permission to associate with the resource share. If you do not specify an ARN for the permission, RAM automatically attaches the default version of the permission for each resource type. You can associate only one permission with each resource type included in the resource share.
�
tagsB2" �A map of tags to assign to the resource share. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
"�
allowExternalPrincipalsB
 `Indicates whether principals outside your organization can be associated with a resource share.
"A
arn" 6The Amazon Resource Name (ARN) of the resource share.
",
name"  The name of the resource share.
"�
permissionArns*" �Specifies the Amazon Resource Names (ARNs) of the RAM permission to associate with the resource share. If you do not specify an ARN for the permission, RAM automatically attaches the default version of the permission for each resource type. You can associate only one permission with each resource type included in the resource share.
"�
tagsB2" �A map of tags to assign to the resource share. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
"�
tagsAll2" xA map of tags assigned to the resource, including those inherited from the provider `default_tags` configuration block.
*�:
Q
ramResourceShareAccepter3aws:ram/resourceShareAccepter:ResourceShareAccepter�4Manage accepting a Resource Access Manager (RAM) Resource Share invitation. From a _receiver_ AWS account, accept an invitation to share resources that were shared by a _sender_ AWS account. To create a resource share in the _sender_, see the `aws.ram.ResourceShare` resource.

> **Note:** If both AWS accounts are in the same Organization and [RAM Sharing with AWS Organizations is enabled](https://docs.aws.amazon.com/ram/latest/userguide/getting-started-sharing.html#getting-started-sharing-orgs), this resource is not necessary as RAM Resource Share invitations are not used.

## Example Usage

This configuration provides an example of using multiple AWS providers to configure two different AWS accounts. In the _sender_ account, the configuration creates a `aws.ram.ResourceShare` and uses a data source in the _receiver_ account to create a `aws.ram.PrincipalAssociation` resource with the _receiver's_ account ID. In the _receiver_ account, the configuration accepts the invitation to share resources with the `aws.ram.ResourceShareAccepter`.

<!--Start PulumiCodeChooser -->
```typescript
import * as pulumi from "@pulumi/pulumi";
import * as aws from "@pulumi/aws";

const senderShare = new aws.ram.ResourceShare("sender_share", {
    name: "tf-test-resource-share",
    allowExternalPrincipals: true,
    tags: {
        Name: "tf-test-resource-share",
    },
});
const receiver = aws.getCallerIdentity({});
const senderInvite = new aws.ram.PrincipalAssociation("sender_invite", {
    principal: receiver.then(receiver => receiver.accountId),
    resourceShareArn: senderShare.arn,
});
const receiverAccept = new aws.ram.ResourceShareAccepter("receiver_accept", {shareArn: senderInvite.resourceShareArn});
```
```python
import pulumi
import pulumi_aws as aws

sender_share = aws.ram.ResourceShare("sender_share",
    name="tf-test-resource-share",
    allow_external_principals=True,
    tags={
        "Name": "tf-test-resource-share",
    })
receiver = aws.get_caller_identity()
sender_invite = aws.ram.PrincipalAssociation("sender_invite",
    principal=receiver.account_id,
    resource_share_arn=sender_share.arn)
receiver_accept = aws.ram.ResourceShareAccepter("receiver_accept", share_arn=sender_invite.resource_share_arn)
```
```csharp
using System.Collections.Generic;
using System.Linq;
using Pulumi;
using Aws = Pulumi.Aws;

return await Deployment.RunAsync(() => 
{
    var senderShare = new Aws.Ram.ResourceShare("sender_share", new()
    {
        Name = "tf-test-resource-share",
        AllowExternalPrincipals = true,
        Tags = 
        {
            { "Name", "tf-test-resource-share" },
        },
    });

    var receiver = Aws.GetCallerIdentity.Invoke();

    var senderInvite = new Aws.Ram.PrincipalAssociation("sender_invite", new()
    {
        Principal = receiver.Apply(getCallerIdentityResult => getCallerIdentityResult.AccountId),
        ResourceShareArn = senderShare.Arn,
    });

    var receiverAccept = new Aws.Ram.ResourceShareAccepter("receiver_accept", new()
    {
        ShareArn = senderInvite.ResourceShareArn,
    });

});
```
```go
package main

import (
	"github.com/pulumi/pulumi-aws/sdk/v6/go/aws"
	"github.com/pulumi/pulumi-aws/sdk/v6/go/aws/ram"
	"github.com/pulumi/pulumi/sdk/v3/go/pulumi"
)

func main() {
	pulumi.Run(func(ctx *pulumi.Context) error {
		senderShare, err := ram.NewResourceShare(ctx, "sender_share", &ram.ResourceShareArgs{
			Name:                    pulumi.String("tf-test-resource-share"),
			AllowExternalPrincipals: pulumi.Bool(true),
			Tags: pulumi.StringMap{
				"Name": pulumi.String("tf-test-resource-share"),
			},
		})
		if err != nil {
			return err
		}
		receiver, err := aws.GetCallerIdentity(ctx, &aws.GetCallerIdentityArgs{}, nil)
		if err != nil {
			return err
		}
		senderInvite, err := ram.NewPrincipalAssociation(ctx, "sender_invite", &ram.PrincipalAssociationArgs{
			Principal:        pulumi.String(receiver.AccountId),
			ResourceShareArn: senderShare.Arn,
		})
		if err != nil {
			return err
		}
		_, err = ram.NewResourceShareAccepter(ctx, "receiver_accept", &ram.ResourceShareAccepterArgs{
			ShareArn: senderInvite.ResourceShareArn,
		})
		if err != nil {
			return err
		}
		return nil
	})
}
```
```java
package generated_program;

import com.pulumi.Context;
import com.pulumi.Pulumi;
import com.pulumi.core.Output;
import com.pulumi.aws.ram.ResourceShare;
import com.pulumi.aws.ram.ResourceShareArgs;
import com.pulumi.aws.AwsFunctions;
import com.pulumi.aws.inputs.GetCallerIdentityArgs;
import com.pulumi.aws.ram.PrincipalAssociation;
import com.pulumi.aws.ram.PrincipalAssociationArgs;
import com.pulumi.aws.ram.ResourceShareAccepter;
import com.pulumi.aws.ram.ResourceShareAccepterArgs;
import java.util.List;
import java.util.ArrayList;
import java.util.Map;
import java.io.File;
import java.nio.file.Files;
import java.nio.file.Paths;

public class App {
    public static void main(String[] args) {
        Pulumi.run(App::stack);
    }

    public static void stack(Context ctx) {
        var senderShare = new ResourceShare("senderShare", ResourceShareArgs.builder()
            .name("tf-test-resource-share")
            .allowExternalPrincipals(true)
            .tags(Map.of("Name", "tf-test-resource-share"))
            .build());

        final var receiver = AwsFunctions.getCallerIdentity();

        var senderInvite = new PrincipalAssociation("senderInvite", PrincipalAssociationArgs.builder()
            .principal(receiver.applyValue(getCallerIdentityResult -> getCallerIdentityResult.accountId()))
            .resourceShareArn(senderShare.arn())
            .build());

        var receiverAccept = new ResourceShareAccepter("receiverAccept", ResourceShareAccepterArgs.builder()
            .shareArn(senderInvite.resourceShareArn())
            .build());

    }
}
```
```yaml
resources:
  senderShare:
    type: aws:ram:ResourceShare
    name: sender_share
    properties:
      name: tf-test-resource-share
      allowExternalPrincipals: true
      tags:
        Name: tf-test-resource-share
  senderInvite:
    type: aws:ram:PrincipalAssociation
    name: sender_invite
    properties:
      principal: ${receiver.accountId}
      resourceShareArn: ${senderShare.arn}
  receiverAccept:
    type: aws:ram:ResourceShareAccepter
    name: receiver_accept
    properties:
      shareArn: ${senderInvite.resourceShareArn}
variables:
  receiver:
    fn::invoke:
      function: aws:getCallerIdentity
      arguments: {}
```
<!--End PulumiCodeChooser -->

## Import

Using `pulumi import`, import resource share accepters using the resource share ARN. For example:

```sh
$ pulumi import aws:ram/resourceShareAccepter:ResourceShareAccepter example arn:aws:ram:us-east-1:123456789012:resource-share/c4b56393-e8d9-89d9-6dc9-883752de4767
```
/
shareArn" The ARN of the resource share.
"?
invitationArn" *The ARN of the resource share invitation.
"^
receiverAccountId" EThe account ID of the receiver account which accepts the invitation.
"N
	resources*" ;A list of the resource ARNs shared via the resource share.
"Z
senderAccountId" CThe account ID of the sender account which submits the invitation.
"/
shareArn" The ARN of the resource share.
"I
shareId" :The ID of the resource share as displayed in the console.
"1
	shareName"  The name of the resource share.
"]
status" OThe status of the resource share (ACTIVE, PENDING, FAILED, DELETING, DELETED).
*�
W
ramSharingWithOrganization7aws:ram/sharingWithOrganization:SharingWithOrganization�Manages Resource Access Manager (RAM) Resource Sharing with AWS Organizations. If you enable sharing with your organization, you can share resources without using invitations. Refer to the [AWS RAM user guide](https://docs.aws.amazon.com/ram/latest/userguide/getting-started-sharing.html#getting-started-sharing-orgs) for more details.

> **NOTE:** Use this resource to manage resource sharing within your organization, **not** the `aws.organizations.Organization` resource with `ram.amazonaws.com` configured in `aws_service_access_principals`.

## Example Usage

<!--Start PulumiCodeChooser -->
```typescript
import * as pulumi from "@pulumi/pulumi";
import * as aws from "@pulumi/aws";

const example = new aws.ram.SharingWithOrganization("example", {});
```
```python
import pulumi
import pulumi_aws as aws

example = aws.ram.SharingWithOrganization("example")
```
```csharp
using System.Collections.Generic;
using System.Linq;
using Pulumi;
using Aws = Pulumi.Aws;

return await Deployment.RunAsync(() => 
{
    var example = new Aws.Ram.SharingWithOrganization("example");

});
```
```go
package main

import (
	"github.com/pulumi/pulumi-aws/sdk/v6/go/aws/ram"
	"github.com/pulumi/pulumi/sdk/v3/go/pulumi"
)

func main() {
	pulumi.Run(func(ctx *pulumi.Context) error {
		_, err := ram.NewSharingWithOrganization(ctx, "example", nil)
		if err != nil {
			return err
		}
		return nil
	})
}
```
```java
package generated_program;

import com.pulumi.Context;
import com.pulumi.Pulumi;
import com.pulumi.core.Output;
import com.pulumi.aws.ram.SharingWithOrganization;
import java.util.List;
import java.util.ArrayList;
import java.util.Map;
import java.io.File;
import java.nio.file.Files;
import java.nio.file.Paths;

public class App {
    public static void main(String[] args) {
        Pulumi.run(App::stack);
    }

    public static void stack(Context ctx) {
        var example = new SharingWithOrganization("example");

    }
}
```
```yaml
resources:
  example:
    type: aws:ram:SharingWithOrganization
```
<!--End PulumiCodeChooser -->

## Import

Using `pulumi import`, import the resource using the current AWS account ID. For example:

```sh
$ pulumi import aws:ram/sharingWithOrganization:SharingWithOrganization example 123456789012
```
*�5
 
rbinRuleaws:rbin/rule:Rule�"Resource for managing an AWS RBin Rule.

## Example Usage

### Basic Usage

<!--Start PulumiCodeChooser -->
```typescript
import * as pulumi from "@pulumi/pulumi";
import * as aws from "@pulumi/aws";

const example = new aws.rbin.Rule("example", {
    description: "example_rule",
    resourceType: "EBS_SNAPSHOT",
    resourceTags: [{
        resourceTagKey: "tag_key",
        resourceTagValue: "tag_value",
    }],
    retentionPeriod: {
        retentionPeriodValue: 10,
        retentionPeriodUnit: "DAYS",
    },
    tags: {
        test_tag_key: "test_tag_value",
    },
});
```
```python
import pulumi
import pulumi_aws as aws

example = aws.rbin.Rule("example",
    description="example_rule",
    resource_type="EBS_SNAPSHOT",
    resource_tags=[{
        "resource_tag_key": "tag_key",
        "resource_tag_value": "tag_value",
    }],
    retention_period={
        "retention_period_value": 10,
        "retention_period_unit": "DAYS",
    },
    tags={
        "test_tag_key": "test_tag_value",
    })
```
```csharp
using System.Collections.Generic;
using System.Linq;
using Pulumi;
using Aws = Pulumi.Aws;

return await Deployment.RunAsync(() => 
{
    var example = new Aws.Rbin.Rule("example", new()
    {
        Description = "example_rule",
        ResourceType = "EBS_SNAPSHOT",
        ResourceTags = new[]
        {
            new Aws.Rbin.Inputs.RuleResourceTagArgs
            {
                ResourceTagKey = "tag_key",
                ResourceTagValue = "tag_value",
            },
        },
        RetentionPeriod = new Aws.Rbin.Inputs.RuleRetentionPeriodArgs
        {
            RetentionPeriodValue = 10,
            RetentionPeriodUnit = "DAYS",
        },
        Tags = 
        {
            { "test_tag_key", "test_tag_value" },
        },
    });

});
```
```go
package main

import (
	"github.com/pulumi/pulumi-aws/sdk/v6/go/aws/rbin"
	"github.com/pulumi/pulumi/sdk/v3/go/pulumi"
)

func main() {
	pulumi.Run(func(ctx *pulumi.Context) error {
		_, err := rbin.NewRule(ctx, "example", &rbin.RuleArgs{
			Description:  pulumi.String("example_rule"),
			ResourceType: pulumi.String("EBS_SNAPSHOT"),
			ResourceTags: rbin.RuleResourceTagArray{
				&rbin.RuleResourceTagArgs{
					ResourceTagKey:   pulumi.String("tag_key"),
					ResourceTagValue: pulumi.String("tag_value"),
				},
			},
			RetentionPeriod: &rbin.RuleRetentionPeriodArgs{
				RetentionPeriodValue: pulumi.Int(10),
				RetentionPeriodUnit:  pulumi.String("DAYS"),
			},
			Tags: pulumi.StringMap{
				"test_tag_key": pulumi.String("test_tag_value"),
			},
		})
		if err != nil {
			return err
		}
		return nil
	})
}
```
```java
package generated_program;

import com.pulumi.Context;
import com.pulumi.Pulumi;
import com.pulumi.core.Output;
import com.pulumi.aws.rbin.Rule;
import com.pulumi.aws.rbin.RuleArgs;
import com.pulumi.aws.rbin.inputs.RuleResourceTagArgs;
import com.pulumi.aws.rbin.inputs.RuleRetentionPeriodArgs;
import java.util.List;
import java.util.ArrayList;
import java.util.Map;
import java.io.File;
import java.nio.file.Files;
import java.nio.file.Paths;

public class App {
    public static void main(String[] args) {
        Pulumi.run(App::stack);
    }

    public static void stack(Context ctx) {
        var example = new Rule("example", RuleArgs.builder()
            .description("example_rule")
            .resourceType("EBS_SNAPSHOT")
            .resourceTags(RuleResourceTagArgs.builder()
                .resourceTagKey("tag_key")
                .resourceTagValue("tag_value")
                .build())
            .retentionPeriod(RuleRetentionPeriodArgs.builder()
                .retentionPeriodValue(10)
                .retentionPeriodUnit("DAYS")
                .build())
            .tags(Map.of("test_tag_key", "test_tag_value"))
            .build());

    }
}
```
```yaml
resources:
  example:
    type: aws:rbin:Rule
    properties:
      description: example_rule
      resourceType: EBS_SNAPSHOT
      resourceTags:
        - resourceTagKey: tag_key
          resourceTagValue: tag_value
      retentionPeriod:
        retentionPeriodValue: 10
        retentionPeriodUnit: DAYS
      tags:
        test_tag_key: test_tag_value
```
<!--End PulumiCodeChooser -->

## Import

Using `pulumi import`, import RBin Rule using the `id`. For example:

```sh
$ pulumi import aws:rbin/rule:Rule example examplerule
```
5
descriptionB"  The retention rule description.
�
lockConfigurationYBW:U
S
rbinRuleLockConfiguration4aws:rbin/RuleLockConfiguration:RuleLockConfigurationYInformation about the retention rule lock configuration. See `lock_configuration` below.
�
resourceTagsIBG*E:C
A
rbinRuleResourceTag(aws:rbin/RuleResourceTag:RuleResourceTag�Specifies the resource tags to use to identify resources that are to be retained by a tag-level retention rule. See `resource_tags` below.
}
resourceType" iThe resource type to be retained by the retention rule. Valid values are `EBS_SNAPSHOT` and `EC2_IMAGE`.
�
retentionPeriodQ:O
M
rbinRuleRetentionPeriod0aws:rbin/RuleRetentionPeriod:RuleRetentionPeriod�Information about the retention period for which the retention rule is to retain resources. See `retention_period` below.

The following arguments are optional:

tagsB2" "	
arn" "3
description"  The retention rule description.
"�
lockConfigurationYBW:U
S
rbinRuleLockConfiguration4aws:rbin/RuleLockConfiguration:RuleLockConfigurationYInformation about the retention rule lock configuration. See `lock_configuration` below.
"�
lockEndTime" �(Timestamp) The date and time at which the unlock delay is set to expire. Only returned for retention rules that have been unlocked and that are still within the unlock delay period.
"�
	lockState" �(Optional) The lock state of the retention rules to list. Only retention rules with the specified lock state are returned. Valid values are `locked`, `pending_unlock`, `unlocked`.
"�
resourceTagsG*E:C
A
rbinRuleResourceTag(aws:rbin/RuleResourceTag:RuleResourceTag�Specifies the resource tags to use to identify resources that are to be retained by a tag-level retention rule. See `resource_tags` below.
"}
resourceType" iThe resource type to be retained by the retention rule. Valid values are `EBS_SNAPSHOT` and `EC2_IMAGE`.
"�
retentionPeriodQ:O
M
rbinRuleRetentionPeriod0aws:rbin/RuleRetentionPeriod:RuleRetentionPeriod�Information about the retention period for which the retention rule is to retain resources. See `retention_period` below.

The following arguments are optional:
"�
status" �(String) The state of the retention rule. Only retention rules that are in the `available` state retain resources. Valid values include `pending` and `available`.
"
tagsB2" "
tagsAll2" 2�
!getArnaws:index/getArn:getArn�Parses an ARN into its constituent parts.

## Example Usage

<!--Start PulumiCodeChooser -->
```typescript
import * as pulumi from "@pulumi/pulumi";
import * as aws from "@pulumi/aws";

const dbInstance = aws.getArn({
    arn: "arn:aws:rds:eu-west-1:123456789012:db:mysql-db",
});
```
```python
import pulumi
import pulumi_aws as aws

db_instance = aws.get_arn(arn="arn:aws:rds:eu-west-1:123456789012:db:mysql-db")
```
```csharp
using System.Collections.Generic;
using System.Linq;
using Pulumi;
using Aws = Pulumi.Aws;

return await Deployment.RunAsync(() => 
{
    var dbInstance = Aws.GetArn.Invoke(new()
    {
        Arn = "arn:aws:rds:eu-west-1:123456789012:db:mysql-db",
    });

});
```
```go
package main

import (
	"github.com/pulumi/pulumi-aws/sdk/v6/go/aws"
	"github.com/pulumi/pulumi/sdk/v3/go/pulumi"
)

func main() {
	pulumi.Run(func(ctx *pulumi.Context) error {
		_, err := aws.GetArn(ctx, &aws.GetArnArgs{
			Arn: "arn:aws:rds:eu-west-1:123456789012:db:mysql-db",
		}, nil)
		if err != nil {
			return err
		}
		return nil
	})
}
```
```java
package generated_program;

import com.pulumi.Context;
import com.pulumi.Pulumi;
import com.pulumi.core.Output;
import com.pulumi.aws.AwsFunctions;
import com.pulumi.aws.inputs.GetArnArgs;
import java.util.List;
import java.util.ArrayList;
import java.util.Map;
import java.io.File;
import java.nio.file.Files;
import java.nio.file.Paths;

public class App {
    public static void main(String[] args) {
        Pulumi.run(App::stack);
    }

    public static void stack(Context ctx) {
        final var dbInstance = AwsFunctions.getArn(GetArnArgs.builder()
            .arn("arn:aws:rds:eu-west-1:123456789012:db:mysql-db")
            .build());

    }
}
```
```yaml
variables:
  dbInstance:
    fn::invoke:
      function: aws:getArn
      arguments:
        arn: arn:aws:rds:eu-west-1:123456789012:db:mysql-db
```
<!--End PulumiCodeChooser -->

arn" ARN to parse.


idB" "�
account" �The [ID](https://docs.aws.amazon.com/general/latest/gr/acct-identifiers.html) of the AWS account that owns the resource, without the hyphens.
"	
arn" "
id" "4
	partition" #Partition that the resource is in.
"�
region" �Region the resource resides in.
Note that the ARNs for some resources do not require a region, so this component might be omitted.
"�
resource" �Content of this part of the ARN varies by service.
It often includes an indicator of the type of resource—for example, an IAM user or Amazon RDS database —followed by a slash (/) or a colon (:), followed by the resource name itself.
"�
service" �The [service namespace](https://docs.aws.amazon.com/general/latest/gr/aws-arns-and-namespaces.html#genref-aws-service-namespaces) that identifies the AWS product.
2�[
HgetAvailabilityZone1aws:index/getAvailabilityZone:getAvailabilityZone�J`aws.getAvailabilityZone` provides details about a specific availability zone (AZ)
in the current region.

This can be used both to validate an availability zone given in a variable
and to split the AZ name into its component parts of an AWS region and an
AZ identifier letter. The latter may be useful e.g., for implementing a
consistent subnet numbering scheme across several regions by mapping both
the region and the subnet letter to network numbers.

This is different from the `aws.getAvailabilityZones` (plural) data source,
which provides a list of the available zones.

## Example Usage

The following example shows how this data source might be used to derive
VPC and subnet CIDR prefixes systematically for an availability zone.

<!--Start PulumiCodeChooser -->
```typescript
import * as pulumi from "@pulumi/pulumi";
import * as aws from "@pulumi/aws";
import * as std from "@pulumi/std";

const config = new pulumi.Config();
const regionNumber = config.getObject("regionNumber") || {
    "ap-northeast-1": 5,
    "eu-central-1": 4,
    "us-east-1": 1,
    "us-west-1": 2,
    "us-west-2": 3,
};
const azNumber = config.getObject("azNumber") || {
    a: 1,
    b: 2,
    c: 3,
    d: 4,
    e: 5,
    f: 6,
};
// Retrieve the AZ where we want to create network resources
// This must be in the region selected on the AWS provider.
const example = aws.getAvailabilityZone({
    name: "eu-central-1a",
});
// Create a VPC for the region associated with the AZ
const exampleVpc = new aws.ec2.Vpc("example", {cidrBlock: example.then(example => std.cidrsubnet({
    input: "10.0.0.0/8",
    newbits: 4,
    netnum: regionNumber[example.region],
})).then(invoke => invoke.result)});
// Create a subnet for the AZ within the regional VPC
const exampleSubnet = new aws.ec2.Subnet("example", {
    vpcId: exampleVpc.id,
    cidrBlock: pulumi.all([exampleVpc.cidrBlock, example]).apply(([cidrBlock, example]) => std.cidrsubnetOutput({
        input: cidrBlock,
        newbits: 4,
        netnum: azNumber[example.nameSuffix],
    })).apply(invoke => invoke.result),
});
```
```python
import pulumi
import pulumi_aws as aws
import pulumi_std as std

config = pulumi.Config()
region_number = config.get_object("regionNumber")
if region_number is None:
    region_number = {
        "ap-northeast-1": 5,
        "eu-central-1": 4,
        "us-east-1": 1,
        "us-west-1": 2,
        "us-west-2": 3,
    }
az_number = config.get_object("azNumber")
if az_number is None:
    az_number = {
        "a": 1,
        "b": 2,
        "c": 3,
        "d": 4,
        "e": 5,
        "f": 6,
    }
# Retrieve the AZ where we want to create network resources
# This must be in the region selected on the AWS provider.
example = aws.get_availability_zone(name="eu-central-1a")
# Create a VPC for the region associated with the AZ
example_vpc = aws.ec2.Vpc("example", cidr_block=std.cidrsubnet(input="10.0.0.0/8",
    newbits=4,
    netnum=region_number[example.region]).result)
# Create a subnet for the AZ within the regional VPC
example_subnet = aws.ec2.Subnet("example",
    vpc_id=example_vpc.id,
    cidr_block=example_vpc.cidr_block.apply(lambda cidr_block: std.cidrsubnet_output(input=cidr_block,
        newbits=4,
        netnum=az_number[example.name_suffix])).apply(lambda invoke: invoke.result))
```
```csharp
using System.Collections.Generic;
using System.Linq;
using Pulumi;
using Aws = Pulumi.Aws;
using Std = Pulumi.Std;

return await Deployment.RunAsync(() => 
{
    var config = new Config();
    var regionNumber = config.GetObject<dynamic>("regionNumber") ?? 
    {
        { "ap-northeast-1", 5 },
        { "eu-central-1", 4 },
        { "us-east-1", 1 },
        { "us-west-1", 2 },
        { "us-west-2", 3 },
    };
    var azNumber = config.GetObject<dynamic>("azNumber") ?? 
    {
        { "a", 1 },
        { "b", 2 },
        { "c", 3 },
        { "d", 4 },
        { "e", 5 },
        { "f", 6 },
    };
    // Retrieve the AZ where we want to create network resources
    // This must be in the region selected on the AWS provider.
    var example = Aws.GetAvailabilityZone.Invoke(new()
    {
        Name = "eu-central-1a",
    });

    // Create a VPC for the region associated with the AZ
    var exampleVpc = new Aws.Ec2.Vpc("example", new()
    {
        CidrBlock = Std.Cidrsubnet.Invoke(new()
        {
            Input = "10.0.0.0/8",
            Newbits = 4,
            Netnum = regionNumber[example.Apply(getAvailabilityZoneResult => getAvailabilityZoneResult.Region)],
        }).Apply(invoke => invoke.Result),
    });

    // Create a subnet for the AZ within the regional VPC
    var exampleSubnet = new Aws.Ec2.Subnet("example", new()
    {
        VpcId = exampleVpc.Id,
        CidrBlock = Output.Tuple(exampleVpc.CidrBlock, example).Apply(values =>
        {
            var cidrBlock = values.Item1;
            var example = values.Item2;
            return Std.Cidrsubnet.Invoke(new()
            {
                Input = cidrBlock,
                Newbits = 4,
                Netnum = azNumber[example.Apply(getAvailabilityZoneResult => getAvailabilityZoneResult.NameSuffix)],
            });
        }).Apply(invoke => invoke.Result),
    });

});
```
```go
package main

import (
	"github.com/pulumi/pulumi-aws/sdk/v6/go/aws"
	"github.com/pulumi/pulumi-aws/sdk/v6/go/aws/ec2"
	"github.com/pulumi/pulumi-std/sdk/go/std"
	"github.com/pulumi/pulumi/sdk/v3/go/pulumi"
	"github.com/pulumi/pulumi/sdk/v3/go/pulumi/config"
)

func main() {
	pulumi.Run(func(ctx *pulumi.Context) error {
		cfg := config.New(ctx, "")
		regionNumber := map[string]interface{}{
			"ap-northeast-1": 5,
			"eu-central-1":   4,
			"us-east-1":      1,
			"us-west-1":      2,
			"us-west-2":      3,
		}
		if param := cfg.GetObject("regionNumber"); param != nil {
			regionNumber = param
		}
		azNumber := map[string]interface{}{
			"a": 1,
			"b": 2,
			"c": 3,
			"d": 4,
			"e": 5,
			"f": 6,
		}
		if param := cfg.GetObject("azNumber"); param != nil {
			azNumber = param
		}
		// Retrieve the AZ where we want to create network resources
		// This must be in the region selected on the AWS provider.
		example, err := aws.GetAvailabilityZone(ctx, &aws.GetAvailabilityZoneArgs{
			Name: pulumi.StringRef("eu-central-1a"),
		}, nil)
		if err != nil {
			return err
		}
		invokeCidrsubnet, err := std.Cidrsubnet(ctx, &std.CidrsubnetArgs{
			Input:   "10.0.0.0/8",
			Newbits: 4,
			Netnum:  regionNumber[example.Region],
		}, nil)
		if err != nil {
			return err
		}
		// Create a VPC for the region associated with the AZ
		exampleVpc, err := ec2.NewVpc(ctx, "example", &ec2.VpcArgs{
			CidrBlock: pulumi.String(invokeCidrsubnet.Result),
		})
		if err != nil {
			return err
		}
		// Create a subnet for the AZ within the regional VPC
		_, err = ec2.NewSubnet(ctx, "example", &ec2.SubnetArgs{
			VpcId: exampleVpc.ID(),
			CidrBlock: pulumi.String(exampleVpc.CidrBlock.ApplyT(func(cidrBlock string) (std.CidrsubnetResult, error) {
				return std.CidrsubnetResult(interface{}(std.CidrsubnetOutput(ctx, std.CidrsubnetOutputArgs{
					Input:   cidrBlock,
					Newbits: 4,
					Netnum:  pulumi.Int(azNumber[example.NameSuffix]),
				}, nil))), nil
			}).(std.CidrsubnetResultOutput).ApplyT(func(invoke std.CidrsubnetResult) (*string, error) {
				return invoke.Result, nil
			}).(pulumi.StringPtrOutput)),
		})
		if err != nil {
			return err
		}
		return nil
	})
}
```
```java
package generated_program;

import com.pulumi.Context;
import com.pulumi.Pulumi;
import com.pulumi.core.Output;
import com.pulumi.aws.AwsFunctions;
import com.pulumi.aws.inputs.GetAvailabilityZoneArgs;
import com.pulumi.aws.ec2.Vpc;
import com.pulumi.aws.ec2.VpcArgs;
import com.pulumi.aws.ec2.Subnet;
import com.pulumi.aws.ec2.SubnetArgs;
import java.util.List;
import java.util.ArrayList;
import java.util.Map;
import java.io.File;
import java.nio.file.Files;
import java.nio.file.Paths;

public class App {
    public static void main(String[] args) {
        Pulumi.run(App::stack);
    }

    public static void stack(Context ctx) {
        final var config = ctx.config();
        final var regionNumber = config.get("regionNumber").orElse(%!v(PANIC=Format method: runtime error: invalid memory address or nil pointer dereference));
        final var azNumber = config.get("azNumber").orElse(%!v(PANIC=Format method: runtime error: invalid memory address or nil pointer dereference));
        // Retrieve the AZ where we want to create network resources
        // This must be in the region selected on the AWS provider.
        final var example = AwsFunctions.getAvailabilityZone(GetAvailabilityZoneArgs.builder()
            .name("eu-central-1a")
            .build());

        // Create a VPC for the region associated with the AZ
        var exampleVpc = new Vpc("exampleVpc", VpcArgs.builder()
            .cidrBlock(StdFunctions.cidrsubnet(CidrsubnetArgs.builder()
                .input("10.0.0.0/8")
                .newbits(4)
                .netnum(regionNumber[example.applyValue(getAvailabilityZoneResult -> getAvailabilityZoneResult.region())])
                .build()).result())
            .build());

        // Create a subnet for the AZ within the regional VPC
        var exampleSubnet = new Subnet("exampleSubnet", SubnetArgs.builder()
            .vpcId(exampleVpc.id())
            .cidrBlock(exampleVpc.cidrBlock().applyValue(cidrBlock -> StdFunctions.cidrsubnet()).applyValue(invoke -> invoke.result()))
            .build());

    }
}
```
<!--End PulumiCodeChooser -->
�
allAvailabilityZonesB
 bSet to `true` to include all Availability Zones and Local Zones regardless of your opt in status.
�
filtersbB`*^:\
ZgetAvailabilityZoneFilter=aws:index/getAvailabilityZoneFilter:getAvailabilityZoneFilter6Configuration block(s) for filtering. Detailed below.
<
nameB" .Full name of the availability zone to select.
z
stateB" kSpecific availability zone state to require. May be any of `"available"`, `"information"` or `"impaired"`.
<
zoneIdB" ,Zone ID of the availability zone to select.
"
allAvailabilityZonesB
 "m
filtersbB`*^:\
ZgetAvailabilityZoneFilter=aws:index/getAvailabilityZoneFilter:getAvailabilityZoneFilter"�
	groupName" �For Availability Zones, this is the same value as the Region name. For Local Zones, the name of the associated group, for example `us-west-2-lax-1`.
"E
id" ;The provider-assigned unique ID for this managed resource.
"

name" "�

nameSuffix" �Part of the AZ name that appears after the region name, uniquely identifying the AZ within its region.
For Availability Zones this is usually a single letter, for example `a` for the `us-west-2a` zone.
For Local and Wavelength Zones this is a longer string, for example `wl1-sfo-wlz-1` for the `us-west-2-wl1-sfo-wlz-1` zone.
"Y
networkBorderGroup" ?The name of the location from which the address is advertised.
"�
optInStatus" �For Availability Zones, this always has the value of `opt-in-not-required`. For Local Zones, this is the opt in status. The possible values are `opted-in` and `not-opted-in`.
"�
parentZoneId" sID of the zone that handles some of the Local Zone or Wavelength Zone control plane operations, such as API calls.
"�
parentZoneName" uName of the zone that handles some of the Local Zone or Wavelength Zone control plane operations, such as API calls.
"�
region" �Region where the selected availability zone resides. This is always the region selected on the provider, since this data source searches only within that region.
"
state" "
zoneId" "c
zoneType" SType of zone. Values are `availability-zone`, `local-zone`, and `wavelength-zone`.
2�\
KgetAvailabilityZones3aws:index/getAvailabilityZones:getAvailabilityZones�PThe Availability Zones data source allows access to the list of AWS
Availability Zones which can be accessed by an AWS account within the region
configured in the provider.

This is different from the `aws.getAvailabilityZone` (singular) data source,
which provides some details about a specific availability zone.

> When [Local Zones](https://aws.amazon.com/about-aws/global-infrastructure/localzones/) are enabled in a region, by default the API and this data source include both Local Zones and Availability Zones. To return only Availability Zones, see the example section below.

## Example Usage

### By State

<!--Start PulumiCodeChooser -->
```typescript
import * as pulumi from "@pulumi/pulumi";
import * as aws from "@pulumi/aws";

// Declare the data source
const available = aws.getAvailabilityZones({
    state: "available",
});
// e.g., Create subnets in the first two available availability zones
const primary = new aws.ec2.Subnet("primary", {availabilityZone: available.then(available => available.names?.[0])});
const secondary = new aws.ec2.Subnet("secondary", {availabilityZone: available.then(available => available.names?.[1])});
```
```python
import pulumi
import pulumi_aws as aws

# Declare the data source
available = aws.get_availability_zones(state="available")
# e.g., Create subnets in the first two available availability zones
primary = aws.ec2.Subnet("primary", availability_zone=available.names[0])
secondary = aws.ec2.Subnet("secondary", availability_zone=available.names[1])
```
```csharp
using System.Collections.Generic;
using System.Linq;
using Pulumi;
using Aws = Pulumi.Aws;

return await Deployment.RunAsync(() => 
{
    // Declare the data source
    var available = Aws.GetAvailabilityZones.Invoke(new()
    {
        State = "available",
    });

    // e.g., Create subnets in the first two available availability zones
    var primary = new Aws.Ec2.Subnet("primary", new()
    {
        AvailabilityZone = available.Apply(getAvailabilityZonesResult => getAvailabilityZonesResult.Names[0]),
    });

    var secondary = new Aws.Ec2.Subnet("secondary", new()
    {
        AvailabilityZone = available.Apply(getAvailabilityZonesResult => getAvailabilityZonesResult.Names[1]),
    });

});
```
```go
package main

import (
	"github.com/pulumi/pulumi-aws/sdk/v6/go/aws"
	"github.com/pulumi/pulumi-aws/sdk/v6/go/aws/ec2"
	"github.com/pulumi/pulumi/sdk/v3/go/pulumi"
)

func main() {
	pulumi.Run(func(ctx *pulumi.Context) error {
		// Declare the data source
		available, err := aws.GetAvailabilityZones(ctx, &aws.GetAvailabilityZonesArgs{
			State: pulumi.StringRef("available"),
		}, nil)
		if err != nil {
			return err
		}
		// e.g., Create subnets in the first two available availability zones
		_, err = ec2.NewSubnet(ctx, "primary", &ec2.SubnetArgs{
			AvailabilityZone: pulumi.String(available.Names[0]),
		})
		if err != nil {
			return err
		}
		_, err = ec2.NewSubnet(ctx, "secondary", &ec2.SubnetArgs{
			AvailabilityZone: pulumi.String(available.Names[1]),
		})
		if err != nil {
			return err
		}
		return nil
	})
}
```
```java
package generated_program;

import com.pulumi.Context;
import com.pulumi.Pulumi;
import com.pulumi.core.Output;
import com.pulumi.aws.AwsFunctions;
import com.pulumi.aws.inputs.GetAvailabilityZonesArgs;
import com.pulumi.aws.ec2.Subnet;
import com.pulumi.aws.ec2.SubnetArgs;
import java.util.List;
import java.util.ArrayList;
import java.util.Map;
import java.io.File;
import java.nio.file.Files;
import java.nio.file.Paths;

public class App {
    public static void main(String[] args) {
        Pulumi.run(App::stack);
    }

    public static void stack(Context ctx) {
        // Declare the data source
        final var available = AwsFunctions.getAvailabilityZones(GetAvailabilityZonesArgs.builder()
            .state("available")
            .build());

        // e.g., Create subnets in the first two available availability zones
        var primary = new Subnet("primary", SubnetArgs.builder()
            .availabilityZone(available.applyValue(getAvailabilityZonesResult -> getAvailabilityZonesResult.names()[0]))
            .build());

        var secondary = new Subnet("secondary", SubnetArgs.builder()
            .availabilityZone(available.applyValue(getAvailabilityZonesResult -> getAvailabilityZonesResult.names()[1]))
            .build());

    }
}
```
```yaml
resources:
  # e.g., Create subnets in the first two available availability zones
  primary:
    type: aws:ec2:Subnet
    properties:
      availabilityZone: ${available.names[0]}
  secondary:
    type: aws:ec2:Subnet
    properties:
      availabilityZone: ${available.names[1]}
variables:
  # Declare the data source
  available:
    fn::invoke:
      function: aws:getAvailabilityZones
      arguments:
        state: available
```
<!--End PulumiCodeChooser -->

### By Filter

All Local Zones (regardless of opt-in status):

<!--Start PulumiCodeChooser -->
```typescript
import * as pulumi from "@pulumi/pulumi";
import * as aws from "@pulumi/aws";

const example = aws.getAvailabilityZones({
    allAvailabilityZones: true,
    filters: [{
        name: "opt-in-status",
        values: [
            "not-opted-in",
            "opted-in",
        ],
    }],
});
```
```python
import pulumi
import pulumi_aws as aws

example = aws.get_availability_zones(all_availability_zones=True,
    filters=[{
        "name": "opt-in-status",
        "values": [
            "not-opted-in",
            "opted-in",
        ],
    }])
```
```csharp
using System.Collections.Generic;
using System.Linq;
using Pulumi;
using Aws = Pulumi.Aws;

return await Deployment.RunAsync(() => 
{
    var example = Aws.GetAvailabilityZones.Invoke(new()
    {
        AllAvailabilityZones = true,
        Filters = new[]
        {
            new Aws.Inputs.GetAvailabilityZonesFilterInputArgs
            {
                Name = "opt-in-status",
                Values = new[]
                {
                    "not-opted-in",
                    "opted-in",
                },
            },
        },
    });

});
```
```go
package main

import (
	"github.com/pulumi/pulumi-aws/sdk/v6/go/aws"
	"github.com/pulumi/pulumi/sdk/v3/go/pulumi"
)

func main() {
	pulumi.Run(func(ctx *pulumi.Context) error {
		_, err := aws.GetAvailabilityZones(ctx, &aws.GetAvailabilityZonesArgs{
			AllAvailabilityZones: pulumi.BoolRef(true),
			Filters: []aws.GetAvailabilityZonesFilter{
				{
					Name: "opt-in-status",
					Values: []string{
						"not-opted-in",
						"opted-in",
					},
				},
			},
		}, nil)
		if err != nil {
			return err
		}
		return nil
	})
}
```
```java
package generated_program;

import com.pulumi.Context;
import com.pulumi.Pulumi;
import com.pulumi.core.Output;
import com.pulumi.aws.AwsFunctions;
import com.pulumi.aws.inputs.GetAvailabilityZonesArgs;
import java.util.List;
import java.util.ArrayList;
import java.util.Map;
import java.io.File;
import java.nio.file.Files;
import java.nio.file.Paths;

public class App {
    public static void main(String[] args) {
        Pulumi.run(App::stack);
    }

    public static void stack(Context ctx) {
        final var example = AwsFunctions.getAvailabilityZones(GetAvailabilityZonesArgs.builder()
            .allAvailabilityZones(true)
            .filters(GetAvailabilityZonesFilterArgs.builder()
                .name("opt-in-status")
                .values(                
                    "not-opted-in",
                    "opted-in")
                .build())
            .build());

    }
}
```
```yaml
variables:
  example:
    fn::invoke:
      function: aws:getAvailabilityZones
      arguments:
        allAvailabilityZones: true
        filters:
          - name: opt-in-status
            values:
              - not-opted-in
              - opted-in
```
<!--End PulumiCodeChooser -->

Only Availability Zones (no Local Zones):

<!--Start PulumiCodeChooser -->
```typescript
import * as pulumi from "@pulumi/pulumi";
import * as aws from "@pulumi/aws";

const example = aws.getAvailabilityZones({
    filters: [{
        name: "opt-in-status",
        values: ["opt-in-not-required"],
    }],
});
```
```python
import pulumi
import pulumi_aws as aws

example = aws.get_availability_zones(filters=[{
    "name": "opt-in-status",
    "values": ["opt-in-not-required"],
}])
```
```csharp
using System.Collections.Generic;
using System.Linq;
using Pulumi;
using Aws = Pulumi.Aws;

return await Deployment.RunAsync(() => 
{
    var example = Aws.GetAvailabilityZones.Invoke(new()
    {
        Filters = new[]
        {
            new Aws.Inputs.GetAvailabilityZonesFilterInputArgs
            {
                Name = "opt-in-status",
                Values = new[]
                {
                    "opt-in-not-required",
                },
            },
        },
    });

});
```
```go
package main

import (
	"github.com/pulumi/pulumi-aws/sdk/v6/go/aws"
	"github.com/pulumi/pulumi/sdk/v3/go/pulumi"
)

func main() {
	pulumi.Run(func(ctx *pulumi.Context) error {
		_, err := aws.GetAvailabilityZones(ctx, &aws.GetAvailabilityZonesArgs{
			Filters: []aws.GetAvailabilityZonesFilter{
				{
					Name: "opt-in-status",
					Values: []string{
						"opt-in-not-required",
					},
				},
			},
		}, nil)
		if err != nil {
			return err
		}
		return nil
	})
}
```
```java
package generated_program;

import com.pulumi.Context;
import com.pulumi.Pulumi;
import com.pulumi.core.Output;
import com.pulumi.aws.AwsFunctions;
import com.pulumi.aws.inputs.GetAvailabilityZonesArgs;
import java.util.List;
import java.util.ArrayList;
import java.util.Map;
import java.io.File;
import java.nio.file.Files;
import java.nio.file.Paths;

public class App {
    public static void main(String[] args) {
        Pulumi.run(App::stack);
    }

    public static void stack(Context ctx) {
        final var example = AwsFunctions.getAvailabilityZones(GetAvailabilityZonesArgs.builder()
            .filters(GetAvailabilityZonesFilterArgs.builder()
                .name("opt-in-status")
                .values("opt-in-not-required")
                .build())
            .build());

    }
}
```
```yaml
variables:
  example:
    fn::invoke:
      function: aws:getAvailabilityZones
      arguments:
        filters:
          - name: opt-in-status
            values:
              - opt-in-not-required
```
<!--End PulumiCodeChooser -->
�
allAvailabilityZonesB
 bSet to `true` to include all Availability Zones and Local Zones regardless of your opt in status.
D
excludeNamesB*" ,List of Availability Zone names to exclude.
D
excludeZoneIdsB*" *List of Availability Zone IDs to exclude.
�
filterseBc*a:_
]getAvailabilityZonesFilter?aws:index/getAvailabilityZonesFilter:getAvailabilityZonesFilter6Configuration block(s) for filtering. Detailed below.
�
stateB" �Allows to filter list of Availability Zones based on their
current state. Can be either `"available"`, `"information"`, `"impaired"` or
`"unavailable"`. By default the list includes a complete set of Availability Zones
to which the underlying AWS account has access, regardless of their state.
"
allAvailabilityZonesB
 "
excludeNamesB*" "
excludeZoneIdsB*" "p
filterseBc*a:_
]getAvailabilityZonesFilter?aws:index/getAvailabilityZonesFilter:getAvailabilityZonesFilter"�

groupNames*" �A set of the Availability Zone Group names. For Availability Zones, this is the same value as the Region name. For Local Zones, the name of the associated group, for example `us-west-2-lax-1`.
"E
id" ;The provider-assigned unique ID for this managed resource.
"M
names*" >List of the Availability Zone names available to the account.
"
stateB" "M
zoneIds*" <List of the Availability Zone IDs available to the account.
2�[
WgetBillingServiceAccount;aws:index/getBillingServiceAccount:getBillingServiceAccount�YUse this data source to get the Account ID of the [AWS Billing and Cost Management Service Account](http://docs.aws.amazon.com/awsaccountbilling/latest/aboutv2/billing-getting-started.html#step-2) for the purpose of permitting in S3 bucket policy.

## Example Usage

<!--Start PulumiCodeChooser -->
```typescript
import * as pulumi from "@pulumi/pulumi";
import * as aws from "@pulumi/aws";

const main = aws.getBillingServiceAccount({});
const billingLogs = new aws.s3.BucketV2("billing_logs", {bucket: "my-billing-tf-test-bucket"});
const billingLogsAcl = new aws.s3.BucketAclV2("billing_logs_acl", {
    bucket: billingLogs.id,
    acl: "private",
});
const allowBillingLogging = pulumi.all([main, billingLogs.arn, main, billingLogs.arn]).apply(([main, billingLogsArn, main1, billingLogsArn1]) => aws.iam.getPolicyDocumentOutput({
    statements: [
        {
            effect: "Allow",
            principals: [{
                type: "AWS",
                identifiers: [main.arn],
            }],
            actions: [
                "s3:GetBucketAcl",
                "s3:GetBucketPolicy",
            ],
            resources: [billingLogsArn],
        },
        {
            effect: "Allow",
            principals: [{
                type: "AWS",
                identifiers: [main1.arn],
            }],
            actions: ["s3:PutObject"],
            resources: [`${billingLogsArn1}/*`],
        },
    ],
}));
const allowBillingLoggingBucketPolicy = new aws.s3.BucketPolicy("allow_billing_logging", {
    bucket: billingLogs.id,
    policy: allowBillingLogging.apply(allowBillingLogging => allowBillingLogging.json),
});
```
```python
import pulumi
import pulumi_aws as aws

main = aws.get_billing_service_account()
billing_logs = aws.s3.BucketV2("billing_logs", bucket="my-billing-tf-test-bucket")
billing_logs_acl = aws.s3.BucketAclV2("billing_logs_acl",
    bucket=billing_logs.id,
    acl="private")
allow_billing_logging = pulumi.Output.all(
    billingLogsArn=billing_logs.arn,
    billingLogsArn1=billing_logs.arn
).apply(lambda resolved_outputs: aws.iam.get_policy_document_output(statements=[
    {
        "effect": "Allow",
        "principals": [{
            "type": "AWS",
            "identifiers": [main.arn],
        }],
        "actions": [
            "s3:GetBucketAcl",
            "s3:GetBucketPolicy",
        ],
        "resources": [resolved_outputs['billingLogsArn']],
    },
    {
        "effect": "Allow",
        "principals": [{
            "type": "AWS",
            "identifiers": [main.arn],
        }],
        "actions": ["s3:PutObject"],
        "resources": [f"{resolved_outputs['billingLogsArn1']}/*"],
    },
]))

allow_billing_logging_bucket_policy = aws.s3.BucketPolicy("allow_billing_logging",
    bucket=billing_logs.id,
    policy=allow_billing_logging.json)
```
```csharp
using System.Collections.Generic;
using System.Linq;
using Pulumi;
using Aws = Pulumi.Aws;

return await Deployment.RunAsync(() => 
{
    var main = Aws.GetBillingServiceAccount.Invoke();

    var billingLogs = new Aws.S3.BucketV2("billing_logs", new()
    {
        Bucket = "my-billing-tf-test-bucket",
    });

    var billingLogsAcl = new Aws.S3.BucketAclV2("billing_logs_acl", new()
    {
        Bucket = billingLogs.Id,
        Acl = "private",
    });

    var allowBillingLogging = Aws.Iam.GetPolicyDocument.Invoke(new()
    {
        Statements = new[]
        {
            new Aws.Iam.Inputs.GetPolicyDocumentStatementInputArgs
            {
                Effect = "Allow",
                Principals = new[]
                {
                    new Aws.Iam.Inputs.GetPolicyDocumentStatementPrincipalInputArgs
                    {
                        Type = "AWS",
                        Identifiers = new[]
                        {
                            main.Apply(getBillingServiceAccountResult => getBillingServiceAccountResult.Arn),
                        },
                    },
                },
                Actions = new[]
                {
                    "s3:GetBucketAcl",
                    "s3:GetBucketPolicy",
                },
                Resources = new[]
                {
                    billingLogs.Arn,
                },
            },
            new Aws.Iam.Inputs.GetPolicyDocumentStatementInputArgs
            {
                Effect = "Allow",
                Principals = new[]
                {
                    new Aws.Iam.Inputs.GetPolicyDocumentStatementPrincipalInputArgs
                    {
                        Type = "AWS",
                        Identifiers = new[]
                        {
                            main.Apply(getBillingServiceAccountResult => getBillingServiceAccountResult.Arn),
                        },
                    },
                },
                Actions = new[]
                {
                    "s3:PutObject",
                },
                Resources = new[]
                {
                    $"{billingLogs.Arn}/*",
                },
            },
        },
    });

    var allowBillingLoggingBucketPolicy = new Aws.S3.BucketPolicy("allow_billing_logging", new()
    {
        Bucket = billingLogs.Id,
        Policy = allowBillingLogging.Apply(getPolicyDocumentResult => getPolicyDocumentResult.Json),
    });

});
```
```go
package main

import (
	"fmt"

	"github.com/pulumi/pulumi-aws/sdk/v6/go/aws"
	"github.com/pulumi/pulumi-aws/sdk/v6/go/aws/iam"
	"github.com/pulumi/pulumi-aws/sdk/v6/go/aws/s3"
	"github.com/pulumi/pulumi/sdk/v3/go/pulumi"
)
func main() {
pulumi.Run(func(ctx *pulumi.Context) error {
main, err := aws.GetBillingServiceAccount(ctx, &aws.GetBillingServiceAccountArgs{
}, nil);
if err != nil {
return err
}
billingLogs, err := s3.NewBucketV2(ctx, "billing_logs", &s3.BucketV2Args{
Bucket: pulumi.String("my-billing-tf-test-bucket"),
})
if err != nil {
return err
}
_, err = s3.NewBucketAclV2(ctx, "billing_logs_acl", &s3.BucketAclV2Args{
Bucket: billingLogs.ID(),
Acl: pulumi.String("private"),
})
if err != nil {
return err
}
allowBillingLogging := pulumi.All(billingLogs.Arn,billingLogs.Arn).ApplyT(func(_args []interface{}) (iam.GetPolicyDocumentResult, error) {
billingLogsArn := _args[0].(string)
billingLogsArn1 := _args[1].(string)
return iam.GetPolicyDocumentResult(interface{}(iam.GetPolicyDocumentOutput(ctx, iam.GetPolicyDocumentOutputArgs{
Statements: []iam.GetPolicyDocumentStatement{
{
Effect: "Allow",
Principals: []iam.GetPolicyDocumentStatementPrincipal{
{
Type: "AWS",
Identifiers: interface{}{
main.Arn,
},
},
},
Actions: []string{
"s3:GetBucketAcl",
"s3:GetBucketPolicy",
},
Resources: []string{
billingLogsArn,
},
},
{
Effect: "Allow",
Principals: []iam.GetPolicyDocumentStatementPrincipal{
{
Type: "AWS",
Identifiers: interface{}{
main.Arn,
},
},
},
Actions: []string{
"s3:PutObject",
},
Resources: []string{
fmt.Sprintf("%v/*", billingLogsArn1),
},
},
},
}, nil))), nil
}).(iam.GetPolicyDocumentResultOutput)
_, err = s3.NewBucketPolicy(ctx, "allow_billing_logging", &s3.BucketPolicyArgs{
Bucket: billingLogs.ID(),
Policy: pulumi.String(allowBillingLogging.ApplyT(func(allowBillingLogging iam.GetPolicyDocumentResult) (*string, error) {
return &allowBillingLogging.Json, nil
}).(pulumi.StringPtrOutput)),
})
if err != nil {
return err
}
return nil
})
}
```
```java
package generated_program;

import com.pulumi.Context;
import com.pulumi.Pulumi;
import com.pulumi.core.Output;
import com.pulumi.aws.AwsFunctions;
import com.pulumi.aws.inputs.GetBillingServiceAccountArgs;
import com.pulumi.aws.s3.BucketV2;
import com.pulumi.aws.s3.BucketV2Args;
import com.pulumi.aws.s3.BucketAclV2;
import com.pulumi.aws.s3.BucketAclV2Args;
import com.pulumi.aws.iam.IamFunctions;
import com.pulumi.aws.iam.inputs.GetPolicyDocumentArgs;
import com.pulumi.aws.s3.BucketPolicy;
import com.pulumi.aws.s3.BucketPolicyArgs;
import java.util.List;
import java.util.ArrayList;
import java.util.Map;
import java.io.File;
import java.nio.file.Files;
import java.nio.file.Paths;

public class App {
    public static void main(String[] args) {
        Pulumi.run(App::stack);
    }

    public static void stack(Context ctx) {
        final var main = AwsFunctions.getBillingServiceAccount();

        var billingLogs = new BucketV2("billingLogs", BucketV2Args.builder()
            .bucket("my-billing-tf-test-bucket")
            .build());

        var billingLogsAcl = new BucketAclV2("billingLogsAcl", BucketAclV2Args.builder()
            .bucket(billingLogs.id())
            .acl("private")
            .build());

        final var allowBillingLogging = IamFunctions.getPolicyDocument(GetPolicyDocumentArgs.builder()
            .statements(            
                GetPolicyDocumentStatementArgs.builder()
                    .effect("Allow")
                    .principals(GetPolicyDocumentStatementPrincipalArgs.builder()
                        .type("AWS")
                        .identifiers(main.applyValue(getBillingServiceAccountResult -> getBillingServiceAccountResult.arn()))
                        .build())
                    .actions(                    
                        "s3:GetBucketAcl",
                        "s3:GetBucketPolicy")
                    .resources(billingLogs.arn())
                    .build(),
                GetPolicyDocumentStatementArgs.builder()
                    .effect("Allow")
                    .principals(GetPolicyDocumentStatementPrincipalArgs.builder()
                        .type("AWS")
                        .identifiers(main.applyValue(getBillingServiceAccountResult -> getBillingServiceAccountResult.arn()))
                        .build())
                    .actions("s3:PutObject")
                    .resources(billingLogs.arn().applyValue(arn -> String.format("%s/*", arn)))
                    .build())
            .build());

        var allowBillingLoggingBucketPolicy = new BucketPolicy("allowBillingLoggingBucketPolicy", BucketPolicyArgs.builder()
            .bucket(billingLogs.id())
            .policy(allowBillingLogging.applyValue(getPolicyDocumentResult -> getPolicyDocumentResult).applyValue(allowBillingLogging -> allowBillingLogging.applyValue(getPolicyDocumentResult -> getPolicyDocumentResult.json())))
            .build());

    }
}
```
```yaml
resources:
  billingLogs:
    type: aws:s3:BucketV2
    name: billing_logs
    properties:
      bucket: my-billing-tf-test-bucket
  billingLogsAcl:
    type: aws:s3:BucketAclV2
    name: billing_logs_acl
    properties:
      bucket: ${billingLogs.id}
      acl: private
  allowBillingLoggingBucketPolicy:
    type: aws:s3:BucketPolicy
    name: allow_billing_logging
    properties:
      bucket: ${billingLogs.id}
      policy: ${allowBillingLogging.json}
variables:
  main:
    fn::invoke:
      function: aws:getBillingServiceAccount
      arguments: {}
  allowBillingLogging:
    fn::invoke:
      function: aws:iam:getPolicyDocument
      arguments:
        statements:
          - effect: Allow
            principals:
              - type: AWS
                identifiers:
                  - ${main.arn}
            actions:
              - s3:GetBucketAcl
              - s3:GetBucketPolicy
            resources:
              - ${billingLogs.arn}
          - effect: Allow
            principals:
              - type: AWS
                identifiers:
                  - ${main.arn}
            actions:
              - s3:PutObject
            resources:
              - ${billingLogs.arn}/*
```
<!--End PulumiCodeChooser -->
3
idB" 'ID of the AWS billing service account.
"3
arn" (ARN of the AWS billing service account.
"1
id" 'ID of the AWS billing service account.
2�
BgetCallerIdentity-aws:index/getCallerIdentity:getCallerIdentity�Use this data source to get the access to the effective Account ID, User ID, and ARN in
which this provider is authorized.

## Example Usage

<!--Start PulumiCodeChooser -->
```typescript
import * as pulumi from "@pulumi/pulumi";
import * as aws from "@pulumi/aws";

const current = aws.getCallerIdentity({});
export const accountId = current.then(current => current.accountId);
export const callerArn = current.then(current => current.arn);
export const callerUser = current.then(current => current.userId);
```
```python
import pulumi
import pulumi_aws as aws

current = aws.get_caller_identity()
pulumi.export("accountId", current.account_id)
pulumi.export("callerArn", current.arn)
pulumi.export("callerUser", current.user_id)
```
```csharp
using System.Collections.Generic;
using System.Linq;
using Pulumi;
using Aws = Pulumi.Aws;

return await Deployment.RunAsync(() => 
{
    var current = Aws.GetCallerIdentity.Invoke();

    return new Dictionary<string, object?>
    {
        ["accountId"] = current.Apply(getCallerIdentityResult => getCallerIdentityResult.AccountId),
        ["callerArn"] = current.Apply(getCallerIdentityResult => getCallerIdentityResult.Arn),
        ["callerUser"] = current.Apply(getCallerIdentityResult => getCallerIdentityResult.UserId),
    };
});
```
```go
package main

import (
	"github.com/pulumi/pulumi-aws/sdk/v6/go/aws"
	"github.com/pulumi/pulumi/sdk/v3/go/pulumi"
)

func main() {
	pulumi.Run(func(ctx *pulumi.Context) error {
		current, err := aws.GetCallerIdentity(ctx, &aws.GetCallerIdentityArgs{}, nil)
		if err != nil {
			return err
		}
		ctx.Export("accountId", current.AccountId)
		ctx.Export("callerArn", current.Arn)
		ctx.Export("callerUser", current.UserId)
		return nil
	})
}
```
```java
package generated_program;

import com.pulumi.Context;
import com.pulumi.Pulumi;
import com.pulumi.core.Output;
import com.pulumi.aws.AwsFunctions;
import com.pulumi.aws.inputs.GetCallerIdentityArgs;
import java.util.List;
import java.util.ArrayList;
import java.util.Map;
import java.io.File;
import java.nio.file.Files;
import java.nio.file.Paths;

public class App {
    public static void main(String[] args) {
        Pulumi.run(App::stack);
    }

    public static void stack(Context ctx) {
        final var current = AwsFunctions.getCallerIdentity();

        ctx.export("accountId", current.applyValue(getCallerIdentityResult -> getCallerIdentityResult.accountId()));
        ctx.export("callerArn", current.applyValue(getCallerIdentityResult -> getCallerIdentityResult.arn()));
        ctx.export("callerUser", current.applyValue(getCallerIdentityResult -> getCallerIdentityResult.userId()));
    }
}
```
```yaml
variables:
  current:
    fn::invoke:
      function: aws:getCallerIdentity
      arguments: {}
outputs:
  accountId: ${current.accountId}
  callerArn: ${current.arn}
  callerUser: ${current.userId}
```
<!--End PulumiCodeChooser -->
W
idB" KAccount ID number of the account that owns or contains the calling entity.
"`
	accountId" OAWS Account ID number of the account that owns or contains the calling entity.
"3
arn" (ARN associated with the calling entity.
"U
id" KAccount ID number of the account that owns or contains the calling entity.
"7
userId" )Unique identifier of the calling entity.
2�
9getDefaultTags'aws:index/getDefaultTags:getDefaultTags�Use this data source to get the default tags configured on the provider.

With this data source, you can apply default tags to resources not _directly_ managed by a resource, such as the instances underneath an Auto Scaling group or the volumes created for an EC2 instance.

## Example Usage

### Basic Usage

<!--Start PulumiCodeChooser -->
```typescript
import * as pulumi from "@pulumi/pulumi";
import * as aws from "@pulumi/aws";

const example = aws.getDefaultTags({});
```
```python
import pulumi
import pulumi_aws as aws

example = aws.get_default_tags()
```
```csharp
using System.Collections.Generic;
using System.Linq;
using Pulumi;
using Aws = Pulumi.Aws;

return await Deployment.RunAsync(() => 
{
    var example = Aws.GetDefaultTags.Invoke();

});
```
```go
package main

import (
	"github.com/pulumi/pulumi-aws/sdk/v6/go/aws"
	"github.com/pulumi/pulumi/sdk/v3/go/pulumi"
)

func main() {
	pulumi.Run(func(ctx *pulumi.Context) error {
		_, err := aws.GetDefaultTags(ctx, &aws.GetDefaultTagsArgs{}, nil)
		if err != nil {
			return err
		}
		return nil
	})
}
```
```java
package generated_program;

import com.pulumi.Context;
import com.pulumi.Pulumi;
import com.pulumi.core.Output;
import com.pulumi.aws.AwsFunctions;
import com.pulumi.aws.inputs.GetDefaultTagsArgs;
import java.util.List;
import java.util.ArrayList;
import java.util.Map;
import java.io.File;
import java.nio.file.Files;
import java.nio.file.Paths;

public class App {
    public static void main(String[] args) {
        Pulumi.run(App::stack);
    }

    public static void stack(Context ctx) {
        final var example = AwsFunctions.getDefaultTags();

    }
}
```
```yaml
variables:
  example:
    fn::invoke:
      function: aws:getDefaultTags
      arguments: {}
```
<!--End PulumiCodeChooser -->

### Dynamically Apply Default Tags to Auto Scaling Group

<!--Start PulumiCodeChooser -->
```typescript
import * as pulumi from "@pulumi/pulumi";
import * as aws from "@pulumi/aws";

const example = aws.getDefaultTags({});
const exampleGroup = new aws.autoscaling.Group("example", {tags: .map(entry => ({
    key: entry.key,
    value: entry.value,
    propagateAtLaunch: true,
}))});
```
```python
import pulumi
import pulumi_aws as aws

example = aws.get_default_tags()
example_group = aws.autoscaling.Group("example", tags=[{
    "key": entry["key"],
    "value": entry["value"],
    "propagate_at_launch": True,
} for entry in [{"key": k, "value": v} for k, v in example.tags]])
```
```csharp
using System.Collections.Generic;
using System.Linq;
using Pulumi;
using Aws = Pulumi.Aws;

return await Deployment.RunAsync(() => 
{
    var example = Aws.GetDefaultTags.Invoke();

    var exampleGroup = new Aws.AutoScaling.Group("example", new()
    {
        Tags = ,
    });

});
```
<!--End PulumiCodeChooser -->


idB" "
id" ":
tags2" ,Key-value mapping of provider default tags.
2�;
0getIpRanges!aws:index/getIpRanges:getIpRanges�1Use this data source to get the IP ranges of various AWS products and services. For more information about the contents of this data source and required JSON syntax if referencing a custom URL, see the [AWS IP Address Ranges documentation](https://docs.aws.amazon.com/general/latest/gr/aws-ip-ranges.html).

## Example Usage

<!--Start PulumiCodeChooser -->
```typescript
import * as pulumi from "@pulumi/pulumi";
import * as aws from "@pulumi/aws";

const europeanEc2 = aws.getIpRanges({
    regions: [
        "eu-west-1",
        "eu-central-1",
    ],
    services: ["ec2"],
});
const fromEurope = new aws.ec2.SecurityGroup("from_europe", {
    name: "from_europe",
    ingress: [{
        fromPort: 443,
        toPort: 443,
        protocol: "tcp",
        cidrBlocks: europeanEc2.then(europeanEc2 => europeanEc2.cidrBlocks),
        ipv6CidrBlocks: europeanEc2.then(europeanEc2 => europeanEc2.ipv6CidrBlocks),
    }],
    tags: {
        CreateDate: europeanEc2.then(europeanEc2 => europeanEc2.createDate),
        SyncToken: europeanEc2.then(europeanEc2 => europeanEc2.syncToken),
    },
});
```
```python
import pulumi
import pulumi_aws as aws

european_ec2 = aws.get_ip_ranges(regions=[
        "eu-west-1",
        "eu-central-1",
    ],
    services=["ec2"])
from_europe = aws.ec2.SecurityGroup("from_europe",
    name="from_europe",
    ingress=[{
        "from_port": 443,
        "to_port": 443,
        "protocol": "tcp",
        "cidr_blocks": european_ec2.cidr_blocks,
        "ipv6_cidr_blocks": european_ec2.ipv6_cidr_blocks,
    }],
    tags={
        "CreateDate": european_ec2.create_date,
        "SyncToken": european_ec2.sync_token,
    })
```
```csharp
using System.Collections.Generic;
using System.Linq;
using Pulumi;
using Aws = Pulumi.Aws;

return await Deployment.RunAsync(() => 
{
    var europeanEc2 = Aws.GetIpRanges.Invoke(new()
    {
        Regions = new[]
        {
            "eu-west-1",
            "eu-central-1",
        },
        Services = new[]
        {
            "ec2",
        },
    });

    var fromEurope = new Aws.Ec2.SecurityGroup("from_europe", new()
    {
        Name = "from_europe",
        Ingress = new[]
        {
            new Aws.Ec2.Inputs.SecurityGroupIngressArgs
            {
                FromPort = 443,
                ToPort = 443,
                Protocol = "tcp",
                CidrBlocks = europeanEc2.Apply(getIpRangesResult => getIpRangesResult.CidrBlocks),
                Ipv6CidrBlocks = europeanEc2.Apply(getIpRangesResult => getIpRangesResult.Ipv6CidrBlocks),
            },
        },
        Tags = 
        {
            { "CreateDate", europeanEc2.Apply(getIpRangesResult => getIpRangesResult.CreateDate) },
            { "SyncToken", europeanEc2.Apply(getIpRangesResult => getIpRangesResult.SyncToken) },
        },
    });

});
```
```go
package main

import (
	"github.com/pulumi/pulumi-aws/sdk/v6/go/aws"
	"github.com/pulumi/pulumi-aws/sdk/v6/go/aws/ec2"
	"github.com/pulumi/pulumi/sdk/v3/go/pulumi"
)

func main() {
	pulumi.Run(func(ctx *pulumi.Context) error {
		europeanEc2, err := aws.GetIpRanges(ctx, &aws.GetIpRangesArgs{
			Regions: []string{
				"eu-west-1",
				"eu-central-1",
			},
			Services: []string{
				"ec2",
			},
		}, nil)
		if err != nil {
			return err
		}
		_, err = ec2.NewSecurityGroup(ctx, "from_europe", &ec2.SecurityGroupArgs{
			Name: pulumi.String("from_europe"),
			Ingress: ec2.SecurityGroupIngressArray{
				&ec2.SecurityGroupIngressArgs{
					FromPort:       pulumi.Int(443),
					ToPort:         pulumi.Int(443),
					Protocol:       pulumi.String("tcp"),
					CidrBlocks:     interface{}(europeanEc2.CidrBlocks),
					Ipv6CidrBlocks: interface{}(europeanEc2.Ipv6CidrBlocks),
				},
			},
			Tags: pulumi.StringMap{
				"CreateDate": pulumi.String(europeanEc2.CreateDate),
				"SyncToken":  pulumi.Int(europeanEc2.SyncToken),
			},
		})
		if err != nil {
			return err
		}
		return nil
	})
}
```
```java
package generated_program;

import com.pulumi.Context;
import com.pulumi.Pulumi;
import com.pulumi.core.Output;
import com.pulumi.aws.AwsFunctions;
import com.pulumi.aws.inputs.GetIpRangesArgs;
import com.pulumi.aws.ec2.SecurityGroup;
import com.pulumi.aws.ec2.SecurityGroupArgs;
import com.pulumi.aws.ec2.inputs.SecurityGroupIngressArgs;
import java.util.List;
import java.util.ArrayList;
import java.util.Map;
import java.io.File;
import java.nio.file.Files;
import java.nio.file.Paths;

public class App {
    public static void main(String[] args) {
        Pulumi.run(App::stack);
    }

    public static void stack(Context ctx) {
        final var europeanEc2 = AwsFunctions.getIpRanges(GetIpRangesArgs.builder()
            .regions(            
                "eu-west-1",
                "eu-central-1")
            .services("ec2")
            .build());

        var fromEurope = new SecurityGroup("fromEurope", SecurityGroupArgs.builder()
            .name("from_europe")
            .ingress(SecurityGroupIngressArgs.builder()
                .fromPort("443")
                .toPort("443")
                .protocol("tcp")
                .cidrBlocks(europeanEc2.applyValue(getIpRangesResult -> getIpRangesResult.cidrBlocks()))
                .ipv6CidrBlocks(europeanEc2.applyValue(getIpRangesResult -> getIpRangesResult.ipv6CidrBlocks()))
                .build())
            .tags(Map.ofEntries(
                Map.entry("CreateDate", europeanEc2.applyValue(getIpRangesResult -> getIpRangesResult.createDate())),
                Map.entry("SyncToken", europeanEc2.applyValue(getIpRangesResult -> getIpRangesResult.syncToken()))
            ))
            .build());

    }
}
```
```yaml
resources:
  fromEurope:
    type: aws:ec2:SecurityGroup
    name: from_europe
    properties:
      name: from_europe
      ingress:
        - fromPort: '443'
          toPort: '443'
          protocol: tcp
          cidrBlocks: ${europeanEc2.cidrBlocks}
          ipv6CidrBlocks: ${europeanEc2.ipv6CidrBlocks}
      tags:
        CreateDate: ${europeanEc2.createDate}
        SyncToken: ${europeanEc2.syncToken}
variables:
  europeanEc2:
    fn::invoke:
      function: aws:getIpRanges
      arguments:
        regions:
          - eu-west-1
          - eu-central-1
        services:
          - ec2
```
<!--End PulumiCodeChooser -->


idB" �
regionsB*" �Filter IP ranges by regions (or include all regions, if
omitted). Valid items are `global` (for `cloudfront`) as well as all AWS regions
(e.g., `eu-central-1`)
�
services*" �Filter IP ranges by services. Valid items are `amazon`
(for amazon.com), `amazon_connect`, `api_gateway`, `cloud9`, `cloudfront`,
`codebuild`, `dynamodb`, `ec2`, `ec2_instance_connect`, `globalaccelerator`,
`route53`, `route53_healthchecks`, `s3` and `workspaces_gateways`. See the
[`service` attribute][2] documentation for other possible values.

> **NOTE:** If the specified combination of regions and services does not yield any
CIDR blocks, this call will fail.
�
urlB" �Custom URL for source JSON file. Syntax must match [AWS IP Address Ranges documentation](https://docs.aws.amazon.com/general/latest/gr/aws-ip-ranges.html). Defaults to `https://ip-ranges.amazonaws.com/ip-ranges.json`.
";

cidrBlocks*" 'Lexically ordered list of CIDR blocks.
"S

createDate" APublication time of the IP ranges (e.g., `2016-08-03-23-46-05`).
"
id" "D
ipv6CidrBlocks*" ,Lexically ordered list of IPv6 CIDR blocks.
"
regionsB*" "
services*" "d
	syncToken SPublication time of the IP ranges, in Unix epoch time format
(e.g., `1470267965`).
"
urlB" 2�"
3getPartition#aws:index/getPartition:getPartition�Use this data source to lookup information about the current AWS partition in
which the provider is working.

## Example Usage

<!--Start PulumiCodeChooser -->
```typescript
import * as pulumi from "@pulumi/pulumi";
import * as aws from "@pulumi/aws";

const current = aws.getPartition({});
const s3Policy = current.then(current => aws.iam.getPolicyDocument({
    statements: [{
        sid: "1",
        actions: ["s3:ListBucket"],
        resources: [`arn:${current.partition}:s3:::my-bucket`],
    }],
}));
```
```python
import pulumi
import pulumi_aws as aws

current = aws.get_partition()
s3_policy = aws.iam.get_policy_document(statements=[{
    "sid": "1",
    "actions": ["s3:ListBucket"],
    "resources": [f"arn:{current.partition}:s3:::my-bucket"],
}])
```
```csharp
using System.Collections.Generic;
using System.Linq;
using Pulumi;
using Aws = Pulumi.Aws;

return await Deployment.RunAsync(() => 
{
    var current = Aws.GetPartition.Invoke();

    var s3Policy = Aws.Iam.GetPolicyDocument.Invoke(new()
    {
        Statements = new[]
        {
            new Aws.Iam.Inputs.GetPolicyDocumentStatementInputArgs
            {
                Sid = "1",
                Actions = new[]
                {
                    "s3:ListBucket",
                },
                Resources = new[]
                {
                    $"arn:{current.Apply(getPartitionResult => getPartitionResult.Partition)}:s3:::my-bucket",
                },
            },
        },
    });

});
```
```go
package main

import (
	"fmt"

	"github.com/pulumi/pulumi-aws/sdk/v6/go/aws"
	"github.com/pulumi/pulumi-aws/sdk/v6/go/aws/iam"
	"github.com/pulumi/pulumi/sdk/v3/go/pulumi"
)

func main() {
	pulumi.Run(func(ctx *pulumi.Context) error {
		current, err := aws.GetPartition(ctx, &aws.GetPartitionArgs{}, nil)
		if err != nil {
			return err
		}
		_, err = iam.GetPolicyDocument(ctx, &iam.GetPolicyDocumentArgs{
			Statements: []iam.GetPolicyDocumentStatement{
				{
					Sid: pulumi.StringRef("1"),
					Actions: []string{
						"s3:ListBucket",
					},
					Resources: []string{
						fmt.Sprintf("arn:%v:s3:::my-bucket", current.Partition),
					},
				},
			},
		}, nil)
		if err != nil {
			return err
		}
		return nil
	})
}
```
```java
package generated_program;

import com.pulumi.Context;
import com.pulumi.Pulumi;
import com.pulumi.core.Output;
import com.pulumi.aws.AwsFunctions;
import com.pulumi.aws.inputs.GetPartitionArgs;
import com.pulumi.aws.iam.IamFunctions;
import com.pulumi.aws.iam.inputs.GetPolicyDocumentArgs;
import java.util.List;
import java.util.ArrayList;
import java.util.Map;
import java.io.File;
import java.nio.file.Files;
import java.nio.file.Paths;

public class App {
    public static void main(String[] args) {
        Pulumi.run(App::stack);
    }

    public static void stack(Context ctx) {
        final var current = AwsFunctions.getPartition();

        final var s3Policy = IamFunctions.getPolicyDocument(GetPolicyDocumentArgs.builder()
            .statements(GetPolicyDocumentStatementArgs.builder()
                .sid("1")
                .actions("s3:ListBucket")
                .resources(String.format("arn:%s:s3:::my-bucket", current.applyValue(getPartitionResult -> getPartitionResult.partition())))
                .build())
            .build());

    }
}
```
```yaml
variables:
  current:
    fn::invoke:
      function: aws:getPartition
      arguments: {}
  s3Policy:
    fn::invoke:
      function: aws:iam:getPolicyDocument
      arguments:
        statements:
          - sid: '1'
            actions:
              - s3:ListBucket
            resources:
              - arn:${current.partition}:s3:::my-bucket
```
<!--End PulumiCodeChooser -->
h
idB" \Identifier of the current partition (e.g., `aws` in AWS Commercial, `aws-cn` in AWS China).
"�
	dnsSuffix" {Base DNS domain name for the current partition (e.g., `amazonaws.com` in AWS Commercial, `amazonaws.com.cn` in AWS China).
"f
id" \Identifier of the current partition (e.g., `aws` in AWS Commercial, `aws-cn` in AWS China).
"m
	partition" \Identifier of the current partition (e.g., `aws` in AWS Commercial, `aws-cn` in AWS China).
"|
reverseDnsPrefix" dPrefix of service names (e.g., `com.amazonaws` in AWS Commercial, `cn.com.amazonaws` in AWS China).
2�
*	getRegionaws:index/getRegion:getRegion�`aws.getRegion` provides details about a specific AWS region.

As well as validating a given region name this resource can be used to
discover the name of the region configured within the provider. The latter
can be useful in a child module which is inheriting an AWS provider
configuration from its parent module.

## Example Usage

The following example shows how the resource might be used to obtain
the name of the AWS region configured on the provider.

<!--Start PulumiCodeChooser -->
```typescript
import * as pulumi from "@pulumi/pulumi";
import * as aws from "@pulumi/aws";

const current = aws.getRegion({});
```
```python
import pulumi
import pulumi_aws as aws

current = aws.get_region()
```
```csharp
using System.Collections.Generic;
using System.Linq;
using Pulumi;
using Aws = Pulumi.Aws;

return await Deployment.RunAsync(() => 
{
    var current = Aws.GetRegion.Invoke();

});
```
```go
package main

import (
	"github.com/pulumi/pulumi-aws/sdk/v6/go/aws"
	"github.com/pulumi/pulumi/sdk/v3/go/pulumi"
)

func main() {
	pulumi.Run(func(ctx *pulumi.Context) error {
		_, err := aws.GetRegion(ctx, &aws.GetRegionArgs{}, nil)
		if err != nil {
			return err
		}
		return nil
	})
}
```
```java
package generated_program;

import com.pulumi.Context;
import com.pulumi.Pulumi;
import com.pulumi.core.Output;
import com.pulumi.aws.AwsFunctions;
import com.pulumi.aws.inputs.GetRegionArgs;
import java.util.List;
import java.util.ArrayList;
import java.util.Map;
import java.io.File;
import java.nio.file.Files;
import java.nio.file.Paths;

public class App {
    public static void main(String[] args) {
        Pulumi.run(App::stack);
    }

    public static void stack(Context ctx) {
        final var current = AwsFunctions.getRegion();

    }
}
```
```yaml
variables:
  current:
    fn::invoke:
      function: aws:getRegion
      arguments: {}
```
<!--End PulumiCodeChooser -->
8
endpointB" &EC2 endpoint of the region to select.


idB" 1
nameB" #Full name of the region to select.
"R
description" ?Region's description in this format: "Location (Region name)".
"6
endpoint" &EC2 endpoint for the selected region.
"
id" ")
name" Name of the selected region.
2�4
-
getRegionsaws:index/getRegions:getRegions�/Provides information about AWS Regions. Can be used to filter regions i.e., by Opt-In status or only regions enabled for current account. To get details like endpoint and description of each region the data source can be combined with the `aws.getRegion` data source.

## Example Usage

Enabled AWS Regions:

<!--Start PulumiCodeChooser -->
```typescript
import * as pulumi from "@pulumi/pulumi";
import * as aws from "@pulumi/aws";

const current = aws.getRegions({});
```
```python
import pulumi
import pulumi_aws as aws

current = aws.get_regions()
```
```csharp
using System.Collections.Generic;
using System.Linq;
using Pulumi;
using Aws = Pulumi.Aws;

return await Deployment.RunAsync(() => 
{
    var current = Aws.GetRegions.Invoke();

});
```
```go
package main

import (
	"github.com/pulumi/pulumi-aws/sdk/v6/go/aws"
	"github.com/pulumi/pulumi/sdk/v3/go/pulumi"
)

func main() {
	pulumi.Run(func(ctx *pulumi.Context) error {
		_, err := aws.GetRegions(ctx, &aws.GetRegionsArgs{}, nil)
		if err != nil {
			return err
		}
		return nil
	})
}
```
```java
package generated_program;

import com.pulumi.Context;
import com.pulumi.Pulumi;
import com.pulumi.core.Output;
import com.pulumi.aws.AwsFunctions;
import com.pulumi.aws.inputs.GetRegionsArgs;
import java.util.List;
import java.util.ArrayList;
import java.util.Map;
import java.io.File;
import java.nio.file.Files;
import java.nio.file.Paths;

public class App {
    public static void main(String[] args) {
        Pulumi.run(App::stack);
    }

    public static void stack(Context ctx) {
        final var current = AwsFunctions.getRegions();

    }
}
```
```yaml
variables:
  current:
    fn::invoke:
      function: aws:getRegions
      arguments: {}
```
<!--End PulumiCodeChooser -->

All the regions regardless of the availability

<!--Start PulumiCodeChooser -->
```typescript
import * as pulumi from "@pulumi/pulumi";
import * as aws from "@pulumi/aws";

const current = aws.getRegions({
    allRegions: true,
});
```
```python
import pulumi
import pulumi_aws as aws

current = aws.get_regions(all_regions=True)
```
```csharp
using System.Collections.Generic;
using System.Linq;
using Pulumi;
using Aws = Pulumi.Aws;

return await Deployment.RunAsync(() => 
{
    var current = Aws.GetRegions.Invoke(new()
    {
        AllRegions = true,
    });

});
```
```go
package main

import (
	"github.com/pulumi/pulumi-aws/sdk/v6/go/aws"
	"github.com/pulumi/pulumi/sdk/v3/go/pulumi"
)

func main() {
	pulumi.Run(func(ctx *pulumi.Context) error {
		_, err := aws.GetRegions(ctx, &aws.GetRegionsArgs{
			AllRegions: pulumi.BoolRef(true),
		}, nil)
		if err != nil {
			return err
		}
		return nil
	})
}
```
```java
package generated_program;

import com.pulumi.Context;
import com.pulumi.Pulumi;
import com.pulumi.core.Output;
import com.pulumi.aws.AwsFunctions;
import com.pulumi.aws.inputs.GetRegionsArgs;
import java.util.List;
import java.util.ArrayList;
import java.util.Map;
import java.io.File;
import java.nio.file.Files;
import java.nio.file.Paths;

public class App {
    public static void main(String[] args) {
        Pulumi.run(App::stack);
    }

    public static void stack(Context ctx) {
        final var current = AwsFunctions.getRegions(GetRegionsArgs.builder()
            .allRegions(true)
            .build());

    }
}
```
```yaml
variables:
  current:
    fn::invoke:
      function: aws:getRegions
      arguments:
        allRegions: true
```
<!--End PulumiCodeChooser -->

To see regions that are filtered by `"not-opted-in"`, the `all_regions` argument needs to be set to `true` or no results will be returned.

<!--Start PulumiCodeChooser -->
```typescript
import * as pulumi from "@pulumi/pulumi";
import * as aws from "@pulumi/aws";

const current = aws.getRegions({
    allRegions: true,
    filters: [{
        name: "opt-in-status",
        values: ["not-opted-in"],
    }],
});
```
```python
import pulumi
import pulumi_aws as aws

current = aws.get_regions(all_regions=True,
    filters=[{
        "name": "opt-in-status",
        "values": ["not-opted-in"],
    }])
```
```csharp
using System.Collections.Generic;
using System.Linq;
using Pulumi;
using Aws = Pulumi.Aws;

return await Deployment.RunAsync(() => 
{
    var current = Aws.GetRegions.Invoke(new()
    {
        AllRegions = true,
        Filters = new[]
        {
            new Aws.Inputs.GetRegionsFilterInputArgs
            {
                Name = "opt-in-status",
                Values = new[]
                {
                    "not-opted-in",
                },
            },
        },
    });

});
```
```go
package main

import (
	"github.com/pulumi/pulumi-aws/sdk/v6/go/aws"
	"github.com/pulumi/pulumi/sdk/v3/go/pulumi"
)

func main() {
	pulumi.Run(func(ctx *pulumi.Context) error {
		_, err := aws.GetRegions(ctx, &aws.GetRegionsArgs{
			AllRegions: pulumi.BoolRef(true),
			Filters: []aws.GetRegionsFilter{
				{
					Name: "opt-in-status",
					Values: []string{
						"not-opted-in",
					},
				},
			},
		}, nil)
		if err != nil {
			return err
		}
		return nil
	})
}
```
```java
package generated_program;

import com.pulumi.Context;
import com.pulumi.Pulumi;
import com.pulumi.core.Output;
import com.pulumi.aws.AwsFunctions;
import com.pulumi.aws.inputs.GetRegionsArgs;
import java.util.List;
import java.util.ArrayList;
import java.util.Map;
import java.io.File;
import java.nio.file.Files;
import java.nio.file.Paths;

public class App {
    public static void main(String[] args) {
        Pulumi.run(App::stack);
    }

    public static void stack(Context ctx) {
        final var current = AwsFunctions.getRegions(GetRegionsArgs.builder()
            .allRegions(true)
            .filters(GetRegionsFilterArgs.builder()
                .name("opt-in-status")
                .values("not-opted-in")
                .build())
            .build());

    }
}
```
```yaml
variables:
  current:
    fn::invoke:
      function: aws:getRegions
      arguments:
        allRegions: true
        filters:
          - name: opt-in-status
            values:
              - not-opted-in
```
<!--End PulumiCodeChooser -->
Z

allRegionsB
 FIf true the source will query all regions regardless of availability.
�
filtersGBE*C:A
?getRegionsFilter+aws:index/getRegionsFilter:getRegionsFilter:Configuration block(s) to use as filters. Detailed below.
h
idB" \Identifier of the current partition (e.g., `aws` in AWS Commercial, `aws-cn` in AWS China).
"

allRegionsB
 "R
filtersGBE*C:A
?getRegionsFilter+aws:index/getRegionsFilter:getRegionsFilter"f
id" \Identifier of the current partition (e.g., `aws` in AWS Commercial, `aws-cn` in AWS China).
"9
names*" *Names of regions that meets the criteria.
2�9
-
getServiceaws:index/getService:getService�0Use this data source to compose and decompose AWS service DNS names.

## Example Usage

### Get Service DNS Name

<!--Start PulumiCodeChooser -->
```typescript
import * as pulumi from "@pulumi/pulumi";
import * as aws from "@pulumi/aws";

const current = aws.getRegion({});
const test = current.then(current => aws.getService({
    region: current.name,
    serviceId: "ec2",
}));
```
```python
import pulumi
import pulumi_aws as aws

current = aws.get_region()
test = aws.get_service(region=current.name,
    service_id="ec2")
```
```csharp
using System.Collections.Generic;
using System.Linq;
using Pulumi;
using Aws = Pulumi.Aws;

return await Deployment.RunAsync(() => 
{
    var current = Aws.GetRegion.Invoke();

    var test = Aws.GetService.Invoke(new()
    {
        Region = current.Apply(getRegionResult => getRegionResult.Name),
        ServiceId = "ec2",
    });

});
```
```go
package main

import (
	"github.com/pulumi/pulumi-aws/sdk/v6/go/aws"
	"github.com/pulumi/pulumi/sdk/v3/go/pulumi"
)

func main() {
	pulumi.Run(func(ctx *pulumi.Context) error {
		current, err := aws.GetRegion(ctx, &aws.GetRegionArgs{}, nil)
		if err != nil {
			return err
		}
		_, err = aws.GetService(ctx, &aws.GetServiceArgs{
			Region:    pulumi.StringRef(current.Name),
			ServiceId: pulumi.StringRef("ec2"),
		}, nil)
		if err != nil {
			return err
		}
		return nil
	})
}
```
```java
package generated_program;

import com.pulumi.Context;
import com.pulumi.Pulumi;
import com.pulumi.core.Output;
import com.pulumi.aws.AwsFunctions;
import com.pulumi.aws.inputs.GetRegionArgs;
import com.pulumi.aws.inputs.GetServiceArgs;
import java.util.List;
import java.util.ArrayList;
import java.util.Map;
import java.io.File;
import java.nio.file.Files;
import java.nio.file.Paths;

public class App {
    public static void main(String[] args) {
        Pulumi.run(App::stack);
    }

    public static void stack(Context ctx) {
        final var current = AwsFunctions.getRegion();

        final var test = AwsFunctions.getService(GetServiceArgs.builder()
            .region(current.applyValue(getRegionResult -> getRegionResult.name()))
            .serviceId("ec2")
            .build());

    }
}
```
```yaml
variables:
  current:
    fn::invoke:
      function: aws:getRegion
      arguments: {}
  test:
    fn::invoke:
      function: aws:getService
      arguments:
        region: ${current.name}
        serviceId: ec2
```
<!--End PulumiCodeChooser -->

### Use Service Reverse DNS Name to Get Components

<!--Start PulumiCodeChooser -->
```typescript
import * as pulumi from "@pulumi/pulumi";
import * as aws from "@pulumi/aws";

const s3 = aws.getService({
    reverseDnsName: "cn.com.amazonaws.cn-north-1.s3",
});
```
```python
import pulumi
import pulumi_aws as aws

s3 = aws.get_service(reverse_dns_name="cn.com.amazonaws.cn-north-1.s3")
```
```csharp
using System.Collections.Generic;
using System.Linq;
using Pulumi;
using Aws = Pulumi.Aws;

return await Deployment.RunAsync(() => 
{
    var s3 = Aws.GetService.Invoke(new()
    {
        ReverseDnsName = "cn.com.amazonaws.cn-north-1.s3",
    });

});
```
```go
package main

import (
	"github.com/pulumi/pulumi-aws/sdk/v6/go/aws"
	"github.com/pulumi/pulumi/sdk/v3/go/pulumi"
)

func main() {
	pulumi.Run(func(ctx *pulumi.Context) error {
		_, err := aws.GetService(ctx, &aws.GetServiceArgs{
			ReverseDnsName: pulumi.StringRef("cn.com.amazonaws.cn-north-1.s3"),
		}, nil)
		if err != nil {
			return err
		}
		return nil
	})
}
```
```java
package generated_program;

import com.pulumi.Context;
import com.pulumi.Pulumi;
import com.pulumi.core.Output;
import com.pulumi.aws.AwsFunctions;
import com.pulumi.aws.inputs.GetServiceArgs;
import java.util.List;
import java.util.ArrayList;
import java.util.Map;
import java.io.File;
import java.nio.file.Files;
import java.nio.file.Paths;

public class App {
    public static void main(String[] args) {
        Pulumi.run(App::stack);
    }

    public static void stack(Context ctx) {
        final var s3 = AwsFunctions.getService(GetServiceArgs.builder()
            .reverseDnsName("cn.com.amazonaws.cn-north-1.s3")
            .build());

    }
}
```
```yaml
variables:
  s3:
    fn::invoke:
      function: aws:getService
      arguments:
        reverseDnsName: cn.com.amazonaws.cn-north-1.s3
```
<!--End PulumiCodeChooser -->

### Determine Regional Support for a Service

<!--Start PulumiCodeChooser -->
```typescript
import * as pulumi from "@pulumi/pulumi";
import * as aws from "@pulumi/aws";

const s3 = aws.getService({
    reverseDnsName: "com.amazonaws.us-gov-west-1.waf",
});
```
```python
import pulumi
import pulumi_aws as aws

s3 = aws.get_service(reverse_dns_name="com.amazonaws.us-gov-west-1.waf")
```
```csharp
using System.Collections.Generic;
using System.Linq;
using Pulumi;
using Aws = Pulumi.Aws;

return await Deployment.RunAsync(() => 
{
    var s3 = Aws.GetService.Invoke(new()
    {
        ReverseDnsName = "com.amazonaws.us-gov-west-1.waf",
    });

});
```
```go
package main

import (
	"github.com/pulumi/pulumi-aws/sdk/v6/go/aws"
	"github.com/pulumi/pulumi/sdk/v3/go/pulumi"
)

func main() {
	pulumi.Run(func(ctx *pulumi.Context) error {
		_, err := aws.GetService(ctx, &aws.GetServiceArgs{
			ReverseDnsName: pulumi.StringRef("com.amazonaws.us-gov-west-1.waf"),
		}, nil)
		if err != nil {
			return err
		}
		return nil
	})
}
```
```java
package generated_program;

import com.pulumi.Context;
import com.pulumi.Pulumi;
import com.pulumi.core.Output;
import com.pulumi.aws.AwsFunctions;
import com.pulumi.aws.inputs.GetServiceArgs;
import java.util.List;
import java.util.ArrayList;
import java.util.Map;
import java.io.File;
import java.nio.file.Files;
import java.nio.file.Paths;

public class App {
    public static void main(String[] args) {
        Pulumi.run(App::stack);
    }

    public static void stack(Context ctx) {
        final var s3 = AwsFunctions.getService(GetServiceArgs.builder()
            .reverseDnsName("com.amazonaws.us-gov-west-1.waf")
            .build());

    }
}
```
```yaml
variables:
  s3:
    fn::invoke:
      function: aws:getService
      arguments:
        reverseDnsName: com.amazonaws.us-gov-west-1.waf
```
<!--End PulumiCodeChooser -->
�
dnsNameB" �DNS name of the service (_e.g.,_ `rds.us-east-1.amazonaws.com`). One of `dns_name`, `reverse_dns_name`, or `service_id` is required.


idB" O
regionB" ?Region of the service (_e.g.,_ `us-west-2`, `ap-northeast-1`).
�
reverseDnsNameB" �Reverse DNS name of the service (_e.g.,_ `com.amazonaws.us-west-2.s3`). One of `dns_name`, `reverse_dns_name`, or `service_id` is required.
~
reverseDnsPrefixB" dPrefix of the service (_e.g.,_ `com.amazonaws` in AWS Commercial, `cn.com.amazonaws` in AWS China).
�
	serviceIdB" �Service endpoint ID (_e.g.,_ `s3`, `rds`, `ec2`). One of `dns_name`, `reverse_dns_name`, or `service_id` is required. A service's endpoint ID can be found in the [_AWS General Reference_](https://docs.aws.amazon.com/general/latest/gr/aws-service-information.html).
"
dnsName" "
id" "
	partition" "
region" "
reverseDnsName" "
reverseDnsPrefix" "
	serviceId" "�
	supported
 uWhether the service is supported in the region's partition. New services may not be listed immediately as supported.
2�
HgetServicePrincipal1aws:index/getServicePrincipal:getServicePrincipal�Use this data source to create a Service Principal Name for a service in a given region. Service Principal Names should always end in the standard global format: `{servicename}.amazonaws.com`. However, in some AWS partitions, AWS may expect a different format.

## Example Usage

<!--Start PulumiCodeChooser -->
```typescript
import * as pulumi from "@pulumi/pulumi";
import * as aws from "@pulumi/aws";

const current = aws.getServicePrincipal({
    serviceName: "s3",
});
const test = aws.getServicePrincipal({
    serviceName: "s3",
    region: "us-iso-east-1",
});
```
```python
import pulumi
import pulumi_aws as aws

current = aws.get_service_principal(service_name="s3")
test = aws.get_service_principal(service_name="s3",
    region="us-iso-east-1")
```
```csharp
using System.Collections.Generic;
using System.Linq;
using Pulumi;
using Aws = Pulumi.Aws;

return await Deployment.RunAsync(() => 
{
    var current = Aws.GetServicePrincipal.Invoke(new()
    {
        ServiceName = "s3",
    });

    var test = Aws.GetServicePrincipal.Invoke(new()
    {
        ServiceName = "s3",
        Region = "us-iso-east-1",
    });

});
```
```go
package main

import (
	"github.com/pulumi/pulumi-aws/sdk/v6/go/aws"
	"github.com/pulumi/pulumi/sdk/v3/go/pulumi"
)

func main() {
	pulumi.Run(func(ctx *pulumi.Context) error {
		_, err := aws.GetServicePrincipal(ctx, &aws.GetServicePrincipalArgs{
			ServiceName: "s3",
		}, nil)
		if err != nil {
			return err
		}
		_, err = aws.GetServicePrincipal(ctx, &aws.GetServicePrincipalArgs{
			ServiceName: "s3",
			Region:      pulumi.StringRef("us-iso-east-1"),
		}, nil)
		if err != nil {
			return err
		}
		return nil
	})
}
```
```java
package generated_program;

import com.pulumi.Context;
import com.pulumi.Pulumi;
import com.pulumi.core.Output;
import com.pulumi.aws.AwsFunctions;
import com.pulumi.aws.inputs.GetServicePrincipalArgs;
import java.util.List;
import java.util.ArrayList;
import java.util.Map;
import java.io.File;
import java.nio.file.Files;
import java.nio.file.Paths;

public class App {
    public static void main(String[] args) {
        Pulumi.run(App::stack);
    }

    public static void stack(Context ctx) {
        final var current = AwsFunctions.getServicePrincipal(GetServicePrincipalArgs.builder()
            .serviceName("s3")
            .build());

        final var test = AwsFunctions.getServicePrincipal(GetServicePrincipalArgs.builder()
            .serviceName("s3")
            .region("us-iso-east-1")
            .build());

    }
}
```
```yaml
variables:
  current:
    fn::invoke:
      function: aws:getServicePrincipal
      arguments:
        serviceName: s3
  test:
    fn::invoke:
      function: aws:getServicePrincipal
      arguments:
        serviceName: s3
        region: us-iso-east-1
```
<!--End PulumiCodeChooser -->
T
regionB" DRegion you'd like the SPN for. By default, uses the current region.
Z
serviceName" GName of the service you want to generate a Service Principal Name for.
"�
id" �Identifier of the current Service Principal (compound of service, region and suffix). (e.g. `logs.us-east-1.amazonaws.com`in AWS Commercial, `logs.cn-north-1.amazonaws.com.cn` in AWS China).
"y
name" mService Principal Name (e.g., `logs.amazonaws.com` in AWS Commercial, `logs.amazonaws.com.cn` in AWS China).
"w
region" iRegion identifier of the generated SPN (e.g., `us-east-1` in AWS Commercial, `cn-north-1` in AWS China).
"
serviceName" "l
suffix" ^Suffix of the SPN (e.g., `amazonaws.com` in AWS Commercial, `amazonaws.com.cn` in AWS China).
2�
4
outpostsgetAssetaws:outposts/getAsset:getAsset<Information about a specific hardware asset in an Outpost.


arn" Outpost ARN.
 
assetId" ID of the asset.
"	
arn" "
assetId" "$
	assetType" Type of the asset.
"_
hostId" QHost ID of the Dedicated Hosts on the asset, if a Dedicated Host is provisioned.
"E
id" ;The provider-assigned unique ID for this managed resource.
"L
rackElevation 7Position of an asset in a rack measured in rack units.
"$
rackId" Rack ID of the asset.
2�4
7
outposts	getAssets aws:outposts/getAssets:getAssets�0Information about hardware assets in an Outpost.

## Example Usage

### Basic

<!--Start PulumiCodeChooser -->
```typescript
import * as pulumi from "@pulumi/pulumi";
import * as aws from "@pulumi/aws";

const example = aws.outposts.getAssets({
    arn: exampleAwsOutpostsOutpost.arn,
});
```
```python
import pulumi
import pulumi_aws as aws

example = aws.outposts.get_assets(arn=example_aws_outposts_outpost["arn"])
```
```csharp
using System.Collections.Generic;
using System.Linq;
using Pulumi;
using Aws = Pulumi.Aws;

return await Deployment.RunAsync(() => 
{
    var example = Aws.Outposts.GetAssets.Invoke(new()
    {
        Arn = exampleAwsOutpostsOutpost.Arn,
    });

});
```
```go
package main

import (
	"github.com/pulumi/pulumi-aws/sdk/v6/go/aws/outposts"
	"github.com/pulumi/pulumi/sdk/v3/go/pulumi"
)

func main() {
	pulumi.Run(func(ctx *pulumi.Context) error {
		_, err := outposts.GetAssets(ctx, &outposts.GetAssetsArgs{
			Arn: exampleAwsOutpostsOutpost.Arn,
		}, nil)
		if err != nil {
			return err
		}
		return nil
	})
}
```
```java
package generated_program;

import com.pulumi.Context;
import com.pulumi.Pulumi;
import com.pulumi.core.Output;
import com.pulumi.aws.outposts.OutpostsFunctions;
import com.pulumi.aws.outposts.inputs.GetAssetsArgs;
import java.util.List;
import java.util.ArrayList;
import java.util.Map;
import java.io.File;
import java.nio.file.Files;
import java.nio.file.Paths;

public class App {
    public static void main(String[] args) {
        Pulumi.run(App::stack);
    }

    public static void stack(Context ctx) {
        final var example = OutpostsFunctions.getAssets(GetAssetsArgs.builder()
            .arn(exampleAwsOutpostsOutpost.arn())
            .build());

    }
}
```
```yaml
variables:
  example:
    fn::invoke:
      function: aws:outposts:getAssets
      arguments:
        arn: ${exampleAwsOutpostsOutpost.arn}
```
<!--End PulumiCodeChooser -->

### With Host ID Filter

<!--Start PulumiCodeChooser -->
```typescript
import * as pulumi from "@pulumi/pulumi";
import * as aws from "@pulumi/aws";

const example = aws.outposts.getAssets({
    arn: exampleAwsOutpostsOutpost.arn,
    hostIdFilters: ["h-x38g5n0yd2a0ueb61"],
});
```
```python
import pulumi
import pulumi_aws as aws

example = aws.outposts.get_assets(arn=example_aws_outposts_outpost["arn"],
    host_id_filters=["h-x38g5n0yd2a0ueb61"])
```
```csharp
using System.Collections.Generic;
using System.Linq;
using Pulumi;
using Aws = Pulumi.Aws;

return await Deployment.RunAsync(() => 
{
    var example = Aws.Outposts.GetAssets.Invoke(new()
    {
        Arn = exampleAwsOutpostsOutpost.Arn,
        HostIdFilters = new[]
        {
            "h-x38g5n0yd2a0ueb61",
        },
    });

});
```
```go
package main

import (
	"github.com/pulumi/pulumi-aws/sdk/v6/go/aws/outposts"
	"github.com/pulumi/pulumi/sdk/v3/go/pulumi"
)

func main() {
	pulumi.Run(func(ctx *pulumi.Context) error {
		_, err := outposts.GetAssets(ctx, &outposts.GetAssetsArgs{
			Arn: exampleAwsOutpostsOutpost.Arn,
			HostIdFilters: []string{
				"h-x38g5n0yd2a0ueb61",
			},
		}, nil)
		if err != nil {
			return err
		}
		return nil
	})
}
```
```java
package generated_program;

import com.pulumi.Context;
import com.pulumi.Pulumi;
import com.pulumi.core.Output;
import com.pulumi.aws.outposts.OutpostsFunctions;
import com.pulumi.aws.outposts.inputs.GetAssetsArgs;
import java.util.List;
import java.util.ArrayList;
import java.util.Map;
import java.io.File;
import java.nio.file.Files;
import java.nio.file.Paths;

public class App {
    public static void main(String[] args) {
        Pulumi.run(App::stack);
    }

    public static void stack(Context ctx) {
        final var example = OutpostsFunctions.getAssets(GetAssetsArgs.builder()
            .arn(exampleAwsOutpostsOutpost.arn())
            .hostIdFilters("h-x38g5n0yd2a0ueb61")
            .build());

    }
}
```
```yaml
variables:
  example:
    fn::invoke:
      function: aws:outposts:getAssets
      arguments:
        arn: ${exampleAwsOutpostsOutpost.arn}
        hostIdFilters:
          - h-x38g5n0yd2a0ueb61
```
<!--End PulumiCodeChooser -->

### With Status ID Filter

<!--Start PulumiCodeChooser -->
```typescript
import * as pulumi from "@pulumi/pulumi";
import * as aws from "@pulumi/aws";

const example = aws.outposts.getAssets({
    arn: exampleAwsOutpostsOutpost.arn,
    statusIdFilters: ["ACTIVE"],
});
```
```python
import pulumi
import pulumi_aws as aws

example = aws.outposts.get_assets(arn=example_aws_outposts_outpost["arn"],
    status_id_filters=["ACTIVE"])
```
```csharp
using System.Collections.Generic;
using System.Linq;
using Pulumi;
using Aws = Pulumi.Aws;

return await Deployment.RunAsync(() => 
{
    var example = Aws.Outposts.GetAssets.Invoke(new()
    {
        Arn = exampleAwsOutpostsOutpost.Arn,
        StatusIdFilters = new[]
        {
            "ACTIVE",
        },
    });

});
```
```go
package main

import (
	"github.com/pulumi/pulumi-aws/sdk/v6/go/aws/outposts"
	"github.com/pulumi/pulumi/sdk/v3/go/pulumi"
)

func main() {
	pulumi.Run(func(ctx *pulumi.Context) error {
		_, err := outposts.GetAssets(ctx, &outposts.GetAssetsArgs{
			Arn: exampleAwsOutpostsOutpost.Arn,
			StatusIdFilters: []string{
				"ACTIVE",
			},
		}, nil)
		if err != nil {
			return err
		}
		return nil
	})
}
```
```java
package generated_program;

import com.pulumi.Context;
import com.pulumi.Pulumi;
import com.pulumi.core.Output;
import com.pulumi.aws.outposts.OutpostsFunctions;
import com.pulumi.aws.outposts.inputs.GetAssetsArgs;
import java.util.List;
import java.util.ArrayList;
import java.util.Map;
import java.io.File;
import java.nio.file.Files;
import java.nio.file.Paths;

public class App {
    public static void main(String[] args) {
        Pulumi.run(App::stack);
    }

    public static void stack(Context ctx) {
        final var example = OutpostsFunctions.getAssets(GetAssetsArgs.builder()
            .arn(exampleAwsOutpostsOutpost.arn())
            .statusIdFilters("ACTIVE")
            .build());

    }
}
```
```yaml
variables:
  example:
    fn::invoke:
      function: aws:outposts:getAssets
      arguments:
        arn: ${exampleAwsOutpostsOutpost.arn}
        statusIdFilters:
          - ACTIVE
```
<!--End PulumiCodeChooser -->

arn" Outpost ARN.
J
hostIdFiltersB*" 1Filters by list of Host IDs of a Dedicated Host.
`
statusIdFiltersB*" EFilters by list of state status. Valid values: "ACTIVE", "RETIRING".
"	
arn" "a
assetIds*" OList of all the asset ids found. This data source will fail if none are found.
"
hostIdFiltersB*" "E
id" ;The provider-assigned unique ID for this managed resource.
"
statusIdFiltersB*" 2�
:
outposts
getOutpost"aws:outposts/getOutpost:getOutpost�Provides details about an Outposts Outpost.

## Example Usage

<!--Start PulumiCodeChooser -->
```typescript
import * as pulumi from "@pulumi/pulumi";
import * as aws from "@pulumi/aws";

const example = aws.outposts.getOutpost({
    name: "example",
});
```
```python
import pulumi
import pulumi_aws as aws

example = aws.outposts.get_outpost(name="example")
```
```csharp
using System.Collections.Generic;
using System.Linq;
using Pulumi;
using Aws = Pulumi.Aws;

return await Deployment.RunAsync(() => 
{
    var example = Aws.Outposts.GetOutpost.Invoke(new()
    {
        Name = "example",
    });

});
```
```go
package main

import (
	"github.com/pulumi/pulumi-aws/sdk/v6/go/aws/outposts"
	"github.com/pulumi/pulumi/sdk/v3/go/pulumi"
)

func main() {
	pulumi.Run(func(ctx *pulumi.Context) error {
		_, err := outposts.GetOutpost(ctx, &outposts.GetOutpostArgs{
			Name: pulumi.StringRef("example"),
		}, nil)
		if err != nil {
			return err
		}
		return nil
	})
}
```
```java
package generated_program;

import com.pulumi.Context;
import com.pulumi.Pulumi;
import com.pulumi.core.Output;
import com.pulumi.aws.outposts.OutpostsFunctions;
import com.pulumi.aws.outposts.inputs.GetOutpostArgs;
import java.util.List;
import java.util.ArrayList;
import java.util.Map;
import java.io.File;
import java.nio.file.Files;
import java.nio.file.Paths;

public class App {
    public static void main(String[] args) {
        Pulumi.run(App::stack);
    }

    public static void stack(Context ctx) {
        final var example = OutpostsFunctions.getOutpost(GetOutpostArgs.builder()
            .name("example")
            .build());

    }
}
```
```yaml
variables:
  example:
    fn::invoke:
      function: aws:outposts:getOutpost
      arguments:
        name: example
```
<!--End PulumiCodeChooser -->

arnB" ARN.
'
idB" Identifier of the Outpost.
#
nameB" Name of the Outpost.
>
ownerIdB" -AWS Account identifier of the Outpost owner.
"
tagsB2" The Outpost tags.
"	
arn" "0
availabilityZone" Availability Zone name.
"8
availabilityZoneId" Availability Zone identifier.
"3
description"  The description of the Outpost.
"
id" ".
lifecycleStatus" The life cycle status.
"

name" "
ownerIdB" ";
siteArn" ,The Amazon Resource Name (ARN) of the site.
""
siteId" The ID of the site.
"0
supportedHardwareType" The hardware type.
" 
tags2" The Outpost tags.
2�!
^
outpostsgetOutpostInstanceType:aws:outposts/getOutpostInstanceType:getOutpostInstanceType�Information about single Outpost Instance Type.

## Example Usage

<!--Start PulumiCodeChooser -->
```typescript
import * as pulumi from "@pulumi/pulumi";
import * as aws from "@pulumi/aws";

const example = aws.outposts.getOutpostInstanceType({
    arn: exampleAwsOutpostsOutpost.arn,
    preferredInstanceTypes: [
        "m5.large",
        "m5.4xlarge",
    ],
});
const exampleEc2Instance = new aws.index.Ec2Instance("example", {instanceType: example.instanceType});
```
```python
import pulumi
import pulumi_aws as aws

example = aws.outposts.get_outpost_instance_type(arn=example_aws_outposts_outpost["arn"],
    preferred_instance_types=[
        "m5.large",
        "m5.4xlarge",
    ])
example_ec2_instance = aws.index.Ec2Instance("example", instance_type=example.instance_type)
```
```csharp
using System.Collections.Generic;
using System.Linq;
using Pulumi;
using Aws = Pulumi.Aws;

return await Deployment.RunAsync(() => 
{
    var example = Aws.Outposts.GetOutpostInstanceType.Invoke(new()
    {
        Arn = exampleAwsOutpostsOutpost.Arn,
        PreferredInstanceTypes = new[]
        {
            "m5.large",
            "m5.4xlarge",
        },
    });

    var exampleEc2Instance = new Aws.Index.Ec2Instance("example", new()
    {
        InstanceType = example.Apply(getOutpostInstanceTypeResult => getOutpostInstanceTypeResult.InstanceType),
    });

});
```
```go
package main

import (
	"github.com/pulumi/pulumi-aws/sdk/v6/go/aws"
	"github.com/pulumi/pulumi-aws/sdk/v6/go/aws/outposts"
	"github.com/pulumi/pulumi/sdk/v3/go/pulumi"
)

func main() {
	pulumi.Run(func(ctx *pulumi.Context) error {
		example, err := outposts.GetOutpostInstanceType(ctx, &outposts.GetOutpostInstanceTypeArgs{
			Arn: exampleAwsOutpostsOutpost.Arn,
			PreferredInstanceTypes: []string{
				"m5.large",
				"m5.4xlarge",
			},
		}, nil)
		if err != nil {
			return err
		}
		_, err = aws.NewEc2Instance(ctx, "example", &aws.Ec2InstanceArgs{
			InstanceType: example.InstanceType,
		})
		if err != nil {
			return err
		}
		return nil
	})
}
```
```java
package generated_program;

import com.pulumi.Context;
import com.pulumi.Pulumi;
import com.pulumi.core.Output;
import com.pulumi.aws.outposts.OutpostsFunctions;
import com.pulumi.aws.outposts.inputs.GetOutpostInstanceTypeArgs;
import com.pulumi.aws.ec2Instance;
import com.pulumi.aws.Ec2InstanceArgs;
import java.util.List;
import java.util.ArrayList;
import java.util.Map;
import java.io.File;
import java.nio.file.Files;
import java.nio.file.Paths;

public class App {
    public static void main(String[] args) {
        Pulumi.run(App::stack);
    }

    public static void stack(Context ctx) {
        final var example = OutpostsFunctions.getOutpostInstanceType(GetOutpostInstanceTypeArgs.builder()
            .arn(exampleAwsOutpostsOutpost.arn())
            .preferredInstanceTypes(            
                "m5.large",
                "m5.4xlarge")
            .build());

        var exampleEc2Instance = new Ec2Instance("exampleEc2Instance", Ec2InstanceArgs.builder()
            .instanceType(example.applyValue(getOutpostInstanceTypeResult -> getOutpostInstanceTypeResult.instanceType()))
            .build());

    }
}
```
```yaml
resources:
  exampleEc2Instance:
    type: aws:ec2Instance
    name: example
    properties:
      instanceType: ${example.instanceType}
variables:
  example:
    fn::invoke:
      function: aws:outposts:getOutpostInstanceType
      arguments:
        arn: ${exampleAwsOutpostsOutpost.arn}
        preferredInstanceTypes:
          - m5.large
          - m5.4xlarge
```
<!--End PulumiCodeChooser -->
?
arn" 4Outpost ARN.

The following arguments are optional:
X
instanceTypeB" BDesired instance type. Conflicts with `preferred_instance_types`.
�
preferredInstanceTypesB*" �Ordered list of preferred instance types. The first match in this list will be returned. If no preferred matches are found and the original search returned more than one result, an error is returned. Conflicts with `instance_type`.
"	
arn" "E
id" ;The provider-assigned unique ID for this managed resource.
"
instanceType" " 
preferredInstanceTypesB*" 2�
a
outpostsgetOutpostInstanceTypes<aws:outposts/getOutpostInstanceTypes:getOutpostInstanceTypes�Information about Outposts Instance Types.

## Example Usage

<!--Start PulumiCodeChooser -->
```typescript
import * as pulumi from "@pulumi/pulumi";
import * as aws from "@pulumi/aws";

const example = aws.outposts.getOutpostInstanceTypes({
    arn: exampleAwsOutpostsOutpost.arn,
});
```
```python
import pulumi
import pulumi_aws as aws

example = aws.outposts.get_outpost_instance_types(arn=example_aws_outposts_outpost["arn"])
```
```csharp
using System.Collections.Generic;
using System.Linq;
using Pulumi;
using Aws = Pulumi.Aws;

return await Deployment.RunAsync(() => 
{
    var example = Aws.Outposts.GetOutpostInstanceTypes.Invoke(new()
    {
        Arn = exampleAwsOutpostsOutpost.Arn,
    });

});
```
```go
package main

import (
	"github.com/pulumi/pulumi-aws/sdk/v6/go/aws/outposts"
	"github.com/pulumi/pulumi/sdk/v3/go/pulumi"
)

func main() {
	pulumi.Run(func(ctx *pulumi.Context) error {
		_, err := outposts.GetOutpostInstanceTypes(ctx, &outposts.GetOutpostInstanceTypesArgs{
			Arn: exampleAwsOutpostsOutpost.Arn,
		}, nil)
		if err != nil {
			return err
		}
		return nil
	})
}
```
```java
package generated_program;

import com.pulumi.Context;
import com.pulumi.Pulumi;
import com.pulumi.core.Output;
import com.pulumi.aws.outposts.OutpostsFunctions;
import com.pulumi.aws.outposts.inputs.GetOutpostInstanceTypesArgs;
import java.util.List;
import java.util.ArrayList;
import java.util.Map;
import java.io.File;
import java.nio.file.Files;
import java.nio.file.Paths;

public class App {
    public static void main(String[] args) {
        Pulumi.run(App::stack);
    }

    public static void stack(Context ctx) {
        final var example = OutpostsFunctions.getOutpostInstanceTypes(GetOutpostInstanceTypesArgs.builder()
            .arn(exampleAwsOutpostsOutpost.arn())
            .build());

    }
}
```
```yaml
variables:
  example:
    fn::invoke:
      function: aws:outposts:getOutpostInstanceTypes
      arguments:
        arn: ${exampleAwsOutpostsOutpost.arn}
```
<!--End PulumiCodeChooser -->

arn" Outpost ARN.
"	
arn" "E
id" ;The provider-assigned unique ID for this managed resource.
".
instanceTypes*" Set of instance types.
2�
=
outpostsgetOutposts$aws:outposts/getOutposts:getOutposts�Provides details about multiple Outposts.

## Example Usage

<!--Start PulumiCodeChooser -->
```typescript
import * as pulumi from "@pulumi/pulumi";
import * as aws from "@pulumi/aws";

const example = aws.outposts.getOutposts({
    siteId: id,
});
```
```python
import pulumi
import pulumi_aws as aws

example = aws.outposts.get_outposts(site_id=id)
```
```csharp
using System.Collections.Generic;
using System.Linq;
using Pulumi;
using Aws = Pulumi.Aws;

return await Deployment.RunAsync(() => 
{
    var example = Aws.Outposts.GetOutposts.Invoke(new()
    {
        SiteId = id,
    });

});
```
```go
package main

import (
	"github.com/pulumi/pulumi-aws/sdk/v6/go/aws/outposts"
	"github.com/pulumi/pulumi/sdk/v3/go/pulumi"
)

func main() {
	pulumi.Run(func(ctx *pulumi.Context) error {
		_, err := outposts.GetOutposts(ctx, &outposts.GetOutpostsArgs{
			SiteId: pulumi.StringRef(id),
		}, nil)
		if err != nil {
			return err
		}
		return nil
	})
}
```
```java
package generated_program;

import com.pulumi.Context;
import com.pulumi.Pulumi;
import com.pulumi.core.Output;
import com.pulumi.aws.outposts.OutpostsFunctions;
import com.pulumi.aws.outposts.inputs.GetOutpostsArgs;
import java.util.List;
import java.util.ArrayList;
import java.util.Map;
import java.io.File;
import java.nio.file.Files;
import java.nio.file.Paths;

public class App {
    public static void main(String[] args) {
        Pulumi.run(App::stack);
    }

    public static void stack(Context ctx) {
        final var example = OutpostsFunctions.getOutposts(GetOutpostsArgs.builder()
            .siteId(id)
            .build());

    }
}
```
```yaml
variables:
  example:
    fn::invoke:
      function: aws:outposts:getOutposts
      arguments:
        siteId: ${id}
```
<!--End PulumiCodeChooser -->
2
availabilityZoneB" Availability Zone name.
:
availabilityZoneIdB" Availability Zone identifier.
>
ownerIdB" -AWS Account identifier of the Outpost owner.
!
siteIdB" Site identifier.
"3
arns*" %Set of Amazon Resource Names (ARNs).
"
availabilityZone" "
availabilityZoneId" "E
id" ;The provider-assigned unique ID for this managed resource.
"!
ids*" Set of identifiers.
"
ownerId" "
siteId" 2�
1
outpostsgetSiteaws:outposts/getSite:getSite�Provides details about an Outposts Site.

## Example Usage

<!--Start PulumiCodeChooser -->
```typescript
import * as pulumi from "@pulumi/pulumi";
import * as aws from "@pulumi/aws";

const example = aws.outposts.getSite({
    name: "example",
});
```
```python
import pulumi
import pulumi_aws as aws

example = aws.outposts.get_site(name="example")
```
```csharp
using System.Collections.Generic;
using System.Linq;
using Pulumi;
using Aws = Pulumi.Aws;

return await Deployment.RunAsync(() => 
{
    var example = Aws.Outposts.GetSite.Invoke(new()
    {
        Name = "example",
    });

});
```
```go
package main

import (
	"github.com/pulumi/pulumi-aws/sdk/v6/go/aws/outposts"
	"github.com/pulumi/pulumi/sdk/v3/go/pulumi"
)

func main() {
	pulumi.Run(func(ctx *pulumi.Context) error {
		_, err := outposts.GetSite(ctx, &outposts.GetSiteArgs{
			Name: pulumi.StringRef("example"),
		}, nil)
		if err != nil {
			return err
		}
		return nil
	})
}
```
```java
package generated_program;

import com.pulumi.Context;
import com.pulumi.Pulumi;
import com.pulumi.core.Output;
import com.pulumi.aws.outposts.OutpostsFunctions;
import com.pulumi.aws.outposts.inputs.GetSiteArgs;
import java.util.List;
import java.util.ArrayList;
import java.util.Map;
import java.io.File;
import java.nio.file.Files;
import java.nio.file.Paths;

public class App {
    public static void main(String[] args) {
        Pulumi.run(App::stack);
    }

    public static void stack(Context ctx) {
        final var example = OutpostsFunctions.getSite(GetSiteArgs.builder()
            .name("example")
            .build());

    }
}
```
```yaml
variables:
  example:
    fn::invoke:
      function: aws:outposts:getSite
      arguments:
        name: example
```
<!--End PulumiCodeChooser -->
$
idB" Identifier of the Site.
 
nameB" Name of the Site.
")
	accountId" AWS Account identifier.
" 
description" Description.
"
id" "

name" 2�
4
outpostsgetSitesaws:outposts/getSites:getSites�Provides details about multiple Outposts Sites.

## Example Usage

<!--Start PulumiCodeChooser -->
```typescript
import * as pulumi from "@pulumi/pulumi";
import * as aws from "@pulumi/aws";

const all = aws.outposts.getSites({});
```
```python
import pulumi
import pulumi_aws as aws

all = aws.outposts.get_sites()
```
```csharp
using System.Collections.Generic;
using System.Linq;
using Pulumi;
using Aws = Pulumi.Aws;

return await Deployment.RunAsync(() => 
{
    var all = Aws.Outposts.GetSites.Invoke();

});
```
```go
package main

import (
	"github.com/pulumi/pulumi-aws/sdk/v6/go/aws/outposts"
	"github.com/pulumi/pulumi/sdk/v3/go/pulumi"
)

func main() {
	pulumi.Run(func(ctx *pulumi.Context) error {
		_, err := outposts.GetSites(ctx, map[string]interface{}{}, nil)
		if err != nil {
			return err
		}
		return nil
	})
}
```
```java
package generated_program;

import com.pulumi.Context;
import com.pulumi.Pulumi;
import com.pulumi.core.Output;
import com.pulumi.aws.outposts.OutpostsFunctions;
import java.util.List;
import java.util.ArrayList;
import java.util.Map;
import java.io.File;
import java.nio.file.Files;
import java.nio.file.Paths;

public class App {
    public static void main(String[] args) {
        Pulumi.run(App::stack);
    }

    public static void stack(Context ctx) {
        final var all = OutpostsFunctions.getSites();

    }
}
```
```yaml
variables:
  all:
    fn::invoke:
      function: aws:outposts:getSites
      arguments: {}
```
<!--End PulumiCodeChooser -->
"E
id" ;The provider-assigned unique ID for this managed resource.
"/
ids*" "Set of Outposts Site identifiers.
2�!
1
polly	getVoicesaws:polly/getVoices:getVoices�Data source for managing an AWS Polly Voices.

## Example Usage

### Basic Usage

<!--Start PulumiCodeChooser -->
```typescript
import * as pulumi from "@pulumi/pulumi";
import * as aws from "@pulumi/aws";

const example = aws.polly.getVoices({});
```
```python
import pulumi
import pulumi_aws as aws

example = aws.polly.get_voices()
```
```csharp
using System.Collections.Generic;
using System.Linq;
using Pulumi;
using Aws = Pulumi.Aws;

return await Deployment.RunAsync(() => 
{
    var example = Aws.Polly.GetVoices.Invoke();

});
```
```go
package main

import (
	"github.com/pulumi/pulumi-aws/sdk/v6/go/aws/polly"
	"github.com/pulumi/pulumi/sdk/v3/go/pulumi"
)

func main() {
	pulumi.Run(func(ctx *pulumi.Context) error {
		_, err := polly.GetVoices(ctx, &polly.GetVoicesArgs{}, nil)
		if err != nil {
			return err
		}
		return nil
	})
}
```
```java
package generated_program;

import com.pulumi.Context;
import com.pulumi.Pulumi;
import com.pulumi.core.Output;
import com.pulumi.aws.polly.PollyFunctions;
import com.pulumi.aws.polly.inputs.GetVoicesArgs;
import java.util.List;
import java.util.ArrayList;
import java.util.Map;
import java.io.File;
import java.nio.file.Files;
import java.nio.file.Paths;

public class App {
    public static void main(String[] args) {
        Pulumi.run(App::stack);
    }

    public static void stack(Context ctx) {
        final var example = PollyFunctions.getVoices();

    }
}
```
```yaml
variables:
  example:
    fn::invoke:
      function: aws:polly:getVoices
      arguments: {}
```
<!--End PulumiCodeChooser -->

### With Language Code

<!--Start PulumiCodeChooser -->
```typescript
import * as pulumi from "@pulumi/pulumi";
import * as aws from "@pulumi/aws";

const example = aws.polly.getVoices({
    languageCode: "en-GB",
});
```
```python
import pulumi
import pulumi_aws as aws

example = aws.polly.get_voices(language_code="en-GB")
```
```csharp
using System.Collections.Generic;
using System.Linq;
using Pulumi;
using Aws = Pulumi.Aws;

return await Deployment.RunAsync(() => 
{
    var example = Aws.Polly.GetVoices.Invoke(new()
    {
        LanguageCode = "en-GB",
    });

});
```
```go
package main

import (
	"github.com/pulumi/pulumi-aws/sdk/v6/go/aws/polly"
	"github.com/pulumi/pulumi/sdk/v3/go/pulumi"
)

func main() {
	pulumi.Run(func(ctx *pulumi.Context) error {
		_, err := polly.GetVoices(ctx, &polly.GetVoicesArgs{
			LanguageCode: pulumi.StringRef("en-GB"),
		}, nil)
		if err != nil {
			return err
		}
		return nil
	})
}
```
```java
package generated_program;

import com.pulumi.Context;
import com.pulumi.Pulumi;
import com.pulumi.core.Output;
import com.pulumi.aws.polly.PollyFunctions;
import com.pulumi.aws.polly.inputs.GetVoicesArgs;
import java.util.List;
import java.util.ArrayList;
import java.util.Map;
import java.io.File;
import java.nio.file.Files;
import java.nio.file.Paths;

public class App {
    public static void main(String[] args) {
        Pulumi.run(App::stack);
    }

    public static void stack(Context ctx) {
        final var example = PollyFunctions.getVoices(GetVoicesArgs.builder()
            .languageCode("en-GB")
            .build());

    }
}
```
```yaml
variables:
  example:
    fn::invoke:
      function: aws:polly:getVoices
      arguments:
        languageCode: en-GB
```
<!--End PulumiCodeChooser -->
�
engineB" �Engine used by Amazon Polly when processing input text for speech synthesis. Valid values are `standard`, `neural`, and `long-form`.
�
includeAdditionalLanguageCodesB
 bWhether to return any bilingual voices that use the specified language as an additional language.
�
languageCodeB" |Language identification tag for filtering the list of voices returned. If not specified, all available voices are returned.
�
voicesHBF*D:B
@
pollygetVoicesVoice'aws:polly/getVoicesVoice:getVoicesVoiceNList of voices with their properties. See `voices` Attribute Reference below.
"
engineB" "*
id"  Amazon Polly assigned voice ID.
"&
includeAdditionalLanguageCodesB
 "2
languageCodeB" Language code of the voice.
"�
voicesHBF*D:B
@
pollygetVoicesVoice'aws:polly/getVoicesVoice:getVoicesVoiceNList of voices with their properties. See `voices` Attribute Reference below.
2�R
8
pricing
getProduct!aws:pricing/getProduct:getProduct�MUse this data source to get the pricing information of all products in AWS.
This data source is only available in a us-east-1 or ap-south-1 provider.

## Example Usage

<!--Start PulumiCodeChooser -->
```typescript
import * as pulumi from "@pulumi/pulumi";
import * as aws from "@pulumi/aws";

const example = aws.pricing.getProduct({
    serviceCode: "AmazonEC2",
    filters: [
        {
            field: "instanceType",
            value: "c5.xlarge",
        },
        {
            field: "operatingSystem",
            value: "Linux",
        },
        {
            field: "location",
            value: "US East (N. Virginia)",
        },
        {
            field: "preInstalledSw",
            value: "NA",
        },
        {
            field: "licenseModel",
            value: "No License required",
        },
        {
            field: "tenancy",
            value: "Shared",
        },
        {
            field: "capacitystatus",
            value: "Used",
        },
    ],
});
```
```python
import pulumi
import pulumi_aws as aws

example = aws.pricing.get_product(service_code="AmazonEC2",
    filters=[
        {
            "field": "instanceType",
            "value": "c5.xlarge",
        },
        {
            "field": "operatingSystem",
            "value": "Linux",
        },
        {
            "field": "location",
            "value": "US East (N. Virginia)",
        },
        {
            "field": "preInstalledSw",
            "value": "NA",
        },
        {
            "field": "licenseModel",
            "value": "No License required",
        },
        {
            "field": "tenancy",
            "value": "Shared",
        },
        {
            "field": "capacitystatus",
            "value": "Used",
        },
    ])
```
```csharp
using System.Collections.Generic;
using System.Linq;
using Pulumi;
using Aws = Pulumi.Aws;

return await Deployment.RunAsync(() => 
{
    var example = Aws.Pricing.GetProduct.Invoke(new()
    {
        ServiceCode = "AmazonEC2",
        Filters = new[]
        {
            new Aws.Pricing.Inputs.GetProductFilterInputArgs
            {
                Field = "instanceType",
                Value = "c5.xlarge",
            },
            new Aws.Pricing.Inputs.GetProductFilterInputArgs
            {
                Field = "operatingSystem",
                Value = "Linux",
            },
            new Aws.Pricing.Inputs.GetProductFilterInputArgs
            {
                Field = "location",
                Value = "US East (N. Virginia)",
            },
            new Aws.Pricing.Inputs.GetProductFilterInputArgs
            {
                Field = "preInstalledSw",
                Value = "NA",
            },
            new Aws.Pricing.Inputs.GetProductFilterInputArgs
            {
                Field = "licenseModel",
                Value = "No License required",
            },
            new Aws.Pricing.Inputs.GetProductFilterInputArgs
            {
                Field = "tenancy",
                Value = "Shared",
            },
            new Aws.Pricing.Inputs.GetProductFilterInputArgs
            {
                Field = "capacitystatus",
                Value = "Used",
            },
        },
    });

});
```
```go
package main

import (
	"github.com/pulumi/pulumi-aws/sdk/v6/go/aws/pricing"
	"github.com/pulumi/pulumi/sdk/v3/go/pulumi"
)

func main() {
	pulumi.Run(func(ctx *pulumi.Context) error {
		_, err := pricing.GetProduct(ctx, &pricing.GetProductArgs{
			ServiceCode: "AmazonEC2",
			Filters: []pricing.GetProductFilter{
				{
					Field: "instanceType",
					Value: "c5.xlarge",
				},
				{
					Field: "operatingSystem",
					Value: "Linux",
				},
				{
					Field: "location",
					Value: "US East (N. Virginia)",
				},
				{
					Field: "preInstalledSw",
					Value: "NA",
				},
				{
					Field: "licenseModel",
					Value: "No License required",
				},
				{
					Field: "tenancy",
					Value: "Shared",
				},
				{
					Field: "capacitystatus",
					Value: "Used",
				},
			},
		}, nil)
		if err != nil {
			return err
		}
		return nil
	})
}
```
```java
package generated_program;

import com.pulumi.Context;
import com.pulumi.Pulumi;
import com.pulumi.core.Output;
import com.pulumi.aws.pricing.PricingFunctions;
import com.pulumi.aws.pricing.inputs.GetProductArgs;
import java.util.List;
import java.util.ArrayList;
import java.util.Map;
import java.io.File;
import java.nio.file.Files;
import java.nio.file.Paths;

public class App {
    public static void main(String[] args) {
        Pulumi.run(App::stack);
    }

    public static void stack(Context ctx) {
        final var example = PricingFunctions.getProduct(GetProductArgs.builder()
            .serviceCode("AmazonEC2")
            .filters(            
                GetProductFilterArgs.builder()
                    .field("instanceType")
                    .value("c5.xlarge")
                    .build(),
                GetProductFilterArgs.builder()
                    .field("operatingSystem")
                    .value("Linux")
                    .build(),
                GetProductFilterArgs.builder()
                    .field("location")
                    .value("US East (N. Virginia)")
                    .build(),
                GetProductFilterArgs.builder()
                    .field("preInstalledSw")
                    .value("NA")
                    .build(),
                GetProductFilterArgs.builder()
                    .field("licenseModel")
                    .value("No License required")
                    .build(),
                GetProductFilterArgs.builder()
                    .field("tenancy")
                    .value("Shared")
                    .build(),
                GetProductFilterArgs.builder()
                    .field("capacitystatus")
                    .value("Used")
                    .build())
            .build());

    }
}
```
```yaml
variables:
  example:
    fn::invoke:
      function: aws:pricing:getProduct
      arguments:
        serviceCode: AmazonEC2
        filters:
          - field: instanceType
            value: c5.xlarge
          - field: operatingSystem
            value: Linux
          - field: location
            value: US East (N. Virginia)
          - field: preInstalledSw
            value: NA
          - field: licenseModel
            value: No License required
          - field: tenancy
            value: Shared
          - field: capacitystatus
            value: Used
```
<!--End PulumiCodeChooser -->

<!--Start PulumiCodeChooser -->
```typescript
import * as pulumi from "@pulumi/pulumi";
import * as aws from "@pulumi/aws";

const example = aws.pricing.getProduct({
    serviceCode: "AmazonRedshift",
    filters: [
        {
            field: "instanceType",
            value: "ds1.xlarge",
        },
        {
            field: "location",
            value: "US East (N. Virginia)",
        },
    ],
});
```
```python
import pulumi
import pulumi_aws as aws

example = aws.pricing.get_product(service_code="AmazonRedshift",
    filters=[
        {
            "field": "instanceType",
            "value": "ds1.xlarge",
        },
        {
            "field": "location",
            "value": "US East (N. Virginia)",
        },
    ])
```
```csharp
using System.Collections.Generic;
using System.Linq;
using Pulumi;
using Aws = Pulumi.Aws;

return await Deployment.RunAsync(() => 
{
    var example = Aws.Pricing.GetProduct.Invoke(new()
    {
        ServiceCode = "AmazonRedshift",
        Filters = new[]
        {
            new Aws.Pricing.Inputs.GetProductFilterInputArgs
            {
                Field = "instanceType",
                Value = "ds1.xlarge",
            },
            new Aws.Pricing.Inputs.GetProductFilterInputArgs
            {
                Field = "location",
                Value = "US East (N. Virginia)",
            },
        },
    });

});
```
```go
package main

import (
	"github.com/pulumi/pulumi-aws/sdk/v6/go/aws/pricing"
	"github.com/pulumi/pulumi/sdk/v3/go/pulumi"
)

func main() {
	pulumi.Run(func(ctx *pulumi.Context) error {
		_, err := pricing.GetProduct(ctx, &pricing.GetProductArgs{
			ServiceCode: "AmazonRedshift",
			Filters: []pricing.GetProductFilter{
				{
					Field: "instanceType",
					Value: "ds1.xlarge",
				},
				{
					Field: "location",
					Value: "US East (N. Virginia)",
				},
			},
		}, nil)
		if err != nil {
			return err
		}
		return nil
	})
}
```
```java
package generated_program;

import com.pulumi.Context;
import com.pulumi.Pulumi;
import com.pulumi.core.Output;
import com.pulumi.aws.pricing.PricingFunctions;
import com.pulumi.aws.pricing.inputs.GetProductArgs;
import java.util.List;
import java.util.ArrayList;
import java.util.Map;
import java.io.File;
import java.nio.file.Files;
import java.nio.file.Paths;

public class App {
    public static void main(String[] args) {
        Pulumi.run(App::stack);
    }

    public static void stack(Context ctx) {
        final var example = PricingFunctions.getProduct(GetProductArgs.builder()
            .serviceCode("AmazonRedshift")
            .filters(            
                GetProductFilterArgs.builder()
                    .field("instanceType")
                    .value("ds1.xlarge")
                    .build(),
                GetProductFilterArgs.builder()
                    .field("location")
                    .value("US East (N. Virginia)")
                    .build())
            .build());

    }
}
```
```yaml
variables:
  example:
    fn::invoke:
      function: aws:pricing:getProduct
      arguments:
        serviceCode: AmazonRedshift
        filters:
          - field: instanceType
            value: ds1.xlarge
          - field: location
            value: US East (N. Virginia)
```
<!--End PulumiCodeChooser -->
�
filtersP*N:L
J
pricinggetProductFilter-aws:pricing/getProductFilter:getProductFilter�List of filters. Passed directly to the API (see GetProducts API reference). These filters must describe a single product, this resource will fail if more than one product is returned by the API.
|
serviceCode" iCode of the service. Available service codes can be fetched using the DescribeServices pricing API call.
"[
filtersP*N:L
J
pricinggetProductFilter-aws:pricing/getProductFilter:getProductFilter"E
id" ;The provider-assigned unique ID for this managed resource.
"8
result" *Set to the product returned from the API.
"
serviceCode" 2�
/
qldb	getLedgeraws:qldb/getLedger:getLedger�Use this data source to fetch information about a Quantum Ledger Database.

## Example Usage

<!--Start PulumiCodeChooser -->
```typescript
import * as pulumi from "@pulumi/pulumi";
import * as aws from "@pulumi/aws";

const example = aws.qldb.getLedger({
    name: "an_example_ledger",
});
```
```python
import pulumi
import pulumi_aws as aws

example = aws.qldb.get_ledger(name="an_example_ledger")
```
```csharp
using System.Collections.Generic;
using System.Linq;
using Pulumi;
using Aws = Pulumi.Aws;

return await Deployment.RunAsync(() => 
{
    var example = Aws.Qldb.GetLedger.Invoke(new()
    {
        Name = "an_example_ledger",
    });

});
```
```go
package main

import (
	"github.com/pulumi/pulumi-aws/sdk/v6/go/aws/qldb"
	"github.com/pulumi/pulumi/sdk/v3/go/pulumi"
)

func main() {
	pulumi.Run(func(ctx *pulumi.Context) error {
		_, err := qldb.LookupLedger(ctx, &qldb.LookupLedgerArgs{
			Name: "an_example_ledger",
		}, nil)
		if err != nil {
			return err
		}
		return nil
	})
}
```
```java
package generated_program;

import com.pulumi.Context;
import com.pulumi.Pulumi;
import com.pulumi.core.Output;
import com.pulumi.aws.qldb.QldbFunctions;
import com.pulumi.aws.qldb.inputs.GetLedgerArgs;
import java.util.List;
import java.util.ArrayList;
import java.util.Map;
import java.io.File;
import java.nio.file.Files;
import java.nio.file.Paths;

public class App {
    public static void main(String[] args) {
        Pulumi.run(App::stack);
    }

    public static void stack(Context ctx) {
        final var example = QldbFunctions.getLedger(GetLedgerArgs.builder()
            .name("an_example_ledger")
            .build());

    }
}
```
```yaml
variables:
  example:
    fn::invoke:
      function: aws:qldb:getLedger
      arguments:
        name: an_example_ledger
```
<!--End PulumiCodeChooser -->
2
name" &Friendly name of the ledger to match.

tagsB2" "	
arn" "
deletionProtection
 "E
id" ;The provider-assigned unique ID for this managed resource.
"
kmsKey" "

name" "
permissionsMode" "
tags2" 2�
A

quicksightgetAnalysis&aws:quicksight/getAnalysis:getAnalysis�Data source for managing an AWS QuickSight Analysis.

## Example Usage

### Basic Usage

<!--Start PulumiCodeChooser -->
```typescript
import * as pulumi from "@pulumi/pulumi";
import * as aws from "@pulumi/aws";

const example = aws.quicksight.getQuicksightAnalysis({
    analysisId: "example-id",
});
```
```python
import pulumi
import pulumi_aws as aws

example = aws.quicksight.get_quicksight_analysis(analysis_id="example-id")
```
```csharp
using System.Collections.Generic;
using System.Linq;
using Pulumi;
using Aws = Pulumi.Aws;

return await Deployment.RunAsync(() => 
{
    var example = Aws.Quicksight.GetQuicksightAnalysis.Invoke(new()
    {
        AnalysisId = "example-id",
    });

});
```
```go
package main

import (
	"github.com/pulumi/pulumi-aws/sdk/v6/go/aws/quicksight"
	"github.com/pulumi/pulumi/sdk/v3/go/pulumi"
)

func main() {
	pulumi.Run(func(ctx *pulumi.Context) error {
		_, err := quicksight.GetQuicksightAnalysis(ctx, &quicksight.GetQuicksightAnalysisArgs{
			AnalysisId: "example-id",
		}, nil)
		if err != nil {
			return err
		}
		return nil
	})
}
```
```java
package generated_program;

import com.pulumi.Context;
import com.pulumi.Pulumi;
import com.pulumi.core.Output;
import com.pulumi.aws.quicksight.QuicksightFunctions;
import com.pulumi.aws.quicksight.inputs.GetQuicksightAnalysisArgs;
import java.util.List;
import java.util.ArrayList;
import java.util.Map;
import java.io.File;
import java.nio.file.Files;
import java.nio.file.Paths;

public class App {
    public static void main(String[] args) {
        Pulumi.run(App::stack);
    }

    public static void stack(Context ctx) {
        final var example = QuicksightFunctions.getQuicksightAnalysis(GetQuicksightAnalysisArgs.builder()
            .analysisId("example-id")
            .build());

    }
}
```
```yaml
variables:
  example:
    fn::invoke:
      function: aws:quicksight:getQuicksightAnalysis
      arguments:
        analysisId: example-id
```
<!--End PulumiCodeChooser -->
V

analysisId" DIdentifier for the analysis.

The following arguments are optional:
&
awsAccountIdB" AWS account ID.

tagsB2" "

analysisId" "	
arn" "
awsAccountId" "
createdTime" "E
id" ;The provider-assigned unique ID for this managed resource.
"
lastPublishedTime" "
lastUpdatedTime" "

name" "t
permissionse*c:a
_

quicksightgetAnalysisPermission:aws:quicksight/getAnalysisPermission:getAnalysisPermission"
status" "
tags2" "
themeArn" 2�
>

quicksight
getDataSet$aws:quicksight/getDataSet:getDataSet�Data source for managing a QuickSight Data Set.

## Example Usage

### Basic Usage

<!--Start PulumiCodeChooser -->
```typescript
import * as pulumi from "@pulumi/pulumi";
import * as aws from "@pulumi/aws";

const example = aws.quicksight.getDataSet({
    dataSetId: "example-id",
});
```
```python
import pulumi
import pulumi_aws as aws

example = aws.quicksight.get_data_set(data_set_id="example-id")
```
```csharp
using System.Collections.Generic;
using System.Linq;
using Pulumi;
using Aws = Pulumi.Aws;

return await Deployment.RunAsync(() => 
{
    var example = Aws.Quicksight.GetDataSet.Invoke(new()
    {
        DataSetId = "example-id",
    });

});
```
```go
package main

import (
	"github.com/pulumi/pulumi-aws/sdk/v6/go/aws/quicksight"
	"github.com/pulumi/pulumi/sdk/v3/go/pulumi"
)

func main() {
	pulumi.Run(func(ctx *pulumi.Context) error {
		_, err := quicksight.LookupDataSet(ctx, &quicksight.LookupDataSetArgs{
			DataSetId: "example-id",
		}, nil)
		if err != nil {
			return err
		}
		return nil
	})
}
```
```java
package generated_program;

import com.pulumi.Context;
import com.pulumi.Pulumi;
import com.pulumi.core.Output;
import com.pulumi.aws.quicksight.QuicksightFunctions;
import com.pulumi.aws.quicksight.inputs.GetDataSetArgs;
import java.util.List;
import java.util.ArrayList;
import java.util.Map;
import java.io.File;
import java.nio.file.Files;
import java.nio.file.Paths;

public class App {
    public static void main(String[] args) {
        Pulumi.run(App::stack);
    }

    public static void stack(Context ctx) {
        final var example = QuicksightFunctions.getDataSet(GetDataSetArgs.builder()
            .dataSetId("example-id")
            .build());

    }
}
```
```yaml
variables:
  example:
    fn::invoke:
      function: aws:quicksight:getDataSet
      arguments:
        dataSetId: example-id
```
<!--End PulumiCodeChooser -->
&
awsAccountIdB" AWS account ID.
U
	dataSetId" DIdentifier for the data set.

The following arguments are optional:

tagsB2" 
tagsAllB2" "	
arn" "
awsAccountId" "u
columnGroupse*c:a
_

quicksightgetDataSetColumnGroup:aws:quicksight/getDataSetColumnGroup:getDataSetColumnGroup"�
columnLevelPermissionRules�*�:�
�

quicksight#getDataSetColumnLevelPermissionRuleVaws:quicksight/getDataSetColumnLevelPermissionRule:getDataSetColumnLevelPermissionRule"
	dataSetId" "�
dataSetUsageConfigurations�*�:�
�

quicksight#getDataSetDataSetUsageConfigurationVaws:quicksight/getDataSetDataSetUsageConfiguration:getDataSetDataSetUsageConfiguration"u
fieldFolderse*c:a
_

quicksightgetDataSetFieldFolder:aws:quicksight/getDataSetFieldFolder:getDataSetFieldFolder"E
id" ;The provider-assigned unique ID for this managed resource.
"

importMode" "�
logicalTableMapsq*o:m
k

quicksightgetDataSetLogicalTableMapBaws:quicksight/getDataSetLogicalTableMap:getDataSetLogicalTableMap"

name" "q
permissionsb*`:^
\

quicksightgetDataSetPermission8aws:quicksight/getDataSetPermission:getDataSetPermission"�
physicalTableMapst*r:p
n

quicksightgetDataSetPhysicalTableMapDaws:quicksight/getDataSetPhysicalTableMap:getDataSetPhysicalTableMap"�
rowLevelPermissionDataSets�*�:�
�

quicksight#getDataSetRowLevelPermissionDataSetVaws:quicksight/getDataSetRowLevelPermissionDataSet:getDataSetRowLevelPermissionDataSet"�
#rowLevelPermissionTagConfigurations�*�:�
�

quicksight,getDataSetRowLevelPermissionTagConfigurationhaws:quicksight/getDataSetRowLevelPermissionTagConfiguration:getDataSetRowLevelPermissionTagConfiguration"
tags2" "
tagsAll2" 2�
_

quicksightgetQuicksightAnalysis:aws:quicksight/getQuicksightAnalysis:getQuicksightAnalysis�Data source for managing an AWS QuickSight Analysis.

## Example Usage

### Basic Usage

<!--Start PulumiCodeChooser -->
```typescript
import * as pulumi from "@pulumi/pulumi";
import * as aws from "@pulumi/aws";

const example = aws.quicksight.getQuicksightAnalysis({
    analysisId: "example-id",
});
```
```python
import pulumi
import pulumi_aws as aws

example = aws.quicksight.get_quicksight_analysis(analysis_id="example-id")
```
```csharp
using System.Collections.Generic;
using System.Linq;
using Pulumi;
using Aws = Pulumi.Aws;

return await Deployment.RunAsync(() => 
{
    var example = Aws.Quicksight.GetQuicksightAnalysis.Invoke(new()
    {
        AnalysisId = "example-id",
    });

});
```
```go
package main

import (
	"github.com/pulumi/pulumi-aws/sdk/v6/go/aws/quicksight"
	"github.com/pulumi/pulumi/sdk/v3/go/pulumi"
)

func main() {
	pulumi.Run(func(ctx *pulumi.Context) error {
		_, err := quicksight.GetQuicksightAnalysis(ctx, &quicksight.GetQuicksightAnalysisArgs{
			AnalysisId: "example-id",
		}, nil)
		if err != nil {
			return err
		}
		return nil
	})
}
```
```java
package generated_program;

import com.pulumi.Context;
import com.pulumi.Pulumi;
import com.pulumi.core.Output;
import com.pulumi.aws.quicksight.QuicksightFunctions;
import com.pulumi.aws.quicksight.inputs.GetQuicksightAnalysisArgs;
import java.util.List;
import java.util.ArrayList;
import java.util.Map;
import java.io.File;
import java.nio.file.Files;
import java.nio.file.Paths;

public class App {
    public static void main(String[] args) {
        Pulumi.run(App::stack);
    }

    public static void stack(Context ctx) {
        final var example = QuicksightFunctions.getQuicksightAnalysis(GetQuicksightAnalysisArgs.builder()
            .analysisId("example-id")
            .build());

    }
}
```
```yaml
variables:
  example:
    fn::invoke:
      function: aws:quicksight:getQuicksightAnalysis
      arguments:
        analysisId: example-id
```
<!--End PulumiCodeChooser -->
V

analysisId" DIdentifier for the analysis.

The following arguments are optional:
&
awsAccountIdB" AWS account ID.

tagsB2" "

analysisId" "	
arn" "
awsAccountId" "
createdTime" "E
id" ;The provider-assigned unique ID for this managed resource.
"
lastPublishedTime" "
lastUpdatedTime" "

name" "�
permissions�*�:
}

quicksightgetQuicksightAnalysisPermissionNaws:quicksight/getQuicksightAnalysisPermission:getQuicksightAnalysisPermission"
status" "
tags2" "
themeArn" 2�
V

quicksightgetQuicksightGroup4aws:quicksight/getQuicksightGroup:getQuicksightGroup�This data source can be used to fetch information about a specific
QuickSight group. By using this data source, you can reference QuickSight group
properties without having to hard code ARNs or unique IDs as input.

## Example Usage

### Basic Usage

<!--Start PulumiCodeChooser -->
```typescript
import * as pulumi from "@pulumi/pulumi";
import * as aws from "@pulumi/aws";

const example = aws.quicksight.getQuicksightGroup({
    groupName: "example",
});
```
```python
import pulumi
import pulumi_aws as aws

example = aws.quicksight.get_quicksight_group(group_name="example")
```
```csharp
using System.Collections.Generic;
using System.Linq;
using Pulumi;
using Aws = Pulumi.Aws;

return await Deployment.RunAsync(() => 
{
    var example = Aws.Quicksight.GetQuicksightGroup.Invoke(new()
    {
        GroupName = "example",
    });

});
```
```go
package main

import (
	"github.com/pulumi/pulumi-aws/sdk/v6/go/aws/quicksight"
	"github.com/pulumi/pulumi/sdk/v3/go/pulumi"
)

func main() {
	pulumi.Run(func(ctx *pulumi.Context) error {
		_, err := quicksight.GetQuicksightGroup(ctx, &quicksight.GetQuicksightGroupArgs{
			GroupName: "example",
		}, nil)
		if err != nil {
			return err
		}
		return nil
	})
}
```
```java
package generated_program;

import com.pulumi.Context;
import com.pulumi.Pulumi;
import com.pulumi.core.Output;
import com.pulumi.aws.quicksight.QuicksightFunctions;
import com.pulumi.aws.quicksight.inputs.GetQuicksightGroupArgs;
import java.util.List;
import java.util.ArrayList;
import java.util.Map;
import java.io.File;
import java.nio.file.Files;
import java.nio.file.Paths;

public class App {
    public static void main(String[] args) {
        Pulumi.run(App::stack);
    }

    public static void stack(Context ctx) {
        final var example = QuicksightFunctions.getQuicksightGroup(GetQuicksightGroupArgs.builder()
            .groupName("example")
            .build());

    }
}
```
```yaml
variables:
  example:
    fn::invoke:
      function: aws:quicksight:getQuicksightGroup
      arguments:
        groupName: example
```
<!--End PulumiCodeChooser -->
&
awsAccountIdB" AWS account ID.
f
	groupName" UThe name of the group that you want to match.

The following arguments are optional:
@
	namespaceB" -QuickSight namespace. Defaults to `default`.
"9
arn" .The Amazon Resource Name (ARN) for the group.
"
awsAccountId" "*
description" The group description.
"
	groupName" "E
id" ;The provider-assigned unique ID for this managed resource.
"
	namespaceB" "2
principalId" The principal ID of the group.
2�
S

quicksightgetQuicksightUser2aws:quicksight/getQuicksightUser:getQuicksightUser�This data source can be used to fetch information about a specific
QuickSight user. By using this data source, you can reference QuickSight user
properties without having to hard code ARNs or unique IDs as input.

## Example Usage

### Basic Usage

<!--Start PulumiCodeChooser -->
```typescript
import * as pulumi from "@pulumi/pulumi";
import * as aws from "@pulumi/aws";

const example = aws.quicksight.getQuicksightUser({
    userName: "example",
});
```
```python
import pulumi
import pulumi_aws as aws

example = aws.quicksight.get_quicksight_user(user_name="example")
```
```csharp
using System.Collections.Generic;
using System.Linq;
using Pulumi;
using Aws = Pulumi.Aws;

return await Deployment.RunAsync(() => 
{
    var example = Aws.Quicksight.GetQuicksightUser.Invoke(new()
    {
        UserName = "example",
    });

});
```
```go
package main

import (
	"github.com/pulumi/pulumi-aws/sdk/v6/go/aws/quicksight"
	"github.com/pulumi/pulumi/sdk/v3/go/pulumi"
)

func main() {
	pulumi.Run(func(ctx *pulumi.Context) error {
		_, err := quicksight.GetQuicksightUser(ctx, &quicksight.GetQuicksightUserArgs{
			UserName: "example",
		}, nil)
		if err != nil {
			return err
		}
		return nil
	})
}
```
```java
package generated_program;

import com.pulumi.Context;
import com.pulumi.Pulumi;
import com.pulumi.core.Output;
import com.pulumi.aws.quicksight.QuicksightFunctions;
import com.pulumi.aws.quicksight.inputs.GetQuicksightUserArgs;
import java.util.List;
import java.util.ArrayList;
import java.util.Map;
import java.io.File;
import java.nio.file.Files;
import java.nio.file.Paths;

public class App {
    public static void main(String[] args) {
        Pulumi.run(App::stack);
    }

    public static void stack(Context ctx) {
        final var example = QuicksightFunctions.getQuicksightUser(GetQuicksightUserArgs.builder()
            .userName("example")
            .build());

    }
}
```
```yaml
variables:
  example:
    fn::invoke:
      function: aws:quicksight:getQuicksightUser
      arguments:
        userName: example
```
<!--End PulumiCodeChooser -->
&
awsAccountIdB" AWS account ID.
@
	namespaceB" -QuickSight namespace. Defaults to `default`.
d
userName" TThe name of the user that you want to match.

The following arguments are optional:
"�
active
 �The active status of user. When you create an Amazon QuickSight user that’s not an IAM user or an Active Directory user, that user is inactive until they sign in and provide a password.
"8
arn" -The Amazon Resource Name (ARN) for the user.
"
awsAccountId" "'
email" The user's email address.
"E
id" ;The provider-assigned unique ID for this managed resource.
"J
identityType" 6The type of identity authentication used by the user.
"
	namespaceB" "1
principalId" The principal ID of the user.
"
userName" "�
userRole" �The Amazon QuickSight role for the user. The user role can be one of the following:.
- `READER`: A user who has read-only access to dashboards.
- `AUTHOR`: A user who can create data sources, datasets, analyzes, and dashboards.
- `ADMIN`: A user who is an author, who can also manage Amazon QuickSight settings.
2�
8

quicksightgetTheme aws:quicksight/getTheme:getTheme�Data source for managing an AWS QuickSight Theme.

## Example Usage

### Basic Usage

<!--Start PulumiCodeChooser -->
```typescript
import * as pulumi from "@pulumi/pulumi";
import * as aws from "@pulumi/aws";

const example = aws.quicksight.getTheme({
    themeId: "example",
});
```
```python
import pulumi
import pulumi_aws as aws

example = aws.quicksight.get_theme(theme_id="example")
```
```csharp
using System.Collections.Generic;
using System.Linq;
using Pulumi;
using Aws = Pulumi.Aws;

return await Deployment.RunAsync(() => 
{
    var example = Aws.Quicksight.GetTheme.Invoke(new()
    {
        ThemeId = "example",
    });

});
```
```go
package main

import (
	"github.com/pulumi/pulumi-aws/sdk/v6/go/aws/quicksight"
	"github.com/pulumi/pulumi/sdk/v3/go/pulumi"
)

func main() {
	pulumi.Run(func(ctx *pulumi.Context) error {
		_, err := quicksight.LookupTheme(ctx, &quicksight.LookupThemeArgs{
			ThemeId: "example",
		}, nil)
		if err != nil {
			return err
		}
		return nil
	})
}
```
```java
package generated_program;

import com.pulumi.Context;
import com.pulumi.Pulumi;
import com.pulumi.core.Output;
import com.pulumi.aws.quicksight.QuicksightFunctions;
import com.pulumi.aws.quicksight.inputs.GetThemeArgs;
import java.util.List;
import java.util.ArrayList;
import java.util.Map;
import java.io.File;
import java.nio.file.Files;
import java.nio.file.Paths;

public class App {
    public static void main(String[] args) {
        Pulumi.run(App::stack);
    }

    public static void stack(Context ctx) {
        final var example = QuicksightFunctions.getTheme(GetThemeArgs.builder()
            .themeId("example")
            .build());

    }
}
```
```yaml
variables:
  example:
    fn::invoke:
      function: aws:quicksight:getTheme
      arguments:
        themeId: example
```
<!--End PulumiCodeChooser -->
&
awsAccountIdB" AWS account ID.
�
tagsB2" xA map of tags assigned to the resource, including those inherited from the provider `default_tags` configuration block.
O
themeId" @Identifier of the theme.

The following arguments are optional:
"
arn" ARN of the theme.
"
awsAccountId" "�
baseThemeId" �The ID of the theme that a custom theme will inherit from. All themes inherit from one of the starting themes defined by Amazon QuickSight.
"�
configurationse*c:a
_

quicksightgetThemeConfiguration:aws:quicksight/getThemeConfiguration:getThemeConfigurationYThe theme configuration, which contains the theme display properties. See configuration.
"8
createdTime" %The time that the theme was created.
"E
id" ;The provider-assigned unique ID for this managed resource.
"A
lastUpdatedTime" *The time that the theme was last updated.
"'
name" Display name of the theme.
"�
permissions\*Z:X
V

quicksightgetThemePermission4aws:quicksight/getThemePermission:getThemePermission=A set of resource permissions on the theme. See permissions.
")
status" The theme creation status.
"�
tags2" xA map of tags assigned to the resource, including those inherited from the provider `default_tags` configuration block.
"
themeId" "\
versionDescription" BA description of the current theme version being created/updated.
">
versionNumber )The version number of the theme version.
2�.
B
ramgetResourceShare)aws:ram/getResourceShare:getResourceShare�$`aws.ram.ResourceShare` Retrieve information about a RAM Resource Share.

## Example Usage

<!--Start PulumiCodeChooser -->
```typescript
import * as pulumi from "@pulumi/pulumi";
import * as aws from "@pulumi/aws";

const example = aws.ram.getResourceShare({
    name: "example",
    resourceOwner: "SELF",
});
```
```python
import pulumi
import pulumi_aws as aws

example = aws.ram.get_resource_share(name="example",
    resource_owner="SELF")
```
```csharp
using System.Collections.Generic;
using System.Linq;
using Pulumi;
using Aws = Pulumi.Aws;

return await Deployment.RunAsync(() => 
{
    var example = Aws.Ram.GetResourceShare.Invoke(new()
    {
        Name = "example",
        ResourceOwner = "SELF",
    });

});
```
```go
package main

import (
	"github.com/pulumi/pulumi-aws/sdk/v6/go/aws/ram"
	"github.com/pulumi/pulumi/sdk/v3/go/pulumi"
)

func main() {
	pulumi.Run(func(ctx *pulumi.Context) error {
		_, err := ram.LookupResourceShare(ctx, &ram.LookupResourceShareArgs{
			Name:          pulumi.StringRef("example"),
			ResourceOwner: "SELF",
		}, nil)
		if err != nil {
			return err
		}
		return nil
	})
}
```
```java
package generated_program;

import com.pulumi.Context;
import com.pulumi.Pulumi;
import com.pulumi.core.Output;
import com.pulumi.aws.ram.RamFunctions;
import com.pulumi.aws.ram.inputs.GetResourceShareArgs;
import java.util.List;
import java.util.ArrayList;
import java.util.Map;
import java.io.File;
import java.nio.file.Files;
import java.nio.file.Paths;

public class App {
    public static void main(String[] args) {
        Pulumi.run(App::stack);
    }

    public static void stack(Context ctx) {
        final var example = RamFunctions.getResourceShare(GetResourceShareArgs.builder()
            .name("example")
            .resourceOwner("SELF")
            .build());

    }
}
```
```yaml
variables:
  example:
    fn::invoke:
      function: aws:ram:getResourceShare
      arguments:
        name: example
        resourceOwner: SELF
```
<!--End PulumiCodeChooser -->

## Search by filters

<!--Start PulumiCodeChooser -->
```typescript
import * as pulumi from "@pulumi/pulumi";
import * as aws from "@pulumi/aws";

const tagFilter = aws.ram.getResourceShare({
    resourceOwner: "SELF",
    filters: [{
        name: "NameOfTag",
        values: ["exampleNameTagValue"],
    }],
});
```
```python
import pulumi
import pulumi_aws as aws

tag_filter = aws.ram.get_resource_share(resource_owner="SELF",
    filters=[{
        "name": "NameOfTag",
        "values": ["exampleNameTagValue"],
    }])
```
```csharp
using System.Collections.Generic;
using System.Linq;
using Pulumi;
using Aws = Pulumi.Aws;

return await Deployment.RunAsync(() => 
{
    var tagFilter = Aws.Ram.GetResourceShare.Invoke(new()
    {
        ResourceOwner = "SELF",
        Filters = new[]
        {
            new Aws.Ram.Inputs.GetResourceShareFilterInputArgs
            {
                Name = "NameOfTag",
                Values = new[]
                {
                    "exampleNameTagValue",
                },
            },
        },
    });

});
```
```go
package main

import (
	"github.com/pulumi/pulumi-aws/sdk/v6/go/aws/ram"
	"github.com/pulumi/pulumi/sdk/v3/go/pulumi"
)

func main() {
	pulumi.Run(func(ctx *pulumi.Context) error {
		_, err := ram.LookupResourceShare(ctx, &ram.LookupResourceShareArgs{
			ResourceOwner: "SELF",
			Filters: []ram.GetResourceShareFilter{
				{
					Name: "NameOfTag",
					Values: []string{
						"exampleNameTagValue",
					},
				},
			},
		}, nil)
		if err != nil {
			return err
		}
		return nil
	})
}
```
```java
package generated_program;

import com.pulumi.Context;
import com.pulumi.Pulumi;
import com.pulumi.core.Output;
import com.pulumi.aws.ram.RamFunctions;
import com.pulumi.aws.ram.inputs.GetResourceShareArgs;
import java.util.List;
import java.util.ArrayList;
import java.util.Map;
import java.io.File;
import java.nio.file.Files;
import java.nio.file.Paths;

public class App {
    public static void main(String[] args) {
        Pulumi.run(App::stack);
    }

    public static void stack(Context ctx) {
        final var tagFilter = RamFunctions.getResourceShare(GetResourceShareArgs.builder()
            .resourceOwner("SELF")
            .filters(GetResourceShareFilterArgs.builder()
                .name("NameOfTag")
                .values("exampleNameTagValue")
                .build())
            .build());

    }
}
```
```yaml
variables:
  tagFilter:
    fn::invoke:
      function: aws:ram:getResourceShare
      arguments:
        resourceOwner: SELF
        filters:
          - name: NameOfTag
            values:
              - exampleNameTagValue
```
<!--End PulumiCodeChooser -->
�
filters\BZ*X:V
T
ramgetResourceShareFilter5aws:ram/getResourceShareFilter:getResourceShareFilter�Filter used to scope the list e.g., by tags. See [related docs] (https://docs.aws.amazon.com/ram/latest/APIReference/API_TagFilter.html).
6
nameB" (Name of the resource share to retrieve.
_
resourceOwner" JOwner of the resource share. Valid values are `SELF` or `OTHER-ACCOUNTS`.
�
resourceShareStatusB" �Specifies that you want to retrieve details of only those resource shares that have this status. Valid values are `PENDING`, `ACTIVE`, `FAILED`, `DELETING`, and `DELETED`.
5
tagsB2" %Tags attached to the resource share.
"&
arn" ARN of the resource share.
"g
filters\BZ*X:V
T
ramgetResourceShareFilter5aws:ram/getResourceShareFilter:getResourceShareFilter"E
id" ;The provider-assigned unique ID for this managed resource.
"

name" "K
owningAccountId" 4ID of the AWS account that owns the resource share.
"R
resourceArns*" <A list of resource ARNs associated with the resource share.
"
resourceOwner" "
resourceShareStatusB" ",
status" Status of the resource share.
"3
tags2" %Tags attached to the resource share.
:�
ZgetAvailabilityZoneFilter=aws:index/getAvailabilityZoneFilter:getAvailabilityZoneFilter�
��
name" �Name of the filter field. Valid values can be found in the [EC2 DescribeAvailabilityZones API Reference](https://docs.aws.amazon.com/AWSEC2/latest/APIReference/API_DescribeAvailabilityZones.html).
�
values*" qSet of values that are accepted for the given filter field. Results will be selected if any given value matches.
:�
]getAvailabilityZonesFilter?aws:index/getAvailabilityZonesFilter:getAvailabilityZonesFilter�
��
name" �Name of the filter field. Valid values can be found in the [EC2 DescribeAvailabilityZones API Reference](https://docs.aws.amazon.com/AWSEC2/latest/APIReference/API_DescribeAvailabilityZones.html).
�
values*" qSet of values that are accepted for the given filter field. Results will be selected if any given value matches.
:�
?getRegionsFilter+aws:index/getRegionsFilter:getRegionsFilter�
�p
name" dName of the filter field. Valid values can be found in the [describe-regions AWS CLI Reference][1].
�
values*" qSet of values that are accepted for the given filter field. Results will be selected if any given value matches.
:�
b
paymentcryptographyKeyKeyAttributes9aws:paymentcryptography/KeyKeyAttributes:KeyKeyAttributes�
�`
keyAlgorithm" LKey algorithm to be use during creation of an AWS Payment Cryptography key.
@
keyClass" 0Type of AWS Payment Cryptography key to create.
�
keyModesOfUse�B�:�
�
paymentcryptographyKeyKeyAttributesKeyModesOfUseSaws:paymentcryptography/KeyKeyAttributesKeyModesOfUse:KeyKeyAttributesKeyModesOfUseEList of cryptographic operations that you can perform using the key.
v
keyUsage" fCryptographic usage of an AWS Payment Cryptography key as defined in section A.5.2 of the TR-31 spec.
:�
�
paymentcryptographyKeyKeyAttributesKeyModesOfUseSaws:paymentcryptography/KeyKeyAttributesKeyModesOfUse:KeyKeyAttributesKeyModesOfUse�
�V
decryptB
 EWhether an AWS Payment Cryptography key can be used to decrypt data.
[
	deriveKeyB
 HWhether an AWS Payment Cryptography key can be used to derive new keys.
V
encryptB
 EWhether an AWS Payment Cryptography key can be used to encrypt data.
�
generateB
 qWhether an AWS Payment Cryptography key can be used to generate and verify other card and PIN verification keys.
�
noRestrictionsB
 uWhether an AWS Payment Cryptography key has no special restrictions other than the restrictions implied by KeyUsage.
O
signB
 AWhether an AWS Payment Cryptography key can be used for signing.
Z
unwrapB
 JWhether an AWS Payment Cryptography key can be used to unwrap other keys.
Z
verifyB
 JWhether an AWS Payment Cryptography key can be used to verify signatures.
V
wrapB
 HWhether an AWS Payment Cryptography key can be used to wrap other keys.
:�
S
paymentcryptographyKeyTimeouts/aws:paymentcryptography/KeyTimeouts:KeyTimeouts�
��
createB" �A string that can be [parsed as a duration](https://pkg.go.dev/time#ParseDuration) consisting of numbers and unit suffixes, such as "30s" or "2h45m". Valid time units are "s" (seconds), "m" (minutes), "h" (hours).
�
deleteB" �A string that can be [parsed as a duration](https://pkg.go.dev/time#ParseDuration) consisting of numbers and unit suffixes, such as "30s" or "2h45m". Valid time units are "s" (seconds), "m" (minutes), "h" (hours). Setting a timeout for a Delete operation is only applicable if changes are saved into state before the destroy operation occurs.
�
updateB" �A string that can be [parsed as a duration](https://pkg.go.dev/time#ParseDuration) consisting of numbers and unit suffixes, such as "30s" or "2h45m". Valid time units are "s" (seconds), "m" (minutes), "h" (hours).
:�
I
pinpointAppCampaignHook,aws:pinpoint/AppCampaignHook:AppCampaignHook�
�l
lambdaFunctionNameB" PLambda function name or ARN to be called for delivery. Conflicts with `web_url`
o
modeB" aWhat mode Lambda should be invoked in. Valid values for this parameter are `DELIVERY`, `FILTER`.
�
webUrlB" �Web URL to call for hook. If the URL has authentication specified it will be added as authentication to the request. Conflicts with `lambda_function_name`
:�
7
pinpoint	AppLimits aws:pinpoint/AppLimits:AppLimits�
�P
dailyB AThe maximum number of messages that the campaign can send daily.
�
maximumDurationB �The length of time (in seconds) that the campaign can run before it ends and message deliveries stop. This duration begins at the scheduled start time for the campaign. The minimum value is 60.
�
messagesPerSecondB qThe number of messages that the campaign can send per second. The minimum value is 50, and the maximum is 20000.
P
totalB AThe maximum total number of messages that the campaign can send.
:�
@
pinpointAppQuietTime&aws:pinpoint/AppQuietTime:AppQuietTime�
�`
endB" SThe default end time for quiet time in ISO 8601 format. Required if `start` is set
b
startB" SThe default start time for quiet time in ISO 8601 format. Required if `end` is set
:�
j
pinpointEmailTemplateEmailTemplateBaws:pinpoint/EmailTemplateEmailTemplate:EmailTemplateEmailTemplate�
��
defaultSubstitutionsB" �JSON object that specifies the default values to use for message variables in the message template. This object is a set of key-value pairs. Each key defines a message variable in the template. The corresponding value defines the default value for that variable. When you create a message that's based on the template, you can override these defaults with message-specific and address-specific variables and values.

descriptionB" �
headers�B�*�:~
|
pinpoint EmailTemplateEmailTemplateHeaderNaws:pinpoint/EmailTemplateEmailTemplateHeader:EmailTemplateEmailTemplateHeader�
htmlPartB" �The message body, in HTML format, to use in email messages that are based on the message template. We recommend using HTML format for email clients that render HTML content. You can include links, formatted text, and more in an HTML message.
�
recommenderIdB" �The unique identifier for the recommender model to use for the message template. Amazon Pinpoint uses this value to determine how to retrieve and process data from a recommender model when it sends messages that use the template, if the template contains message variables for recommendation data.
j
subjectB" YSubject line, or title, to use in email messages that are based on the message template.
�
textPartB" �Message body, in plain text format, to use in email messages that are based on the message template. We recommend using plain text format for email clients that don't render HTML content and clients that are connected to high-latency networks, such as mobile devices.
:�
|
pinpoint EmailTemplateEmailTemplateHeaderNaws:pinpoint/EmailTemplateEmailTemplateHeader:EmailTemplateEmailTemplateHeader�
�\
nameB" NName of the message header. The header name can contain up to 126 characters.
�
valueB" �Value of the message header. The header value can contain up to 870 characters, including the length of any rendered attributes. For example if you add the {CreationDate} attribute, it renders as YYYY-MM-DDTHH:MM:SS.SSSZ and is 24 characters in length.
:�
s
pinpointSmsvoicev2PhoneNumberTimeoutsHaws:pinpoint/Smsvoicev2PhoneNumberTimeouts:Smsvoicev2PhoneNumberTimeouts�
��
createB" �A string that can be [parsed as a duration](https://pkg.go.dev/time#ParseDuration) consisting of numbers and unit suffixes, such as "30s" or "2h45m". Valid time units are "s" (seconds), "m" (minutes), "h" (hours).
�
deleteB" �A string that can be [parsed as a duration](https://pkg.go.dev/time#ParseDuration) consisting of numbers and unit suffixes, such as "30s" or "2h45m". Valid time units are "s" (seconds), "m" (minutes), "h" (hours). Setting a timeout for a Delete operation is only applicable if changes are saved into state before the destroy operation occurs.
�
updateB" �A string that can be [parsed as a duration](https://pkg.go.dev/time#ParseDuration) consisting of numbers and unit suffixes, such as "30s" or "2h45m". Valid time units are "s" (seconds), "m" (minutes), "h" (hours).
:�
^
pipesPipeEnrichmentParameters;aws:pipes/PipeEnrichmentParameters:PipeEnrichmentParameters�
��
httpParameters�B�:�
�
pipes&PipeEnrichmentParametersHttpParametersWaws:pipes/PipeEnrichmentParametersHttpParameters:PipeEnrichmentParametersHttpParameters�Contains the HTTP parameters to use when the target is a API Gateway REST endpoint or EventBridge ApiDestination. If you specify an API Gateway REST API or EventBridge ApiDestination as a target, you can use this parameter to specify headers, path parameters, and query string keys/values as part of your target invoking request. If you're using ApiDestinations, the corresponding Connection can also have these values configured. In case of any conflicting keys, values from the Connection take precedence. Detailed below.
�
inputTemplateB" �Valid JSON text passed to the target. In this case, nothing from the event itself is passed to the target. Maximum length of 8192 characters.
:�
�
pipes&PipeEnrichmentParametersHttpParametersWaws:pipes/PipeEnrichmentParametersHttpParameters:PipeEnrichmentParametersHttpParameters\
Z
headerParametersB2" 
pathParameterValuesB" 
queryStringParametersB2" :�

R
pipesPipeLogConfiguration3aws:pipes/PipeLogConfiguration:PipeLogConfiguration�	
�	�
cloudwatchLogsLogDestination�B�:�
�
pipes0PipeLogConfigurationCloudwatchLogsLogDestinationkaws:pipes/PipeLogConfigurationCloudwatchLogsLogDestination:PipeLogConfigurationCloudwatchLogsLogDestinationTAmazon CloudWatch Logs logging configuration settings for the pipe. Detailed below.
�
firehoseLogDestination�B�:�
�
pipes*PipeLogConfigurationFirehoseLogDestination_aws:pipes/PipeLogConfigurationFirehoseLogDestination:PipeLogConfigurationFirehoseLogDestinationZAmazon Kinesis Data Firehose logging configuration settings for the pipe. Detailed below.
�
includeExecutionDatasB*" �String list that specifies whether the execution data (specifically, the `payload`, `awsRequest`, and `awsResponse` fields) is included in the log messages for this pipe. This applies to all log destinations for the pipe. Valid values `ALL`.
f
level" YThe level of logging detail to include. Valid values `OFF`, `ERROR`, `INFO` and `TRACE`.
�
s3LogDestination�B�:�
�
pipes$PipeLogConfigurationS3LogDestinationSaws:pipes/PipeLogConfigurationS3LogDestination:PipeLogConfigurationS3LogDestinationGAmazon S3 logging configuration settings for the pipe. Detailed below.
:�
�
pipes0PipeLogConfigurationCloudwatchLogsLogDestinationkaws:pipes/PipeLogConfigurationCloudwatchLogsLogDestination:PipeLogConfigurationCloudwatchLogsLogDestination�
��
logGroupArn" qAmazon Web Services Resource Name (ARN) for the CloudWatch log group to which EventBridge sends the log records.
:�
�
pipes*PipeLogConfigurationFirehoseLogDestination_aws:pipes/PipeLogConfigurationFirehoseLogDestination:PipeLogConfigurationFirehoseLogDestination�
��
deliveryStreamArn" |Amazon Resource Name (ARN) of the Kinesis Data Firehose delivery stream to which EventBridge delivers the pipe log records.
:�
�
pipes$PipeLogConfigurationS3LogDestinationSaws:pipes/PipeLogConfigurationS3LogDestination:PipeLogConfigurationS3LogDestination�
�k

bucketName" YName of the Amazon S3 bucket to which EventBridge delivers the log records for the pipe.
�
bucketOwner" wAmazon Web Services account that owns the Amazon S3 bucket to which EventBridge delivers the log records for the pipe.
f
outputFormatB" PEventBridge format for the log records. Valid values `json`, `plain` and `w3c`.
L
prefixB" <Prefix text with which to begin Amazon S3 log object names.
:�
R
pipesPipeSourceParameters3aws:pipes/PipeSourceParameters:PipeSourceParameters�
��
activemqBrokerParameters�B�:�
�
pipes,PipeSourceParametersActivemqBrokerParameterscaws:pipes/PipeSourceParametersActivemqBrokerParameters:PipeSourceParametersActivemqBrokerParametersJThe parameters for using an Active MQ broker as a source. Detailed below.
�
dynamodbStreamParameters�B�:�
�
pipes,PipeSourceParametersDynamodbStreamParameterscaws:pipes/PipeSourceParametersDynamodbStreamParameters:PipeSourceParametersDynamodbStreamParametersIThe parameters for using a DynamoDB stream as a source.  Detailed below.
�
filterCriteria�B�:~
|
pipes"PipeSourceParametersFilterCriteriaOaws:pipes/PipeSourceParametersFilterCriteria:PipeSourceParametersFilterCriteria�The collection of event patterns used to [filter events](https://docs.aws.amazon.com/eventbridge/latest/userguide/eb-pipes-event-filtering.html). Detailed below.
�
kinesisStreamParameters�B�:�
�
pipes+PipeSourceParametersKinesisStreamParametersaaws:pipes/PipeSourceParametersKinesisStreamParameters:PipeSourceParametersKinesisStreamParametersGThe parameters for using a Kinesis stream as a source. Detailed below.
�
managedStreamingKafkaParameters�B�:�
�
pipes3PipeSourceParametersManagedStreamingKafkaParametersqaws:pipes/PipeSourceParametersManagedStreamingKafkaParameters:PipeSourceParametersManagedStreamingKafkaParametersDThe parameters for using an MSK stream as a source. Detailed below.
�
rabbitmqBrokerParameters�B�:�
�
pipes,PipeSourceParametersRabbitmqBrokerParameterscaws:pipes/PipeSourceParametersRabbitmqBrokerParameters:PipeSourceParametersRabbitmqBrokerParametersIThe parameters for using a Rabbit MQ broker as a source. Detailed below.
�
selfManagedKafkaParameters�B�:�
�
pipes.PipeSourceParametersSelfManagedKafkaParametersgaws:pipes/PipeSourceParametersSelfManagedKafkaParameters:PipeSourceParametersSelfManagedKafkaParametersYThe parameters for using a self-managed Apache Kafka stream as a source. Detailed below.
�
sqsQueueParameters�B�:�
�
pipes&PipeSourceParametersSqsQueueParametersWaws:pipes/PipeSourceParametersSqsQueueParameters:PipeSourceParametersSqsQueueParametersJThe parameters for using a Amazon SQS stream as a source. Detailed below.
:�
�
pipes,PipeSourceParametersActivemqBrokerParameterscaws:pipes/PipeSourceParametersActivemqBrokerParameters:PipeSourceParametersActivemqBrokerParameters�
�c
	batchSizeB PThe maximum number of records to include in each batch. Maximum value of 10000.
�
credentials�:�
�
pipes7PipeSourceParametersActivemqBrokerParametersCredentialsyaws:pipes/PipeSourceParametersActivemqBrokerParametersCredentials:PipeSourceParametersActivemqBrokerParametersCredentials?The credentials needed to access the resource. Detailed below.
o
maximumBatchingWindowInSecondsB GThe maximum length of a time to wait for events. Maximum value of 300.
W
	queueName" FThe name of the destination queue to consume. Maximum length of 1000.
:�
�
pipes7PipeSourceParametersActivemqBrokerParametersCredentialsyaws:pipes/PipeSourceParametersActivemqBrokerParametersCredentials:PipeSourceParametersActivemqBrokerParametersCredentialsW
US
	basicAuth" BThe ARN of the Secrets Manager secret containing the credentials.
:�
�
pipes,PipeSourceParametersDynamodbStreamParameterscaws:pipes/PipeSourceParametersDynamodbStreamParameters:PipeSourceParametersDynamodbStreamParameters�
�c
	batchSizeB PThe maximum number of records to include in each batch. Maximum value of 10000.
�
deadLetterConfig�B�:�
�
pipes<PipeSourceParametersDynamodbStreamParametersDeadLetterConfig�aws:pipes/PipeSourceParametersDynamodbStreamParametersDeadLetterConfig:PipeSourceParametersDynamodbStreamParametersDeadLetterConfigMDefine the target queue to send dead-letter queue events to. Detailed below.
o
maximumBatchingWindowInSecondsB GThe maximum length of a time to wait for events. Maximum value of 300.
�
maximumRecordAgeInSecondsB �Discard records older than the specified age. The default value is -1, which sets the maximum age to infinite. When the value is set to infinite, EventBridge never discards old records. Maximum value of 604,800.
�
maximumRetryAttemptsB �Discard records after the specified number of retries. The default value is -1, which sets the maximum number of retries to infinite. When MaximumRetryAttempts is infinite, EventBridge retries failed records until the record expires in the event source. Maximum value of 10,000.
�
onPartialBatchItemFailureB" �Define how to handle item process failures. AUTOMATIC_BISECT halves each batch and retry each half until all the records are processed or there is one failed message left in the batch. Valid values: AUTOMATIC_BISECT.
�
parallelizationFactorB lThe number of batches to process concurrently from each shard. The default value is 1. Maximum value of 10.
r
startingPosition" ZThe position in a stream from which to start reading. Valid values: TRIM_HORIZON, LATEST.
:�
�
pipes<PipeSourceParametersDynamodbStreamParametersDeadLetterConfig�aws:pipes/PipeSourceParametersDynamodbStreamParametersDeadLetterConfig:PipeSourceParametersDynamodbStreamParametersDeadLetterConfig#
!
arnB" ARN of this pipe.
:�
|
pipes"PipeSourceParametersFilterCriteriaOaws:pipes/PipeSourceParametersFilterCriteria:PipeSourceParametersFilterCriteria�
��
filters�B�*�:�
�
pipes(PipeSourceParametersFilterCriteriaFilter[aws:pipes/PipeSourceParametersFilterCriteriaFilter:PipeSourceParametersFilterCriteriaFilter4An array of up to 5 event patterns. Detailed below.
:�
�
pipes(PipeSourceParametersFilterCriteriaFilter[aws:pipes/PipeSourceParametersFilterCriteriaFilter:PipeSourceParametersFilterCriteriaFilter?
=;
pattern" ,The event pattern. At most 4096 characters.
:�
�
pipes+PipeSourceParametersKinesisStreamParametersaaws:pipes/PipeSourceParametersKinesisStreamParameters:PipeSourceParametersKinesisStreamParameters�
�c
	batchSizeB PThe maximum number of records to include in each batch. Maximum value of 10000.
�
deadLetterConfig�B�:�
�
pipes;PipeSourceParametersKinesisStreamParametersDeadLetterConfig�aws:pipes/PipeSourceParametersKinesisStreamParametersDeadLetterConfig:PipeSourceParametersKinesisStreamParametersDeadLetterConfigMDefine the target queue to send dead-letter queue events to. Detailed below.
o
maximumBatchingWindowInSecondsB GThe maximum length of a time to wait for events. Maximum value of 300.
�
maximumRecordAgeInSecondsB �Discard records older than the specified age. The default value is -1, which sets the maximum age to infinite. When the value is set to infinite, EventBridge never discards old records. Maximum value of 604,800.
�
maximumRetryAttemptsB �Discard records after the specified number of retries. The default value is -1, which sets the maximum number of retries to infinite. When MaximumRetryAttempts is infinite, EventBridge retries failed records until the record expires in the event source. Maximum value of 10,000.
�
onPartialBatchItemFailureB" �Define how to handle item process failures. AUTOMATIC_BISECT halves each batch and retry each half until all the records are processed or there is one failed message left in the batch. Valid values: AUTOMATIC_BISECT.
�
parallelizationFactorB lThe number of batches to process concurrently from each shard. The default value is 1. Maximum value of 10.
r
startingPosition" ZThe position in a stream from which to start reading. Valid values: TRIM_HORIZON, LATEST.
�
startingPositionTimestampB" gWith StartingPosition set to AT_TIMESTAMP, the time from which to start reading, in Unix time seconds.
:�
�
pipes;PipeSourceParametersKinesisStreamParametersDeadLetterConfig�aws:pipes/PipeSourceParametersKinesisStreamParametersDeadLetterConfig:PipeSourceParametersKinesisStreamParametersDeadLetterConfig#
!
arnB" ARN of this pipe.
:�
�
pipes3PipeSourceParametersManagedStreamingKafkaParametersqaws:pipes/PipeSourceParametersManagedStreamingKafkaParameters:PipeSourceParametersManagedStreamingKafkaParameters�
�c
	batchSizeB PThe maximum number of records to include in each batch. Maximum value of 10000.
]
consumerGroupIdB" DThe name of the destination queue to consume. Maximum value of 200.
�
credentials�B�:�
�
pipes>PipeSourceParametersManagedStreamingKafkaParametersCredentials�aws:pipes/PipeSourceParametersManagedStreamingKafkaParametersCredentials:PipeSourceParametersManagedStreamingKafkaParametersCredentials?The credentials needed to access the resource. Detailed below.
o
maximumBatchingWindowInSecondsB GThe maximum length of a time to wait for events. Maximum value of 300.
t
startingPositionB" ZThe position in a stream from which to start reading. Valid values: TRIM_HORIZON, LATEST.
\
	topicName" KThe name of the topic that the pipe will read from. Maximum length of 249.
:�
�
pipes>PipeSourceParametersManagedStreamingKafkaParametersCredentials�aws:pipes/PipeSourceParametersManagedStreamingKafkaParametersCredentials:PipeSourceParametersManagedStreamingKafkaParametersCredentials�
�d
clientCertificateTlsAuthB" BThe ARN of the Secrets Manager secret containing the credentials.
\
saslScram512AuthB" BThe ARN of the Secrets Manager secret containing the credentials.
:�
�
pipes,PipeSourceParametersRabbitmqBrokerParameterscaws:pipes/PipeSourceParametersRabbitmqBrokerParameters:PipeSourceParametersRabbitmqBrokerParameters�
�c
	batchSizeB PThe maximum number of records to include in each batch. Maximum value of 10000.
�
credentials�:�
�
pipes7PipeSourceParametersRabbitmqBrokerParametersCredentialsyaws:pipes/PipeSourceParametersRabbitmqBrokerParametersCredentials:PipeSourceParametersRabbitmqBrokerParametersCredentials?The credentials needed to access the resource. Detailed below.
o
maximumBatchingWindowInSecondsB GThe maximum length of a time to wait for events. Maximum value of 300.
W
	queueName" FThe name of the destination queue to consume. Maximum length of 1000.
l
virtualHostB" WThe name of the virtual host associated with the source broker. Maximum length of 200.
:�
�
pipes7PipeSourceParametersRabbitmqBrokerParametersCredentialsyaws:pipes/PipeSourceParametersRabbitmqBrokerParametersCredentials:PipeSourceParametersRabbitmqBrokerParametersCredentialsW
US
	basicAuth" BThe ARN of the Secrets Manager secret containing the credentials.
:�
�
pipes.PipeSourceParametersSelfManagedKafkaParametersgaws:pipes/PipeSourceParametersSelfManagedKafkaParameters:PipeSourceParametersSelfManagedKafkaParameters�

�
v
additionalBootstrapServersB*" PAn array of server URLs. Maximum number of 2 items, each of maximum length 300.
c
	batchSizeB PThe maximum number of records to include in each batch. Maximum value of 10000.
]
consumerGroupIdB" DThe name of the destination queue to consume. Maximum value of 200.
�
credentials�B�:�
�
pipes9PipeSourceParametersSelfManagedKafkaParametersCredentials}aws:pipes/PipeSourceParametersSelfManagedKafkaParametersCredentials:PipeSourceParametersSelfManagedKafkaParametersCredentials?The credentials needed to access the resource. Detailed below.
o
maximumBatchingWindowInSecondsB GThe maximum length of a time to wait for events. Maximum value of 300.
_
serverRootCaCertificateB" >The ARN of the Secrets Manager secret used for certification.
t
startingPositionB" ZThe position in a stream from which to start reading. Valid values: TRIM_HORIZON, LATEST.
\
	topicName" KThe name of the topic that the pipe will read from. Maximum length of 249.
�
vpc�B�:�
�
pipes1PipeSourceParametersSelfManagedKafkaParametersVpcmaws:pipes/PipeSourceParametersSelfManagedKafkaParametersVpc:PipeSourceParametersSelfManagedKafkaParametersVpc�This structure specifies the VPC subnets and security groups for the stream, and whether a public IP address is to be used. Detailed below.
:�
�
pipes9PipeSourceParametersSelfManagedKafkaParametersCredentials}aws:pipes/PipeSourceParametersSelfManagedKafkaParametersCredentials:PipeSourceParametersSelfManagedKafkaParametersCredentials�
�U
	basicAuthB" BThe ARN of the Secrets Manager secret containing the credentials.
d
clientCertificateTlsAuthB" BThe ARN of the Secrets Manager secret containing the credentials.
\
saslScram256AuthB" BThe ARN of the Secrets Manager secret containing the credentials.
\
saslScram512AuthB" BThe ARN of the Secrets Manager secret containing the credentials.
:�
�
pipes1PipeSourceParametersSelfManagedKafkaParametersVpcmaws:pipes/PipeSourceParametersSelfManagedKafkaParametersVpc:PipeSourceParametersSelfManagedKafkaParametersVpc/
-
securityGroupsB*" 
subnetsB*" :�
�
pipes&PipeSourceParametersSqsQueueParametersWaws:pipes/PipeSourceParametersSqsQueueParameters:PipeSourceParametersSqsQueueParameters�
�c
	batchSizeB PThe maximum number of records to include in each batch. Maximum value of 10000.
o
maximumBatchingWindowInSecondsB GThe maximum length of a time to wait for events. Maximum value of 300.
:�
R
pipesPipeTargetParameters3aws:pipes/PipeTargetParameters:PipeTargetParameters�
��
batchJobParameters�B�:�
�
pipes&PipeTargetParametersBatchJobParametersWaws:pipes/PipeTargetParametersBatchJobParameters:PipeTargetParametersBatchJobParametersGThe parameters for using an AWS Batch job as a target. Detailed below.
�
cloudwatchLogsParameters�B�:�
�
pipes,PipeTargetParametersCloudwatchLogsParameterscaws:pipes/PipeTargetParametersCloudwatchLogsParameters:PipeTargetParametersCloudwatchLogsParametersTThe parameters for using an CloudWatch Logs log stream as a target. Detailed below.
�
ecsTaskParameters�B�:�
�
pipes%PipeTargetParametersEcsTaskParametersUaws:pipes/PipeTargetParametersEcsTaskParameters:PipeTargetParametersEcsTaskParametersIThe parameters for using an Amazon ECS task as a target. Detailed below.
�
eventbridgeEventBusParameters�B�:�
�
pipes1PipeTargetParametersEventbridgeEventBusParametersmaws:pipes/PipeTargetParametersEventbridgeEventBusParameters:PipeTargetParametersEventbridgeEventBusParametersOThe parameters for using an EventBridge event bus as a target. Detailed below.
�
httpParameters�B�:~
|
pipes"PipeTargetParametersHttpParametersOaws:pipes/PipeTargetParametersHttpParameters:PipeTargetParametersHttpParameters�These are custom parameter to be used when the target is an API Gateway REST APIs or EventBridge ApiDestinations. Detailed below.
�
inputTemplateB" �Valid JSON text passed to the target. In this case, nothing from the event itself is passed to the target. Maximum length of 8192 characters.
�
kinesisStreamParameters�B�:�
�
pipes+PipeTargetParametersKinesisStreamParametersaaws:pipes/PipeTargetParametersKinesisStreamParameters:PipeTargetParametersKinesisStreamParametersGThe parameters for using a Kinesis stream as a source. Detailed below.
�
lambdaFunctionParameters�B�:�
�
pipes,PipeTargetParametersLambdaFunctionParameterscaws:pipes/PipeTargetParametersLambdaFunctionParameters:PipeTargetParametersLambdaFunctionParametersHThe parameters for using a Lambda function as a target. Detailed below.
�
redshiftDataParameters�B�:�
�
pipes*PipeTargetParametersRedshiftDataParameters_aws:pipes/PipeTargetParametersRedshiftDataParameters:PipeTargetParametersRedshiftDataParameters�These are custom parameters to be used when the target is a Amazon Redshift cluster to invoke the Amazon Redshift Data API BatchExecuteStatement. Detailed below.
�
sagemakerPipelineParameters�B�:�
�
pipes/PipeTargetParametersSagemakerPipelineParametersiaws:pipes/PipeTargetParametersSagemakerPipelineParameters:PipeTargetParametersSagemakerPipelineParametersKThe parameters for using a SageMaker pipeline as a target. Detailed below.
�
sqsQueueParameters�B�:�
�
pipes&PipeTargetParametersSqsQueueParametersWaws:pipes/PipeTargetParametersSqsQueueParameters:PipeTargetParametersSqsQueueParametersJThe parameters for using a Amazon SQS stream as a target. Detailed below.
�
"stepFunctionStateMachineParameters�B�:�
�
pipes6PipeTargetParametersStepFunctionStateMachineParameterswaws:pipes/PipeTargetParametersStepFunctionStateMachineParameters:PipeTargetParametersStepFunctionStateMachineParametersUThe parameters for using a Step Functions state machine as a target. Detailed below.
:�
�
pipes&PipeTargetParametersBatchJobParametersWaws:pipes/PipeTargetParametersBatchJobParameters:PipeTargetParametersBatchJobParameters�
��
arrayProperties�B�:�
�
pipes5PipeTargetParametersBatchJobParametersArrayPropertiesuaws:pipes/PipeTargetParametersBatchJobParametersArrayProperties:PipeTargetParametersBatchJobParametersArrayProperties�The array properties for the submitted job, such as the size of the array. The array size can be between 2 and 10,000. If you specify array properties for a job, it becomes an array job. This parameter is used only if the target is an AWS Batch job. Detailed below.
�
containerOverrides�B�:�
�
pipes8PipeTargetParametersBatchJobParametersContainerOverrides{aws:pipes/PipeTargetParametersBatchJobParametersContainerOverrides:PipeTargetParametersBatchJobParametersContainerOverrides<The overrides that are sent to a container. Detailed below.
�

dependsOns�B�*�:�
�
pipes/PipeTargetParametersBatchJobParametersDependsOniaws:pipes/PipeTargetParametersBatchJobParametersDependsOn:PipeTargetParametersBatchJobParametersDependsOn�A list of dependencies for the job. A job can depend upon a maximum of 20 jobs. You can specify a SEQUENTIAL type dependency without specifying a job ID for array jobs so that each child array job completes sequentially, starting at index 0. You can also specify an N_TO_N type dependency with a job ID for array jobs. In that case, each index child of this job must wait for the corresponding index child of each dependency to complete before it can begin. Detailed below.
�
jobDefinition" �The job definition used by this job. This value can be one of name, name:revision, or the Amazon Resource Name (ARN) for the job definition. If name is specified without a revision then the latest active revision is used.
F
jobName" 7The name of the job. It can be up to 128 letters long.
�

parametersB2" �Additional parameters passed to the job that replace parameter substitution placeholders that are set in the job definition. Parameters are specified as a key and value pair mapping. Parameters included here override any corresponding parameter defaults from the job definition. Detailed below.
�
retryStrategy�B�:�
�
pipes3PipeTargetParametersBatchJobParametersRetryStrategyqaws:pipes/PipeTargetParametersBatchJobParametersRetryStrategy:PipeTargetParametersBatchJobParametersRetryStrategy�The retry strategy to use for failed jobs. When a retry strategy is specified here, it overrides the retry strategy defined in the job definition. Detailed below.
:�
�
pipes5PipeTargetParametersBatchJobParametersArrayPropertiesuaws:pipes/PipeTargetParametersBatchJobParametersArrayProperties:PipeTargetParametersBatchJobParametersArrayPropertiesu
sq
sizeB cThe size of the array, if this is an array batch job. Minimum value of 2. Maximum value of 10,000.
:�
�
pipes8PipeTargetParametersBatchJobParametersContainerOverrides{aws:pipes/PipeTargetParametersBatchJobParametersContainerOverrides:PipeTargetParametersBatchJobParametersContainerOverrides�
��
commandsB*" �List of commands to send to the container that overrides the default command from the Docker image or the task definition. You must also specify a container name.
�
environments�B�*�:�
�
pipesCPipeTargetParametersBatchJobParametersContainerOverridesEnvironment�aws:pipes/PipeTargetParametersBatchJobParametersContainerOverridesEnvironment:PipeTargetParametersBatchJobParametersContainerOverridesEnvironment�The environment variables to send to the container. You can add new environment variables, which are added to the container at launch, or you can override the existing environment variables from the Docker image or the task definition. You must also specify a container name. Detailed below.
�
instanceTypeB" �The instance type to use for a multi-node parallel job. This parameter isn't applicable to single-node container jobs or jobs that run on Fargate resources, and shouldn't be provided.
�
resourceRequirements�B�*�:�
�
pipesKPipeTargetParametersBatchJobParametersContainerOverridesResourceRequirement�aws:pipes/PipeTargetParametersBatchJobParametersContainerOverridesResourceRequirement:PipeTargetParametersBatchJobParametersContainerOverridesResourceRequirement�The type and amount of a resource to assign to a container, instead of the default value from the task definition. The only supported resource is a GPU. Detailed below.
:�
�
pipesCPipeTargetParametersBatchJobParametersContainerOverridesEnvironment�aws:pipes/PipeTargetParametersBatchJobParametersContainerOverridesEnvironment:PipeTargetParametersBatchJobParametersContainerOverridesEnvironment�
�z
nameB" lName of the pipe. If omitted, the provider will assign a random, unique name. Conflicts with `name_prefix`.
u
valueB" fValue of parameter to start execution of a SageMaker Model Building Pipeline. Maximum length of 1024.
:�
�
pipesKPipeTargetParametersBatchJobParametersContainerOverridesResourceRequirement�aws:pipes/PipeTargetParametersBatchJobParametersContainerOverridesResourceRequirement:PipeTargetParametersBatchJobParametersContainerOverridesResourceRequirement�
��
type" �The type of placement strategy. The random placement strategy randomly places tasks on available candidates. The spread placement strategy spreads placement across available candidates evenly based on the field parameter. The binpack strategy places tasks on available candidates that have the least available amount of the resource that is specified with the field parameter. For example, if you binpack on memory, a task is placed on the instance with the least amount of remaining memory (but still enough to run the task). Valid Values: random, spread, binpack.
s
value" fValue of parameter to start execution of a SageMaker Model Building Pipeline. Maximum length of 1024.
:�
�
pipes/PipeTargetParametersBatchJobParametersDependsOniaws:pipes/PipeTargetParametersBatchJobParametersDependsOn:PipeTargetParametersBatchJobParametersDependsOn�
�W
jobIdB" HThe job ID of the AWS Batch job that's associated with this dependency.
�
typeB" �The type of placement strategy. The random placement strategy randomly places tasks on available candidates. The spread placement strategy spreads placement across available candidates evenly based on the field parameter. The binpack strategy places tasks on available candidates that have the least available amount of the resource that is specified with the field parameter. For example, if you binpack on memory, a task is placed on the instance with the least amount of remaining memory (but still enough to run the task). Valid Values: random, spread, binpack.
:�
�
pipes3PipeTargetParametersBatchJobParametersRetryStrategyqaws:pipes/PipeTargetParametersBatchJobParametersRetryStrategy:PipeTargetParametersBatchJobParametersRetryStrategy�
��
attemptsB �The number of times to move a job to the RUNNABLE status. If the value of attempts is greater than one, the job is retried on failure the same number of attempts as the value. Maximum value of 10.
:�
�
pipes,PipeTargetParametersCloudwatchLogsParameterscaws:pipes/PipeTargetParametersCloudwatchLogsParameters:PipeTargetParametersCloudwatchLogsParameters�
�3
logStreamNameB" The name of the log stream.
�
	timestampB" �The time the event occurred, expressed as the number of milliseconds after Jan 1, 1970 00:00:00 UTC. This is the JSON path to the field in the event e.g. $.detail.timestamp
:� 
�
pipes%PipeTargetParametersEcsTaskParametersUaws:pipes/PipeTargetParametersEcsTaskParameters:PipeTargetParametersEcsTaskParameters�
��
capacityProviderStrategies�B�*�:�
�
pipes=PipeTargetParametersEcsTaskParametersCapacityProviderStrategy�aws:pipes/PipeTargetParametersEcsTaskParametersCapacityProviderStrategy:PipeTargetParametersEcsTaskParametersCapacityProviderStrategy�List of capacity provider strategies to use for the task. If a capacityProviderStrategy is specified, the launchType parameter must be omitted. If no capacityProviderStrategy or launchType is specified, the defaultCapacityProviderStrategy for the cluster is used. Detailed below.
{
enableEcsManagedTagsB
 ]Specifies whether to enable Amazon ECS managed tags for the task. Valid values: true, false.
�
enableExecuteCommandB
 �Whether or not to enable the execute command functionality for the containers in this task. If true, this enables execute command functionality on all containers in the task. Valid values: true, false.
f
groupB" WSpecifies an Amazon ECS task group for the task. The maximum length is 255 characters.
�

launchTypeB" �Specifies the launch type on which your task is running. The launch type that you specify here must match one of the launch type (compatibilities) of the target task. The FARGATE value is supported only in the Regions where AWS Fargate with Amazon ECS is supported. Valid Values: EC2, FARGATE, EXTERNAL
�
networkConfiguration�B�:�
�
pipes9PipeTargetParametersEcsTaskParametersNetworkConfiguration}aws:pipes/PipeTargetParametersEcsTaskParametersNetworkConfiguration:PipeTargetParametersEcsTaskParametersNetworkConfiguration�Use this structure if the Amazon ECS task uses the awsvpc network mode. This structure specifies the VPC subnets and security groups associated with the task, and whether a public IP address is to be used. This structure is required if LaunchType is FARGATE because the awsvpc mode is required for Fargate tasks. If you specify NetworkConfiguration when the target ECS task does not use the awsvpc network mode, the task fails. Detailed below.
�
	overrides�B�:�
�
pipes.PipeTargetParametersEcsTaskParametersOverridesgaws:pipes/PipeTargetParametersEcsTaskParametersOverrides:PipeTargetParametersEcsTaskParametersOverrides?The overrides that are associated with a task. Detailed below.
�
placementConstraints�B�*�:�
�
pipes8PipeTargetParametersEcsTaskParametersPlacementConstraint{aws:pipes/PipeTargetParametersEcsTaskParametersPlacementConstraint:PipeTargetParametersEcsTaskParametersPlacementConstraint�An array of placement constraint objects to use for the task. You can specify up to 10 constraints per task (including constraints in the task definition and those specified at runtime). Detailed below.
�
placementStrategies�B�*�:�
�
pipes6PipeTargetParametersEcsTaskParametersPlacementStrategywaws:pipes/PipeTargetParametersEcsTaskParametersPlacementStrategy:PipeTargetParametersEcsTaskParametersPlacementStrategyThe placement strategy objects to use for the task. You can specify a maximum of five strategy rules per task. Detailed below.
�
platformVersionB" �Specifies the platform version for the task. Specify only the numeric portion of the platform version, such as 1.1.0. This structure is used only if LaunchType is FARGATE.
�
propagateTagsB" �Specifies whether to propagate the tags from the task definition to the task. If no value is specified, the tags are not propagated. Tags can only be propagated to the task during task creation. To add tags to a task after task creation, use the TagResource API action. Valid Values: TASK_DEFINITION
T
referenceIdB" ?The reference ID to use for the task. Maximum length of 1,024.
�
tagsB2" �Key-value mapping of resource tags. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
\
	taskCountB IThe number of tasks to create based on TaskDefinition. The default is 1.
j
taskDefinitionArn" QThe ARN of the task definition to use if the event target is an Amazon ECS task.
:�
�
pipes=PipeTargetParametersEcsTaskParametersCapacityProviderStrategy�aws:pipes/PipeTargetParametersEcsTaskParametersCapacityProviderStrategy:PipeTargetParametersEcsTaskParametersCapacityProviderStrategy�
��
baseB �The base value designates how many tasks, at a minimum, to run on the specified capacity provider. Only one capacity provider in a capacity provider strategy can have a base defined. If no value is specified, the default value of 0 is used. Maximum value of 100,000.
W
capacityProvider" ?The short name of the capacity provider. Maximum value of 255.
�
weightB �The weight value designates the relative percentage of the total number of tasks launched that should use the specified capacity provider. The weight value is taken into consideration after the base value, if defined, is satisfied. Maximum value of 1,000.
:�
�
pipes9PipeTargetParametersEcsTaskParametersNetworkConfiguration}aws:pipes/PipeTargetParametersEcsTaskParametersNetworkConfiguration:PipeTargetParametersEcsTaskParametersNetworkConfiguration�
��
awsVpcConfiguration�B�:�
�
pipesLPipeTargetParametersEcsTaskParametersNetworkConfigurationAwsVpcConfiguration�aws:pipes/PipeTargetParametersEcsTaskParametersNetworkConfigurationAwsVpcConfiguration:PipeTargetParametersEcsTaskParametersNetworkConfigurationAwsVpcConfiguration�Use this structure to specify the VPC subnets and security groups for the task, and whether a public IP address is to be used. This structure is relevant only for ECS tasks that use the awsvpc network mode. Detailed below.
:�
�
pipesLPipeTargetParametersEcsTaskParametersNetworkConfigurationAwsVpcConfiguration�aws:pipes/PipeTargetParametersEcsTaskParametersNetworkConfigurationAwsVpcConfiguration:PipeTargetParametersEcsTaskParametersNetworkConfigurationAwsVpcConfiguration�
��
assignPublicIpB" �Specifies whether the task's elastic network interface receives a public IP address. You can specify ENABLED only when LaunchType in EcsParameters is set to FARGATE. Valid Values: ENABLED, DISABLED.

securityGroupsB*" 
subnetsB*" :�
�
pipes.PipeTargetParametersEcsTaskParametersOverridesgaws:pipes/PipeTargetParametersEcsTaskParametersOverrides:PipeTargetParametersEcsTaskParametersOverrides�
��
containerOverrides�B�*�:�
�
pipes?PipeTargetParametersEcsTaskParametersOverridesContainerOverride�aws:pipes/PipeTargetParametersEcsTaskParametersOverridesContainerOverride:PipeTargetParametersEcsTaskParametersOverridesContainerOverrideIOne or more container overrides that are sent to a task. Detailed below.
�
cpuB" �The number of cpu units reserved for the container, instead of the default value from the task definition. You must also specify a container name.
�
ephemeralStorage�B�:�
�
pipes>PipeTargetParametersEcsTaskParametersOverridesEphemeralStorage�aws:pipes/PipeTargetParametersEcsTaskParametersOverridesEphemeralStorage:PipeTargetParametersEcsTaskParametersOverridesEphemeralStorageFThe ephemeral storage setting override for the task.  Detailed below.
o
executionRoleArnB" UThe Amazon Resource Name (ARN) of the task execution IAM role override for the task.
�
inferenceAcceleratorOverrides�B�*�:�
�
pipesJPipeTargetParametersEcsTaskParametersOverridesInferenceAcceleratorOverride�aws:pipes/PipeTargetParametersEcsTaskParametersOverridesInferenceAcceleratorOverride:PipeTargetParametersEcsTaskParametersOverridesInferenceAcceleratorOverrideNList of Elastic Inference accelerator overrides for the task. Detailed below.
�
memoryB" �The hard limit (in MiB) of memory to present to the container, instead of the default value from the task definition. If your container attempts to exceed the memory specified here, the container is killed. You must also specify a container name.
�
taskRoleArnB" �The Amazon Resource Name (ARN) of the IAM role that containers in this task can assume. All containers in this task are granted the permissions that are specified in this role.
:�
�
pipes?PipeTargetParametersEcsTaskParametersOverridesContainerOverride�aws:pipes/PipeTargetParametersEcsTaskParametersOverridesContainerOverride:PipeTargetParametersEcsTaskParametersOverridesContainerOverride�
��
commandsB*" �List of commands to send to the container that overrides the default command from the Docker image or the task definition. You must also specify a container name.
�
cpuB �The number of cpu units reserved for the container, instead of the default value from the task definition. You must also specify a container name.
�
environmentFiles�B�*�:�
�
pipesNPipeTargetParametersEcsTaskParametersOverridesContainerOverrideEnvironmentFile�aws:pipes/PipeTargetParametersEcsTaskParametersOverridesContainerOverrideEnvironmentFile:PipeTargetParametersEcsTaskParametersOverridesContainerOverrideEnvironmentFile�A list of files containing the environment variables to pass to a container, instead of the value from the container definition. Detailed below.
�
environments�B�*�:�
�
pipesJPipeTargetParametersEcsTaskParametersOverridesContainerOverrideEnvironment�aws:pipes/PipeTargetParametersEcsTaskParametersOverridesContainerOverrideEnvironment:PipeTargetParametersEcsTaskParametersOverridesContainerOverrideEnvironment�The environment variables to send to the container. You can add new environment variables, which are added to the container at launch, or you can override the existing environment variables from the Docker image or the task definition. You must also specify a container name. Detailed below.
�
memoryB �The hard limit (in MiB) of memory to present to the container, instead of the default value from the task definition. If your container attempts to exceed the memory specified here, the container is killed. You must also specify a container name.
�
memoryReservationB �The soft limit (in MiB) of memory to reserve for the container, instead of the default value from the task definition. You must also specify a container name.
z
nameB" lName of the pipe. If omitted, the provider will assign a random, unique name. Conflicts with `name_prefix`.
�
resourceRequirements�B�*�:�
�
pipesRPipeTargetParametersEcsTaskParametersOverridesContainerOverrideResourceRequirement�aws:pipes/PipeTargetParametersEcsTaskParametersOverridesContainerOverrideResourceRequirement:PipeTargetParametersEcsTaskParametersOverridesContainerOverrideResourceRequirement�The type and amount of a resource to assign to a container, instead of the default value from the task definition. The only supported resource is a GPU. Detailed below.
:�
�
pipesJPipeTargetParametersEcsTaskParametersOverridesContainerOverrideEnvironment�aws:pipes/PipeTargetParametersEcsTaskParametersOverridesContainerOverrideEnvironment:PipeTargetParametersEcsTaskParametersOverridesContainerOverrideEnvironment�
�z
nameB" lName of the pipe. If omitted, the provider will assign a random, unique name. Conflicts with `name_prefix`.
u
valueB" fValue of parameter to start execution of a SageMaker Model Building Pipeline. Maximum length of 1024.
:�
�
pipesNPipeTargetParametersEcsTaskParametersOverridesContainerOverrideEnvironmentFile�aws:pipes/PipeTargetParametersEcsTaskParametersOverridesContainerOverrideEnvironmentFile:PipeTargetParametersEcsTaskParametersOverridesContainerOverrideEnvironmentFile�
��
type" �The type of placement strategy. The random placement strategy randomly places tasks on available candidates. The spread placement strategy spreads placement across available candidates evenly based on the field parameter. The binpack strategy places tasks on available candidates that have the least available amount of the resource that is specified with the field parameter. For example, if you binpack on memory, a task is placed on the instance with the least amount of remaining memory (but still enough to run the task). Valid Values: random, spread, binpack.
s
value" fValue of parameter to start execution of a SageMaker Model Building Pipeline. Maximum length of 1024.
:�
�
pipesRPipeTargetParametersEcsTaskParametersOverridesContainerOverrideResourceRequirement�aws:pipes/PipeTargetParametersEcsTaskParametersOverridesContainerOverrideResourceRequirement:PipeTargetParametersEcsTaskParametersOverridesContainerOverrideResourceRequirement�
��
type" �The type of placement strategy. The random placement strategy randomly places tasks on available candidates. The spread placement strategy spreads placement across available candidates evenly based on the field parameter. The binpack strategy places tasks on available candidates that have the least available amount of the resource that is specified with the field parameter. For example, if you binpack on memory, a task is placed on the instance with the least amount of remaining memory (but still enough to run the task). Valid Values: random, spread, binpack.
s
value" fValue of parameter to start execution of a SageMaker Model Building Pipeline. Maximum length of 1024.
:�
�
pipes>PipeTargetParametersEcsTaskParametersOverridesEphemeralStorage�aws:pipes/PipeTargetParametersEcsTaskParametersOverridesEphemeralStorage:PipeTargetParametersEcsTaskParametersOverridesEphemeralStorage�
��
	sizeInGib �The total amount, in GiB, of ephemeral storage to set for the task. The minimum supported value is 21 GiB and the maximum supported value is 200 GiB.
:�
�
pipesJPipeTargetParametersEcsTaskParametersOverridesInferenceAcceleratorOverride�aws:pipes/PipeTargetParametersEcsTaskParametersOverridesInferenceAcceleratorOverride:PipeTargetParametersEcsTaskParametersOverridesInferenceAcceleratorOverride�
��

deviceNameB" �The Elastic Inference accelerator device name to override for the task. This parameter must match a deviceName specified in the task definition.
C

deviceTypeB" /The Elastic Inference accelerator type to use.
:�
�
pipes8PipeTargetParametersEcsTaskParametersPlacementConstraint{aws:pipes/PipeTargetParametersEcsTaskParametersPlacementConstraint:PipeTargetParametersEcsTaskParametersPlacementConstraint�
��

expressionB" �A cluster query language expression to apply to the constraint. You cannot specify an expression if the constraint type is distinctInstance. Maximum length of 2,000.
�
typeB" �The type of placement strategy. The random placement strategy randomly places tasks on available candidates. The spread placement strategy spreads placement across available candidates evenly based on the field parameter. The binpack strategy places tasks on available candidates that have the least available amount of the resource that is specified with the field parameter. For example, if you binpack on memory, a task is placed on the instance with the least amount of remaining memory (but still enough to run the task). Valid Values: random, spread, binpack.
:�	
�
pipes6PipeTargetParametersEcsTaskParametersPlacementStrategywaws:pipes/PipeTargetParametersEcsTaskParametersPlacementStrategy:PipeTargetParametersEcsTaskParametersPlacementStrategy�
��
fieldB" �The field to apply the placement strategy against. For the spread placement strategy, valid values are instanceId (or host, which has the same effect), or any platform or custom attribute that is applied to a container instance, such as attribute:ecs.availability-zone. For the binpack placement strategy, valid values are cpu and memory. For the random placement strategy, this field is not used. Maximum length of 255.
�
typeB" �The type of placement strategy. The random placement strategy randomly places tasks on available candidates. The spread placement strategy spreads placement across available candidates evenly based on the field parameter. The binpack strategy places tasks on available candidates that have the least available amount of the resource that is specified with the field parameter. For example, if you binpack on memory, a task is placed on the instance with the least amount of remaining memory (but still enough to run the task). Valid Values: random, spread, binpack.
:�
�
pipes1PipeTargetParametersEventbridgeEventBusParametersmaws:pipes/PipeTargetParametersEventbridgeEventBusParameters:PipeTargetParametersEventbridgeEventBusParameters�
��

detailTypeB" pA free-form string, with a maximum of 128 characters, used to decide what fields to expect in the event detail.
�

endpointIdB" �The URL subdomain of the endpoint. For example, if the URL for Endpoint is https://abcde.veo.endpoints.event.amazonaws.com, then the EndpointId is abcde.veo.
�
	resourcesB*" �List of AWS resources, identified by Amazon Resource Name (ARN), which the event primarily concerns. Any number, including zero, may be present.
�
sourceB" �Source resource of the pipe. This field typically requires an ARN (Amazon Resource Name). However, when using a self-managed Kafka cluster, you should use a different format. Instead of an ARN, use 'smk://' followed by the bootstrap server's address.
�
timeB" �The time stamp of the event, per RFC3339. If no time stamp is provided, the time stamp of the PutEvents call is used. This is the JSON path to the field in the event e.g. $.detail.timestamp
:�
|
pipes"PipeTargetParametersHttpParametersOaws:pipes/PipeTargetParametersHttpParameters:PipeTargetParametersHttpParameters\
Z
headerParametersB2" 
pathParameterValuesB" 
queryStringParametersB2" :�
�
pipes+PipeTargetParametersKinesisStreamParametersaaws:pipes/PipeTargetParametersKinesisStreamParameters:PipeTargetParametersKinesisStreamParameters�
��
partitionKey" �Determines which shard in the stream the data record is assigned to. Partition keys are Unicode strings with a maximum length limit of 256 characters for each key. Amazon Kinesis Data Streams uses the partition key as input to a hash function that maps the partition key and associated data to a specific shard. Specifically, an MD5 hash function is used to map partition keys to 128-bit integer values and to map associated data records to shards. As a result of this hashing mechanism, all data records with the same partition key map to the same shard within the stream.
:�
�
pipes,PipeTargetParametersLambdaFunctionParameterscaws:pipes/PipeTargetParametersLambdaFunctionParameters:PipeTargetParametersLambdaFunctionParameters�
��
invocationType" ySpecify whether to invoke the function synchronously or asynchronously. Valid Values: REQUEST_RESPONSE, FIRE_AND_FORGET.
:�
�
pipes*PipeTargetParametersRedshiftDataParameters_aws:pipes/PipeTargetParametersRedshiftDataParameters:PipeTargetParametersRedshiftDataParameters�
�d
database" TThe name of the database. Required when authenticating using temporary credentials.
b
dbUserB" RThe database user name. Required when authenticating using temporary credentials.
�
secretManagerArnB" wThe name or ARN of the secret that enables access to the database. Required when authenticating using Secrets Manager.
U
sqls*" GList of SQL statements text to run, each of maximum length of 100,000.

statementNameB" hThe name of the SQL statement. You can name the SQL statement when you create it to identify the query.
h
	withEventB
 UIndicates whether to send an event back to EventBridge after the SQL statement runs.
:�
�
pipes/PipeTargetParametersSagemakerPipelineParametersiaws:pipes/PipeTargetParametersSagemakerPipelineParameters:PipeTargetParametersSagemakerPipelineParameters�
��
pipelineParameters�B�*�:�
�
pipes@PipeTargetParametersSagemakerPipelineParametersPipelineParameter�aws:pipes/PipeTargetParametersSagemakerPipelineParametersPipelineParameter:PipeTargetParametersSagemakerPipelineParametersPipelineParameterdList of Parameter names and values for SageMaker Model Building Pipeline execution. Detailed below.
:�
�
pipes@PipeTargetParametersSagemakerPipelineParametersPipelineParameter�aws:pipes/PipeTargetParametersSagemakerPipelineParametersPipelineParameter:PipeTargetParametersSagemakerPipelineParametersPipelineParameter�
�x
name" lName of the pipe. If omitted, the provider will assign a random, unique name. Conflicts with `name_prefix`.
s
value" fValue of parameter to start execution of a SageMaker Model Building Pipeline. Maximum length of 1024.
:�
�
pipes&PipeTargetParametersSqsQueueParametersWaws:pipes/PipeTargetParametersSqsQueueParameters:PipeTargetParametersSqsQueueParameters�
��
messageDeduplicationIdB" tThis parameter applies only to FIFO (first-in-first-out) queues. The token used for deduplication of sent messages.
H
messageGroupIdB" 0The FIFO message group ID to use as the target.
:�
�
pipes6PipeTargetParametersStepFunctionStateMachineParameterswaws:pipes/PipeTargetParametersStepFunctionStateMachineParameters:PipeTargetParametersStepFunctionStateMachineParameters�
��
invocationType" ySpecify whether to invoke the function synchronously or asynchronously. Valid Values: REQUEST_RESPONSE, FIRE_AND_FORGET.
:�
@
pollygetVoicesVoice'aws:polly/getVoicesVoice:getVoicesVoice�
��
additionalLanguageCodes*" fAdditional codes for languages available for the specified voice in addition to its default language.
#
gender" Gender of the voice.
*
id"  Amazon Polly assigned voice ID.
�
languageCode" |Language identification tag for filtering the list of voices returned. If not specified, all available voices are returned.
D
languageName" 0Human readable name of the language in English.

name" Name of the voice.
R
supportedEngines*" 8Specifies which engines are supported by a given voice.
:�
J
pricinggetProductFilter-aws:pricing/getProductFilter:getProductFilter�
�@
field" 3Product attribute name that you want to filter on.
A
value" 4Product attribute value that you want to filter on.
:�
b
qldbStreamKinesisConfiguration>aws:qldb/StreamKinesisConfiguration:StreamKinesisConfiguration�
��
aggregationEnabledB
 �Enables QLDB to publish multiple data records in a single Kinesis Data Streams record, increasing the number of records sent per API call. Default: `true`.
V
	streamArn" EThe Amazon Resource Name (ARN) of the Kinesis Data Streams resource.
:�
V

quicksightAnalysisParameters4aws:quicksight/AnalysisParameters:AnalysisParameters�
��
dateTimeParameters�B�*�:�
�

quicksight#AnalysisParametersDateTimeParameterVaws:quicksight/AnalysisParametersDateTimeParameter:AnalysisParametersDateTimeParameter�A list of parameters that have a data type of date-time. See [AWS API Documentation for complete description](https://docs.aws.amazon.com/quicksight/latest/APIReference/API_DateTimeParameter.html).
�
decimalParameters�B�*�:�
�

quicksight"AnalysisParametersDecimalParameterTaws:quicksight/AnalysisParametersDecimalParameter:AnalysisParametersDecimalParameter�A list of parameters that have a data type of decimal. See [AWS API Documentation for complete description](https://docs.aws.amazon.com/quicksight/latest/APIReference/API_DecimalParameter.html).
�
integerParameters�B�*�:�
�

quicksight"AnalysisParametersIntegerParameterTaws:quicksight/AnalysisParametersIntegerParameter:AnalysisParametersIntegerParameter�A list of parameters that have a data type of integer. See [AWS API Documentation for complete description](https://docs.aws.amazon.com/quicksight/latest/APIReference/API_IntegerParameter.html).
�
stringParameters�B�*�:�
�

quicksight!AnalysisParametersStringParameterRaws:quicksight/AnalysisParametersStringParameter:AnalysisParametersStringParameter�A list of parameters that have a data type of string. See [AWS API Documentation for complete description](https://docs.aws.amazon.com/quicksight/latest/APIReference/API_StringParameter.html).
:�
�

quicksight#AnalysisParametersDateTimeParameterVaws:quicksight/AnalysisParametersDateTimeParameter:AnalysisParametersDateTimeParameterf
dR
name" FDisplay name for the analysis.

The following arguments are optional:

values*" :�
�

quicksight"AnalysisParametersDecimalParameterTaws:quicksight/AnalysisParametersDecimalParameter:AnalysisParametersDecimalParameterf
dR
name" FDisplay name for the analysis.

The following arguments are optional:

values* :�
�

quicksight"AnalysisParametersIntegerParameterTaws:quicksight/AnalysisParametersIntegerParameter:AnalysisParametersIntegerParameterf
dR
name" FDisplay name for the analysis.

The following arguments are optional:

values* :�
�

quicksight!AnalysisParametersStringParameterRaws:quicksight/AnalysisParametersStringParameter:AnalysisParametersStringParameterf
dR
name" FDisplay name for the analysis.

The following arguments are optional:

values*" :�
V

quicksightAnalysisPermission4aws:quicksight/AnalysisPermission:AnalysisPermission�
�H
actions*" 7List of IAM actions to grant or revoke permissions on.
�
	principal" �ARN of the principal. See the [ResourcePermission documentation](https://docs.aws.amazon.com/quicksight/latest/APIReference/API_ResourcePermission.html) for the applicable ARN values.
:�
\

quicksightAnalysisSourceEntity8aws:quicksight/AnalysisSourceEntity:AnalysisSourceEntity�
��
sourceTemplate�B�:�
�

quicksight"AnalysisSourceEntitySourceTemplateTaws:quicksight/AnalysisSourceEntitySourceTemplate:AnalysisSourceEntitySourceTemplate*The source template. See source_template.
:�
�

quicksight"AnalysisSourceEntitySourceTemplateTaws:quicksight/AnalysisSourceEntitySourceTemplate:AnalysisSourceEntitySourceTemplate�
�;
arn" 0The Amazon Resource Name (ARN) of the resource.
�
dataSetReferences�*�:�
�

quicksight2AnalysisSourceEntitySourceTemplateDataSetReferencetaws:quicksight/AnalysisSourceEntitySourceTemplateDataSetReference:AnalysisSourceEntitySourceTemplateDataSetReference5List of dataset references. See data_set_references.
:�
�

quicksight2AnalysisSourceEntitySourceTemplateDataSetReferencetaws:quicksight/AnalysisSourceEntitySourceTemplateDataSetReference:AnalysisSourceEntitySourceTemplateDataSetReferencek
i6

dataSetArn" $Dataset Amazon Resource Name (ARN).
/
dataSetPlaceholder" Dataset placeholder.
:�
�

quicksight DashboardDashboardPublishOptionsPaws:quicksight/DashboardDashboardPublishOptions:DashboardDashboardPublishOptions�
��
adHocFilteringOption�B�:�
�

quicksight4DashboardDashboardPublishOptionsAdHocFilteringOptionxaws:quicksight/DashboardDashboardPublishOptionsAdHocFilteringOption:DashboardDashboardPublishOptionsAdHocFilteringOptionAAd hoc (one-time) filtering option. See ad_hoc_filtering_option.
�
dataPointDrillUpDownOption�B�:�
�

quicksight:DashboardDashboardPublishOptionsDataPointDrillUpDownOption�aws:quicksight/DashboardDashboardPublishOptionsDataPointDrillUpDownOption:DashboardDashboardPublishOptionsDataPointDrillUpDownOption[The drill-down options of data points in a dashboard. See data_point_drill_up_down_option.
�
dataPointMenuLabelOption�B�:�
�

quicksight8DashboardDashboardPublishOptionsDataPointMenuLabelOption�aws:quicksight/DashboardDashboardPublishOptionsDataPointMenuLabelOption:DashboardDashboardPublishOptionsDataPointMenuLabelOptionTThe data point menu label options of a dashboard. See data_point_menu_label_option.
�
dataPointTooltipOption�B�:�
�

quicksight6DashboardDashboardPublishOptionsDataPointTooltipOption|aws:quicksight/DashboardDashboardPublishOptionsDataPointTooltipOption:DashboardDashboardPublishOptionsDataPointTooltipOptionOThe data point tool tip options of a dashboard. See data_point_tooltip_option.
�
exportToCsvOption�B�:�
�

quicksight1DashboardDashboardPublishOptionsExportToCsvOptionraws:quicksight/DashboardDashboardPublishOptionsExportToCsvOption:DashboardDashboardPublishOptionsExportToCsvOption1Export to .csv option. See export_to_csv_option.
�
exportWithHiddenFieldsOption�B�:�
�

quicksight<DashboardDashboardPublishOptionsExportWithHiddenFieldsOption�aws:quicksight/DashboardDashboardPublishOptionsExportWithHiddenFieldsOption:DashboardDashboardPublishOptionsExportWithHiddenFieldsOptionaDetermines if hidden fields are exported with a dashboard. See export_with_hidden_fields_option.
�
sheetControlsOption�B�:�
�

quicksight3DashboardDashboardPublishOptionsSheetControlsOptionvaws:quicksight/DashboardDashboardPublishOptionsSheetControlsOption:DashboardDashboardPublishOptionsSheetControlsOption2Sheet controls option. See sheet_controls_option.
�
$sheetLayoutElementMaximizationOption�B�:�
�

quicksightDDashboardDashboardPublishOptionsSheetLayoutElementMaximizationOption�aws:quicksight/DashboardDashboardPublishOptionsSheetLayoutElementMaximizationOption:DashboardDashboardPublishOptionsSheetLayoutElementMaximizationOptiondThe sheet layout maximization options of a dashboard. See sheet_layout_element_maximization_option.
�
visualAxisSortOption�B�:�
�

quicksight4DashboardDashboardPublishOptionsVisualAxisSortOptionxaws:quicksight/DashboardDashboardPublishOptionsVisualAxisSortOption:DashboardDashboardPublishOptionsVisualAxisSortOptionCThe axis sort options of a dashboard. See visual_axis_sort_option.
�
visualMenuOption�B�:�
�

quicksight0DashboardDashboardPublishOptionsVisualMenuOptionpaws:quicksight/DashboardDashboardPublishOptionsVisualMenuOption:DashboardDashboardPublishOptionsVisualMenuOptionEThe menu options of a visual in a dashboard. See visual_menu_option.
:�
�

quicksight4DashboardDashboardPublishOptionsAdHocFilteringOptionxaws:quicksight/DashboardDashboardPublishOptionsAdHocFilteringOption:DashboardDashboardPublishOptionsAdHocFilteringOptionZ
XV
availabilityStatusB" :Availability status. Possibles values: ENABLED, DISABLED.
:�
�

quicksight:DashboardDashboardPublishOptionsDataPointDrillUpDownOption�aws:quicksight/DashboardDashboardPublishOptionsDataPointDrillUpDownOption:DashboardDashboardPublishOptionsDataPointDrillUpDownOptionZ
XV
availabilityStatusB" :Availability status. Possibles values: ENABLED, DISABLED.
:�
�

quicksight8DashboardDashboardPublishOptionsDataPointMenuLabelOption�aws:quicksight/DashboardDashboardPublishOptionsDataPointMenuLabelOption:DashboardDashboardPublishOptionsDataPointMenuLabelOptionZ
XV
availabilityStatusB" :Availability status. Possibles values: ENABLED, DISABLED.
:�
�

quicksight6DashboardDashboardPublishOptionsDataPointTooltipOption|aws:quicksight/DashboardDashboardPublishOptionsDataPointTooltipOption:DashboardDashboardPublishOptionsDataPointTooltipOptionZ
XV
availabilityStatusB" :Availability status. Possibles values: ENABLED, DISABLED.
:�
�

quicksight1DashboardDashboardPublishOptionsExportToCsvOptionraws:quicksight/DashboardDashboardPublishOptionsExportToCsvOption:DashboardDashboardPublishOptionsExportToCsvOptionZ
XV
availabilityStatusB" :Availability status. Possibles values: ENABLED, DISABLED.
:�
�

quicksight<DashboardDashboardPublishOptionsExportWithHiddenFieldsOption�aws:quicksight/DashboardDashboardPublishOptionsExportWithHiddenFieldsOption:DashboardDashboardPublishOptionsExportWithHiddenFieldsOptionZ
XV
availabilityStatusB" :Availability status. Possibles values: ENABLED, DISABLED.
:�
�

quicksight3DashboardDashboardPublishOptionsSheetControlsOptionvaws:quicksight/DashboardDashboardPublishOptionsSheetControlsOption:DashboardDashboardPublishOptionsSheetControlsOptionV
TR
visibilityStateB" 9Visibility state. Possibles values: EXPANDED, COLLAPSED.
:�
�

quicksightDDashboardDashboardPublishOptionsSheetLayoutElementMaximizationOption�aws:quicksight/DashboardDashboardPublishOptionsSheetLayoutElementMaximizationOption:DashboardDashboardPublishOptionsSheetLayoutElementMaximizationOptionZ
XV
availabilityStatusB" :Availability status. Possibles values: ENABLED, DISABLED.
:�
�

quicksight4DashboardDashboardPublishOptionsVisualAxisSortOptionxaws:quicksight/DashboardDashboardPublishOptionsVisualAxisSortOption:DashboardDashboardPublishOptionsVisualAxisSortOptionZ
XV
availabilityStatusB" :Availability status. Possibles values: ENABLED, DISABLED.
:�
�

quicksight0DashboardDashboardPublishOptionsVisualMenuOptionpaws:quicksight/DashboardDashboardPublishOptionsVisualMenuOption:DashboardDashboardPublishOptionsVisualMenuOptionZ
XV
availabilityStatusB" :Availability status. Possibles values: ENABLED, DISABLED.
:�
Y

quicksightDashboardParameters6aws:quicksight/DashboardParameters:DashboardParameters�
��
dateTimeParameters�B�*�:�
�

quicksight$DashboardParametersDateTimeParameterXaws:quicksight/DashboardParametersDateTimeParameter:DashboardParametersDateTimeParameter�A list of parameters that have a data type of date-time. See [AWS API Documentation for complete description](https://docs.aws.amazon.com/quicksight/latest/APIReference/API_DateTimeParameter.html).
�
decimalParameters�B�*�:�
�

quicksight#DashboardParametersDecimalParameterVaws:quicksight/DashboardParametersDecimalParameter:DashboardParametersDecimalParameter�A list of parameters that have a data type of decimal. See [AWS API Documentation for complete description](https://docs.aws.amazon.com/quicksight/latest/APIReference/API_DecimalParameter.html).
�
integerParameters�B�*�:�
�

quicksight#DashboardParametersIntegerParameterVaws:quicksight/DashboardParametersIntegerParameter:DashboardParametersIntegerParameter�A list of parameters that have a data type of integer. See [AWS API Documentation for complete description](https://docs.aws.amazon.com/quicksight/latest/APIReference/API_IntegerParameter.html).
�
stringParameters�B�*�:�
�

quicksight"DashboardParametersStringParameterTaws:quicksight/DashboardParametersStringParameter:DashboardParametersStringParameter�A list of parameters that have a data type of string. See [AWS API Documentation for complete description](https://docs.aws.amazon.com/quicksight/latest/APIReference/API_StringParameter.html).
:�
�

quicksight$DashboardParametersDateTimeParameterXaws:quicksight/DashboardParametersDateTimeParameter:DashboardParametersDateTimeParameter@
>,
name"  Display name for the dashboard.

values*" :�
�

quicksight#DashboardParametersDecimalParameterVaws:quicksight/DashboardParametersDecimalParameter:DashboardParametersDecimalParameter@
>,
name"  Display name for the dashboard.

values* :�
�

quicksight#DashboardParametersIntegerParameterVaws:quicksight/DashboardParametersIntegerParameter:DashboardParametersIntegerParameter@
>,
name"  Display name for the dashboard.

values* :�
�

quicksight"DashboardParametersStringParameterTaws:quicksight/DashboardParametersStringParameter:DashboardParametersStringParameter@
>,
name"  Display name for the dashboard.

values*" :�
Y

quicksightDashboardPermission6aws:quicksight/DashboardPermission:DashboardPermission�
�H
actions*" 7List of IAM actions to grant or revoke permissions on.
�
	principal" �ARN of the principal. See the [ResourcePermission documentation](https://docs.aws.amazon.com/quicksight/latest/APIReference/API_ResourcePermission.html) for the applicable ARN values.
:�
_

quicksightDashboardSourceEntity:aws:quicksight/DashboardSourceEntity:DashboardSourceEntity�
��
sourceTemplate�B�:�
�

quicksight#DashboardSourceEntitySourceTemplateVaws:quicksight/DashboardSourceEntitySourceTemplate:DashboardSourceEntitySourceTemplate*The source template. See source_template.
:�
�

quicksight#DashboardSourceEntitySourceTemplateVaws:quicksight/DashboardSourceEntitySourceTemplate:DashboardSourceEntitySourceTemplate�
�;
arn" 0The Amazon Resource Name (ARN) of the resource.
�
dataSetReferences�*�:�
�

quicksight3DashboardSourceEntitySourceTemplateDataSetReferencevaws:quicksight/DashboardSourceEntitySourceTemplateDataSetReference:DashboardSourceEntitySourceTemplateDataSetReference5List of dataset references. See data_set_references.
:�
�

quicksight3DashboardSourceEntitySourceTemplateDataSetReferencevaws:quicksight/DashboardSourceEntitySourceTemplateDataSetReference:DashboardSourceEntitySourceTemplateDataSetReferencek
i6

dataSetArn" $Dataset Amazon Resource Name (ARN).
/
dataSetPlaceholder" Dataset placeholder.
:�
V

quicksightDataSetColumnGroup4aws:quicksight/DataSetColumnGroup:DataSetColumnGroup�
��
geoSpatialColumnGroup�B�:�
�

quicksight'DataSetColumnGroupGeoSpatialColumnGroup^aws:quicksight/DataSetColumnGroupGeoSpatialColumnGroup:DataSetColumnGroupGeoSpatialColumnGroupPGeospatial column group that denotes a hierarchy. See geo_spatial_column_group.
:�
�

quicksight'DataSetColumnGroupGeoSpatialColumnGroup^aws:quicksight/DataSetColumnGroupGeoSpatialColumnGroup:DataSetColumnGroupGeoSpatialColumnGroup�
�,
columns*" Columns in this hierarchy.
8
countryCode" %Country code. Valid values are `US`.
.
name" "A display name for the hierarchy.
:�
�

quicksight DataSetColumnLevelPermissionRulePaws:quicksight/DataSetColumnLevelPermissionRule:DataSetColumnLevelPermissionRule�
�1
columnNamesB*" An array of column names.
N

principalsB*" 8An array of ARNs for Amazon QuickSight users or groups.
:�
�

quicksight DataSetDataSetUsageConfigurationPaws:quicksight/DataSetDataSetUsageConfiguration:DataSetDataSetUsageConfiguration�
�|
disableUseAsDirectQuerySourceB
 UControls whether a child dataset of a direct query can use this dataset as a source.
�
disableUseAsImportedSourceB
 _Controls whether a child dataset that's stored in QuickSight can use this dataset as a source.
:�
V

quicksightDataSetFieldFolder4aws:quicksight/DataSetFieldFolder:DataSetFieldFolder�
�f
columnsB*" SAn array of column names to add to the folder. A column can only be in one folder.
/
descriptionB" Field folder description.
3
fieldFoldersId" Key of the field folder map.
:�
b

quicksightDataSetLogicalTableMap<aws:quicksight/DataSetLogicalTableMap:DataSetLogicalTableMap�
�3
alias" &A display name for the logical table.
�
dataTransforms�B�*�:�
�

quicksight#DataSetLogicalTableMapDataTransformVaws:quicksight/DataSetLogicalTableMapDataTransform:DataSetLogicalTableMapDataTransform�Transform operations that act on this logical table. For this structure to be valid, only one of the attributes can be non-null. See data_transforms.
7
logicalTableMapId" Key of the logical table map.
�
sourcex:v
t

quicksightDataSetLogicalTableMapSourceHaws:quicksight/DataSetLogicalTableMapSource:DataSetLogicalTableMapSource*Source of this logical table. See source.
:�
�

quicksight#DataSetLogicalTableMapDataTransformVaws:quicksight/DataSetLogicalTableMapDataTransform:DataSetLogicalTableMapDataTransform�
��
castColumnTypeOperation�B�:�
�

quicksight:DataSetLogicalTableMapDataTransformCastColumnTypeOperation�aws:quicksight/DataSetLogicalTableMapDataTransformCastColumnTypeOperation:DataSetLogicalTableMapDataTransformCastColumnTypeOperation_A transform operation that casts a column to a different type. See cast_column_type_operation.
�
createColumnsOperation�B�:�
�

quicksight9DataSetLogicalTableMapDataTransformCreateColumnsOperation�aws:quicksight/DataSetLogicalTableMapDataTransformCreateColumnsOperation:DataSetLogicalTableMapDataTransformCreateColumnsOperation�An operation that creates calculated columns. Columns created in one such operation form a lexical closure. See create_columns_operation.
�
filterOperation�B�:�
�

quicksight2DataSetLogicalTableMapDataTransformFilterOperationtaws:quicksight/DataSetLogicalTableMapDataTransformFilterOperation:DataSetLogicalTableMapDataTransformFilterOperationNAn operation that filters rows based on some condition. See filter_operation.
�
projectOperation�B�:�
�

quicksight3DataSetLogicalTableMapDataTransformProjectOperationvaws:quicksight/DataSetLogicalTableMapDataTransformProjectOperation:DataSetLogicalTableMapDataTransformProjectOperation�An operation that projects columns. Operations that come after a projection can only refer to projected columns. See project_operation.
�
renameColumnOperation�B�:�
�

quicksight8DataSetLogicalTableMapDataTransformRenameColumnOperation�aws:quicksight/DataSetLogicalTableMapDataTransformRenameColumnOperation:DataSetLogicalTableMapDataTransformRenameColumnOperationAAn operation that renames a column. See rename_column_operation.
�
tagColumnOperation�B�:�
�

quicksight5DataSetLogicalTableMapDataTransformTagColumnOperationzaws:quicksight/DataSetLogicalTableMapDataTransformTagColumnOperation:DataSetLogicalTableMapDataTransformTagColumnOperationWAn operation that tags a column with additional information. See tag_column_operation.
�
untagColumnOperation�B�:�
�

quicksight7DataSetLogicalTableMapDataTransformUntagColumnOperation~aws:quicksight/DataSetLogicalTableMapDataTransformUntagColumnOperation:DataSetLogicalTableMapDataTransformUntagColumnOperation^A transform operation that removes tags associated with a column. See untag_column_operation.
:�
�

quicksight:DataSetLogicalTableMapDataTransformCastColumnTypeOperation�aws:quicksight/DataSetLogicalTableMapDataTransformCastColumnTypeOperation:DataSetLogicalTableMapDataTransformCastColumnTypeOperation�
�

columnName" Column name.
�
formatB" �When casting a column from string to datetime type, you can supply a string in a format supported by Amazon QuickSight to denote the source data format.
h
newColumnType" SNew column data type. Valid values are `STRING`, `INTEGER`, `DECIMAL`, `DATETIME`.
:�
�

quicksight9DataSetLogicalTableMapDataTransformCreateColumnsOperation�aws:quicksight/DataSetLogicalTableMapDataTransformCreateColumnsOperation:DataSetLogicalTableMapDataTransformCreateColumnsOperation�
��
columns�*�:�
�

quicksight?DataSetLogicalTableMapDataTransformCreateColumnsOperationColumn�aws:quicksight/DataSetLogicalTableMapDataTransformCreateColumnsOperationColumn:DataSetLogicalTableMapDataTransformCreateColumnsOperationColumn+Calculated columns to create. See columns.
:�
�

quicksight?DataSetLogicalTableMapDataTransformCreateColumnsOperationColumn�aws:quicksight/DataSetLogicalTableMapDataTransformCreateColumnsOperationColumn:DataSetLogicalTableMapDataTransformCreateColumnsOperationColumn�
��
columnId" �A unique ID to identify a calculated column. During a dataset update, if the column ID of a calculated column matches that of an existing calculated column, Amazon QuickSight preserves the existing calculated column.


columnName" Column name.
D

expression" 2An expression that defines the calculated column.
:�
�

quicksight2DataSetLogicalTableMapDataTransformFilterOperationtaws:quicksight/DataSetLogicalTableMapDataTransformFilterOperation:DataSetLogicalTableMapDataTransformFilterOperation�
��
conditionExpression" ~An expression that must evaluate to a Boolean value. Rows for which the expression evaluates to true are kept in the dataset.
:�
�

quicksight3DataSetLogicalTableMapDataTransformProjectOperationvaws:quicksight/DataSetLogicalTableMapDataTransformProjectOperation:DataSetLogicalTableMapDataTransformProjectOperation1
/-
projectedColumns*" Projected columns.
:�
�

quicksight8DataSetLogicalTableMapDataTransformRenameColumnOperation�aws:quicksight/DataSetLogicalTableMapDataTransformRenameColumnOperation:DataSetLogicalTableMapDataTransformRenameColumnOperation\
Z(

columnName" Column to be renamed.
.
newColumnName" New name for the column.
:�
�

quicksight5DataSetLogicalTableMapDataTransformTagColumnOperationzaws:quicksight/DataSetLogicalTableMapDataTransformTagColumnOperation:DataSetLogicalTableMapDataTransformTagColumnOperation�
�

columnName" Column name.
�
tags�*�:�
�

quicksight8DataSetLogicalTableMapDataTransformTagColumnOperationTag�aws:quicksight/DataSetLogicalTableMapDataTransformTagColumnOperationTag:DataSetLogicalTableMapDataTransformTagColumnOperationTagSThe dataset column tag, currently only used for geospatial type tagging. See tags.
:�
�

quicksight8DataSetLogicalTableMapDataTransformTagColumnOperationTag�aws:quicksight/DataSetLogicalTableMapDataTransformTagColumnOperationTag:DataSetLogicalTableMapDataTransformTagColumnOperationTag�
��
columnDescription�B�:�
�

quicksightIDataSetLogicalTableMapDataTransformTagColumnOperationTagColumnDescription�aws:quicksight/DataSetLogicalTableMapDataTransformTagColumnOperationTagColumnDescription:DataSetLogicalTableMapDataTransformTagColumnOperationTagColumnDescription4A description for a column. See column_description.
�
columnGeographicRoleB" �A geospatial role for a column. Valid values are `COUNTRY`, `STATE`, `COUNTY`, `CITY`, `POSTCODE`, `LONGITUDE`, and `LATITUDE`.
:�
�

quicksightIDataSetLogicalTableMapDataTransformTagColumnOperationTagColumnDescription�aws:quicksight/DataSetLogicalTableMapDataTransformTagColumnOperationTagColumnDescription:DataSetLogicalTableMapDataTransformTagColumnOperationTagColumnDescription:
86
textB" (The text of a description for a column.
:�
�

quicksight7DataSetLogicalTableMapDataTransformUntagColumnOperation~aws:quicksight/DataSetLogicalTableMapDataTransformUntagColumnOperation:DataSetLogicalTableMapDataTransformUntagColumnOperationc
a

columnName" Column name.
>
tagNames*" ,The column tags to remove from this column.
:�
t

quicksightDataSetLogicalTableMapSourceHaws:quicksight/DataSetLogicalTableMapSource:DataSetLogicalTableMapSource�
�0

dataSetArnB" ARN of the parent data set.
�
joinInstruction�B�:�
�

quicksight+DataSetLogicalTableMapSourceJoinInstructionfaws:quicksight/DataSetLogicalTableMapSourceJoinInstruction:DataSetLogicalTableMapSourceJoinInstructionLSpecifies the result of a join of two logical tables. See join_instruction.
,
physicalTableIdB" Physical table ID.
:�
�

quicksight+DataSetLogicalTableMapSourceJoinInstructionfaws:quicksight/DataSetLogicalTableMapSourceJoinInstruction:DataSetLogicalTableMapSourceJoinInstruction�
��
leftJoinKeyProperties�B�:�
�

quicksight@DataSetLogicalTableMapSourceJoinInstructionLeftJoinKeyProperties�aws:quicksight/DataSetLogicalTableMapSourceJoinInstructionLeftJoinKeyProperties:DataSetLogicalTableMapSourceJoinInstructionLeftJoinKeyPropertiesGJoin key properties of the left operand. See left_join_key_properties.
7
leftOperand" $Operand on the left side of a join.
G
onClause" 7Join instructions provided in the ON clause of a join.
�
rightJoinKeyProperties�B�:�
�

quicksightADataSetLogicalTableMapSourceJoinInstructionRightJoinKeyProperties�aws:quicksight/DataSetLogicalTableMapSourceJoinInstructionRightJoinKeyProperties:DataSetLogicalTableMapSourceJoinInstructionRightJoinKeyPropertiesIJoin key properties of the right operand. See right_join_key_properties.
9
rightOperand" %Operand on the right side of a join.
R
type" FType of join. Valid values are `INNER`, `OUTER`, `LEFT`, and `RIGHT`.
:�
�

quicksight@DataSetLogicalTableMapSourceJoinInstructionLeftJoinKeyProperties�aws:quicksight/DataSetLogicalTableMapSourceJoinInstructionLeftJoinKeyProperties:DataSetLogicalTableMapSourceJoinInstructionLeftJoinKeyProperties�
��
	uniqueKeyB
 �A value that indicates that a row in a table is uniquely identified by the columns in a join key. This is used by Amazon QuickSight to optimize query performance.
:�
�

quicksightADataSetLogicalTableMapSourceJoinInstructionRightJoinKeyProperties�aws:quicksight/DataSetLogicalTableMapSourceJoinInstructionRightJoinKeyProperties:DataSetLogicalTableMapSourceJoinInstructionRightJoinKeyProperties�
��
	uniqueKeyB
 �A value that indicates that a row in a table is uniquely identified by the columns in a join key. This is used by Amazon QuickSight to optimize query performance.
:�
Y

quicksightDataSetOutputColumn6aws:quicksight/DataSetOutputColumn:DataSetOutputColumno
m/
descriptionB" Field folder description.
,
nameB" Display name for the dataset.

typeB" :�
S

quicksightDataSetPermission2aws:quicksight/DataSetPermission:DataSetPermission�
�H
actions*" 7List of IAM actions to grant or revoke permissions on.
�
	principal" �ARN of the principal. See the [ResourcePermission documentation](https://docs.aws.amazon.com/quicksight/latest/APIReference/API_ResourcePermission.html) for the applicable ARN values.
:�
e

quicksightDataSetPhysicalTableMap>aws:quicksight/DataSetPhysicalTableMap:DataSetPhysicalTableMap�
��
	customSql�B�:�
�

quicksight DataSetPhysicalTableMapCustomSqlPaws:quicksight/DataSetPhysicalTableMapCustomSql:DataSetPhysicalTableMapCustomSqlVA physical table type built from the results of the custom SQL query. See custom_sql.
9
physicalTableMapId" Key of the physical table map.
�
relationalTable�B�:�
�

quicksight&DataSetPhysicalTableMapRelationalTable\aws:quicksight/DataSetPhysicalTableMapRelationalTable:DataSetPhysicalTableMapRelationalTableIA physical table type for relational data sources. See relational_table.
�
s3Source�B�:
}

quicksightDataSetPhysicalTableMapS3SourceNaws:quicksight/DataSetPhysicalTableMapS3Source:DataSetPhysicalTableMapS3Source<A physical table type for as S3 data source. See s3_source.
:�
�

quicksight DataSetPhysicalTableMapCustomSqlPaws:quicksight/DataSetPhysicalTableMapCustomSql:DataSetPhysicalTableMapCustomSql�
��
columns�B�*�:�
�

quicksight&DataSetPhysicalTableMapCustomSqlColumn\aws:quicksight/DataSetPhysicalTableMapCustomSqlColumn:DataSetPhysicalTableMapCustomSqlColumn:Column schema from the SQL query result set. See columns.
-
dataSourceArn" ARN of the data source.
3
name" 'Display name for the SQL query result.

sqlQuery" SQL query.
:�
�

quicksight&DataSetPhysicalTableMapCustomSqlColumn\aws:quicksight/DataSetPhysicalTableMapCustomSqlColumn:DataSetPhysicalTableMapCustomSqlColumnj
h?
name" 3Name of this column in the underlying data source.
%
type" Data type of the column.
:�
�

quicksight&DataSetPhysicalTableMapRelationalTable\aws:quicksight/DataSetPhysicalTableMapRelationalTable:DataSetPhysicalTableMapRelationalTable�
�4
catalogB" #Catalog associated with the table.
-
dataSourceArn" ARN of the data source.
�
inputColumns�*�:�
�

quicksight1DataSetPhysicalTableMapRelationalTableInputColumnraws:quicksight/DataSetPhysicalTableMapRelationalTableInputColumn:DataSetPhysicalTableMapRelationalTableInputColumn/Column schema of the table. See input_columns.
*
name" Name of the relational table.
W
schemaB" GSchema name. This name applies to certain relational database engines.
:�
�

quicksight1DataSetPhysicalTableMapRelationalTableInputColumnraws:quicksight/DataSetPhysicalTableMapRelationalTableInputColumn:DataSetPhysicalTableMapRelationalTableInputColumnj
h?
name" 3Name of this column in the underlying data source.
%
type" Data type of the column.
:�
}

quicksightDataSetPhysicalTableMapS3SourceNaws:quicksight/DataSetPhysicalTableMapS3Source:DataSetPhysicalTableMapS3Source�
�-
dataSourceArn" ARN of the data source.
�
inputColumns�*�:�
�

quicksight*DataSetPhysicalTableMapS3SourceInputColumndaws:quicksight/DataSetPhysicalTableMapS3SourceInputColumn:DataSetPhysicalTableMapS3SourceInputColumn/Column schema of the table. See input_columns.
�
uploadSettings�:�
�

quicksight-DataSetPhysicalTableMapS3SourceUploadSettingsjaws:quicksight/DataSetPhysicalTableMapS3SourceUploadSettings:DataSetPhysicalTableMapS3SourceUploadSettingsSInformation about the format for the S3 source file or files. See upload_settings.
:�
�

quicksight*DataSetPhysicalTableMapS3SourceInputColumndaws:quicksight/DataSetPhysicalTableMapS3SourceInputColumn:DataSetPhysicalTableMapS3SourceInputColumnj
h?
name" 3Name of this column in the underlying data source.
%
type" Data type of the column.
:�
�

quicksight-DataSetPhysicalTableMapS3SourceUploadSettingsjaws:quicksight/DataSetPhysicalTableMapS3SourceUploadSettings:DataSetPhysicalTableMapS3SourceUploadSettings�
�`
containsHeaderB
 HWhether the file has a header row, or the files each have a header row.
9
	delimiterB" &Delimiter between values in the file.
^
formatB" NFile format. Valid values are `CSV`, `TSV`, `CLF`, `ELF`, `XLSX`, and `JSON`.
?
startFromRowB )A row number to start reading data from.
[
textQualifierB" DText qualifier. Valid values are `DOUBLE_QUOTE` and `SINGLE_QUOTE`.
:�
h

quicksightDataSetRefreshProperties@aws:quicksight/DataSetRefreshProperties:DataSetRefreshProperties�
��
refreshConfiguration�:�
�

quicksight,DataSetRefreshPropertiesRefreshConfigurationhaws:quicksight/DataSetRefreshPropertiesRefreshConfiguration:DataSetRefreshPropertiesRefreshConfigurationGThe refresh configuration for the data set. See refresh_configuration.
:�
�

quicksight,DataSetRefreshPropertiesRefreshConfigurationhaws:quicksight/DataSetRefreshPropertiesRefreshConfiguration:DataSetRefreshPropertiesRefreshConfiguration�
��
incrementalRefresh�:�
�

quicksight>DataSetRefreshPropertiesRefreshConfigurationIncrementalRefresh�aws:quicksight/DataSetRefreshPropertiesRefreshConfigurationIncrementalRefresh:DataSetRefreshPropertiesRefreshConfigurationIncrementalRefreshCThe incremental refresh for the data set. See incremental_refresh.
:�
�

quicksight>DataSetRefreshPropertiesRefreshConfigurationIncrementalRefresh�aws:quicksight/DataSetRefreshPropertiesRefreshConfigurationIncrementalRefresh:DataSetRefreshPropertiesRefreshConfigurationIncrementalRefresh�
��
lookbackWindow�:�
�

quicksightLDataSetRefreshPropertiesRefreshConfigurationIncrementalRefreshLookbackWindow�aws:quicksight/DataSetRefreshPropertiesRefreshConfigurationIncrementalRefreshLookbackWindow:DataSetRefreshPropertiesRefreshConfigurationIncrementalRefreshLookbackWindowYThe lookback window setup for an incremental refresh configuration. See lookback_window.
:�
�

quicksightLDataSetRefreshPropertiesRefreshConfigurationIncrementalRefreshLookbackWindow�aws:quicksight/DataSetRefreshPropertiesRefreshConfigurationIncrementalRefreshLookbackWindow:DataSetRefreshPropertiesRefreshConfigurationIncrementalRefreshLookbackWindow�
�:

columnName" (The name of the lookback window column.
-
size !The lookback window column size.
�
sizeUnit" zThe size unit that is used for the lookback window column. Valid values for this structure are `HOUR`, `DAY`, and `WEEK`.
:�
�

quicksight DataSetRowLevelPermissionDataSetPaws:quicksight/DataSetRowLevelPermissionDataSet:DataSetRowLevelPermissionDataSet�
�A
arn" 6ARN of the dataset that contains permissions for RLS.
j
formatVersionB" SUser or group rules associated with the dataset that contains permissions for RLS.
\
	namespaceB" INamespace associated with the dataset that contains permissions for RLS.
�
permissionPolicy" yType of permissions to use when interpreting the permissions for RLS. Valid values are `GRANT_ACCESS` and `DENY_ACCESS`.
�
statusB" �Status of the row-level security permission dataset. If enabled, the status is `ENABLED`. If disabled, the status is `DISABLED`.
:�
�

quicksight)DataSetRowLevelPermissionTagConfigurationbaws:quicksight/DataSetRowLevelPermissionTagConfiguration:DataSetRowLevelPermissionTagConfiguration�
��
statusB" sThe status of row-level security tags. If enabled, the status is `ENABLED`. If disabled, the status is `DISABLED`.
�
tagRules�*�:�
�

quicksight0DataSetRowLevelPermissionTagConfigurationTagRulepaws:quicksight/DataSetRowLevelPermissionTagConfigurationTagRule:DataSetRowLevelPermissionTagConfigurationTagRuleA set of rules associated with row-level security, such as the tag names and columns that they are assigned to. See tag_rules.
:�
�

quicksight0DataSetRowLevelPermissionTagConfigurationTagRulepaws:quicksight/DataSetRowLevelPermissionTagConfigurationTagRule:DataSetRowLevelPermissionTagConfigurationTagRule�
�=

columnName" +Column name that a tag key is assigned to.
�
matchAllValueB" �A string that you want to use to filter by all the values in a column in the dataset and don’t want to list the values one by one.
$
tagKey" Unique key for a tag.
z
tagMultiValueDelimiterB" ZA string that you want to use to delimit the values when you pass the values at run time.
:�
_

quicksightDataSourceCredentials:aws:quicksight/DataSourceCredentials:DataSourceCredentials�
��
copySourceArnB" �The Amazon Resource Name (ARN) of a data source that has the credential pair that you want to use.
When the value is not null, the `credential_pair` from the data source in the ARN is used.
�
credentialPair�B�:�
�

quicksight#DataSourceCredentialsCredentialPairVaws:quicksight/DataSourceCredentialsCredentialPair:DataSourceCredentialsCredentialPair=Credential pair. See Credential Pair below for more details.
{
	secretArnB" hThe Amazon Resource Name (ARN) of the secret associated with the data source in Amazon Secrets Manager.
:�
�

quicksight#DataSourceCredentialsCredentialPairVaws:quicksight/DataSourceCredentialsCredentialPair:DataSourceCredentialsCredentialPair
}=
password" -Password, maximum length of 1024 characters.
<
username" ,User name, maximum length of 64 characters.
:�
\

quicksightDataSourceParameters8aws:quicksight/DataSourceParameters:DataSourceParameters�
��
amazonElasticsearch�B�:�
�

quicksight'DataSourceParametersAmazonElasticsearch^aws:quicksight/DataSourceParametersAmazonElasticsearch:DataSourceParametersAmazonElasticsearch3Parameters for connecting to Amazon Elasticsearch.
�
athenatBr:p
n

quicksightDataSourceParametersAthenaDaws:quicksight/DataSourceParametersAthena:DataSourceParametersAthena%Parameters for connecting to Athena.
�
auroratBr:p
n

quicksightDataSourceParametersAuroraDaws:quicksight/DataSourceParametersAurora:DataSourceParametersAurora+Parameters for connecting to Aurora MySQL.
�
auroraPostgresql�B�:�
�

quicksight$DataSourceParametersAuroraPostgresqlXaws:quicksight/DataSourceParametersAuroraPostgresql:DataSourceParametersAuroraPostgresql0Parameters for connecting to Aurora Postgresql.
�
awsIotAnalytics�B�:�
�

quicksight#DataSourceParametersAwsIotAnalyticsVaws:quicksight/DataSourceParametersAwsIotAnalytics:DataSourceParametersAwsIotAnalytics0Parameters for connecting to AWS IOT Analytics.
�

databricks�B~:|
z

quicksightDataSourceParametersDatabricksLaws:quicksight/DataSourceParametersDatabricks:DataSourceParametersDatabricks)Parameters for connecting to Databricks.
�
jiranBl:j
h

quicksightDataSourceParametersJira@aws:quicksight/DataSourceParametersJira:DataSourceParametersJira#Parameters for connecting to Jira.
�
mariaDbwBu:s
q

quicksightDataSourceParametersMariaDbFaws:quicksight/DataSourceParametersMariaDb:DataSourceParametersMariaDb&Parameters for connecting to MariaDB.
�
mysqlqBo:m
k

quicksightDataSourceParametersMysqlBaws:quicksight/DataSourceParametersMysql:DataSourceParametersMysql$Parameters for connecting to MySQL.
�
oracletBr:p
n

quicksightDataSourceParametersOracleDaws:quicksight/DataSourceParametersOracle:DataSourceParametersOracle%Parameters for connecting to Oracle.
�

postgresql�B~:|
z

quicksightDataSourceParametersPostgresqlLaws:quicksight/DataSourceParametersPostgresql:DataSourceParametersPostgresql)Parameters for connecting to Postgresql.
�
prestotBr:p
n

quicksightDataSourceParametersPrestoDaws:quicksight/DataSourceParametersPresto:DataSourceParametersPresto%Parameters for connecting to Presto.
�
rdskBi:g
e

quicksightDataSourceParametersRds>aws:quicksight/DataSourceParametersRds:DataSourceParametersRds"Parameters for connecting to RDS.
�
redshiftzBx:v
t

quicksightDataSourceParametersRedshiftHaws:quicksight/DataSourceParametersRedshift:DataSourceParametersRedshift'Parameters for connecting to Redshift.
�
s3hBf:d
b

quicksightDataSourceParametersS3<aws:quicksight/DataSourceParametersS3:DataSourceParametersS3!Parameters for connecting to S3.
�

serviceNow�B~:|
z

quicksightDataSourceParametersServiceNowLaws:quicksight/DataSourceParametersServiceNow:DataSourceParametersServiceNow)Parameters for connecting to ServiceNow.
�
	snowflake}B{:y
w

quicksightDataSourceParametersSnowflakeJaws:quicksight/DataSourceParametersSnowflake:DataSourceParametersSnowflake(Parameters for connecting to Snowflake.
�
sparkqBo:m
k

quicksightDataSourceParametersSparkBaws:quicksight/DataSourceParametersSpark:DataSourceParametersSpark$Parameters for connecting to Spark.
�
	sqlServer}B{:y
w

quicksightDataSourceParametersSqlServerJaws:quicksight/DataSourceParametersSqlServer:DataSourceParametersSqlServer)Parameters for connecting to SQL Server.
�
teradatazBx:v
t

quicksightDataSourceParametersTeradataHaws:quicksight/DataSourceParametersTeradata:DataSourceParametersTeradata'Parameters for connecting to Teradata.
�
twitterwBu:s
q

quicksightDataSourceParametersTwitterFaws:quicksight/DataSourceParametersTwitter:DataSourceParametersTwitter&Parameters for connecting to Twitter.
:�
�

quicksight'DataSourceParametersAmazonElasticsearch^aws:quicksight/DataSourceParametersAmazonElasticsearch:DataSourceParametersAmazonElasticsearch)
'%
domain" The OpenSearch domain.
:�
n

quicksightDataSourceParametersAthenaDaws:quicksight/DataSourceParametersAthena:DataSourceParametersAthena;
97
	workGroupB" $The work-group to which to connect.
:�
n

quicksightDataSourceParametersAuroraDaws:quicksight/DataSourceParametersAurora:DataSourceParametersAurora�
�2
database" "The database to which to connect.
*
host" The host to which to connect.
*
port The port to which to connect.
:�
�

quicksight$DataSourceParametersAuroraPostgresqlXaws:quicksight/DataSourceParametersAuroraPostgresql:DataSourceParametersAuroraPostgresql�
�2
database" "The database to which to connect.
*
host" The host to which to connect.
*
port The port to which to connect.
:�
�

quicksight#DataSourceParametersAwsIotAnalyticsVaws:quicksight/DataSourceParametersAwsIotAnalytics:DataSourceParametersAwsIotAnalyticsE
CA
dataSetName" .The name of the data set to which to connect.
:�
z

quicksightDataSourceParametersDatabricksLaws:quicksight/DataSourceParametersDatabricks:DataSourceParametersDatabricks�
�9
host" -The host name of the Databricks data source.
5
port )The port for the Databricks data source.
D
sqlEndpointPath" -The HTTP path of the Databricks data source.
:�
h

quicksightDataSourceParametersJira@aws:quicksight/DataSourceParametersJira:DataSourceParametersJiraU
SQ
siteBaseUrl" >The base URL of the Jira instance's site to which to connect.
:�
q

quicksightDataSourceParametersMariaDbFaws:quicksight/DataSourceParametersMariaDb:DataSourceParametersMariaDb�
�2
database" "The database to which to connect.
*
host" The host to which to connect.
*
port The port to which to connect.
:�
k

quicksightDataSourceParametersMysqlBaws:quicksight/DataSourceParametersMysql:DataSourceParametersMysql�
�2
database" "The database to which to connect.
*
host" The host to which to connect.
*
port The port to which to connect.
:�
n

quicksightDataSourceParametersOracleDaws:quicksight/DataSourceParametersOracle:DataSourceParametersOracle�
�2
database" "The database to which to connect.
*
host" The host to which to connect.
*
port The port to which to connect.
:�
z

quicksightDataSourceParametersPostgresqlLaws:quicksight/DataSourceParametersPostgresql:DataSourceParametersPostgresql�
�2
database" "The database to which to connect.
*
host" The host to which to connect.
*
port The port to which to connect.
:�
n

quicksightDataSourceParametersPrestoDaws:quicksight/DataSourceParametersPresto:DataSourceParametersPresto�
�0
catalog" !The catalog to which to connect.
*
host" The host to which to connect.
*
port The port to which to connect.
:�
e

quicksightDataSourceParametersRds>aws:quicksight/DataSourceParametersRds:DataSourceParametersRdso
m2
database" "The database to which to connect.
7

instanceId" %The instance ID to which to connect.
:�
t

quicksightDataSourceParametersRedshiftHaws:quicksight/DataSourceParametersRedshift:DataSourceParametersRedshift�
�>
	clusterIdB" +The ID of the cluster to which to connect.
2
database" "The database to which to connect.
,
hostB" The host to which to connect.
,
portB The port to which to connect.
:�
b

quicksightDataSourceParametersS3<aws:quicksight/DataSourceParametersS3:DataSourceParametersS3�
��
manifestFileLocation�:�
�

quicksight*DataSourceParametersS3ManifestFileLocationdaws:quicksight/DataSourceParametersS3ManifestFileLocation:DataSourceParametersS3ManifestFileLocation>An object containing the S3 location of the S3 manifest file.
:�
�

quicksight*DataSourceParametersS3ManifestFileLocationdaws:quicksight/DataSourceParametersS3ManifestFileLocation:DataSourceParametersS3ManifestFileLocation�
�F
bucket" 8The name of the bucket that contains the manifest file.
;
key" 0The key of the manifest file within the bucket.
:�
z

quicksightDataSourceParametersServiceNowLaws:quicksight/DataSourceParametersServiceNow:DataSourceParametersServiceNowU
SQ
siteBaseUrl" >The base URL of the Jira instance's site to which to connect.
:�
w

quicksightDataSourceParametersSnowflakeJaws:quicksight/DataSourceParametersSnowflake:DataSourceParametersSnowflake�
�2
database" "The database to which to connect.
*
host" The host to which to connect.
4
	warehouse" #The warehouse to which to connect.
:�
k

quicksightDataSourceParametersSparkBaws:quicksight/DataSourceParametersSpark:DataSourceParametersSpark_
]*
host" The host to which to connect.
/
port #The warehouse to which to connect.
:�
w

quicksightDataSourceParametersSqlServerJaws:quicksight/DataSourceParametersSqlServer:DataSourceParametersSqlServer�
�2
database" "The database to which to connect.
*
host" The host to which to connect.
/
port #The warehouse to which to connect.
:�
t

quicksightDataSourceParametersTeradataHaws:quicksight/DataSourceParametersTeradata:DataSourceParametersTeradata�
�2
database" "The database to which to connect.
*
host" The host to which to connect.
/
port #The warehouse to which to connect.
:�
q

quicksightDataSourceParametersTwitterFaws:quicksight/DataSourceParametersTwitter:DataSourceParametersTwittero
m4
maxRows %The maximum number of rows to query.
5
query" (The Twitter query to retrieve the data.
:�
\

quicksightDataSourcePermission8aws:quicksight/DataSourcePermission:DataSourcePermission�
�X
actions*" GSet of IAM actions to grant or revoke permissions on. Max of 16 items.
B
	principal" 1The Amazon Resource Name (ARN) of the principal.
:�
e

quicksightDataSourceSslProperties>aws:quicksight/DataSourceSslProperties:DataSourceSslPropertiesR
PN

disableSsl
 <A Boolean option to control whether SSL should be disabled.
:�
�

quicksight!DataSourceVpcConnectionPropertiesRaws:quicksight/DataSourceVpcConnectionProperties:DataSourceVpcConnectionPropertiesS
QO
vpcConnectionArn" 7The Amazon Resource Name (ARN) for the VPC connection.
:�
P

quicksightFolderPermission0aws:quicksight/FolderPermission:FolderPermission�
�H
actions*" 7List of IAM actions to grant or revoke permissions on.
�
	principal" �ARN of the principal. See the [ResourcePermission documentation](https://docs.aws.amazon.com/quicksight/latest/APIReference/API_ResourcePermission.html) for the applicable ARN values.
:�
w

quicksightIamPolicyAssignmentIdentitiesJaws:quicksight/IamPolicyAssignmentIdentities:IamPolicyAssignmentIdentities�
�K
groupsB*" 9Array of Quicksight group names to assign the policy to.
I
usersB*" 8Array of Quicksight user names to assign the policy to.
:�
S

quicksightNamespaceTimeouts2aws:quicksight/NamespaceTimeouts:NamespaceTimeouts�
��
createB" �A string that can be [parsed as a duration](https://pkg.go.dev/time#ParseDuration) consisting of numbers and unit suffixes, such as "30s" or "2h45m". Valid time units are "s" (seconds), "m" (minutes), "h" (hours).
�
deleteB" �A string that can be [parsed as a duration](https://pkg.go.dev/time#ParseDuration) consisting of numbers and unit suffixes, such as "30s" or "2h45m". Valid time units are "s" (seconds), "m" (minutes), "h" (hours). Setting a timeout for a Delete operation is only applicable if changes are saved into state before the destroy operation occurs.
:�
e

quicksightRefreshScheduleSchedule>aws:quicksight/RefreshScheduleSchedule:RefreshScheduleSchedule�
�~
refreshType" kThe type of refresh that the dataset undergoes. Valid values are `INCREMENTAL_REFRESH` and `FULL_REFRESH`.
�
scheduleFrequency�B�:�
�

quicksight(RefreshScheduleScheduleScheduleFrequency`aws:quicksight/RefreshScheduleScheduleScheduleFrequency:RefreshScheduleScheduleScheduleFrequency�The configuration of the [schedule frequency](https://docs.aws.amazon.com/quicksight/latest/APIReference/API_RefreshFrequency.html). See schedule_frequency.
}
startAfterDateTimeB" aTime after which the refresh schedule can be started, expressed in `YYYY-MM-DDTHH:MM:SS` format.
:�
�

quicksight(RefreshScheduleScheduleScheduleFrequency`aws:quicksight/RefreshScheduleScheduleScheduleFrequency:RefreshScheduleScheduleScheduleFrequency�
��
interval" ~The interval between scheduled refreshes. Valid values are `MINUTE15`, `MINUTE30`, `HOURLY`, `DAILY`, `WEEKLY` and `MONTHLY`.
�
refreshOnDay�B�:�
�

quicksight4RefreshScheduleScheduleScheduleFrequencyRefreshOnDayxaws:quicksight/RefreshScheduleScheduleScheduleFrequencyRefreshOnDay:RefreshScheduleScheduleScheduleFrequencyRefreshOnDay�The [refresh on entity](https://docs.aws.amazon.com/quicksight/latest/APIReference/API_ScheduleRefreshOnEntity.html) configuration for weekly or monthly schedules. See refresh_on_day.
�
timeOfTheDayB" �The time of day that you want the dataset to refresh. This value is expressed in `HH:MM` format. This field is not required for schedules that refresh hourly.
J
timezoneB" 8The timezone that you want the refresh schedule to use.
:�
�

quicksight4RefreshScheduleScheduleScheduleFrequencyRefreshOnDayxaws:quicksight/RefreshScheduleScheduleScheduleFrequencyRefreshOnDay:RefreshScheduleScheduleScheduleFrequencyRefreshOnDay�
�O

dayOfMonthB" ;The day of the month that you want to schedule refresh on.
�
	dayOfWeekB" �The day of the week that you want to schedule a refresh on. Valid values are `SUNDAY`, `MONDAY`, `TUESDAY`, `WEDNESDAY`, `THURSDAY`, `FRIDAY` and `SATURDAY`.
:�
V

quicksightTemplatePermission4aws:quicksight/TemplatePermission:TemplatePermission�
�H
actions*" 7List of IAM actions to grant or revoke permissions on.
�
	principal" �ARN of the principal. See the [ResourcePermission documentation](https://docs.aws.amazon.com/quicksight/latest/APIReference/API_ResourcePermission.html) for the applicable ARN values.
:�
\

quicksightTemplateSourceEntity8aws:quicksight/TemplateSourceEntity:TemplateSourceEntity�
��
sourceAnalysis�B�:�
�

quicksight"TemplateSourceEntitySourceAnalysisTaws:quicksight/TemplateSourceEntitySourceAnalysis:TemplateSourceEntitySourceAnalysis�The source analysis, if it is based on an analysis.. Only one of `source_analysis` or `source_template` should be configured. See source_analysis.
�
sourceTemplate�B�:�
�

quicksight"TemplateSourceEntitySourceTemplateTaws:quicksight/TemplateSourceEntitySourceTemplate:TemplateSourceEntitySourceTemplate�The source template, if it is based on an template.. Only one of `source_analysis` or `source_template` should be configured. See source_template.
:�
�

quicksight"TemplateSourceEntitySourceAnalysisTaws:quicksight/TemplateSourceEntitySourceAnalysis:TemplateSourceEntitySourceAnalysis�
�;
arn" 0The Amazon Resource Name (ARN) of the resource.
�
dataSetReferences�*�:�
�

quicksight2TemplateSourceEntitySourceAnalysisDataSetReferencetaws:quicksight/TemplateSourceEntitySourceAnalysisDataSetReference:TemplateSourceEntitySourceAnalysisDataSetReference\A list of dataset references used as placeholders in the template. See data_set_references.
:�
�

quicksight2TemplateSourceEntitySourceAnalysisDataSetReferencetaws:quicksight/TemplateSourceEntitySourceAnalysisDataSetReference:TemplateSourceEntitySourceAnalysisDataSetReferencek
i6

dataSetArn" $Dataset Amazon Resource Name (ARN).
/
dataSetPlaceholder" Dataset placeholder.
:�
�

quicksight"TemplateSourceEntitySourceTemplateTaws:quicksight/TemplateSourceEntitySourceTemplate:TemplateSourceEntitySourceTemplate?
=;
arn" 0The Amazon Resource Name (ARN) of the resource.
:�
V

quicksightThemeConfiguration4aws:quicksight/ThemeConfiguration:ThemeConfiguration�
��
dataColorPalette�B�:�
�

quicksight"ThemeConfigurationDataColorPaletteTaws:quicksight/ThemeConfigurationDataColorPalette:ThemeConfigurationDataColorPaletteJColor properties that apply to chart data colors. See data_color_palette.
�
sheetkBi:g
e

quicksightThemeConfigurationSheet>aws:quicksight/ThemeConfigurationSheet:ThemeConfigurationSheet.Display options related to sheets. See sheet.
�

typographyzBx:v
t

quicksightThemeConfigurationTypographyHaws:quicksight/ThemeConfigurationTypography:ThemeConfigurationTypography3Determines the typography options. See typography.
�
uiColorPalette�B�:�
�

quicksight ThemeConfigurationUiColorPalettePaws:quicksight/ThemeConfigurationUiColorPalette:ThemeConfigurationUiColorPalettetColor properties that apply to the UI and to charts, excluding the colors that apply to data. See ui_color_palette.
:�
�

quicksight"ThemeConfigurationDataColorPaletteTaws:quicksight/ThemeConfigurationDataColorPalette:ThemeConfigurationDataColorPalette�
�h
colorsB*" VList of hexadecimal codes for the colors. Minimum of 8 items and maximum of 20 items.
t
emptyFillColorB" \The hexadecimal code of a color that applies to charts where a lack of data is highlighted.
~
minMaxGradientsB*" cThe minimum and maximum hexadecimal codes that describe a color gradient. List of exactly 2 items.
:�
e

quicksightThemeConfigurationSheet>aws:quicksight/ThemeConfigurationSheet:ThemeConfigurationSheet�
��
tilewBu:s
q

quicksightThemeConfigurationSheetTileFaws:quicksight/ThemeConfigurationSheetTile:ThemeConfigurationSheetTile)The display options for tiles. See tile.
�

tileLayout�B�:�
�

quicksight!ThemeConfigurationSheetTileLayoutRaws:quicksight/ThemeConfigurationSheetTileLayout:ThemeConfigurationSheetTileLayout/The layout options for tiles. See tile_layout.
:�
q

quicksightThemeConfigurationSheetTileFaws:quicksight/ThemeConfigurationSheetTile:ThemeConfigurationSheetTile�
��
border�B�:�
�

quicksight!ThemeConfigurationSheetTileBorderRaws:quicksight/ThemeConfigurationSheetTileBorder:ThemeConfigurationSheetTileBorder&The border around a tile. See border.
:�
�

quicksight!ThemeConfigurationSheetTileBorderRaws:quicksight/ThemeConfigurationSheetTileBorder:ThemeConfigurationSheetTileBorderG
EC
showB
 5The option to enable display of borders for visuals.
:�
�

quicksight!ThemeConfigurationSheetTileLayoutRaws:quicksight/ThemeConfigurationSheetTileLayout:ThemeConfigurationSheetTileLayout�
��
gutter�B�:�
�

quicksight'ThemeConfigurationSheetTileLayoutGutter^aws:quicksight/ThemeConfigurationSheetTileLayoutGutter:ThemeConfigurationSheetTileLayoutGutter:The gutter settings that apply between tiles. See gutter.
�
margin�B�:�
�

quicksight'ThemeConfigurationSheetTileLayoutMargin^aws:quicksight/ThemeConfigurationSheetTileLayoutMargin:ThemeConfigurationSheetTileLayoutMarginNThe margin settings that apply around the outside edge of sheets. See margin.
:�
�

quicksight'ThemeConfigurationSheetTileLayoutGutter^aws:quicksight/ThemeConfigurationSheetTileLayoutGutter:ThemeConfigurationSheetTileLayoutGuttere
ca
showB
 SThis Boolean value controls whether to display a gutter space between sheet tiles.
:�
�

quicksight'ThemeConfigurationSheetTileLayoutMargin^aws:quicksight/ThemeConfigurationSheetTileLayoutMargin:ThemeConfigurationSheetTileLayoutMarginP
NL
showB
 >This Boolean value controls whether to display sheet margins.
:�
t

quicksightThemeConfigurationTypographyHaws:quicksight/ThemeConfigurationTypography:ThemeConfigurationTypography�
��
fontFamilies�B�*�:�
�

quicksight&ThemeConfigurationTypographyFontFamily\aws:quicksight/ThemeConfigurationTypographyFontFamily:ThemeConfigurationTypographyFontFamilyTDetermines the list of font families. Maximum number of 5 items. See font_families.
:�
�

quicksight&ThemeConfigurationTypographyFontFamily\aws:quicksight/ThemeConfigurationTypographyFontFamily:ThemeConfigurationTypographyFontFamily*
(&

fontFamilyB" Font family name.
:�
�

quicksight ThemeConfigurationUiColorPalettePaws:quicksight/ThemeConfigurationUiColorPalette:ThemeConfigurationUiColorPalette�
�Q
accentB" AColor (hexadecimal) that applies to selected states and buttons.
|
accentForegroundB" bColor (hexadecimal) that applies to any text or other elements that appear over the accent color.
D
dangerB" 4Color (hexadecimal) that applies to error messages.
{
dangerForegroundB" aColor (hexadecimal) that applies to any text or other elements that appear over the error color.
n
	dimensionB" [Color (hexadecimal) that applies to the names of fields that are identified as dimensions.
�
dimensionForegroundB" eColor (hexadecimal) that applies to any text or other elements that appear over the dimension color.
j
measureB" YColor (hexadecimal) that applies to the names of fields that are identified as measures.
~
measureForegroundB" cColor (hexadecimal) that applies to any text or other elements that appear over the measure color.
c
primaryBackgroundB" HColor (hexadecimal) that applies to visuals and other high emphasis UI.
�
primaryForegroundB" �Color (hexadecimal) of text and other foreground elements that appear over the primary background regions, such as grid lines, borders, table banding, icons, and so on.
j
secondaryBackgroundB" MColor (hexadecimal) that applies to the sheet background and sheet controls.
�
secondaryForegroundB" {Color (hexadecimal) that applies to any sheet title, sheet control text, or UI that appears over the secondary background.
}
successB" lColor (hexadecimal) that applies to success messages, for example the check mark for a successful download.
~
successForegroundB" cColor (hexadecimal) that applies to any text or other elements that appear over the success color.
Y
warningB" HColor (hexadecimal) that applies to warning and informational messages.
~
warningForegroundB" cColor (hexadecimal) that applies to any text or other elements that appear over the warning color.
:�
M

quicksightThemePermission.aws:quicksight/ThemePermission:ThemePermission�
�H
actions*" 7List of IAM actions to grant or revoke permissions on.
�
	principal" �ARN of the principal. See the [ResourcePermission documentation](https://docs.aws.amazon.com/quicksight/latest/APIReference/API_ResourcePermission.html) for the applicable ARN values.
:�
_

quicksightVpcConnectionTimeouts:aws:quicksight/VpcConnectionTimeouts:VpcConnectionTimeouts�
��
createB" �A string that can be [parsed as a duration](https://pkg.go.dev/time#ParseDuration) consisting of numbers and unit suffixes, such as "30s" or "2h45m". Valid time units are "s" (seconds), "m" (minutes), "h" (hours).
�
deleteB" �A string that can be [parsed as a duration](https://pkg.go.dev/time#ParseDuration) consisting of numbers and unit suffixes, such as "30s" or "2h45m". Valid time units are "s" (seconds), "m" (minutes), "h" (hours). Setting a timeout for a Delete operation is only applicable if changes are saved into state before the destroy operation occurs.
�
updateB" �A string that can be [parsed as a duration](https://pkg.go.dev/time#ParseDuration) consisting of numbers and unit suffixes, such as "30s" or "2h45m". Valid time units are "s" (seconds), "m" (minutes), "h" (hours).
:�
_

quicksightgetAnalysisPermission:aws:quicksight/getAnalysisPermission:getAnalysisPermission$
"
actions*" 
	principal" :�
_

quicksightgetDataSetColumnGroup:aws:quicksight/getDataSetColumnGroup:getDataSetColumnGroup�
��
geoSpatialColumnGroups�*�:�
�

quicksight*getDataSetColumnGroupGeoSpatialColumnGroupdaws:quicksight/getDataSetColumnGroupGeoSpatialColumnGroup:getDataSetColumnGroupGeoSpatialColumnGroup:�
�

quicksight*getDataSetColumnGroupGeoSpatialColumnGroupdaws:quicksight/getDataSetColumnGroupGeoSpatialColumnGroup:getDataSetColumnGroupGeoSpatialColumnGroup2
0
columns*" 
countryCode" 

name" :�
�

quicksight#getDataSetColumnLevelPermissionRuleVaws:quicksight/getDataSetColumnLevelPermissionRule:getDataSetColumnLevelPermissionRule+
)
columnNames*" 

principals*" :�
�

quicksight#getDataSetDataSetUsageConfigurationVaws:quicksight/getDataSetDataSetUsageConfiguration:getDataSetDataSetUsageConfigurationI
G#
disableUseAsDirectQuerySource
  
disableUseAsImportedSource
 :�
_

quicksightgetDataSetFieldFolder:aws:quicksight/getDataSetFieldFolder:getDataSetFieldFolder<
:
columns*" 
description" 
fieldFoldersId" :�
k

quicksightgetDataSetLogicalTableMapBaws:quicksight/getDataSetLogicalTableMap:getDataSetLogicalTableMap�
�
alias" �
dataTransforms�*�:�
�

quicksight&getDataSetLogicalTableMapDataTransform\aws:quicksight/getDataSetLogicalTableMapDataTransform:getDataSetLogicalTableMapDataTransform
logicalTableMapId" �
sources�*�:
}

quicksightgetDataSetLogicalTableMapSourceNaws:quicksight/getDataSetLogicalTableMapSource:getDataSetLogicalTableMapSource:�
�

quicksight&getDataSetLogicalTableMapDataTransform\aws:quicksight/getDataSetLogicalTableMapDataTransform:getDataSetLogicalTableMapDataTransform�
��
castColumnTypeOperations�*�:�
�

quicksight=getDataSetLogicalTableMapDataTransformCastColumnTypeOperation�aws:quicksight/getDataSetLogicalTableMapDataTransformCastColumnTypeOperation:getDataSetLogicalTableMapDataTransformCastColumnTypeOperation�
createColumnsOperations�*�:�
�

quicksight<getDataSetLogicalTableMapDataTransformCreateColumnsOperation�aws:quicksight/getDataSetLogicalTableMapDataTransformCreateColumnsOperation:getDataSetLogicalTableMapDataTransformCreateColumnsOperation�
filterOperations�*�:�
�

quicksight5getDataSetLogicalTableMapDataTransformFilterOperationzaws:quicksight/getDataSetLogicalTableMapDataTransformFilterOperation:getDataSetLogicalTableMapDataTransformFilterOperation�
projectOperations�*�:�
�

quicksight6getDataSetLogicalTableMapDataTransformProjectOperation|aws:quicksight/getDataSetLogicalTableMapDataTransformProjectOperation:getDataSetLogicalTableMapDataTransformProjectOperation�
renameColumnOperations�*�:�
�

quicksight;getDataSetLogicalTableMapDataTransformRenameColumnOperation�aws:quicksight/getDataSetLogicalTableMapDataTransformRenameColumnOperation:getDataSetLogicalTableMapDataTransformRenameColumnOperation�
tagColumnOperations�*�:�
�

quicksight8getDataSetLogicalTableMapDataTransformTagColumnOperation�aws:quicksight/getDataSetLogicalTableMapDataTransformTagColumnOperation:getDataSetLogicalTableMapDataTransformTagColumnOperation�
untagColumnOperations�*�:�
�

quicksight:getDataSetLogicalTableMapDataTransformUntagColumnOperation�aws:quicksight/getDataSetLogicalTableMapDataTransformUntagColumnOperation:getDataSetLogicalTableMapDataTransformUntagColumnOperation:�
�

quicksight=getDataSetLogicalTableMapDataTransformCastColumnTypeOperation�aws:quicksight/getDataSetLogicalTableMapDataTransformCastColumnTypeOperation:getDataSetLogicalTableMapDataTransformCastColumnTypeOperation7
5

columnName" 
format" 
newColumnType" :�
�

quicksight<getDataSetLogicalTableMapDataTransformCreateColumnsOperation�aws:quicksight/getDataSetLogicalTableMapDataTransformCreateColumnsOperation:getDataSetLogicalTableMapDataTransformCreateColumnsOperation�
��
columns�*�:�
�

quicksightBgetDataSetLogicalTableMapDataTransformCreateColumnsOperationColumn�aws:quicksight/getDataSetLogicalTableMapDataTransformCreateColumnsOperationColumn:getDataSetLogicalTableMapDataTransformCreateColumnsOperationColumn:�
�

quicksightBgetDataSetLogicalTableMapDataTransformCreateColumnsOperationColumn�aws:quicksight/getDataSetLogicalTableMapDataTransformCreateColumnsOperationColumn:getDataSetLogicalTableMapDataTransformCreateColumnsOperationColumn6
4
columnId" 

columnName" 

expression" :�
�

quicksight5getDataSetLogicalTableMapDataTransformFilterOperationzaws:quicksight/getDataSetLogicalTableMapDataTransformFilterOperation:getDataSetLogicalTableMapDataTransformFilterOperation

conditionExpression" :�
�

quicksight6getDataSetLogicalTableMapDataTransformProjectOperation|aws:quicksight/getDataSetLogicalTableMapDataTransformProjectOperation:getDataSetLogicalTableMapDataTransformProjectOperation

projectedColumns*" :�
�

quicksight;getDataSetLogicalTableMapDataTransformRenameColumnOperation�aws:quicksight/getDataSetLogicalTableMapDataTransformRenameColumnOperation:getDataSetLogicalTableMapDataTransformRenameColumnOperation)
'

columnName" 
newColumnName" :�
�

quicksight8getDataSetLogicalTableMapDataTransformTagColumnOperation�aws:quicksight/getDataSetLogicalTableMapDataTransformTagColumnOperation:getDataSetLogicalTableMapDataTransformTagColumnOperation�
�

columnName" �
tags�*�:�
�

quicksight;getDataSetLogicalTableMapDataTransformTagColumnOperationTag�aws:quicksight/getDataSetLogicalTableMapDataTransformTagColumnOperationTag:getDataSetLogicalTableMapDataTransformTagColumnOperationTag:�
�

quicksight;getDataSetLogicalTableMapDataTransformTagColumnOperationTag�aws:quicksight/getDataSetLogicalTableMapDataTransformTagColumnOperationTag:getDataSetLogicalTableMapDataTransformTagColumnOperationTag�
��
columnDescriptions�*�:�
�

quicksightLgetDataSetLogicalTableMapDataTransformTagColumnOperationTagColumnDescription�aws:quicksight/getDataSetLogicalTableMapDataTransformTagColumnOperationTagColumnDescription:getDataSetLogicalTableMapDataTransformTagColumnOperationTagColumnDescription
columnGeographicRole" :�
�

quicksightLgetDataSetLogicalTableMapDataTransformTagColumnOperationTagColumnDescription�aws:quicksight/getDataSetLogicalTableMapDataTransformTagColumnOperationTagColumnDescription:getDataSetLogicalTableMapDataTransformTagColumnOperationTagColumnDescription


text" :�
�

quicksight:getDataSetLogicalTableMapDataTransformUntagColumnOperation�aws:quicksight/getDataSetLogicalTableMapDataTransformUntagColumnOperation:getDataSetLogicalTableMapDataTransformUntagColumnOperation&
$

columnName" 
tagNames*" :�
}

quicksightgetDataSetLogicalTableMapSourceNaws:quicksight/getDataSetLogicalTableMapSource:getDataSetLogicalTableMapSource�
�

dataSetArn" �
joinInstructions�*�:�
�

quicksight.getDataSetLogicalTableMapSourceJoinInstructionlaws:quicksight/getDataSetLogicalTableMapSourceJoinInstruction:getDataSetLogicalTableMapSourceJoinInstruction
physicalTableId" :�
�

quicksight.getDataSetLogicalTableMapSourceJoinInstructionlaws:quicksight/getDataSetLogicalTableMapSourceJoinInstruction:getDataSetLogicalTableMapSourceJoinInstruction�
��
leftJoinKeyProperties�*�:�
�

quicksightAgetDataSetLogicalTableMapSourceJoinInstructionLeftJoinKeyProperty�aws:quicksight/getDataSetLogicalTableMapSourceJoinInstructionLeftJoinKeyProperty:getDataSetLogicalTableMapSourceJoinInstructionLeftJoinKeyProperty
leftOperand" 
onClause" �
rightJoinKeyProperties�*�:�
�

quicksightBgetDataSetLogicalTableMapSourceJoinInstructionRightJoinKeyProperty�aws:quicksight/getDataSetLogicalTableMapSourceJoinInstructionRightJoinKeyProperty:getDataSetLogicalTableMapSourceJoinInstructionRightJoinKeyProperty
rightOperand" 

type" :�
�

quicksightAgetDataSetLogicalTableMapSourceJoinInstructionLeftJoinKeyProperty�aws:quicksight/getDataSetLogicalTableMapSourceJoinInstructionLeftJoinKeyProperty:getDataSetLogicalTableMapSourceJoinInstructionLeftJoinKeyProperty

	uniqueKey
 :�
�

quicksightBgetDataSetLogicalTableMapSourceJoinInstructionRightJoinKeyProperty�aws:quicksight/getDataSetLogicalTableMapSourceJoinInstructionRightJoinKeyProperty:getDataSetLogicalTableMapSourceJoinInstructionRightJoinKeyProperty

	uniqueKey
 :�
\

quicksightgetDataSetPermission8aws:quicksight/getDataSetPermission:getDataSetPermission$
"
actions*" 
	principal" :�
n

quicksightgetDataSetPhysicalTableMapDaws:quicksight/getDataSetPhysicalTableMap:getDataSetPhysicalTableMap�
��

customSqls�*�:�
�

quicksight#getDataSetPhysicalTableMapCustomSqlVaws:quicksight/getDataSetPhysicalTableMapCustomSql:getDataSetPhysicalTableMapCustomSql
physicalTableMapId" �
relationalTables�*�:�
�

quicksight)getDataSetPhysicalTableMapRelationalTablebaws:quicksight/getDataSetPhysicalTableMapRelationalTable:getDataSetPhysicalTableMapRelationalTable�
	s3Sources�*�:�
�

quicksight"getDataSetPhysicalTableMapS3SourceTaws:quicksight/getDataSetPhysicalTableMapS3Source:getDataSetPhysicalTableMapS3Source:�
�

quicksight#getDataSetPhysicalTableMapCustomSqlVaws:quicksight/getDataSetPhysicalTableMapCustomSql:getDataSetPhysicalTableMapCustomSql�
��
columns�*�:�
�

quicksight)getDataSetPhysicalTableMapCustomSqlColumnbaws:quicksight/getDataSetPhysicalTableMapCustomSqlColumn:getDataSetPhysicalTableMapCustomSqlColumn
dataSourceArn" 

name" 
sqlQuery" :�
�

quicksight)getDataSetPhysicalTableMapCustomSqlColumnbaws:quicksight/getDataSetPhysicalTableMapCustomSqlColumn:getDataSetPhysicalTableMapCustomSqlColumn


name" 

type" :�
�

quicksight)getDataSetPhysicalTableMapRelationalTablebaws:quicksight/getDataSetPhysicalTableMapRelationalTable:getDataSetPhysicalTableMapRelationalTable�
�
catalog" 
dataSourceArn" �
inputColumns�*�:�
�

quicksight4getDataSetPhysicalTableMapRelationalTableInputColumnxaws:quicksight/getDataSetPhysicalTableMapRelationalTableInputColumn:getDataSetPhysicalTableMapRelationalTableInputColumn

name" 
schema" :�
�

quicksight4getDataSetPhysicalTableMapRelationalTableInputColumnxaws:quicksight/getDataSetPhysicalTableMapRelationalTableInputColumn:getDataSetPhysicalTableMapRelationalTableInputColumn


name" 

type" :�
�

quicksight"getDataSetPhysicalTableMapS3SourceTaws:quicksight/getDataSetPhysicalTableMapS3Source:getDataSetPhysicalTableMapS3Source�
�
dataSourceArn" �
inputColumns�*�:�
�

quicksight-getDataSetPhysicalTableMapS3SourceInputColumnjaws:quicksight/getDataSetPhysicalTableMapS3SourceInputColumn:getDataSetPhysicalTableMapS3SourceInputColumn�
uploadSettings�*�:�
�

quicksight/getDataSetPhysicalTableMapS3SourceUploadSettingnaws:quicksight/getDataSetPhysicalTableMapS3SourceUploadSetting:getDataSetPhysicalTableMapS3SourceUploadSetting:�
�

quicksight-getDataSetPhysicalTableMapS3SourceInputColumnjaws:quicksight/getDataSetPhysicalTableMapS3SourceInputColumn:getDataSetPhysicalTableMapS3SourceInputColumn


name" 

type" :�
�

quicksight/getDataSetPhysicalTableMapS3SourceUploadSettingnaws:quicksight/getDataSetPhysicalTableMapS3SourceUploadSetting:getDataSetPhysicalTableMapS3SourceUploadSetting`
^
containsHeader
 
	delimiter" 
format" 
startFromRow 
textQualifier" :�
�

quicksight#getDataSetRowLevelPermissionDataSetVaws:quicksight/getDataSetRowLevelPermissionDataSet:getDataSetRowLevelPermissionDataSetY
W	
arn" 
formatVersion" 
	namespace" 
permissionPolicy" 
status" :�
�

quicksight,getDataSetRowLevelPermissionTagConfigurationhaws:quicksight/getDataSetRowLevelPermissionTagConfiguration:getDataSetRowLevelPermissionTagConfiguration�
�
status" �
tagRules�*�:�
�

quicksight3getDataSetRowLevelPermissionTagConfigurationTagRulevaws:quicksight/getDataSetRowLevelPermissionTagConfigurationTagRule:getDataSetRowLevelPermissionTagConfigurationTagRule:�
�

quicksight3getDataSetRowLevelPermissionTagConfigurationTagRulevaws:quicksight/getDataSetRowLevelPermissionTagConfigurationTagRule:getDataSetRowLevelPermissionTagConfigurationTagRuleU
S

columnName" 
matchAllValue" 
tagKey" 
tagMultiValueDelimiter" :�
}

quicksightgetQuicksightAnalysisPermissionNaws:quicksight/getQuicksightAnalysisPermission:getQuicksightAnalysisPermission$
"
actions*" 
	principal" :�
_

quicksightgetThemeConfiguration:aws:quicksight/getThemeConfiguration:getThemeConfiguration�
��
dataColorPalettes�*�:�
�

quicksight%getThemeConfigurationDataColorPaletteZaws:quicksight/getThemeConfigurationDataColorPalette:getThemeConfigurationDataColorPaletteJColor properties that apply to chart data colors. See data_color_palette.
�
sheetst*r:p
n

quicksightgetThemeConfigurationSheetDaws:quicksight/getThemeConfigurationSheet:getThemeConfigurationSheet.Display options related to sheets. See sheet.
�
typographies�*�:
}

quicksightgetThemeConfigurationTypographyNaws:quicksight/getThemeConfigurationTypography:getThemeConfigurationTypography3Determines the typography options. See typography.
�
uiColorPalettes�*�:�
�

quicksight#getThemeConfigurationUiColorPaletteVaws:quicksight/getThemeConfigurationUiColorPalette:getThemeConfigurationUiColorPalettetColor properties that apply to the UI and to charts, excluding the colors that apply to data. See ui_color_palette.
:�
�

quicksight%getThemeConfigurationDataColorPaletteZaws:quicksight/getThemeConfigurationDataColorPalette:getThemeConfigurationDataColorPalette�
�f
colors*" VList of hexadecimal codes for the colors. Minimum of 8 items and maximum of 20 items.
r
emptyFillColor" \The hexadecimal code of a color that applies to charts where a lack of data is highlighted.
|
minMaxGradients*" cThe minimum and maximum hexadecimal codes that describe a color gradient. List of exactly 2 items.
:�
n

quicksightgetThemeConfigurationSheetDaws:quicksight/getThemeConfigurationSheet:getThemeConfigurationSheet�
��
tileLayouts�*�:�
�

quicksight$getThemeConfigurationSheetTileLayoutXaws:quicksight/getThemeConfigurationSheetTileLayout:getThemeConfigurationSheetTileLayout/The layout options for tiles. See tile_layout.
�
tiles�*~:|
z

quicksightgetThemeConfigurationSheetTileLaws:quicksight/getThemeConfigurationSheetTile:getThemeConfigurationSheetTile)The display options for tiles. See tile.
:�
z

quicksightgetThemeConfigurationSheetTileLaws:quicksight/getThemeConfigurationSheetTile:getThemeConfigurationSheetTile�
��
borders�*�:�
�

quicksight$getThemeConfigurationSheetTileBorderXaws:quicksight/getThemeConfigurationSheetTileBorder:getThemeConfigurationSheetTileBorder&The border around a tile. See border.
:�
�

quicksight$getThemeConfigurationSheetTileBorderXaws:quicksight/getThemeConfigurationSheetTileBorder:getThemeConfigurationSheetTileBorderN
LJ
show
 >This Boolean value controls whether to display sheet margins.
:�
�

quicksight$getThemeConfigurationSheetTileLayoutXaws:quicksight/getThemeConfigurationSheetTileLayout:getThemeConfigurationSheetTileLayout�
��
gutters�*�:�
�

quicksight*getThemeConfigurationSheetTileLayoutGutterdaws:quicksight/getThemeConfigurationSheetTileLayoutGutter:getThemeConfigurationSheetTileLayoutGutter:The gutter settings that apply between tiles. See gutter.
�
margins�*�:�
�

quicksight*getThemeConfigurationSheetTileLayoutMargindaws:quicksight/getThemeConfigurationSheetTileLayoutMargin:getThemeConfigurationSheetTileLayoutMarginNThe margin settings that apply around the outside edge of sheets. See margin.
:�
�

quicksight*getThemeConfigurationSheetTileLayoutGutterdaws:quicksight/getThemeConfigurationSheetTileLayoutGutter:getThemeConfigurationSheetTileLayoutGutterN
LJ
show
 >This Boolean value controls whether to display sheet margins.
:�
�

quicksight*getThemeConfigurationSheetTileLayoutMargindaws:quicksight/getThemeConfigurationSheetTileLayoutMargin:getThemeConfigurationSheetTileLayoutMarginN
LJ
show
 >This Boolean value controls whether to display sheet margins.
:�
}

quicksightgetThemeConfigurationTypographyNaws:quicksight/getThemeConfigurationTypography:getThemeConfigurationTypography�
��
fontFamilies�*�:�
�

quicksight)getThemeConfigurationTypographyFontFamilybaws:quicksight/getThemeConfigurationTypographyFontFamily:getThemeConfigurationTypographyFontFamilyTDetermines the list of font families. Maximum number of 5 items. See font_families.
:�
�

quicksight)getThemeConfigurationTypographyFontFamilybaws:quicksight/getThemeConfigurationTypographyFontFamily:getThemeConfigurationTypographyFontFamily(
&$

fontFamily" Font family name.
:�
�

quicksight#getThemeConfigurationUiColorPaletteVaws:quicksight/getThemeConfigurationUiColorPalette:getThemeConfigurationUiColorPalette�
�O
accent" AColor (hexadecimal) that applies to selected states and buttons.
z
accentForeground" bColor (hexadecimal) that applies to any text or other elements that appear over the accent color.
B
danger" 4Color (hexadecimal) that applies to error messages.
y
dangerForeground" aColor (hexadecimal) that applies to any text or other elements that appear over the error color.
l
	dimension" [Color (hexadecimal) that applies to the names of fields that are identified as dimensions.
�
dimensionForeground" eColor (hexadecimal) that applies to any text or other elements that appear over the dimension color.
h
measure" YColor (hexadecimal) that applies to the names of fields that are identified as measures.
|
measureForeground" cColor (hexadecimal) that applies to any text or other elements that appear over the measure color.
a
primaryBackground" HColor (hexadecimal) that applies to visuals and other high emphasis UI.
�
primaryForeground" �Color (hexadecimal) of text and other foreground elements that appear over the primary background regions, such as grid lines, borders, table banding, icons, and so on.
h
secondaryBackground" MColor (hexadecimal) that applies to the sheet background and sheet controls.
�
secondaryForeground" {Color (hexadecimal) that applies to any sheet title, sheet control text, or UI that appears over the secondary background.
{
success" lColor (hexadecimal) that applies to success messages, for example the check mark for a successful download.
|
successForeground" cColor (hexadecimal) that applies to any text or other elements that appear over the success color.
W
warning" HColor (hexadecimal) that applies to warning and informational messages.
|
warningForeground" cColor (hexadecimal) that applies to any text or other elements that appear over the warning color.
:�
V

quicksightgetThemePermission4aws:quicksight/getThemePermission:getThemePermission�
�H
actions*" 7List of IAM actions to grant or revoke permissions on.
�
	principal" �ARN of the principal. See the [ResourcePermission documentation](https://docs.aws.amazon.com/quicksight/latest/APIReference/API_ResourcePermission.html) for the applicable ARN values.
:�
T
ramgetResourceShareFilter5aws:ram/getResourceShareFilter:getResourceShareFilterZ
X.
name" "Name of the tag key to filter on.
&
values*" Value of the tag key.
:�
S
rbinRuleLockConfiguration4aws:rbin/RuleLockConfiguration:RuleLockConfiguration�
��
unlockDelayx:v
t
rbin RuleLockConfigurationUnlockDelayJaws:rbin/RuleLockConfigurationUnlockDelay:RuleLockConfigurationUnlockDelayMInformation about the retention rule unlock delay. See `unlock_delay` below.
:�
t
rbin RuleLockConfigurationUnlockDelayJaws:rbin/RuleLockConfigurationUnlockDelay:RuleLockConfigurationUnlockDelay�
��
unlockDelayUnit" pThe unit of time in which to measure the unlock delay. Currently, the unlock delay can be measure only in days.
e
unlockDelayValue MThe unlock delay period, measured in the unit specified for UnlockDelayUnit.
:�
A
rbinRuleResourceTag(aws:rbin/RuleResourceTag:RuleResourceTagw
uH
resourceTagKey" 2The tag key.

The following argument is optional:
)
resourceTagValueB" The tag value.
:�
M
rbinRuleRetentionPeriod0aws:rbin/RuleRetentionPeriod:RuleRetentionPeriod�
�z
retentionPeriodUnit" _The unit of time in which the retention period is measured. Currently, only DAYS is supported.
�
retentionPeriodValue �The period value for which the retention rule is to retain resources. The period is measured using the unit specified for RetentionPeriodUnit.
