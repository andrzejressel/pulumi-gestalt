
awsAWS"6.66.2*�@
R
macieCustomDataIdentifier3aws:macie/customDataIdentifier:CustomDataIdentifier�#Provides a resource to manage an [AWS Macie Custom Data Identifier](https://docs.aws.amazon.com/macie/latest/APIReference/custom-data-identifiers-id.html).

## Example Usage

<!--Start PulumiCodeChooser -->
```typescript
import * as pulumi from "@pulumi/pulumi";
import * as aws from "@pulumi/aws";

const example = new aws.macie2.Account("example", {});
const exampleCustomDataIdentifier = new aws.macie.CustomDataIdentifier("example", {
    name: "NAME OF CUSTOM DATA IDENTIFIER",
    regex: "[0-9]{3}-[0-9]{2}-[0-9]{4}",
    description: "DESCRIPTION",
    maximumMatchDistance: 10,
    keywords: ["keyword"],
    ignoreWords: ["ignore"],
}, {
    dependsOn: [test],
});
```
```python
import pulumi
import pulumi_aws as aws

example = aws.macie2.Account("example")
example_custom_data_identifier = aws.macie.CustomDataIdentifier("example",
    name="NAME OF CUSTOM DATA IDENTIFIER",
    regex="[0-9]{3}-[0-9]{2}-[0-9]{4}",
    description="DESCRIPTION",
    maximum_match_distance=10,
    keywords=["keyword"],
    ignore_words=["ignore"],
    opts = pulumi.ResourceOptions(depends_on=[test]))
```
```csharp
using System.Collections.Generic;
using System.Linq;
using Pulumi;
using Aws = Pulumi.Aws;

return await Deployment.RunAsync(() => 
{
    var example = new Aws.Macie2.Account("example");

    var exampleCustomDataIdentifier = new Aws.Macie.CustomDataIdentifier("example", new()
    {
        Name = "NAME OF CUSTOM DATA IDENTIFIER",
        Regex = "[0-9]{3}-[0-9]{2}-[0-9]{4}",
        Description = "DESCRIPTION",
        MaximumMatchDistance = 10,
        Keywords = new[]
        {
            "keyword",
        },
        IgnoreWords = new[]
        {
            "ignore",
        },
    }, new CustomResourceOptions
    {
        DependsOn =
        {
            test,
        },
    });

});
```
```go
package main

import (
	"github.com/pulumi/pulumi-aws/sdk/v6/go/aws/macie"
	"github.com/pulumi/pulumi-aws/sdk/v6/go/aws/macie2"
	"github.com/pulumi/pulumi/sdk/v3/go/pulumi"
)

func main() {
	pulumi.Run(func(ctx *pulumi.Context) error {
		_, err := macie2.NewAccount(ctx, "example", nil)
		if err != nil {
			return err
		}
		_, err = macie.NewCustomDataIdentifier(ctx, "example", &macie.CustomDataIdentifierArgs{
			Name:                 pulumi.String("NAME OF CUSTOM DATA IDENTIFIER"),
			Regex:                pulumi.String("[0-9]{3}-[0-9]{2}-[0-9]{4}"),
			Description:          pulumi.String("DESCRIPTION"),
			MaximumMatchDistance: pulumi.Int(10),
			Keywords: pulumi.StringArray{
				pulumi.String("keyword"),
			},
			IgnoreWords: pulumi.StringArray{
				pulumi.String("ignore"),
			},
		}, pulumi.DependsOn([]pulumi.Resource{
			test,
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
import com.pulumi.aws.macie2.Account;
import com.pulumi.aws.macie.CustomDataIdentifier;
import com.pulumi.aws.macie.CustomDataIdentifierArgs;
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
        var example = new Account("example");

        var exampleCustomDataIdentifier = new CustomDataIdentifier("exampleCustomDataIdentifier", CustomDataIdentifierArgs.builder()
            .name("NAME OF CUSTOM DATA IDENTIFIER")
            .regex("[0-9]{3}-[0-9]{2}-[0-9]{4}")
            .description("DESCRIPTION")
            .maximumMatchDistance(10)
            .keywords("keyword")
            .ignoreWords("ignore")
            .build(), CustomResourceOptions.builder()
                .dependsOn(test)
                .build());

    }
}
```
```yaml
resources:
  example:
    type: aws:macie2:Account
  exampleCustomDataIdentifier:
    type: aws:macie:CustomDataIdentifier
    name: example
    properties:
      name: NAME OF CUSTOM DATA IDENTIFIER
      regex: '[0-9]{3}-[0-9]{2}-[0-9]{4}'
      description: DESCRIPTION
      maximumMatchDistance: 10
      keywords:
        - keyword
      ignoreWords:
        - ignore
    options:
      dependsOn:
        - ${test}
```
<!--End PulumiCodeChooser -->

## Import

Using `pulumi import`, import `aws_macie2_custom_data_identifier` using the id. For example:

```sh
$ pulumi import aws:macie/customDataIdentifier:CustomDataIdentifier example abcd1
```
�
descriptionB" kA custom description of the custom data identifier. The description can contain as many as 512 characters.
�
ignoreWordsB*" �An array that lists specific character sequences (ignore words) to exclude from the results. If the text matched by the regular expression is the same as any string in this array, Amazon Macie ignores it. The array can contain as many as 10 ignore words. Each ignore word can contain 4 - 90 characters. Ignore words are case sensitive.
�
keywordsB*" �An array that lists specific character sequences (keywords), one of which must be within proximity (`maximum_match_distance`) of the regular expression to match. The array can contain as many as 50 keywords. Each keyword can contain 3 - 90 characters. Keywords aren't case sensitive.
�
maximumMatchDistanceB �The maximum number of characters that can exist between text that matches the regex pattern and the character sequences specified by the keywords array. Macie includes or excludes a result based on the proximity of a keyword to text that matches the regex pattern. The distance can be 1 - 300 characters. The default value is 50.
�
nameB" �A custom name for the custom data identifier. The name can contain as many as 128 characters. If omitted, the provider will assign a random, unique name. Conflicts with `name_prefix`.
f

namePrefixB" RCreates a unique name beginning with the specified prefix. Conflicts with `name`.
�
regexB" xThe regular expression (regex) that defines the pattern to match. The expression can contain as many as 512 characters.
o
tagsB2" _A map of key-value pairs that specifies the tags to associate with the custom data identifier.
"I
arn" >The Amazon Resource Name (ARN) of the custom data identifier.
"t
	createdAt" cThe date and time, in UTC and extended RFC 3339 format, when the Amazon Macie account was created.
"�
descriptionB" kA custom description of the custom data identifier. The description can contain as many as 512 characters.
"�
ignoreWordsB*" �An array that lists specific character sequences (ignore words) to exclude from the results. If the text matched by the regular expression is the same as any string in this array, Amazon Macie ignores it. The array can contain as many as 10 ignore words. Each ignore word can contain 4 - 90 characters. Ignore words are case sensitive.
"�
keywordsB*" �An array that lists specific character sequences (keywords), one of which must be within proximity (`maximum_match_distance`) of the regular expression to match. The array can contain as many as 50 keywords. Each keyword can contain 3 - 90 characters. Keywords aren't case sensitive.
"�
maximumMatchDistance �The maximum number of characters that can exist between text that matches the regex pattern and the character sequences specified by the keywords array. Macie includes or excludes a result based on the proximity of a keyword to text that matches the regex pattern. The distance can be 1 - 300 characters. The default value is 50.
"�
name" �A custom name for the custom data identifier. The name can contain as many as 128 characters. If omitted, the provider will assign a random, unique name. Conflicts with `name_prefix`.
"d

namePrefix" RCreates a unique name beginning with the specified prefix. Conflicts with `name`.
"�
regexB" xThe regular expression (regex) that defines the pattern to match. The expression can contain as many as 512 characters.
"o
tagsB2" _A map of key-value pairs that specifies the tags to associate with the custom data identifier.
"
tagsAll2" *�;
@
macieFindingsFilter'aws:macie/findingsFilter:FindingsFilter�'Provides a resource to manage an [Amazon Macie Findings Filter](https://docs.aws.amazon.com/macie/latest/APIReference/findingsfilters-id.html).

## Example Usage

<!--Start PulumiCodeChooser -->
```typescript
import * as pulumi from "@pulumi/pulumi";
import * as aws from "@pulumi/aws";

const example = new aws.macie2.Account("example", {});
const test = new aws.macie.FindingsFilter("test", {
    name: "NAME OF THE FINDINGS FILTER",
    description: "DESCRIPTION",
    position: 1,
    action: "ARCHIVE",
    findingCriteria: {
        criterions: [{
            field: "region",
            eqs: [current.name],
        }],
    },
}, {
    dependsOn: [testAwsMacie2Account],
});
```
```python
import pulumi
import pulumi_aws as aws

example = aws.macie2.Account("example")
test = aws.macie.FindingsFilter("test",
    name="NAME OF THE FINDINGS FILTER",
    description="DESCRIPTION",
    position=1,
    action="ARCHIVE",
    finding_criteria={
        "criterions": [{
            "field": "region",
            "eqs": [current["name"]],
        }],
    },
    opts = pulumi.ResourceOptions(depends_on=[test_aws_macie2_account]))
```
```csharp
using System.Collections.Generic;
using System.Linq;
using Pulumi;
using Aws = Pulumi.Aws;

return await Deployment.RunAsync(() => 
{
    var example = new Aws.Macie2.Account("example");

    var test = new Aws.Macie.FindingsFilter("test", new()
    {
        Name = "NAME OF THE FINDINGS FILTER",
        Description = "DESCRIPTION",
        Position = 1,
        Action = "ARCHIVE",
        FindingCriteria = new Aws.Macie.Inputs.FindingsFilterFindingCriteriaArgs
        {
            Criterions = new[]
            {
                new Aws.Macie.Inputs.FindingsFilterFindingCriteriaCriterionArgs
                {
                    Field = "region",
                    Eqs = new[]
                    {
                        current.Name,
                    },
                },
            },
        },
    }, new CustomResourceOptions
    {
        DependsOn =
        {
            testAwsMacie2Account,
        },
    });

});
```
```go
package main

import (
	"github.com/pulumi/pulumi-aws/sdk/v6/go/aws/macie"
	"github.com/pulumi/pulumi-aws/sdk/v6/go/aws/macie2"
	"github.com/pulumi/pulumi/sdk/v3/go/pulumi"
)

func main() {
	pulumi.Run(func(ctx *pulumi.Context) error {
		_, err := macie2.NewAccount(ctx, "example", nil)
		if err != nil {
			return err
		}
		_, err = macie.NewFindingsFilter(ctx, "test", &macie.FindingsFilterArgs{
			Name:        pulumi.String("NAME OF THE FINDINGS FILTER"),
			Description: pulumi.String("DESCRIPTION"),
			Position:    pulumi.Int(1),
			Action:      pulumi.String("ARCHIVE"),
			FindingCriteria: &macie.FindingsFilterFindingCriteriaArgs{
				Criterions: macie.FindingsFilterFindingCriteriaCriterionArray{
					&macie.FindingsFilterFindingCriteriaCriterionArgs{
						Field: pulumi.String("region"),
						Eqs: pulumi.StringArray{
							current.Name,
						},
					},
				},
			},
		}, pulumi.DependsOn([]pulumi.Resource{
			testAwsMacie2Account,
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
import com.pulumi.aws.macie2.Account;
import com.pulumi.aws.macie.FindingsFilter;
import com.pulumi.aws.macie.FindingsFilterArgs;
import com.pulumi.aws.macie.inputs.FindingsFilterFindingCriteriaArgs;
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
        var example = new Account("example");

        var test = new FindingsFilter("test", FindingsFilterArgs.builder()
            .name("NAME OF THE FINDINGS FILTER")
            .description("DESCRIPTION")
            .position(1)
            .action("ARCHIVE")
            .findingCriteria(FindingsFilterFindingCriteriaArgs.builder()
                .criterions(FindingsFilterFindingCriteriaCriterionArgs.builder()
                    .field("region")
                    .eqs(current.name())
                    .build())
                .build())
            .build(), CustomResourceOptions.builder()
                .dependsOn(testAwsMacie2Account)
                .build());

    }
}
```
```yaml
resources:
  example:
    type: aws:macie2:Account
  test:
    type: aws:macie:FindingsFilter
    properties:
      name: NAME OF THE FINDINGS FILTER
      description: DESCRIPTION
      position: 1
      action: ARCHIVE
      findingCriteria:
        criterions:
          - field: region
            eqs:
              - ${current.name}
    options:
      dependsOn:
        - ${testAwsMacie2Account}
```
<!--End PulumiCodeChooser -->

## Import

Using `pulumi import`, import `aws_macie2_findings_filter` using the id. For example:

```sh
$ pulumi import aws:macie/findingsFilter:FindingsFilter example abcd1
```
�
action" �The action to perform on findings that meet the filter criteria (`finding_criteria`). Valid values are: `ARCHIVE`, suppress (automatically archive) the findings; and, `NOOP`, don't perform any action on the findings.
p
descriptionB" [A custom description of the filter. The description can contain as many as 512 characters.
�
findingCriteriaq:o
m
macieFindingsFilterFindingCriteriaEaws:macie/FindingsFilterFindingCriteria:FindingsFilterFindingCriteria(The criteria to use to filter findings.
�
nameB" �A custom name for the filter. The name must contain at least 3 characters and can contain as many as 64 characters. If omitted, the provider will assign a random, unique name. Conflicts with `name_prefix`.
f

namePrefixB" RCreates a unique name beginning with the specified prefix. Conflicts with `name`.
�
positionB �The position of the filter in the list of saved filters on the Amazon Macie console. This value also determines the order in which the filter is applied to findings, relative to other filters that are also applied to the findings.
_
tagsB2" OA map of key-value pairs that specifies the tags to associate with the filter.
"�
action" �The action to perform on findings that meet the filter criteria (`finding_criteria`). Valid values are: `ARCHIVE`, suppress (automatically archive) the findings; and, `NOOP`, don't perform any action on the findings.
"B
arn" 7The Amazon Resource Name (ARN) of the Findings Filter.
"p
descriptionB" [A custom description of the filter. The description can contain as many as 512 characters.
"�
findingCriteriaq:o
m
macieFindingsFilterFindingCriteriaEaws:macie/FindingsFilterFindingCriteria:FindingsFilterFindingCriteria(The criteria to use to filter findings.
"�
name" �A custom name for the filter. The name must contain at least 3 characters and can contain as many as 64 characters. If omitted, the provider will assign a random, unique name. Conflicts with `name_prefix`.
"d

namePrefix" RCreates a unique name beginning with the specified prefix. Conflicts with `name`.
"�
position �The position of the filter in the list of saved filters on the Amazon Macie console. This value also determines the order in which the filter is applied to findings, relative to other filters that are also applied to the findings.
"_
tagsB2" OA map of key-value pairs that specifies the tags to associate with the filter.
"
tagsAll2" *�
-
macie2Accountaws:macie2/account:Account�Provides a resource to manage an [AWS Macie Account](https://docs.aws.amazon.com/macie/latest/APIReference/macie.html).

## Example Usage

<!--Start PulumiCodeChooser -->
```typescript
import * as pulumi from "@pulumi/pulumi";
import * as aws from "@pulumi/aws";

const test = new aws.macie2.Account("test", {
    findingPublishingFrequency: "FIFTEEN_MINUTES",
    status: "ENABLED",
});
```
```python
import pulumi
import pulumi_aws as aws

test = aws.macie2.Account("test",
    finding_publishing_frequency="FIFTEEN_MINUTES",
    status="ENABLED")
```
```csharp
using System.Collections.Generic;
using System.Linq;
using Pulumi;
using Aws = Pulumi.Aws;

return await Deployment.RunAsync(() => 
{
    var test = new Aws.Macie2.Account("test", new()
    {
        FindingPublishingFrequency = "FIFTEEN_MINUTES",
        Status = "ENABLED",
    });

});
```
```go
package main

import (
	"github.com/pulumi/pulumi-aws/sdk/v6/go/aws/macie2"
	"github.com/pulumi/pulumi/sdk/v3/go/pulumi"
)

func main() {
	pulumi.Run(func(ctx *pulumi.Context) error {
		_, err := macie2.NewAccount(ctx, "test", &macie2.AccountArgs{
			FindingPublishingFrequency: pulumi.String("FIFTEEN_MINUTES"),
			Status:                     pulumi.String("ENABLED"),
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
import com.pulumi.aws.macie2.Account;
import com.pulumi.aws.macie2.AccountArgs;
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
        var test = new Account("test", AccountArgs.builder()
            .findingPublishingFrequency("FIFTEEN_MINUTES")
            .status("ENABLED")
            .build());

    }
}
```
```yaml
resources:
  test:
    type: aws:macie2:Account
    properties:
      findingPublishingFrequency: FIFTEEN_MINUTES
      status: ENABLED
```
<!--End PulumiCodeChooser -->

## Import

Using `pulumi import`, import `aws_macie2_account` using the id. For example:

```sh
$ pulumi import aws:macie2/account:Account example abcd1
```
�
findingPublishingFrequencyB" �Specifies how often to publish updates to policy findings for the account. This includes publishing updates to AWS Security Hub and Amazon EventBridge (formerly called Amazon CloudWatch Events). Valid values are `FIFTEEN_MINUTES`, `ONE_HOUR` or `SIX_HOURS`.
�
statusB" �Specifies the status for the account. To enable Amazon Macie and start all Macie activities for the account, set this value to `ENABLED`. Valid values are `ENABLED` or `PAUSED`.
"t
	createdAt" cThe date and time, in UTC and extended RFC 3339 format, when the Amazon Macie account was created.
"�
findingPublishingFrequency" �Specifies how often to publish updates to policy findings for the account. This includes publishing updates to AWS Security Hub and Amazon EventBridge (formerly called Amazon CloudWatch Events). Valid values are `FIFTEEN_MINUTES`, `ONE_HOUR` or `SIX_HOURS`.
"�
serviceRole" �The Amazon Resource Name (ARN) of the service-linked role that allows Macie to monitor and analyze data in AWS resources for the account.
"�
status" �Specifies the status for the account. To enable Amazon Macie and start all Macie activities for the account, set this value to `ENABLED`. Valid values are `ENABLED` or `PAUSED`.
"�
	updatedAt" vThe date and time, in UTC and extended RFC 3339 format, of the most recent change to the status of the Macie account.
*�(
{
macie2!ClassificationExportConfigurationNaws:macie2/classificationExportConfiguration:ClassificationExportConfiguration�#Provides a resource to manage an [Amazon Macie Classification Export Configuration](https://docs.aws.amazon.com/macie/latest/APIReference/classification-export-configuration.html).

## Example Usage

<!--Start PulumiCodeChooser -->
```typescript
import * as pulumi from "@pulumi/pulumi";
import * as aws from "@pulumi/aws";

const example = new aws.macie2.Account("example", {});
const exampleClassificationExportConfiguration = new aws.macie2.ClassificationExportConfiguration("example", {s3Destination: {
    bucketName: exampleAwsS3Bucket.bucket,
    keyPrefix: "exampleprefix/",
    kmsKeyArn: exampleAwsKmsKey.arn,
}}, {
    dependsOn: [example],
});
```
```python
import pulumi
import pulumi_aws as aws

example = aws.macie2.Account("example")
example_classification_export_configuration = aws.macie2.ClassificationExportConfiguration("example", s3_destination={
    "bucket_name": example_aws_s3_bucket["bucket"],
    "key_prefix": "exampleprefix/",
    "kms_key_arn": example_aws_kms_key["arn"],
},
opts = pulumi.ResourceOptions(depends_on=[example]))
```
```csharp
using System.Collections.Generic;
using System.Linq;
using Pulumi;
using Aws = Pulumi.Aws;

return await Deployment.RunAsync(() => 
{
    var example = new Aws.Macie2.Account("example");

    var exampleClassificationExportConfiguration = new Aws.Macie2.ClassificationExportConfiguration("example", new()
    {
        S3Destination = new Aws.Macie2.Inputs.ClassificationExportConfigurationS3DestinationArgs
        {
            BucketName = exampleAwsS3Bucket.Bucket,
            KeyPrefix = "exampleprefix/",
            KmsKeyArn = exampleAwsKmsKey.Arn,
        },
    }, new CustomResourceOptions
    {
        DependsOn =
        {
            example,
        },
    });

});
```
```go
package main

import (
	"github.com/pulumi/pulumi-aws/sdk/v6/go/aws/macie2"
	"github.com/pulumi/pulumi/sdk/v3/go/pulumi"
)

func main() {
	pulumi.Run(func(ctx *pulumi.Context) error {
		example, err := macie2.NewAccount(ctx, "example", nil)
		if err != nil {
			return err
		}
		_, err = macie2.NewClassificationExportConfiguration(ctx, "example", &macie2.ClassificationExportConfigurationArgs{
			S3Destination: &macie2.ClassificationExportConfigurationS3DestinationArgs{
				BucketName: pulumi.Any(exampleAwsS3Bucket.Bucket),
				KeyPrefix:  pulumi.String("exampleprefix/"),
				KmsKeyArn:  pulumi.Any(exampleAwsKmsKey.Arn),
			},
		}, pulumi.DependsOn([]pulumi.Resource{
			example,
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
import com.pulumi.aws.macie2.Account;
import com.pulumi.aws.macie2.ClassificationExportConfiguration;
import com.pulumi.aws.macie2.ClassificationExportConfigurationArgs;
import com.pulumi.aws.macie2.inputs.ClassificationExportConfigurationS3DestinationArgs;
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
        var example = new Account("example");

        var exampleClassificationExportConfiguration = new ClassificationExportConfiguration("exampleClassificationExportConfiguration", ClassificationExportConfigurationArgs.builder()
            .s3Destination(ClassificationExportConfigurationS3DestinationArgs.builder()
                .bucketName(exampleAwsS3Bucket.bucket())
                .keyPrefix("exampleprefix/")
                .kmsKeyArn(exampleAwsKmsKey.arn())
                .build())
            .build(), CustomResourceOptions.builder()
                .dependsOn(example)
                .build());

    }
}
```
```yaml
resources:
  example:
    type: aws:macie2:Account
  exampleClassificationExportConfiguration:
    type: aws:macie2:ClassificationExportConfiguration
    name: example
    properties:
      s3Destination:
        bucketName: ${exampleAwsS3Bucket.bucket}
        keyPrefix: exampleprefix/
        kmsKeyArn: ${exampleAwsKmsKey.arn}
    options:
      dependsOn:
        - ${example}
```
<!--End PulumiCodeChooser -->

## Import

Using `pulumi import`, import `aws_macie2_classification_export_configuration` using the account ID and region. For example:

```sh
$ pulumi import aws:macie2/classificationExportConfiguration:ClassificationExportConfiguration example 123456789012:us-west-2
```
�
s3Destination�B�:�
�
macie2.ClassificationExportConfigurationS3Destinationhaws:macie2/ClassificationExportConfigurationS3Destination:ClassificationExportConfigurationS3Destination8Configuration block for a S3 Destination. Defined below
"�
s3Destination�B�:�
�
macie2.ClassificationExportConfigurationS3Destinationhaws:macie2/ClassificationExportConfigurationS3Destination:ClassificationExportConfigurationS3Destination8Configuration block for a S3 Destination. Defined below
*�P
K
macie2ClassificationJob.aws:macie2/classificationJob:ClassificationJob�&Provides a resource to manage an [AWS Macie Classification Job](https://docs.aws.amazon.com/macie/latest/APIReference/jobs.html).

## Example Usage

<!--Start PulumiCodeChooser -->
```typescript
import * as pulumi from "@pulumi/pulumi";
import * as aws from "@pulumi/aws";

const test = new aws.macie2.Account("test", {});
const testClassificationJob = new aws.macie2.ClassificationJob("test", {
    jobType: "ONE_TIME",
    name: "NAME OF THE CLASSIFICATION JOB",
    s3JobDefinition: {
        bucketDefinitions: [{
            accountId: "ACCOUNT ID",
            buckets: ["S3 BUCKET NAME"],
        }],
    },
}, {
    dependsOn: [test],
});
```
```python
import pulumi
import pulumi_aws as aws

test = aws.macie2.Account("test")
test_classification_job = aws.macie2.ClassificationJob("test",
    job_type="ONE_TIME",
    name="NAME OF THE CLASSIFICATION JOB",
    s3_job_definition={
        "bucket_definitions": [{
            "account_id": "ACCOUNT ID",
            "buckets": ["S3 BUCKET NAME"],
        }],
    },
    opts = pulumi.ResourceOptions(depends_on=[test]))
```
```csharp
using System.Collections.Generic;
using System.Linq;
using Pulumi;
using Aws = Pulumi.Aws;

return await Deployment.RunAsync(() => 
{
    var test = new Aws.Macie2.Account("test");

    var testClassificationJob = new Aws.Macie2.ClassificationJob("test", new()
    {
        JobType = "ONE_TIME",
        Name = "NAME OF THE CLASSIFICATION JOB",
        S3JobDefinition = new Aws.Macie2.Inputs.ClassificationJobS3JobDefinitionArgs
        {
            BucketDefinitions = new[]
            {
                new Aws.Macie2.Inputs.ClassificationJobS3JobDefinitionBucketDefinitionArgs
                {
                    AccountId = "ACCOUNT ID",
                    Buckets = new[]
                    {
                        "S3 BUCKET NAME",
                    },
                },
            },
        },
    }, new CustomResourceOptions
    {
        DependsOn =
        {
            test,
        },
    });

});
```
```go
package main

import (
	"github.com/pulumi/pulumi-aws/sdk/v6/go/aws/macie2"
	"github.com/pulumi/pulumi/sdk/v3/go/pulumi"
)

func main() {
	pulumi.Run(func(ctx *pulumi.Context) error {
		test, err := macie2.NewAccount(ctx, "test", nil)
		if err != nil {
			return err
		}
		_, err = macie2.NewClassificationJob(ctx, "test", &macie2.ClassificationJobArgs{
			JobType: pulumi.String("ONE_TIME"),
			Name:    pulumi.String("NAME OF THE CLASSIFICATION JOB"),
			S3JobDefinition: &macie2.ClassificationJobS3JobDefinitionArgs{
				BucketDefinitions: macie2.ClassificationJobS3JobDefinitionBucketDefinitionArray{
					&macie2.ClassificationJobS3JobDefinitionBucketDefinitionArgs{
						AccountId: pulumi.String("ACCOUNT ID"),
						Buckets: pulumi.StringArray{
							pulumi.String("S3 BUCKET NAME"),
						},
					},
				},
			},
		}, pulumi.DependsOn([]pulumi.Resource{
			test,
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
import com.pulumi.aws.macie2.Account;
import com.pulumi.aws.macie2.ClassificationJob;
import com.pulumi.aws.macie2.ClassificationJobArgs;
import com.pulumi.aws.macie2.inputs.ClassificationJobS3JobDefinitionArgs;
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
        var test = new Account("test");

        var testClassificationJob = new ClassificationJob("testClassificationJob", ClassificationJobArgs.builder()
            .jobType("ONE_TIME")
            .name("NAME OF THE CLASSIFICATION JOB")
            .s3JobDefinition(ClassificationJobS3JobDefinitionArgs.builder()
                .bucketDefinitions(ClassificationJobS3JobDefinitionBucketDefinitionArgs.builder()
                    .accountId("ACCOUNT ID")
                    .buckets("S3 BUCKET NAME")
                    .build())
                .build())
            .build(), CustomResourceOptions.builder()
                .dependsOn(test)
                .build());

    }
}
```
```yaml
resources:
  test:
    type: aws:macie2:Account
  testClassificationJob:
    type: aws:macie2:ClassificationJob
    name: test
    properties:
      jobType: ONE_TIME
      name: NAME OF THE CLASSIFICATION JOB
      s3JobDefinition:
        bucketDefinitions:
          - accountId: ACCOUNT ID
            buckets:
              - S3 BUCKET NAME
    options:
      dependsOn:
        - ${test}
```
<!--End PulumiCodeChooser -->

## Import

Using `pulumi import`, import `aws_macie2_classification_job` using the id. For example:

```sh
$ pulumi import aws:macie2/classificationJob:ClassificationJob example abcd1
```
l
customDataIdentifierIdsB*" IThe custom data identifiers to use for data analysis and classification.
m
descriptionB" XA custom description of the job. The description can contain as many as 200 characters.
v

initialRunB
 bSpecifies whether to analyze all existing, eligible objects immediately after the job is created.
f
	jobStatusB" SThe status for the job. Valid values are: `CANCELLED`, `RUNNING` and `USER_PAUSED`
�
jobType" �The schedule for running the job. Valid values are: `ONE_TIME` - Run the job only once. If you specify this value, don't specify a value for the `schedule_frequency` property. `SCHEDULED` - Run the job on a daily, weekly, or monthly basis. If you specify this value, use the `schedule_frequency` property to define the recurrence pattern for the job.
�
nameB" �A custom name for the job. The name can contain as many as 500 characters. If omitted, the provider will assign a random, unique name. Conflicts with `name_prefix`.
f

namePrefixB" RCreates a unique name beginning with the specified prefix. Conflicts with `name`.
�
s3JobDefinition|:z
x
macie2 ClassificationJobS3JobDefinitionLaws:macie2/ClassificationJobS3JobDefinition:ClassificationJobS3JobDefinitiongThe S3 buckets that contain the objects to analyze, and the scope of that analysis. (documented below)
�
samplingPercentageB �The sampling depth, as a percentage, to apply when processing objects. This value determines the percentage of eligible objects that the job analyzes. If this value is less than 100, Amazon Macie selects the objects to analyze at random, up to the specified percentage, and analyzes all the data in those objects.
�
scheduleFrequency�B�:�
~
macie2"ClassificationJobScheduleFrequencyPaws:macie2/ClassificationJobScheduleFrequency:ClassificationJobScheduleFrequency�The recurrence pattern for running the job. To run the job only once, don't specify a value for this property and set the value for the `job_type` property to `ONE_TIME`. (documented below)
�
tagsB2" �A map of key-value pairs that specifies the tags to associate with the job. A job can have a maximum of 50 tags. Each tag consists of a tag key and an associated tag value. The maximum length of a tag key is 128 characters. The maximum length of a tag value is 256 characters.
"c
	createdAt" RThe date and time, in UTC and extended RFC 3339 format, when the job was created.
"j
customDataIdentifierIds*" IThe custom data identifiers to use for data analysis and classification.
"k
description" XA custom description of the job. The description can contain as many as 200 characters.
"v

initialRunB
 bSpecifies whether to analyze all existing, eligible objects immediately after the job is created.
"
jobArn" "
jobId" "d
	jobStatus" SThe status for the job. Valid values are: `CANCELLED`, `RUNNING` and `USER_PAUSED`
"�
jobType" �The schedule for running the job. Valid values are: `ONE_TIME` - Run the job only once. If you specify this value, don't specify a value for the `schedule_frequency` property. `SCHEDULED` - Run the job on a daily, weekly, or monthly basis. If you specify this value, use the `schedule_frequency` property to define the recurrence pattern for the job.
"�
name" �A custom name for the job. The name can contain as many as 500 characters. If omitted, the provider will assign a random, unique name. Conflicts with `name_prefix`.
"d

namePrefix" RCreates a unique name beginning with the specified prefix. Conflicts with `name`.
"�
s3JobDefinition|:z
x
macie2 ClassificationJobS3JobDefinitionLaws:macie2/ClassificationJobS3JobDefinition:ClassificationJobS3JobDefinitiongThe S3 buckets that contain the objects to analyze, and the scope of that analysis. (documented below)
"�
samplingPercentage �The sampling depth, as a percentage, to apply when processing objects. This value determines the percentage of eligible objects that the job analyzes. If this value is less than 100, Amazon Macie selects the objects to analyze at random, up to the specified percentage, and analyzes all the data in those objects.
"�
scheduleFrequency�:�
~
macie2"ClassificationJobScheduleFrequencyPaws:macie2/ClassificationJobScheduleFrequency:ClassificationJobScheduleFrequency�The recurrence pattern for running the job. To run the job only once, don't specify a value for this property and set the value for the `job_type` property to `ONE_TIME`. (documented below)
"�
tagsB2" �A map of key-value pairs that specifies the tags to associate with the job. A job can have a maximum of 50 tags. Each tag consists of a tag key and an associated tag value. The maximum length of a tag key is 128 characters. The maximum length of a tag value is 256 characters.
"
tagsAll2" "�
userPausedDetails�*:}
{
macie2!ClassificationJobUserPausedDetailNaws:macie2/ClassificationJobUserPausedDetail:ClassificationJobUserPausedDetail�If the current status of the job is `USER_PAUSED`, specifies when the job was paused and when the job or job run will expire and be canceled if it isn't resumed. This value is present only if the value for `job-status` is `USER_PAUSED`.
*�,
N
macie2InvitationAccepter0aws:macie2/invitationAccepter:InvitationAccepter�*Provides a resource to manage an [Amazon Macie Invitation Accepter](https://docs.aws.amazon.com/macie/latest/APIReference/invitations-accept.html).

## Example Usage

<!--Start PulumiCodeChooser -->
```typescript
import * as pulumi from "@pulumi/pulumi";
import * as aws from "@pulumi/aws";

const primary = new aws.macie2.Account("primary", {});
const member = new aws.macie2.Account("member", {});
const primaryMember = new aws.macie2.Member("primary", {
    accountId: "ACCOUNT ID",
    email: "EMAIL",
    invite: true,
    invitationMessage: "Message of the invite",
}, {
    dependsOn: [primary],
});
const memberInvitationAccepter = new aws.macie2.InvitationAccepter("member", {administratorAccountId: "ADMINISTRATOR ACCOUNT ID"}, {
    dependsOn: [primaryMember],
});
```
```python
import pulumi
import pulumi_aws as aws

primary = aws.macie2.Account("primary")
member = aws.macie2.Account("member")
primary_member = aws.macie2.Member("primary",
    account_id="ACCOUNT ID",
    email="EMAIL",
    invite=True,
    invitation_message="Message of the invite",
    opts = pulumi.ResourceOptions(depends_on=[primary]))
member_invitation_accepter = aws.macie2.InvitationAccepter("member", administrator_account_id="ADMINISTRATOR ACCOUNT ID",
opts = pulumi.ResourceOptions(depends_on=[primary_member]))
```
```csharp
using System.Collections.Generic;
using System.Linq;
using Pulumi;
using Aws = Pulumi.Aws;

return await Deployment.RunAsync(() => 
{
    var primary = new Aws.Macie2.Account("primary");

    var member = new Aws.Macie2.Account("member");

    var primaryMember = new Aws.Macie2.Member("primary", new()
    {
        AccountId = "ACCOUNT ID",
        Email = "EMAIL",
        Invite = true,
        InvitationMessage = "Message of the invite",
    }, new CustomResourceOptions
    {
        DependsOn =
        {
            primary,
        },
    });

    var memberInvitationAccepter = new Aws.Macie2.InvitationAccepter("member", new()
    {
        AdministratorAccountId = "ADMINISTRATOR ACCOUNT ID",
    }, new CustomResourceOptions
    {
        DependsOn =
        {
            primaryMember,
        },
    });

});
```
```go
package main

import (
	"github.com/pulumi/pulumi-aws/sdk/v6/go/aws/macie2"
	"github.com/pulumi/pulumi/sdk/v3/go/pulumi"
)

func main() {
	pulumi.Run(func(ctx *pulumi.Context) error {
		primary, err := macie2.NewAccount(ctx, "primary", nil)
		if err != nil {
			return err
		}
		_, err = macie2.NewAccount(ctx, "member", nil)
		if err != nil {
			return err
		}
		primaryMember, err := macie2.NewMember(ctx, "primary", &macie2.MemberArgs{
			AccountId:         pulumi.String("ACCOUNT ID"),
			Email:             pulumi.String("EMAIL"),
			Invite:            pulumi.Bool(true),
			InvitationMessage: pulumi.String("Message of the invite"),
		}, pulumi.DependsOn([]pulumi.Resource{
			primary,
		}))
		if err != nil {
			return err
		}
		_, err = macie2.NewInvitationAccepter(ctx, "member", &macie2.InvitationAccepterArgs{
			AdministratorAccountId: pulumi.String("ADMINISTRATOR ACCOUNT ID"),
		}, pulumi.DependsOn([]pulumi.Resource{
			primaryMember,
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
import com.pulumi.aws.macie2.Account;
import com.pulumi.aws.macie2.Member;
import com.pulumi.aws.macie2.MemberArgs;
import com.pulumi.aws.macie2.InvitationAccepter;
import com.pulumi.aws.macie2.InvitationAccepterArgs;
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
        var primary = new Account("primary");

        var member = new Account("member");

        var primaryMember = new Member("primaryMember", MemberArgs.builder()
            .accountId("ACCOUNT ID")
            .email("EMAIL")
            .invite(true)
            .invitationMessage("Message of the invite")
            .build(), CustomResourceOptions.builder()
                .dependsOn(primary)
                .build());

        var memberInvitationAccepter = new InvitationAccepter("memberInvitationAccepter", InvitationAccepterArgs.builder()
            .administratorAccountId("ADMINISTRATOR ACCOUNT ID")
            .build(), CustomResourceOptions.builder()
                .dependsOn(primaryMember)
                .build());

    }
}
```
```yaml
resources:
  primary:
    type: aws:macie2:Account
  member:
    type: aws:macie2:Account
  primaryMember:
    type: aws:macie2:Member
    name: primary
    properties:
      accountId: ACCOUNT ID
      email: EMAIL
      invite: true
      invitationMessage: Message of the invite
    options:
      dependsOn:
        - ${primary}
  memberInvitationAccepter:
    type: aws:macie2:InvitationAccepter
    name: member
    properties:
      administratorAccountId: ADMINISTRATOR ACCOUNT ID
    options:
      dependsOn:
        - ${primaryMember}
```
<!--End PulumiCodeChooser -->

## Import

Using `pulumi import`, import `aws_macie2_invitation_accepter` using the admin account ID. For example:

```sh
$ pulumi import aws:macie2/invitationAccepter:InvitationAccepter example 123456789012
```
[
administratorAccountId" =The AWS account ID for the account that sent the invitation.
"[
administratorAccountId" =The AWS account ID for the account that sent the invitation.
">
invitationId" *The unique identifier for the invitation.
*�4
*
macie2Memberaws:macie2/member:Member�Provides a resource to manage an [Amazon Macie Member](https://docs.aws.amazon.com/macie/latest/APIReference/members-id.html).

## Example Usage

<!--Start PulumiCodeChooser -->
```typescript
import * as pulumi from "@pulumi/pulumi";
import * as aws from "@pulumi/aws";

const example = new aws.macie2.Account("example", {});
const exampleMember = new aws.macie2.Member("example", {
    accountId: "AWS ACCOUNT ID",
    email: "EMAIL",
    invite: true,
    invitationMessage: "Message of the invitation",
    invitationDisableEmailNotification: true,
}, {
    dependsOn: [example],
});
```
```python
import pulumi
import pulumi_aws as aws

example = aws.macie2.Account("example")
example_member = aws.macie2.Member("example",
    account_id="AWS ACCOUNT ID",
    email="EMAIL",
    invite=True,
    invitation_message="Message of the invitation",
    invitation_disable_email_notification=True,
    opts = pulumi.ResourceOptions(depends_on=[example]))
```
```csharp
using System.Collections.Generic;
using System.Linq;
using Pulumi;
using Aws = Pulumi.Aws;

return await Deployment.RunAsync(() => 
{
    var example = new Aws.Macie2.Account("example");

    var exampleMember = new Aws.Macie2.Member("example", new()
    {
        AccountId = "AWS ACCOUNT ID",
        Email = "EMAIL",
        Invite = true,
        InvitationMessage = "Message of the invitation",
        InvitationDisableEmailNotification = true,
    }, new CustomResourceOptions
    {
        DependsOn =
        {
            example,
        },
    });

});
```
```go
package main

import (
	"github.com/pulumi/pulumi-aws/sdk/v6/go/aws/macie2"
	"github.com/pulumi/pulumi/sdk/v3/go/pulumi"
)

func main() {
	pulumi.Run(func(ctx *pulumi.Context) error {
		example, err := macie2.NewAccount(ctx, "example", nil)
		if err != nil {
			return err
		}
		_, err = macie2.NewMember(ctx, "example", &macie2.MemberArgs{
			AccountId:                          pulumi.String("AWS ACCOUNT ID"),
			Email:                              pulumi.String("EMAIL"),
			Invite:                             pulumi.Bool(true),
			InvitationMessage:                  pulumi.String("Message of the invitation"),
			InvitationDisableEmailNotification: pulumi.Bool(true),
		}, pulumi.DependsOn([]pulumi.Resource{
			example,
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
import com.pulumi.aws.macie2.Account;
import com.pulumi.aws.macie2.Member;
import com.pulumi.aws.macie2.MemberArgs;
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
        var example = new Account("example");

        var exampleMember = new Member("exampleMember", MemberArgs.builder()
            .accountId("AWS ACCOUNT ID")
            .email("EMAIL")
            .invite(true)
            .invitationMessage("Message of the invitation")
            .invitationDisableEmailNotification(true)
            .build(), CustomResourceOptions.builder()
                .dependsOn(example)
                .build());

    }
}
```
```yaml
resources:
  example:
    type: aws:macie2:Account
  exampleMember:
    type: aws:macie2:Member
    name: example
    properties:
      accountId: AWS ACCOUNT ID
      email: EMAIL
      invite: true
      invitationMessage: Message of the invitation
      invitationDisableEmailNotification: true
    options:
      dependsOn:
        - ${example}
```
<!--End PulumiCodeChooser -->

## Import

Using `pulumi import`, import `aws_macie2_member` using the account ID of the member account. For example:

```sh
$ pulumi import aws:macie2/member:Member example 123456789012
```
5
	accountId" $The AWS account ID for the account.
0
email" #The email address for the account.
�
"invitationDisableEmailNotificationB
 �Specifies whether to send an email notification to the root user of each account that the invitation will be sent to. This notification is in addition to an alert that the root user receives in AWS Personal Health Dashboard. To send an email notification to the root user of each account, set this value to `true`.
�
invitationMessageB" �A custom message to include in the invitation. Amazon Macie adds this message to the standard content that it sends for an invitation.
/
inviteB
 Send an invitation to a member
�
statusB" �Specifies the status for the account. To enable Amazon Macie and start all Macie activities for the account, set this value to `ENABLED`. Valid values are `ENABLED` or `PAUSED`.
p
tagsB2" `A map of key-value pairs that specifies the tags to associate with the account in Amazon Macie.
"5
	accountId" $The AWS account ID for the account.
"P
administratorAccountId" 2The AWS account ID for the administrator account.
":
arn" /The Amazon Resource Name (ARN) of the account.
"0
email" #The email address for the account.
"�
"invitationDisableEmailNotificationB
 �Specifies whether to send an email notification to the root user of each account that the invitation will be sent to. This notification is in addition to an alert that the root user receives in AWS Personal Health Dashboard. To send an email notification to the root user of each account, set this value to `true`.
"�
invitationMessageB" �A custom message to include in the invitation. Amazon Macie adds this message to the standard content that it sends for an invitation.
"-
invite
 Send an invitation to a member
"�
	invitedAt" �The date and time, in UTC and extended RFC 3339 format, when an Amazon Macie membership invitation was last sent to the account. This value is null if a Macie invitation hasn't been sent to the account.
"
masterAccountId" "t
relationshipStatus" ZThe current status of the relationship between the account and the administrator account.
"�
status" �Specifies the status for the account. To enable Amazon Macie and start all Macie activities for the account, set this value to `ENABLED`. Valid values are `ENABLED` or `PAUSED`.
"p
tagsB2" `A map of key-value pairs that specifies the tags to associate with the account in Amazon Macie.
"
tagsAll2" "�
	updatedAt" �The date and time, in UTC and extended RFC 3339 format, of the most recent change to the status of the relationship between the account and the administrator account.
*�
`
macie2OrganizationAdminAccount<aws:macie2/organizationAdminAccount:OrganizationAdminAccount�Provides a resource to manage an [Amazon Macie Organization Admin Account](https://docs.aws.amazon.com/macie/latest/APIReference/admin.html).

## Example Usage

<!--Start PulumiCodeChooser -->
```typescript
import * as pulumi from "@pulumi/pulumi";
import * as aws from "@pulumi/aws";

const example = new aws.macie2.Account("example", {});
const exampleOrganizationAdminAccount = new aws.macie2.OrganizationAdminAccount("example", {adminAccountId: "ID OF THE ADMIN ACCOUNT"}, {
    dependsOn: [example],
});
```
```python
import pulumi
import pulumi_aws as aws

example = aws.macie2.Account("example")
example_organization_admin_account = aws.macie2.OrganizationAdminAccount("example", admin_account_id="ID OF THE ADMIN ACCOUNT",
opts = pulumi.ResourceOptions(depends_on=[example]))
```
```csharp
using System.Collections.Generic;
using System.Linq;
using Pulumi;
using Aws = Pulumi.Aws;

return await Deployment.RunAsync(() => 
{
    var example = new Aws.Macie2.Account("example");

    var exampleOrganizationAdminAccount = new Aws.Macie2.OrganizationAdminAccount("example", new()
    {
        AdminAccountId = "ID OF THE ADMIN ACCOUNT",
    }, new CustomResourceOptions
    {
        DependsOn =
        {
            example,
        },
    });

});
```
```go
package main

import (
	"github.com/pulumi/pulumi-aws/sdk/v6/go/aws/macie2"
	"github.com/pulumi/pulumi/sdk/v3/go/pulumi"
)

func main() {
	pulumi.Run(func(ctx *pulumi.Context) error {
		example, err := macie2.NewAccount(ctx, "example", nil)
		if err != nil {
			return err
		}
		_, err = macie2.NewOrganizationAdminAccount(ctx, "example", &macie2.OrganizationAdminAccountArgs{
			AdminAccountId: pulumi.String("ID OF THE ADMIN ACCOUNT"),
		}, pulumi.DependsOn([]pulumi.Resource{
			example,
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
import com.pulumi.aws.macie2.Account;
import com.pulumi.aws.macie2.OrganizationAdminAccount;
import com.pulumi.aws.macie2.OrganizationAdminAccountArgs;
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
        var example = new Account("example");

        var exampleOrganizationAdminAccount = new OrganizationAdminAccount("exampleOrganizationAdminAccount", OrganizationAdminAccountArgs.builder()
            .adminAccountId("ID OF THE ADMIN ACCOUNT")
            .build(), CustomResourceOptions.builder()
                .dependsOn(example)
                .build());

    }
}
```
```yaml
resources:
  example:
    type: aws:macie2:Account
  exampleOrganizationAdminAccount:
    type: aws:macie2:OrganizationAdminAccount
    name: example
    properties:
      adminAccountId: ID OF THE ADMIN ACCOUNT
    options:
      dependsOn:
        - ${example}
```
<!--End PulumiCodeChooser -->

## Import

Using `pulumi import`, import `aws_macie2_organization_admin_account` using the id. For example:

```sh
$ pulumi import aws:macie2/organizationAdminAccount:OrganizationAdminAccount example abcd1
```
�
adminAccountId" zThe AWS account ID for the account to designate as the delegated Amazon Macie administrator account for the organization.
"�
adminAccountId" zThe AWS account ID for the account to designate as the delegated Amazon Macie administrator account for the organization.
*�
3
mediaconvertQueueaws:mediaconvert/queue:Queue�Provides an AWS Elemental MediaConvert Queue.

## Example Usage

<!--Start PulumiCodeChooser -->
```typescript
import * as pulumi from "@pulumi/pulumi";
import * as aws from "@pulumi/aws";

const test = new aws.mediaconvert.Queue("test", {name: "tf-test-queue"});
```
```python
import pulumi
import pulumi_aws as aws

test = aws.mediaconvert.Queue("test", name="tf-test-queue")
```
```csharp
using System.Collections.Generic;
using System.Linq;
using Pulumi;
using Aws = Pulumi.Aws;

return await Deployment.RunAsync(() => 
{
    var test = new Aws.MediaConvert.Queue("test", new()
    {
        Name = "tf-test-queue",
    });

});
```
```go
package main

import (
	"github.com/pulumi/pulumi-aws/sdk/v6/go/aws/mediaconvert"
	"github.com/pulumi/pulumi/sdk/v3/go/pulumi"
)

func main() {
	pulumi.Run(func(ctx *pulumi.Context) error {
		_, err := mediaconvert.NewQueue(ctx, "test", &mediaconvert.QueueArgs{
			Name: pulumi.String("tf-test-queue"),
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
import com.pulumi.aws.mediaconvert.Queue;
import com.pulumi.aws.mediaconvert.QueueArgs;
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
        var test = new Queue("test", QueueArgs.builder()
            .name("tf-test-queue")
            .build());

    }
}
```
```yaml
resources:
  test:
    type: aws:mediaconvert:Queue
    properties:
      name: tf-test-queue
```
<!--End PulumiCodeChooser -->

## Import

Using `pulumi import`, import Media Convert Queue using the queue name. For example:

```sh
$ pulumi import aws:mediaconvert/queue:Queue test tf-test-queue
```
0
descriptionB" A description of the queue
7
nameB" )A unique identifier describing the queue
�
pricingPlanB" �Specifies whether the pricing plan for the queue is on-demand or reserved. Valid values are `ON_DEMAND` or `RESERVED`. Default to `ON_DEMAND`.
�
reservationPlanSettings~B|:z
x
mediaconvertQueueReservationPlanSettingsJaws:mediaconvert/QueueReservationPlanSettings:QueueReservationPlanSettings9A detail pricing plan of the  reserved queue. See below.
e
statusB" UA status of the queue. Valid values are `ACTIVE` or `RESERVED`. Default to `PAUSED`.
�
tagsB2" �A map of tags to assign to the resource. .If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
" 
arn" The Arn of the queue
"0
descriptionB" A description of the queue
"5
name" )A unique identifier describing the queue
"�
pricingPlanB" �Specifies whether the pricing plan for the queue is on-demand or reserved. Valid values are `ON_DEMAND` or `RESERVED`. Default to `ON_DEMAND`.
"�
reservationPlanSettings|:z
x
mediaconvertQueueReservationPlanSettingsJaws:mediaconvert/QueueReservationPlanSettings:QueueReservationPlanSettings9A detail pricing plan of the  reserved queue. See below.
"e
statusB" UA status of the queue. Valid values are `ACTIVE` or `RESERVED`. Default to `PAUSED`.
"�
tagsB2" �A map of tags to assign to the resource. .If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
"�
tagsAll2" xA map of tags assigned to the resource, including those inherited from the provider `default_tags` configuration block.
*��
3
	medialiveChannelaws:medialive/channel:Channel��Resource for managing an AWS MediaLive Channel.

## Example Usage

### Basic Usage

<!--Start PulumiCodeChooser -->
```typescript
import * as pulumi from "@pulumi/pulumi";
import * as aws from "@pulumi/aws";

const example = new aws.medialive.Channel("example", {
    name: "example-channel",
    channelClass: "STANDARD",
    roleArn: exampleAwsIamRole.arn,
    inputSpecification: {
        codec: "AVC",
        inputResolution: "HD",
        maximumBitrate: "MAX_20_MBPS",
    },
    inputAttachments: [{
        inputAttachmentName: "example-input",
        inputId: exampleAwsMedialiveInput.id,
    }],
    destinations: [{
        id: "destination",
        settings: [
            {
                url: `s3://${main.id}/test1`,
            },
            {
                url: `s3://${main2.id}/test2`,
            },
        ],
    }],
    encoderSettings: {
        timecodeConfig: {
            source: "EMBEDDED",
        },
        audioDescriptions: [{
            audioSelectorName: "example audio selector",
            name: "audio-selector",
        }],
        videoDescriptions: [{
            name: "example-video",
        }],
        outputGroups: [{
            outputGroupSettings: {
                archiveGroupSettings: [{
                    destination: {
                        destinationRefId: "destination",
                    },
                }],
            },
            outputs: [{
                outputName: "example-name",
                videoDescriptionName: "example-video",
                audioDescriptionNames: ["audio-selector"],
                outputSettings: {
                    archiveOutputSettings: {
                        nameModifier: "_1",
                        extension: "m2ts",
                        containerSettings: {
                            m2tsSettings: {
                                audioBufferModel: "ATSC",
                                bufferModel: "MULTIPLEX",
                                rateMode: "CBR",
                            },
                        },
                    },
                },
            }],
        }],
    },
});
```
```python
import pulumi
import pulumi_aws as aws

example = aws.medialive.Channel("example",
    name="example-channel",
    channel_class="STANDARD",
    role_arn=example_aws_iam_role["arn"],
    input_specification={
        "codec": "AVC",
        "input_resolution": "HD",
        "maximum_bitrate": "MAX_20_MBPS",
    },
    input_attachments=[{
        "input_attachment_name": "example-input",
        "input_id": example_aws_medialive_input["id"],
    }],
    destinations=[{
        "id": "destination",
        "settings": [
            {
                "url": f"s3://{main['id']}/test1",
            },
            {
                "url": f"s3://{main2['id']}/test2",
            },
        ],
    }],
    encoder_settings={
        "timecode_config": {
            "source": "EMBEDDED",
        },
        "audio_descriptions": [{
            "audio_selector_name": "example audio selector",
            "name": "audio-selector",
        }],
        "video_descriptions": [{
            "name": "example-video",
        }],
        "output_groups": [{
            "output_group_settings": {
                "archive_group_settings": [{
                    "destination": {
                        "destination_ref_id": "destination",
                    },
                }],
            },
            "outputs": [{
                "output_name": "example-name",
                "video_description_name": "example-video",
                "audio_description_names": ["audio-selector"],
                "output_settings": {
                    "archive_output_settings": {
                        "name_modifier": "_1",
                        "extension": "m2ts",
                        "container_settings": {
                            "m2ts_settings": {
                                "audio_buffer_model": "ATSC",
                                "buffer_model": "MULTIPLEX",
                                "rate_mode": "CBR",
                            },
                        },
                    },
                },
            }],
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
    var example = new Aws.MediaLive.Channel("example", new()
    {
        Name = "example-channel",
        ChannelClass = "STANDARD",
        RoleArn = exampleAwsIamRole.Arn,
        InputSpecification = new Aws.MediaLive.Inputs.ChannelInputSpecificationArgs
        {
            Codec = "AVC",
            InputResolution = "HD",
            MaximumBitrate = "MAX_20_MBPS",
        },
        InputAttachments = new[]
        {
            new Aws.MediaLive.Inputs.ChannelInputAttachmentArgs
            {
                InputAttachmentName = "example-input",
                InputId = exampleAwsMedialiveInput.Id,
            },
        },
        Destinations = new[]
        {
            new Aws.MediaLive.Inputs.ChannelDestinationArgs
            {
                Id = "destination",
                Settings = new[]
                {
                    new Aws.MediaLive.Inputs.ChannelDestinationSettingArgs
                    {
                        Url = $"s3://{main.Id}/test1",
                    },
                    new Aws.MediaLive.Inputs.ChannelDestinationSettingArgs
                    {
                        Url = $"s3://{main2.Id}/test2",
                    },
                },
            },
        },
        EncoderSettings = new Aws.MediaLive.Inputs.ChannelEncoderSettingsArgs
        {
            TimecodeConfig = new Aws.MediaLive.Inputs.ChannelEncoderSettingsTimecodeConfigArgs
            {
                Source = "EMBEDDED",
            },
            AudioDescriptions = new[]
            {
                new Aws.MediaLive.Inputs.ChannelEncoderSettingsAudioDescriptionArgs
                {
                    AudioSelectorName = "example audio selector",
                    Name = "audio-selector",
                },
            },
            VideoDescriptions = new[]
            {
                new Aws.MediaLive.Inputs.ChannelEncoderSettingsVideoDescriptionArgs
                {
                    Name = "example-video",
                },
            },
            OutputGroups = new[]
            {
                new Aws.MediaLive.Inputs.ChannelEncoderSettingsOutputGroupArgs
                {
                    OutputGroupSettings = new Aws.MediaLive.Inputs.ChannelEncoderSettingsOutputGroupOutputGroupSettingsArgs
                    {
                        ArchiveGroupSettings = new[]
                        {
                            new Aws.MediaLive.Inputs.ChannelEncoderSettingsOutputGroupOutputGroupSettingsArchiveGroupSettingArgs
                            {
                                Destination = new Aws.MediaLive.Inputs.ChannelEncoderSettingsOutputGroupOutputGroupSettingsArchiveGroupSettingDestinationArgs
                                {
                                    DestinationRefId = "destination",
                                },
                            },
                        },
                    },
                    Outputs = new[]
                    {
                        new Aws.MediaLive.Inputs.ChannelEncoderSettingsOutputGroupOutputArgs
                        {
                            OutputName = "example-name",
                            VideoDescriptionName = "example-video",
                            AudioDescriptionNames = new[]
                            {
                                "audio-selector",
                            },
                            OutputSettings = new Aws.MediaLive.Inputs.ChannelEncoderSettingsOutputGroupOutputOutputSettingsArgs
                            {
                                ArchiveOutputSettings = new Aws.MediaLive.Inputs.ChannelEncoderSettingsOutputGroupOutputOutputSettingsArchiveOutputSettingsArgs
                                {
                                    NameModifier = "_1",
                                    Extension = "m2ts",
                                    ContainerSettings = new Aws.MediaLive.Inputs.ChannelEncoderSettingsOutputGroupOutputOutputSettingsArchiveOutputSettingsContainerSettingsArgs
                                    {
                                        M2tsSettings = new Aws.MediaLive.Inputs.ChannelEncoderSettingsOutputGroupOutputOutputSettingsArchiveOutputSettingsContainerSettingsM2tsSettingsArgs
                                        {
                                            AudioBufferModel = "ATSC",
                                            BufferModel = "MULTIPLEX",
                                            RateMode = "CBR",
                                        },
                                    },
                                },
                            },
                        },
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
	"fmt"

	"github.com/pulumi/pulumi-aws/sdk/v6/go/aws/medialive"
	"github.com/pulumi/pulumi/sdk/v3/go/pulumi"
)

func main() {
	pulumi.Run(func(ctx *pulumi.Context) error {
		_, err := medialive.NewChannel(ctx, "example", &medialive.ChannelArgs{
			Name:         pulumi.String("example-channel"),
			ChannelClass: pulumi.String("STANDARD"),
			RoleArn:      pulumi.Any(exampleAwsIamRole.Arn),
			InputSpecification: &medialive.ChannelInputSpecificationArgs{
				Codec:           pulumi.String("AVC"),
				InputResolution: pulumi.String("HD"),
				MaximumBitrate:  pulumi.String("MAX_20_MBPS"),
			},
			InputAttachments: medialive.ChannelInputAttachmentArray{
				&medialive.ChannelInputAttachmentArgs{
					InputAttachmentName: pulumi.String("example-input"),
					InputId:             pulumi.Any(exampleAwsMedialiveInput.Id),
				},
			},
			Destinations: medialive.ChannelDestinationArray{
				&medialive.ChannelDestinationArgs{
					Id: pulumi.String("destination"),
					Settings: medialive.ChannelDestinationSettingArray{
						&medialive.ChannelDestinationSettingArgs{
							Url: pulumi.Sprintf("s3://%v/test1", main.Id),
						},
						&medialive.ChannelDestinationSettingArgs{
							Url: pulumi.Sprintf("s3://%v/test2", main2.Id),
						},
					},
				},
			},
			EncoderSettings: &medialive.ChannelEncoderSettingsArgs{
				TimecodeConfig: &medialive.ChannelEncoderSettingsTimecodeConfigArgs{
					Source: pulumi.String("EMBEDDED"),
				},
				AudioDescriptions: medialive.ChannelEncoderSettingsAudioDescriptionArray{
					&medialive.ChannelEncoderSettingsAudioDescriptionArgs{
						AudioSelectorName: pulumi.String("example audio selector"),
						Name:              pulumi.String("audio-selector"),
					},
				},
				VideoDescriptions: medialive.ChannelEncoderSettingsVideoDescriptionArray{
					&medialive.ChannelEncoderSettingsVideoDescriptionArgs{
						Name: pulumi.String("example-video"),
					},
				},
				OutputGroups: medialive.ChannelEncoderSettingsOutputGroupArray{
					&medialive.ChannelEncoderSettingsOutputGroupArgs{
						OutputGroupSettings: &medialive.ChannelEncoderSettingsOutputGroupOutputGroupSettingsArgs{
							ArchiveGroupSettings: medialive.ChannelEncoderSettingsOutputGroupOutputGroupSettingsArchiveGroupSettingArray{
								&medialive.ChannelEncoderSettingsOutputGroupOutputGroupSettingsArchiveGroupSettingArgs{
									Destination: &medialive.ChannelEncoderSettingsOutputGroupOutputGroupSettingsArchiveGroupSettingDestinationArgs{
										DestinationRefId: pulumi.String("destination"),
									},
								},
							},
						},
						Outputs: medialive.ChannelEncoderSettingsOutputGroupOutputTypeArray{
							&medialive.ChannelEncoderSettingsOutputGroupOutputTypeArgs{
								OutputName:           pulumi.String("example-name"),
								VideoDescriptionName: pulumi.String("example-video"),
								AudioDescriptionNames: pulumi.StringArray{
									pulumi.String("audio-selector"),
								},
								OutputSettings: &medialive.ChannelEncoderSettingsOutputGroupOutputOutputSettingsArgs{
									ArchiveOutputSettings: &medialive.ChannelEncoderSettingsOutputGroupOutputOutputSettingsArchiveOutputSettingsArgs{
										NameModifier: pulumi.String("_1"),
										Extension:    pulumi.String("m2ts"),
										ContainerSettings: &medialive.ChannelEncoderSettingsOutputGroupOutputOutputSettingsArchiveOutputSettingsContainerSettingsArgs{
											M2tsSettings: &medialive.ChannelEncoderSettingsOutputGroupOutputOutputSettingsArchiveOutputSettingsContainerSettingsM2tsSettingsArgs{
												AudioBufferModel: pulumi.String("ATSC"),
												BufferModel:      pulumi.String("MULTIPLEX"),
												RateMode:         pulumi.String("CBR"),
											},
										},
									},
								},
							},
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
import com.pulumi.aws.medialive.Channel;
import com.pulumi.aws.medialive.ChannelArgs;
import com.pulumi.aws.medialive.inputs.ChannelInputSpecificationArgs;
import com.pulumi.aws.medialive.inputs.ChannelInputAttachmentArgs;
import com.pulumi.aws.medialive.inputs.ChannelDestinationArgs;
import com.pulumi.aws.medialive.inputs.ChannelEncoderSettingsArgs;
import com.pulumi.aws.medialive.inputs.ChannelEncoderSettingsTimecodeConfigArgs;
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
        var example = new Channel("example", ChannelArgs.builder()
            .name("example-channel")
            .channelClass("STANDARD")
            .roleArn(exampleAwsIamRole.arn())
            .inputSpecification(ChannelInputSpecificationArgs.builder()
                .codec("AVC")
                .inputResolution("HD")
                .maximumBitrate("MAX_20_MBPS")
                .build())
            .inputAttachments(ChannelInputAttachmentArgs.builder()
                .inputAttachmentName("example-input")
                .inputId(exampleAwsMedialiveInput.id())
                .build())
            .destinations(ChannelDestinationArgs.builder()
                .id("destination")
                .settings(                
                    ChannelDestinationSettingArgs.builder()
                        .url(String.format("s3://%s/test1", main.id()))
                        .build(),
                    ChannelDestinationSettingArgs.builder()
                        .url(String.format("s3://%s/test2", main2.id()))
                        .build())
                .build())
            .encoderSettings(ChannelEncoderSettingsArgs.builder()
                .timecodeConfig(ChannelEncoderSettingsTimecodeConfigArgs.builder()
                    .source("EMBEDDED")
                    .build())
                .audioDescriptions(ChannelEncoderSettingsAudioDescriptionArgs.builder()
                    .audioSelectorName("example audio selector")
                    .name("audio-selector")
                    .build())
                .videoDescriptions(ChannelEncoderSettingsVideoDescriptionArgs.builder()
                    .name("example-video")
                    .build())
                .outputGroups(ChannelEncoderSettingsOutputGroupArgs.builder()
                    .outputGroupSettings(ChannelEncoderSettingsOutputGroupOutputGroupSettingsArgs.builder()
                        .archiveGroupSettings(ChannelEncoderSettingsOutputGroupOutputGroupSettingsArchiveGroupSettingArgs.builder()
                            .destination(ChannelEncoderSettingsOutputGroupOutputGroupSettingsArchiveGroupSettingDestinationArgs.builder()
                                .destinationRefId("destination")
                                .build())
                            .build())
                        .build())
                    .outputs(ChannelEncoderSettingsOutputGroupOutputArgs.builder()
                        .outputName("example-name")
                        .videoDescriptionName("example-video")
                        .audioDescriptionNames("audio-selector")
                        .outputSettings(ChannelEncoderSettingsOutputGroupOutputOutputSettingsArgs.builder()
                            .archiveOutputSettings(ChannelEncoderSettingsOutputGroupOutputOutputSettingsArchiveOutputSettingsArgs.builder()
                                .nameModifier("_1")
                                .extension("m2ts")
                                .containerSettings(ChannelEncoderSettingsOutputGroupOutputOutputSettingsArchiveOutputSettingsContainerSettingsArgs.builder()
                                    .m2tsSettings(ChannelEncoderSettingsOutputGroupOutputOutputSettingsArchiveOutputSettingsContainerSettingsM2tsSettingsArgs.builder()
                                        .audioBufferModel("ATSC")
                                        .bufferModel("MULTIPLEX")
                                        .rateMode("CBR")
                                        .build())
                                    .build())
                                .build())
                            .build())
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
    type: aws:medialive:Channel
    properties:
      name: example-channel
      channelClass: STANDARD
      roleArn: ${exampleAwsIamRole.arn}
      inputSpecification:
        codec: AVC
        inputResolution: HD
        maximumBitrate: MAX_20_MBPS
      inputAttachments:
        - inputAttachmentName: example-input
          inputId: ${exampleAwsMedialiveInput.id}
      destinations:
        - id: destination
          settings:
            - url: s3://${main.id}/test1
            - url: s3://${main2.id}/test2
      encoderSettings:
        timecodeConfig:
          source: EMBEDDED
        audioDescriptions:
          - audioSelectorName: example audio selector
            name: audio-selector
        videoDescriptions:
          - name: example-video
        outputGroups:
          - outputGroupSettings:
              archiveGroupSettings:
                - destination:
                    destinationRefId: destination
            outputs:
              - outputName: example-name
                videoDescriptionName: example-video
                audioDescriptionNames:
                  - audio-selector
                outputSettings:
                  archiveOutputSettings:
                    nameModifier: _1
                    extension: m2ts
                    containerSettings:
                      m2tsSettings:
                        audioBufferModel: ATSC
                        bufferModel: MULTIPLEX
                        rateMode: CBR
```
<!--End PulumiCodeChooser -->

## Import

Using `pulumi import`, import MediaLive Channel using the `channel_id`. For example:

```sh
$ pulumi import aws:medialive/channel:Channel example 1234567
```
�
cdiInputSpecificationxBv:t
r
	medialiveChannelCdiInputSpecificationGaws:medialive/ChannelCdiInputSpecification:ChannelCdiInputSpecification\Specification of CDI inputs for this channel. See CDI Input Specification for more details.
2
channelClass" Concise argument description.
�
destinationsZ*X:V
T
	medialiveChannelDestination3aws:medialive/ChannelDestination:ChannelDestination=Destinations for channel. See Destinations for more details.
�
encoderSettingsd:b
`
	medialiveChannelEncoderSettings;aws:medialive/ChannelEncoderSettings:ChannelEncoderSettings9Encoder settings. See Encoder Settings for more details.
�
inputAttachmentsf*d:b
`
	medialiveChannelInputAttachment;aws:medialive/ChannelInputAttachment:ChannelInputAttachmentKInput attachments for the channel. See Input Attachments for more details.
�
inputSpecificationm:k
i
	medialiveChannelInputSpecificationAaws:medialive/ChannelInputSpecification:ChannelInputSpecification:Specification of network and file inputs for the channel.
=
logLevelB" +The log level to write to Cloudwatch logs.
�
maintenanceZBX:V
T
	medialiveChannelMaintenance3aws:medialive/ChannelMaintenance:ChannelMaintenanceIMaintenance settings for this channel. See Maintenance for more details.
J
nameB" <Name of the Channel.

The following arguments are optional:
/
roleArnB" Concise argument description.
F
startChannelB
 0Whether to start/stop channel. Default: `false`
�
tagsB2" �A map of tags to assign to the channel. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
�
vpcBB@:>
<
	medialive
ChannelVpc#aws:medialive/ChannelVpc:ChannelVpc8Settings for the VPC outputs. See VPC for more details.
"
arn" ARN of the Channel.
"�
cdiInputSpecificationxBv:t
r
	medialiveChannelCdiInputSpecificationGaws:medialive/ChannelCdiInputSpecification:ChannelCdiInputSpecification\Specification of CDI inputs for this channel. See CDI Input Specification for more details.
"2
channelClass" Concise argument description.
"$
	channelId" ID of the Channel.
"�
destinationsZ*X:V
T
	medialiveChannelDestination3aws:medialive/ChannelDestination:ChannelDestination=Destinations for channel. See Destinations for more details.
"�
encoderSettingsd:b
`
	medialiveChannelEncoderSettings;aws:medialive/ChannelEncoderSettings:ChannelEncoderSettings9Encoder settings. See Encoder Settings for more details.
"�
inputAttachmentsf*d:b
`
	medialiveChannelInputAttachment;aws:medialive/ChannelInputAttachment:ChannelInputAttachmentKInput attachments for the channel. See Input Attachments for more details.
"�
inputSpecificationm:k
i
	medialiveChannelInputSpecificationAaws:medialive/ChannelInputSpecification:ChannelInputSpecification:Specification of network and file inputs for the channel.
";
logLevel" +The log level to write to Cloudwatch logs.
"�
maintenanceX:V
T
	medialiveChannelMaintenance3aws:medialive/ChannelMaintenance:ChannelMaintenanceIMaintenance settings for this channel. See Maintenance for more details.
"H
name" <Name of the Channel.

The following arguments are optional:
"/
roleArnB" Concise argument description.
"F
startChannelB
 0Whether to start/stop channel. Default: `false`
"�
tagsB2" �A map of tags to assign to the channel. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
"
tagsAll2" "�
vpcBB@:>
<
	medialive
ChannelVpc#aws:medialive/ChannelVpc:ChannelVpc8Settings for the VPC outputs. See VPC for more details.
*�;
-
	medialiveInputaws:medialive/input:Input�#Resource for managing an AWS MediaLive Input.

## Example Usage

### Basic Usage

<!--Start PulumiCodeChooser -->
```typescript
import * as pulumi from "@pulumi/pulumi";
import * as aws from "@pulumi/aws";

const example = new aws.medialive.InputSecurityGroup("example", {
    whitelistRules: [{
        cidr: "10.0.0.8/32",
    }],
    tags: {
        ENVIRONMENT: "prod",
    },
});
const exampleInput = new aws.medialive.Input("example", {
    name: "example-input",
    inputSecurityGroups: [example.id],
    type: "UDP_PUSH",
    tags: {
        ENVIRONMENT: "prod",
    },
});
```
```python
import pulumi
import pulumi_aws as aws

example = aws.medialive.InputSecurityGroup("example",
    whitelist_rules=[{
        "cidr": "10.0.0.8/32",
    }],
    tags={
        "ENVIRONMENT": "prod",
    })
example_input = aws.medialive.Input("example",
    name="example-input",
    input_security_groups=[example.id],
    type="UDP_PUSH",
    tags={
        "ENVIRONMENT": "prod",
    })
```
```csharp
using System.Collections.Generic;
using System.Linq;
using Pulumi;
using Aws = Pulumi.Aws;

return await Deployment.RunAsync(() => 
{
    var example = new Aws.MediaLive.InputSecurityGroup("example", new()
    {
        WhitelistRules = new[]
        {
            new Aws.MediaLive.Inputs.InputSecurityGroupWhitelistRuleArgs
            {
                Cidr = "10.0.0.8/32",
            },
        },
        Tags = 
        {
            { "ENVIRONMENT", "prod" },
        },
    });

    var exampleInput = new Aws.MediaLive.Input("example", new()
    {
        Name = "example-input",
        InputSecurityGroups = new[]
        {
            example.Id,
        },
        Type = "UDP_PUSH",
        Tags = 
        {
            { "ENVIRONMENT", "prod" },
        },
    });

});
```
```go
package main

import (
	"github.com/pulumi/pulumi-aws/sdk/v6/go/aws/medialive"
	"github.com/pulumi/pulumi/sdk/v3/go/pulumi"
)

func main() {
	pulumi.Run(func(ctx *pulumi.Context) error {
		example, err := medialive.NewInputSecurityGroup(ctx, "example", &medialive.InputSecurityGroupArgs{
			WhitelistRules: medialive.InputSecurityGroupWhitelistRuleArray{
				&medialive.InputSecurityGroupWhitelistRuleArgs{
					Cidr: pulumi.String("10.0.0.8/32"),
				},
			},
			Tags: pulumi.StringMap{
				"ENVIRONMENT": pulumi.String("prod"),
			},
		})
		if err != nil {
			return err
		}
		_, err = medialive.NewInput(ctx, "example", &medialive.InputArgs{
			Name: pulumi.String("example-input"),
			InputSecurityGroups: pulumi.StringArray{
				example.ID(),
			},
			Type: pulumi.String("UDP_PUSH"),
			Tags: pulumi.StringMap{
				"ENVIRONMENT": pulumi.String("prod"),
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
import com.pulumi.aws.medialive.InputSecurityGroup;
import com.pulumi.aws.medialive.InputSecurityGroupArgs;
import com.pulumi.aws.medialive.inputs.InputSecurityGroupWhitelistRuleArgs;
import com.pulumi.aws.medialive.Input;
import com.pulumi.aws.medialive.InputArgs;
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
        var example = new InputSecurityGroup("example", InputSecurityGroupArgs.builder()
            .whitelistRules(InputSecurityGroupWhitelistRuleArgs.builder()
                .cidr("10.0.0.8/32")
                .build())
            .tags(Map.of("ENVIRONMENT", "prod"))
            .build());

        var exampleInput = new Input("exampleInput", InputArgs.builder()
            .name("example-input")
            .inputSecurityGroups(example.id())
            .type("UDP_PUSH")
            .tags(Map.of("ENVIRONMENT", "prod"))
            .build());

    }
}
```
```yaml
resources:
  example:
    type: aws:medialive:InputSecurityGroup
    properties:
      whitelistRules:
        - cidr: 10.0.0.8/32
      tags:
        ENVIRONMENT: prod
  exampleInput:
    type: aws:medialive:Input
    name: example
    properties:
      name: example-input
      inputSecurityGroups:
        - ${example.id}
      type: UDP_PUSH
      tags:
        ENVIRONMENT: prod
```
<!--End PulumiCodeChooser -->

## Import

Using `pulumi import`, import MediaLive Input using the `id`. For example:

```sh
$ pulumi import aws:medialive/input:Input example 12345678
```
�
destinationsVBT*R:P
N
	medialiveInputDestination/aws:medialive/InputDestination:InputDestinationNDestination settings for PUSH type inputs. See Destinations for more details.
�
inputDevicesVBT*R:P
N
	medialiveInputInputDevice/aws:medialive/InputInputDevice:InputInputDevice>Settings for the devices. See Input Devices for more details.
>
inputSecurityGroupsB*" List of input security groups.
�
mediaConnectFlowseBc*a:_
]
	medialiveInputMediaConnectFlow9aws:medialive/InputMediaConnectFlow:InputMediaConnectFlowLA list of the MediaConnect Flows. See Media Connect Flows for more details.
!
nameB" Name of the input.
S
roleArnB" BThe ARN of the role this input assumes during and after creation.
�
sourcesGBE*C:A
?
	medialiveInputSource%aws:medialive/InputSource:InputSourceEThe source URLs for a PULL-type input. See Sources for more details.
�
tagsB2" �A map of tags to assign to the Input. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
x
type" lThe different types of inputs that AWS Elemental MediaLive supports.

The following arguments are optional:
�
vpc<B::8
6
	medialiveInputVpcaws:medialive/InputVpc:InputVpc<Settings for a private VPC Input. See VPC for more details.
"
arn" ARN of the Input.
"6
attachedChannels*" Channels attached to Input.
"�
destinationsVBT*R:P
N
	medialiveInputDestination/aws:medialive/InputDestination:InputDestinationNDestination settings for PUSH type inputs. See Destinations for more details.
"#

inputClass" The input class.
"�
inputDevicesT*R:P
N
	medialiveInputInputDevice/aws:medialive/InputInputDevice:InputInputDevice>Settings for the devices. See Input Devices for more details.
"V
inputPartnerIds*" =A list of IDs for all Inputs which are partners of this one.
">
inputSecurityGroupsB*" List of input security groups.
"1
inputSourceType" Source type of the input.
"�
mediaConnectFlowsc*a:_
]
	medialiveInputMediaConnectFlow9aws:medialive/InputMediaConnectFlow:InputMediaConnectFlowLA list of the MediaConnect Flows. See Media Connect Flows for more details.
"
name" Name of the input.
"Q
roleArn" BThe ARN of the role this input assumes during and after creation.
"�
sourcesE*C:A
?
	medialiveInputSource%aws:medialive/InputSource:InputSourceEThe source URLs for a PULL-type input. See Sources for more details.
"�
tagsB2" �A map of tags to assign to the Input. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
"
tagsAll2" "x
type" lThe different types of inputs that AWS Elemental MediaLive supports.

The following arguments are optional:
"�
vpc<B::8
6
	medialiveInputVpcaws:medialive/InputVpc:InputVpc<Settings for a private VPC Input. See VPC for more details.
*� 
T
	medialiveInputSecurityGroup3aws:medialive/inputSecurityGroup:InputSecurityGroup�Resource for managing an AWS MediaLive InputSecurityGroup.

## Example Usage

### Basic Usage

<!--Start PulumiCodeChooser -->
```typescript
import * as pulumi from "@pulumi/pulumi";
import * as aws from "@pulumi/aws";

const example = new aws.medialive.InputSecurityGroup("example", {
    whitelistRules: [{
        cidr: "10.0.0.8/32",
    }],
    tags: {
        ENVIRONMENT: "prod",
    },
});
```
```python
import pulumi
import pulumi_aws as aws

example = aws.medialive.InputSecurityGroup("example",
    whitelist_rules=[{
        "cidr": "10.0.0.8/32",
    }],
    tags={
        "ENVIRONMENT": "prod",
    })
```
```csharp
using System.Collections.Generic;
using System.Linq;
using Pulumi;
using Aws = Pulumi.Aws;

return await Deployment.RunAsync(() => 
{
    var example = new Aws.MediaLive.InputSecurityGroup("example", new()
    {
        WhitelistRules = new[]
        {
            new Aws.MediaLive.Inputs.InputSecurityGroupWhitelistRuleArgs
            {
                Cidr = "10.0.0.8/32",
            },
        },
        Tags = 
        {
            { "ENVIRONMENT", "prod" },
        },
    });

});
```
```go
package main

import (
	"github.com/pulumi/pulumi-aws/sdk/v6/go/aws/medialive"
	"github.com/pulumi/pulumi/sdk/v3/go/pulumi"
)

func main() {
	pulumi.Run(func(ctx *pulumi.Context) error {
		_, err := medialive.NewInputSecurityGroup(ctx, "example", &medialive.InputSecurityGroupArgs{
			WhitelistRules: medialive.InputSecurityGroupWhitelistRuleArray{
				&medialive.InputSecurityGroupWhitelistRuleArgs{
					Cidr: pulumi.String("10.0.0.8/32"),
				},
			},
			Tags: pulumi.StringMap{
				"ENVIRONMENT": pulumi.String("prod"),
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
import com.pulumi.aws.medialive.InputSecurityGroup;
import com.pulumi.aws.medialive.InputSecurityGroupArgs;
import com.pulumi.aws.medialive.inputs.InputSecurityGroupWhitelistRuleArgs;
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
        var example = new InputSecurityGroup("example", InputSecurityGroupArgs.builder()
            .whitelistRules(InputSecurityGroupWhitelistRuleArgs.builder()
                .cidr("10.0.0.8/32")
                .build())
            .tags(Map.of("ENVIRONMENT", "prod"))
            .build());

    }
}
```
```yaml
resources:
  example:
    type: aws:medialive:InputSecurityGroup
    properties:
      whitelistRules:
        - cidr: 10.0.0.8/32
      tags:
        ENVIRONMENT: prod
```
<!--End PulumiCodeChooser -->

## Import

Using `pulumi import`, import MediaLive InputSecurityGroup using the `id`. For example:

```sh
$ pulumi import aws:medialive/inputSecurityGroup:InputSecurityGroup example 123456
```
�
tagsB2" �A map of tags to assign to the InputSecurityGroup. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
�
whitelistRules�*:}
{
	medialiveInputSecurityGroupWhitelistRuleMaws:medialive/InputSecurityGroupWhitelistRule:InputSecurityGroupWhitelistRule^Whitelist rules. See Whitelist Rules for more details.

The following arguments are optional:
"*
arn" ARN of the InputSecurityGroup.
"L
inputs*" <The list of inputs currently using this InputSecurityGroup.
"�
tagsB2" �A map of tags to assign to the InputSecurityGroup. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
"
tagsAll2" "�
whitelistRules�*:}
{
	medialiveInputSecurityGroupWhitelistRuleMaws:medialive/InputSecurityGroupWhitelistRule:InputSecurityGroupWhitelistRule^Whitelist rules. See Whitelist Rules for more details.

The following arguments are optional:
*�9
9
	medialive	Multiplex!aws:medialive/multiplex:Multiplex�.Resource for managing an AWS MediaLive Multiplex.

## Example Usage

### Basic Usage

<!--Start PulumiCodeChooser -->
```typescript
import * as pulumi from "@pulumi/pulumi";
import * as aws from "@pulumi/aws";

const available = aws.getAvailabilityZones({
    state: "available",
});
const example = new aws.medialive.Multiplex("example", {
    name: "example-multiplex-changed",
    availabilityZones: [
        available.then(available => available.names?.[0]),
        available.then(available => available.names?.[1]),
    ],
    multiplexSettings: {
        transportStreamBitrate: 1000000,
        transportStreamId: 1,
        transportStreamReservedBitrate: 1,
        maximumVideoBufferDelayMilliseconds: 1000,
    },
    startMultiplex: true,
    tags: {
        tag1: "value1",
    },
});
```
```python
import pulumi
import pulumi_aws as aws

available = aws.get_availability_zones(state="available")
example = aws.medialive.Multiplex("example",
    name="example-multiplex-changed",
    availability_zones=[
        available.names[0],
        available.names[1],
    ],
    multiplex_settings={
        "transport_stream_bitrate": 1000000,
        "transport_stream_id": 1,
        "transport_stream_reserved_bitrate": 1,
        "maximum_video_buffer_delay_milliseconds": 1000,
    },
    start_multiplex=True,
    tags={
        "tag1": "value1",
    })
```
```csharp
using System.Collections.Generic;
using System.Linq;
using Pulumi;
using Aws = Pulumi.Aws;

return await Deployment.RunAsync(() => 
{
    var available = Aws.GetAvailabilityZones.Invoke(new()
    {
        State = "available",
    });

    var example = new Aws.MediaLive.Multiplex("example", new()
    {
        Name = "example-multiplex-changed",
        AvailabilityZones = new[]
        {
            available.Apply(getAvailabilityZonesResult => getAvailabilityZonesResult.Names[0]),
            available.Apply(getAvailabilityZonesResult => getAvailabilityZonesResult.Names[1]),
        },
        MultiplexSettings = new Aws.MediaLive.Inputs.MultiplexMultiplexSettingsArgs
        {
            TransportStreamBitrate = 1000000,
            TransportStreamId = 1,
            TransportStreamReservedBitrate = 1,
            MaximumVideoBufferDelayMilliseconds = 1000,
        },
        StartMultiplex = true,
        Tags = 
        {
            { "tag1", "value1" },
        },
    });

});
```
```go
package main

import (
	"github.com/pulumi/pulumi-aws/sdk/v6/go/aws"
	"github.com/pulumi/pulumi-aws/sdk/v6/go/aws/medialive"
	"github.com/pulumi/pulumi/sdk/v3/go/pulumi"
)

func main() {
	pulumi.Run(func(ctx *pulumi.Context) error {
		available, err := aws.GetAvailabilityZones(ctx, &aws.GetAvailabilityZonesArgs{
			State: pulumi.StringRef("available"),
		}, nil)
		if err != nil {
			return err
		}
		_, err = medialive.NewMultiplex(ctx, "example", &medialive.MultiplexArgs{
			Name: pulumi.String("example-multiplex-changed"),
			AvailabilityZones: pulumi.StringArray{
				pulumi.String(available.Names[0]),
				pulumi.String(available.Names[1]),
			},
			MultiplexSettings: &medialive.MultiplexMultiplexSettingsArgs{
				TransportStreamBitrate:              pulumi.Int(1000000),
				TransportStreamId:                   pulumi.Int(1),
				TransportStreamReservedBitrate:      pulumi.Int(1),
				MaximumVideoBufferDelayMilliseconds: pulumi.Int(1000),
			},
			StartMultiplex: pulumi.Bool(true),
			Tags: pulumi.StringMap{
				"tag1": pulumi.String("value1"),
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
import com.pulumi.aws.inputs.GetAvailabilityZonesArgs;
import com.pulumi.aws.medialive.Multiplex;
import com.pulumi.aws.medialive.MultiplexArgs;
import com.pulumi.aws.medialive.inputs.MultiplexMultiplexSettingsArgs;
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
        final var available = AwsFunctions.getAvailabilityZones(GetAvailabilityZonesArgs.builder()
            .state("available")
            .build());

        var example = new Multiplex("example", MultiplexArgs.builder()
            .name("example-multiplex-changed")
            .availabilityZones(            
                available.applyValue(getAvailabilityZonesResult -> getAvailabilityZonesResult.names()[0]),
                available.applyValue(getAvailabilityZonesResult -> getAvailabilityZonesResult.names()[1]))
            .multiplexSettings(MultiplexMultiplexSettingsArgs.builder()
                .transportStreamBitrate(1000000)
                .transportStreamId(1)
                .transportStreamReservedBitrate(1)
                .maximumVideoBufferDelayMilliseconds(1000)
                .build())
            .startMultiplex(true)
            .tags(Map.of("tag1", "value1"))
            .build());

    }
}
```
```yaml
resources:
  example:
    type: aws:medialive:Multiplex
    properties:
      name: example-multiplex-changed
      availabilityZones:
        - ${available.names[0]}
        - ${available.names[1]}
      multiplexSettings:
        transportStreamBitrate: 1e+06
        transportStreamId: 1
        transportStreamReservedBitrate: 1
        maximumVideoBufferDelayMilliseconds: 1000
      startMultiplex: true
      tags:
        tag1: value1
variables:
  available:
    fn::invoke:
      function: aws:getAvailabilityZones
      arguments:
        state: available
```
<!--End PulumiCodeChooser -->

## Import

Using `pulumi import`, import MediaLive Multiplex using the `id`. For example:

```sh
$ pulumi import aws:medialive/multiplex:Multiplex example 12345678
```
W
availabilityZones*" <A list of availability zones. You must specify exactly two.
�
multiplexSettingsrBp:n
l
	medialiveMultiplexMultiplexSettingsCaws:medialive/MultiplexMultiplexSettings:MultiplexMultiplexSettings=Multiplex settings. See Multiplex Settings for more details.
H
nameB" :name of Multiplex.

The following arguments are optional:
M
startMultiplexB
 5Whether to start the Multiplex. Defaults to `false`.
�
tagsB2" �A map of tags to assign to the Multiplex. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
"!
arn" ARN of the Multiplex.
"W
availabilityZones*" <A list of availability zones. You must specify exactly two.
"�
multiplexSettingsrBp:n
l
	medialiveMultiplexMultiplexSettingsCaws:medialive/MultiplexMultiplexSettings:MultiplexMultiplexSettings=Multiplex settings. See Multiplex Settings for more details.
"F
name" :name of Multiplex.

The following arguments are optional:
"M
startMultiplexB
 5Whether to start the Multiplex. Defaults to `false`.
"�
tagsB2" �A map of tags to assign to the Multiplex. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
"
tagsAll2" *�M
N
	medialiveMultiplexProgram/aws:medialive/multiplexProgram:MultiplexProgram�FResource for managing an AWS MediaLive MultiplexProgram.

## Example Usage

### Basic Usage

<!--Start PulumiCodeChooser -->
```typescript
import * as pulumi from "@pulumi/pulumi";
import * as aws from "@pulumi/aws";

const available = aws.getAvailabilityZones({
    state: "available",
});
const example = new aws.medialive.Multiplex("example", {
    name: "example-multiplex-changed",
    availabilityZones: [
        available.then(available => available.names?.[0]),
        available.then(available => available.names?.[1]),
    ],
    multiplexSettings: {
        transportStreamBitrate: 1000000,
        transportStreamId: 1,
        transportStreamReservedBitrate: 1,
        maximumVideoBufferDelayMilliseconds: 1000,
    },
    startMultiplex: true,
    tags: {
        tag1: "value1",
    },
});
const exampleMultiplexProgram = new aws.medialive.MultiplexProgram("example", {
    programName: "example_program",
    multiplexId: example.id,
    multiplexProgramSettings: {
        programNumber: 1,
        preferredChannelPipeline: "CURRENTLY_ACTIVE",
        videoSettings: {
            constantBitrate: 100000,
        },
    },
});
```
```python
import pulumi
import pulumi_aws as aws

available = aws.get_availability_zones(state="available")
example = aws.medialive.Multiplex("example",
    name="example-multiplex-changed",
    availability_zones=[
        available.names[0],
        available.names[1],
    ],
    multiplex_settings={
        "transport_stream_bitrate": 1000000,
        "transport_stream_id": 1,
        "transport_stream_reserved_bitrate": 1,
        "maximum_video_buffer_delay_milliseconds": 1000,
    },
    start_multiplex=True,
    tags={
        "tag1": "value1",
    })
example_multiplex_program = aws.medialive.MultiplexProgram("example",
    program_name="example_program",
    multiplex_id=example.id,
    multiplex_program_settings={
        "program_number": 1,
        "preferred_channel_pipeline": "CURRENTLY_ACTIVE",
        "video_settings": {
            "constant_bitrate": 100000,
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
    var available = Aws.GetAvailabilityZones.Invoke(new()
    {
        State = "available",
    });

    var example = new Aws.MediaLive.Multiplex("example", new()
    {
        Name = "example-multiplex-changed",
        AvailabilityZones = new[]
        {
            available.Apply(getAvailabilityZonesResult => getAvailabilityZonesResult.Names[0]),
            available.Apply(getAvailabilityZonesResult => getAvailabilityZonesResult.Names[1]),
        },
        MultiplexSettings = new Aws.MediaLive.Inputs.MultiplexMultiplexSettingsArgs
        {
            TransportStreamBitrate = 1000000,
            TransportStreamId = 1,
            TransportStreamReservedBitrate = 1,
            MaximumVideoBufferDelayMilliseconds = 1000,
        },
        StartMultiplex = true,
        Tags = 
        {
            { "tag1", "value1" },
        },
    });

    var exampleMultiplexProgram = new Aws.MediaLive.MultiplexProgram("example", new()
    {
        ProgramName = "example_program",
        MultiplexId = example.Id,
        MultiplexProgramSettings = new Aws.MediaLive.Inputs.MultiplexProgramMultiplexProgramSettingsArgs
        {
            ProgramNumber = 1,
            PreferredChannelPipeline = "CURRENTLY_ACTIVE",
            VideoSettings = new Aws.MediaLive.Inputs.MultiplexProgramMultiplexProgramSettingsVideoSettingsArgs
            {
                ConstantBitrate = 100000,
            },
        },
    });

});
```
```go
package main

import (
	"github.com/pulumi/pulumi-aws/sdk/v6/go/aws"
	"github.com/pulumi/pulumi-aws/sdk/v6/go/aws/medialive"
	"github.com/pulumi/pulumi/sdk/v3/go/pulumi"
)

func main() {
	pulumi.Run(func(ctx *pulumi.Context) error {
		available, err := aws.GetAvailabilityZones(ctx, &aws.GetAvailabilityZonesArgs{
			State: pulumi.StringRef("available"),
		}, nil)
		if err != nil {
			return err
		}
		example, err := medialive.NewMultiplex(ctx, "example", &medialive.MultiplexArgs{
			Name: pulumi.String("example-multiplex-changed"),
			AvailabilityZones: pulumi.StringArray{
				pulumi.String(available.Names[0]),
				pulumi.String(available.Names[1]),
			},
			MultiplexSettings: &medialive.MultiplexMultiplexSettingsArgs{
				TransportStreamBitrate:              pulumi.Int(1000000),
				TransportStreamId:                   pulumi.Int(1),
				TransportStreamReservedBitrate:      pulumi.Int(1),
				MaximumVideoBufferDelayMilliseconds: pulumi.Int(1000),
			},
			StartMultiplex: pulumi.Bool(true),
			Tags: pulumi.StringMap{
				"tag1": pulumi.String("value1"),
			},
		})
		if err != nil {
			return err
		}
		_, err = medialive.NewMultiplexProgram(ctx, "example", &medialive.MultiplexProgramArgs{
			ProgramName: pulumi.String("example_program"),
			MultiplexId: example.ID(),
			MultiplexProgramSettings: &medialive.MultiplexProgramMultiplexProgramSettingsArgs{
				ProgramNumber:            pulumi.Int(1),
				PreferredChannelPipeline: pulumi.String("CURRENTLY_ACTIVE"),
				VideoSettings: &medialive.MultiplexProgramMultiplexProgramSettingsVideoSettingsArgs{
					ConstantBitrate: pulumi.Int(100000),
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
import com.pulumi.aws.AwsFunctions;
import com.pulumi.aws.inputs.GetAvailabilityZonesArgs;
import com.pulumi.aws.medialive.Multiplex;
import com.pulumi.aws.medialive.MultiplexArgs;
import com.pulumi.aws.medialive.inputs.MultiplexMultiplexSettingsArgs;
import com.pulumi.aws.medialive.MultiplexProgram;
import com.pulumi.aws.medialive.MultiplexProgramArgs;
import com.pulumi.aws.medialive.inputs.MultiplexProgramMultiplexProgramSettingsArgs;
import com.pulumi.aws.medialive.inputs.MultiplexProgramMultiplexProgramSettingsVideoSettingsArgs;
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
        final var available = AwsFunctions.getAvailabilityZones(GetAvailabilityZonesArgs.builder()
            .state("available")
            .build());

        var example = new Multiplex("example", MultiplexArgs.builder()
            .name("example-multiplex-changed")
            .availabilityZones(            
                available.applyValue(getAvailabilityZonesResult -> getAvailabilityZonesResult.names()[0]),
                available.applyValue(getAvailabilityZonesResult -> getAvailabilityZonesResult.names()[1]))
            .multiplexSettings(MultiplexMultiplexSettingsArgs.builder()
                .transportStreamBitrate(1000000)
                .transportStreamId(1)
                .transportStreamReservedBitrate(1)
                .maximumVideoBufferDelayMilliseconds(1000)
                .build())
            .startMultiplex(true)
            .tags(Map.of("tag1", "value1"))
            .build());

        var exampleMultiplexProgram = new MultiplexProgram("exampleMultiplexProgram", MultiplexProgramArgs.builder()
            .programName("example_program")
            .multiplexId(example.id())
            .multiplexProgramSettings(MultiplexProgramMultiplexProgramSettingsArgs.builder()
                .programNumber(1)
                .preferredChannelPipeline("CURRENTLY_ACTIVE")
                .videoSettings(MultiplexProgramMultiplexProgramSettingsVideoSettingsArgs.builder()
                    .constantBitrate(100000)
                    .build())
                .build())
            .build());

    }
}
```
```yaml
resources:
  example:
    type: aws:medialive:Multiplex
    properties:
      name: example-multiplex-changed
      availabilityZones:
        - ${available.names[0]}
        - ${available.names[1]}
      multiplexSettings:
        transportStreamBitrate: 1e+06
        transportStreamId: 1
        transportStreamReservedBitrate: 1
        maximumVideoBufferDelayMilliseconds: 1000
      startMultiplex: true
      tags:
        tag1: value1
  exampleMultiplexProgram:
    type: aws:medialive:MultiplexProgram
    name: example
    properties:
      programName: example_program
      multiplexId: ${example.id}
      multiplexProgramSettings:
        programNumber: 1
        preferredChannelPipeline: CURRENTLY_ACTIVE
        videoSettings:
          constantBitrate: 100000
variables:
  available:
    fn::invoke:
      function: aws:getAvailabilityZones
      arguments:
        state: available
```
<!--End PulumiCodeChooser -->

## Import

Using `pulumi import`, import MediaLive MultiplexProgram using the `id`, or a combination of "`program_name`/`multiplex_id`". For example:

```sh
$ pulumi import aws:medialive/multiplexProgram:MultiplexProgram example example_program/1234567
```
!
multiplexId" Multiplex ID.
�
multiplexProgramSettings�B�:�
�
	medialive(MultiplexProgramMultiplexProgramSettings_aws:medialive/MultiplexProgramMultiplexProgramSettings:MultiplexProgramMultiplexProgramSettingssMultiplexProgram settings. See Multiplex Program Settings for more details.

The following arguments are optional:
(
programName" Unique program name.
"!
multiplexId" Multiplex ID.
"�
multiplexProgramSettings�B�:�
�
	medialive(MultiplexProgramMultiplexProgramSettings_aws:medialive/MultiplexProgramMultiplexProgramSettings:MultiplexProgramMultiplexProgramSettingssMultiplexProgram settings. See Multiplex Program Settings for more details.

The following arguments are optional:
"(
programName" Unique program name.
*�
9
mediapackageChannel aws:mediapackage/channel:Channel�Provides an AWS Elemental MediaPackage Channel.

## Example Usage

<!--Start PulumiCodeChooser -->
```typescript
import * as pulumi from "@pulumi/pulumi";
import * as aws from "@pulumi/aws";

const kittens = new aws.mediapackage.Channel("kittens", {
    channelId: "kitten-channel",
    description: "A channel dedicated to amusing videos of kittens.",
});
```
```python
import pulumi
import pulumi_aws as aws

kittens = aws.mediapackage.Channel("kittens",
    channel_id="kitten-channel",
    description="A channel dedicated to amusing videos of kittens.")
```
```csharp
using System.Collections.Generic;
using System.Linq;
using Pulumi;
using Aws = Pulumi.Aws;

return await Deployment.RunAsync(() => 
{
    var kittens = new Aws.MediaPackage.Channel("kittens", new()
    {
        ChannelId = "kitten-channel",
        Description = "A channel dedicated to amusing videos of kittens.",
    });

});
```
```go
package main

import (
	"github.com/pulumi/pulumi-aws/sdk/v6/go/aws/mediapackage"
	"github.com/pulumi/pulumi/sdk/v3/go/pulumi"
)

func main() {
	pulumi.Run(func(ctx *pulumi.Context) error {
		_, err := mediapackage.NewChannel(ctx, "kittens", &mediapackage.ChannelArgs{
			ChannelId:   pulumi.String("kitten-channel"),
			Description: pulumi.String("A channel dedicated to amusing videos of kittens."),
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
import com.pulumi.aws.mediapackage.Channel;
import com.pulumi.aws.mediapackage.ChannelArgs;
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
        var kittens = new Channel("kittens", ChannelArgs.builder()
            .channelId("kitten-channel")
            .description("A channel dedicated to amusing videos of kittens.")
            .build());

    }
}
```
```yaml
resources:
  kittens:
    type: aws:mediapackage:Channel
    properties:
      channelId: kitten-channel
      description: A channel dedicated to amusing videos of kittens.
```
<!--End PulumiCodeChooser -->

## Import

Using `pulumi import`, import Media Package Channels using the channel ID. For example:

```sh
$ pulumi import aws:mediapackage/channel:Channel kittens kittens-channel
```
<
	channelId" +A unique identifier describing the channel
2
descriptionB" A description of the channel
�
tagsB2" �A map of tags to assign to the resource. .If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
""
arn" The ARN of the channel
"<
	channelId" +A unique identifier describing the channel
"0
description" A description of the channel
"�

hlsIngestsZ*X:V
T
mediapackageChannelHlsIngest2aws:mediapackage/ChannelHlsIngest:ChannelHlsIngest-A single item list of HLS ingest information
"�
tagsB2" �A map of tags to assign to the resource. .If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
"�
tagsAll2" xA map of tags assigned to the resource, including those inherited from the provider `default_tags` configuration block.
*�
;

mediastore	Container"aws:mediastore/container:Container�Provides a MediaStore Container.

## Example Usage

<!--Start PulumiCodeChooser -->
```typescript
import * as pulumi from "@pulumi/pulumi";
import * as aws from "@pulumi/aws";

const example = new aws.mediastore.Container("example", {name: "example"});
```
```python
import pulumi
import pulumi_aws as aws

example = aws.mediastore.Container("example", name="example")
```
```csharp
using System.Collections.Generic;
using System.Linq;
using Pulumi;
using Aws = Pulumi.Aws;

return await Deployment.RunAsync(() => 
{
    var example = new Aws.MediaStore.Container("example", new()
    {
        Name = "example",
    });

});
```
```go
package main

import (
	"github.com/pulumi/pulumi-aws/sdk/v6/go/aws/mediastore"
	"github.com/pulumi/pulumi/sdk/v3/go/pulumi"
)

func main() {
	pulumi.Run(func(ctx *pulumi.Context) error {
		_, err := mediastore.NewContainer(ctx, "example", &mediastore.ContainerArgs{
			Name: pulumi.String("example"),
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
import com.pulumi.aws.mediastore.Container;
import com.pulumi.aws.mediastore.ContainerArgs;
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
        var example = new Container("example", ContainerArgs.builder()
            .name("example")
            .build());

    }
}
```
```yaml
resources:
  example:
    type: aws:mediastore:Container
    properties:
      name: example
```
<!--End PulumiCodeChooser -->

## Import

Using `pulumi import`, import MediaStore Container using the MediaStore Container Name. For example:

```sh
$ pulumi import aws:mediastore/container:Container example example
```
^
nameB" PThe name of the container. Must contain alphanumeric characters or underscores.
�
tagsB2" �A map of tags to assign to the resource. .If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
"%
arn" The ARN of the container.
"3
endpoint" #The DNS endpoint of the container.
"\
name" PThe name of the container. Must contain alphanumeric characters or underscores.
"�
tagsB2" �A map of tags to assign to the resource. .If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
"�
tagsAll2" xA map of tags assigned to the resource, including those inherited from the provider `default_tags` configuration block.
*�V
M

mediastoreContainerPolicy.aws:mediastore/containerPolicy:ContainerPolicy�T## Example Usage

<!--Start PulumiCodeChooser -->
```typescript
import * as pulumi from "@pulumi/pulumi";
import * as aws from "@pulumi/aws";

const current = aws.getRegion({});
const currentGetCallerIdentity = aws.getCallerIdentity({});
const exampleContainer = new aws.mediastore.Container("example", {name: "example"});
const example = aws.iam.getPolicyDocumentOutput({
    statements: [{
        sid: "MediaStoreFullAccess",
        effect: "Allow",
        principals: [{
            type: "AWS",
            identifiers: [currentGetCallerIdentity.then(currentGetCallerIdentity => `arn:aws:iam::${currentGetCallerIdentity.accountId}:root`)],
        }],
        actions: ["mediastore:*"],
        resources: [pulumi.all([current, currentGetCallerIdentity, exampleContainer.name]).apply(([current, currentGetCallerIdentity, name]) => `arn:aws:mediastore:${current.name}:${currentGetCallerIdentity.accountId}:container/${name}/*`)],
        conditions: [{
            test: "Bool",
            variable: "aws:SecureTransport",
            values: ["true"],
        }],
    }],
});
const exampleContainerPolicy = new aws.mediastore.ContainerPolicy("example", {
    containerName: exampleContainer.name,
    policy: example.apply(example => example.json),
});
```
```python
import pulumi
import pulumi_aws as aws

current = aws.get_region()
current_get_caller_identity = aws.get_caller_identity()
example_container = aws.mediastore.Container("example", name="example")
example = aws.iam.get_policy_document_output(statements=[{
    "sid": "MediaStoreFullAccess",
    "effect": "Allow",
    "principals": [{
        "type": "AWS",
        "identifiers": [f"arn:aws:iam::{current_get_caller_identity.account_id}:root"],
    }],
    "actions": ["mediastore:*"],
    "resources": [example_container.name.apply(lambda name: f"arn:aws:mediastore:{current.name}:{current_get_caller_identity.account_id}:container/{name}/*")],
    "conditions": [{
        "test": "Bool",
        "variable": "aws:SecureTransport",
        "values": ["true"],
    }],
}])
example_container_policy = aws.mediastore.ContainerPolicy("example",
    container_name=example_container.name,
    policy=example.json)
```
```csharp
using System.Collections.Generic;
using System.Linq;
using Pulumi;
using Aws = Pulumi.Aws;

return await Deployment.RunAsync(() => 
{
    var current = Aws.GetRegion.Invoke();

    var currentGetCallerIdentity = Aws.GetCallerIdentity.Invoke();

    var exampleContainer = new Aws.MediaStore.Container("example", new()
    {
        Name = "example",
    });

    var example = Aws.Iam.GetPolicyDocument.Invoke(new()
    {
        Statements = new[]
        {
            new Aws.Iam.Inputs.GetPolicyDocumentStatementInputArgs
            {
                Sid = "MediaStoreFullAccess",
                Effect = "Allow",
                Principals = new[]
                {
                    new Aws.Iam.Inputs.GetPolicyDocumentStatementPrincipalInputArgs
                    {
                        Type = "AWS",
                        Identifiers = new[]
                        {
                            $"arn:aws:iam::{currentGetCallerIdentity.Apply(getCallerIdentityResult => getCallerIdentityResult.AccountId)}:root",
                        },
                    },
                },
                Actions = new[]
                {
                    "mediastore:*",
                },
                Resources = new[]
                {
                    $"arn:aws:mediastore:{current.Apply(getRegionResult => getRegionResult.Name)}:{currentGetCallerIdentity.Apply(getCallerIdentityResult => getCallerIdentityResult.AccountId)}:container/{exampleContainer.Name}/*",
                },
                Conditions = new[]
                {
                    new Aws.Iam.Inputs.GetPolicyDocumentStatementConditionInputArgs
                    {
                        Test = "Bool",
                        Variable = "aws:SecureTransport",
                        Values = new[]
                        {
                            "true",
                        },
                    },
                },
            },
        },
    });

    var exampleContainerPolicy = new Aws.MediaStore.ContainerPolicy("example", new()
    {
        ContainerName = exampleContainer.Name,
        Policy = example.Apply(getPolicyDocumentResult => getPolicyDocumentResult.Json),
    });

});
```
```go
package main

import (
	"fmt"

	"github.com/pulumi/pulumi-aws/sdk/v6/go/aws"
	"github.com/pulumi/pulumi-aws/sdk/v6/go/aws/iam"
	"github.com/pulumi/pulumi-aws/sdk/v6/go/aws/mediastore"
	"github.com/pulumi/pulumi/sdk/v3/go/pulumi"
)

func main() {
	pulumi.Run(func(ctx *pulumi.Context) error {
		current, err := aws.GetRegion(ctx, &aws.GetRegionArgs{}, nil)
		if err != nil {
			return err
		}
		currentGetCallerIdentity, err := aws.GetCallerIdentity(ctx, &aws.GetCallerIdentityArgs{}, nil)
		if err != nil {
			return err
		}
		exampleContainer, err := mediastore.NewContainer(ctx, "example", &mediastore.ContainerArgs{
			Name: pulumi.String("example"),
		})
		if err != nil {
			return err
		}
		example := iam.GetPolicyDocumentOutput(ctx, iam.GetPolicyDocumentOutputArgs{
			Statements: iam.GetPolicyDocumentStatementArray{
				&iam.GetPolicyDocumentStatementArgs{
					Sid:    pulumi.String("MediaStoreFullAccess"),
					Effect: pulumi.String("Allow"),
					Principals: iam.GetPolicyDocumentStatementPrincipalArray{
						&iam.GetPolicyDocumentStatementPrincipalArgs{
							Type: pulumi.String("AWS"),
							Identifiers: pulumi.StringArray{
								pulumi.Sprintf("arn:aws:iam::%v:root", currentGetCallerIdentity.AccountId),
							},
						},
					},
					Actions: pulumi.StringArray{
						pulumi.String("mediastore:*"),
					},
					Resources: pulumi.StringArray{
						exampleContainer.Name.ApplyT(func(name string) (string, error) {
							return fmt.Sprintf("arn:aws:mediastore:%v:%v:container/%v/*", current.Name, currentGetCallerIdentity.AccountId, name), nil
						}).(pulumi.StringOutput),
					},
					Conditions: iam.GetPolicyDocumentStatementConditionArray{
						&iam.GetPolicyDocumentStatementConditionArgs{
							Test:     pulumi.String("Bool"),
							Variable: pulumi.String("aws:SecureTransport"),
							Values: pulumi.StringArray{
								pulumi.String("true"),
							},
						},
					},
				},
			},
		}, nil)
		_, err = mediastore.NewContainerPolicy(ctx, "example", &mediastore.ContainerPolicyArgs{
			ContainerName: exampleContainer.Name,
			Policy: pulumi.String(example.ApplyT(func(example iam.GetPolicyDocumentResult) (*string, error) {
				return &example.Json, nil
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
import com.pulumi.aws.inputs.GetRegionArgs;
import com.pulumi.aws.inputs.GetCallerIdentityArgs;
import com.pulumi.aws.mediastore.Container;
import com.pulumi.aws.mediastore.ContainerArgs;
import com.pulumi.aws.iam.IamFunctions;
import com.pulumi.aws.iam.inputs.GetPolicyDocumentArgs;
import com.pulumi.aws.mediastore.ContainerPolicy;
import com.pulumi.aws.mediastore.ContainerPolicyArgs;
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

        final var currentGetCallerIdentity = AwsFunctions.getCallerIdentity();

        var exampleContainer = new Container("exampleContainer", ContainerArgs.builder()
            .name("example")
            .build());

        final var example = IamFunctions.getPolicyDocument(GetPolicyDocumentArgs.builder()
            .statements(GetPolicyDocumentStatementArgs.builder()
                .sid("MediaStoreFullAccess")
                .effect("Allow")
                .principals(GetPolicyDocumentStatementPrincipalArgs.builder()
                    .type("AWS")
                    .identifiers(String.format("arn:aws:iam::%s:root", currentGetCallerIdentity.applyValue(getCallerIdentityResult -> getCallerIdentityResult.accountId())))
                    .build())
                .actions("mediastore:*")
                .resources(exampleContainer.name().applyValue(name -> String.format("arn:aws:mediastore:%s:%s:container/%s/*", current.applyValue(getRegionResult -> getRegionResult.name()),currentGetCallerIdentity.applyValue(getCallerIdentityResult -> getCallerIdentityResult.accountId()),name)))
                .conditions(GetPolicyDocumentStatementConditionArgs.builder()
                    .test("Bool")
                    .variable("aws:SecureTransport")
                    .values("true")
                    .build())
                .build())
            .build());

        var exampleContainerPolicy = new ContainerPolicy("exampleContainerPolicy", ContainerPolicyArgs.builder()
            .containerName(exampleContainer.name())
            .policy(example.applyValue(getPolicyDocumentResult -> getPolicyDocumentResult).applyValue(example -> example.applyValue(getPolicyDocumentResult -> getPolicyDocumentResult.json())))
            .build());

    }
}
```
```yaml
resources:
  exampleContainer:
    type: aws:mediastore:Container
    name: example
    properties:
      name: example
  exampleContainerPolicy:
    type: aws:mediastore:ContainerPolicy
    name: example
    properties:
      containerName: ${exampleContainer.name}
      policy: ${example.json}
variables:
  current:
    fn::invoke:
      function: aws:getRegion
      arguments: {}
  currentGetCallerIdentity:
    fn::invoke:
      function: aws:getCallerIdentity
      arguments: {}
  example:
    fn::invoke:
      function: aws:iam:getPolicyDocument
      arguments:
        statements:
          - sid: MediaStoreFullAccess
            effect: Allow
            principals:
              - type: AWS
                identifiers:
                  - arn:aws:iam::${currentGetCallerIdentity.accountId}:root
            actions:
              - mediastore:*
            resources:
              - arn:aws:mediastore:${current.name}:${currentGetCallerIdentity.accountId}:container/${exampleContainer.name}/*
            conditions:
              - test: Bool
                variable: aws:SecureTransport
                values:
                  - 'true'
```
<!--End PulumiCodeChooser -->

## Import

Using `pulumi import`, import MediaStore Container Policy using the MediaStore Container Name. For example:

```sh
$ pulumi import aws:mediastore/containerPolicy:ContainerPolicy example example
```
0
containerName" The name of the container.
*
policy" The contents of the policy.
"0
containerName" The name of the container.
"*
policy" The contents of the policy.
*�
%
memorydbAclaws:memorydb/acl:Acl�Provides a MemoryDB ACL.

More information about users and ACL-s can be found in the [MemoryDB User Guide](https://docs.aws.amazon.com/memorydb/latest/devguide/clusters.acls.html).

## Example Usage

<!--Start PulumiCodeChooser -->
```typescript
import * as pulumi from "@pulumi/pulumi";
import * as aws from "@pulumi/aws";

const example = new aws.memorydb.Acl("example", {
    name: "my-acl",
    userNames: [
        "my-user-1",
        "my-user-2",
    ],
});
```
```python
import pulumi
import pulumi_aws as aws

example = aws.memorydb.Acl("example",
    name="my-acl",
    user_names=[
        "my-user-1",
        "my-user-2",
    ])
```
```csharp
using System.Collections.Generic;
using System.Linq;
using Pulumi;
using Aws = Pulumi.Aws;

return await Deployment.RunAsync(() => 
{
    var example = new Aws.MemoryDb.Acl("example", new()
    {
        Name = "my-acl",
        UserNames = new[]
        {
            "my-user-1",
            "my-user-2",
        },
    });

});
```
```go
package main

import (
	"github.com/pulumi/pulumi-aws/sdk/v6/go/aws/memorydb"
	"github.com/pulumi/pulumi/sdk/v3/go/pulumi"
)

func main() {
	pulumi.Run(func(ctx *pulumi.Context) error {
		_, err := memorydb.NewAcl(ctx, "example", &memorydb.AclArgs{
			Name: pulumi.String("my-acl"),
			UserNames: pulumi.StringArray{
				pulumi.String("my-user-1"),
				pulumi.String("my-user-2"),
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
import com.pulumi.aws.memorydb.Acl;
import com.pulumi.aws.memorydb.AclArgs;
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
        var example = new Acl("example", AclArgs.builder()
            .name("my-acl")
            .userNames(            
                "my-user-1",
                "my-user-2")
            .build());

    }
}
```
```yaml
resources:
  example:
    type: aws:memorydb:Acl
    properties:
      name: my-acl
      userNames:
        - my-user-1
        - my-user-2
```
<!--End PulumiCodeChooser -->

## Import

Using `pulumi import`, import an ACL using the `name`. For example:

```sh
$ pulumi import aws:memorydb/acl:Acl example my-acl
```
y
nameB" kName of the ACL. If omitted, the provider will assign a random, unique name. Conflicts with `name_prefix`.
f

namePrefixB" RCreates a unique name beginning with the specified prefix. Conflicts with `name`.
�
tagsB2" �A map of tags to assign to the resource. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
L
	userNamesB*" 7Set of MemoryDB user names to be included in this ACL.
"
arn" The ARN of the ACL.
"M
minimumEngineVersion" 1The minimum engine version supported by the ACL.
"w
name" kName of the ACL. If omitted, the provider will assign a random, unique name. Conflicts with `name_prefix`.
"d

namePrefix" RCreates a unique name beginning with the specified prefix. Conflicts with `name`.
"�
tagsB2" �A map of tags to assign to the resource. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
"�
tagsAll2" xA map of tags assigned to the resource, including those inherited from the provider `default_tags` configuration block.
"L
	userNamesB*" 7Set of MemoryDB user names to be included in this ACL.
*�]
1
memorydbClusteraws:memorydb/cluster:Cluster�Provides a MemoryDB Cluster.

More information about MemoryDB can be found in the [Developer Guide](https://docs.aws.amazon.com/memorydb/latest/devguide/what-is-memorydb-for-redis.html).

## Example Usage

<!--Start PulumiCodeChooser -->
```typescript
import * as pulumi from "@pulumi/pulumi";
import * as aws from "@pulumi/aws";

const example = new aws.memorydb.Cluster("example", {
    aclName: "open-access",
    name: "my-cluster",
    nodeType: "db.t4g.small",
    engine: "redis",
    engineVersion: "7.1",
    numShards: 2,
    securityGroupIds: [exampleAwsSecurityGroup.id],
    snapshotRetentionLimit: 7,
    subnetGroupName: exampleAwsMemorydbSubnetGroup.id,
});
```
```python
import pulumi
import pulumi_aws as aws

example = aws.memorydb.Cluster("example",
    acl_name="open-access",
    name="my-cluster",
    node_type="db.t4g.small",
    engine="redis",
    engine_version="7.1",
    num_shards=2,
    security_group_ids=[example_aws_security_group["id"]],
    snapshot_retention_limit=7,
    subnet_group_name=example_aws_memorydb_subnet_group["id"])
```
```csharp
using System.Collections.Generic;
using System.Linq;
using Pulumi;
using Aws = Pulumi.Aws;

return await Deployment.RunAsync(() => 
{
    var example = new Aws.MemoryDb.Cluster("example", new()
    {
        AclName = "open-access",
        Name = "my-cluster",
        NodeType = "db.t4g.small",
        Engine = "redis",
        EngineVersion = "7.1",
        NumShards = 2,
        SecurityGroupIds = new[]
        {
            exampleAwsSecurityGroup.Id,
        },
        SnapshotRetentionLimit = 7,
        SubnetGroupName = exampleAwsMemorydbSubnetGroup.Id,
    });

});
```
```go
package main

import (
	"github.com/pulumi/pulumi-aws/sdk/v6/go/aws/memorydb"
	"github.com/pulumi/pulumi/sdk/v3/go/pulumi"
)

func main() {
	pulumi.Run(func(ctx *pulumi.Context) error {
		_, err := memorydb.NewCluster(ctx, "example", &memorydb.ClusterArgs{
			AclName:       pulumi.String("open-access"),
			Name:          pulumi.String("my-cluster"),
			NodeType:      pulumi.String("db.t4g.small"),
			Engine:        pulumi.String("redis"),
			EngineVersion: pulumi.String("7.1"),
			NumShards:     pulumi.Int(2),
			SecurityGroupIds: pulumi.StringArray{
				exampleAwsSecurityGroup.Id,
			},
			SnapshotRetentionLimit: pulumi.Int(7),
			SubnetGroupName:        pulumi.Any(exampleAwsMemorydbSubnetGroup.Id),
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
import com.pulumi.aws.memorydb.Cluster;
import com.pulumi.aws.memorydb.ClusterArgs;
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
        var example = new Cluster("example", ClusterArgs.builder()
            .aclName("open-access")
            .name("my-cluster")
            .nodeType("db.t4g.small")
            .engine("redis")
            .engineVersion("7.1")
            .numShards(2)
            .securityGroupIds(exampleAwsSecurityGroup.id())
            .snapshotRetentionLimit(7)
            .subnetGroupName(exampleAwsMemorydbSubnetGroup.id())
            .build());

    }
}
```
```yaml
resources:
  example:
    type: aws:memorydb:Cluster
    properties:
      aclName: open-access
      name: my-cluster
      nodeType: db.t4g.small
      engine: redis
      engineVersion: '7.1'
      numShards: 2
      securityGroupIds:
        - ${exampleAwsSecurityGroup.id}
      snapshotRetentionLimit: 7
      subnetGroupName: ${exampleAwsMemorydbSubnetGroup.id}
```
<!--End PulumiCodeChooser -->

## Import

Using `pulumi import`, import a cluster using the `name`. For example:

```sh
$ pulumi import aws:memorydb/cluster:Cluster example my-cluster
```
R
aclName" CThe name of the Access Control List to associate with the cluster.
�
autoMinorVersionUpgradeB
 {When set to `true`, the cluster will automatically receive minor engine version upgrades after launch. Defaults to `true`.
�
dataTieringB
 �Enables data tiering. This option is not supported by all instance types. For more information, see [Data tiering](https://docs.aws.amazon.com/memorydb/latest/devguide/data-tiering.html).
U
descriptionB" @Description for the cluster. Defaults to `"Managed by Pulumi"`.
c
engineB" SThe engine that will run on your nodes. Supported values are `redis` and `valkey`.
n
engineVersionB" WVersion number of the engine to be used for the cluster. Downgrades are not supported.
�
finalSnapshotNameB" |Name of the final cluster snapshot to be created when this resource is deleted. If omitted, no final snapshot will be made.
K
	kmsKeyArnB" 8ARN of the KMS key used to encrypt the cluster at rest.
�
maintenanceWindowB" �Specifies the weekly time range during which maintenance on the cluster is performed. Specify as a range in the format `ddd:hh24:mi-ddd:hh24:mi` (24H Clock UTC). The minimum maintenance window is a 60 minute period. Example: `sun:23:00-mon:01:30`.
t
multiRegionClusterNameB" TThe multi region cluster identifier specified on `aws.memorydb.MultiRegionCluster`.
}
nameB" oName of the cluster. If omitted, the provider will assign a random, unique name. Conflicts with `name_prefix`.
f

namePrefixB" RCreates a unique name beginning with the specified prefix. Conflicts with `name`.
�
nodeType" �The compute and memory capacity of the nodes in the cluster. See AWS documentation on [supported node types](https://docs.aws.amazon.com/memorydb/latest/devguide/nodes.supportedtypes.html) as well as [vertical scaling](https://docs.aws.amazon.com/memorydb/latest/devguide/cluster-vertical-scaling.html).

The following arguments are optional:
�
numReplicasPerShardB oThe number of replicas to apply to each shard, up to a maximum of 5. Defaults to `1` (i.e. 2 nodes per shard).
I
	numShardsB 6The number of shards in the cluster. Defaults to `1`.
Y
parameterGroupNameB" =The name of the parameter group associated with the cluster.
b
portB TThe port number on which each of the nodes accepts connections. Defaults to `6379`.
[
securityGroupIdsB*" ?Set of VPC Security Group ID-s to associate with this cluster.
�
snapshotArnsB*" �List of ARN-s that uniquely identify RDB snapshot files stored in S3. The snapshot files will be used to populate the new cluster. Object names in the ARN-s cannot contain any commas.
^
snapshotNameB" HThe name of a snapshot from which to restore data into the new cluster.
�
snapshotRetentionLimitB �The number of days for which MemoryDB retains automatic snapshots before deleting them. When set to `0`, automatic backups are disabled. Defaults to `0`.
�
snapshotWindowB" zThe daily time range (in UTC) during which MemoryDB begins taking a daily snapshot of your shard. Example: `05:00-09:00`.
S
snsTopicArnB" >ARN of the SNS topic to which cluster notifications are sent.
�
subnetGroupNameB" wThe name of the subnet group to be used for the cluster. Defaults to a subnet group consisting of default VPC subnets.
�
tagsB2" �A map of tags to assign to the resource. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
�

tlsEnabledB
 �A flag to enable in-transit encryption on the cluster. When set to `false`, the `acl_name` must be `open-access`. Defaults to `true`.
"R
aclName" CThe name of the Access Control List to associate with the cluster.
"#
arn" The ARN of the cluster.
"�
autoMinorVersionUpgradeB
 {When set to `true`, the cluster will automatically receive minor engine version upgrades after launch. Defaults to `true`.
"x
clusterEndpointsd*b:`
^
memorydbClusterClusterEndpoint:aws:memorydb/ClusterClusterEndpoint:ClusterClusterEndpoint"�
dataTieringB
 �Enables data tiering. This option is not supported by all instance types. For more information, see [Data tiering](https://docs.aws.amazon.com/memorydb/latest/devguide/data-tiering.html).
"U
descriptionB" @Description for the cluster. Defaults to `"Managed by Pulumi"`.
"a
engine" SThe engine that will run on your nodes. Supported values are `redis` and `valkey`.
"R
enginePatchVersion" 8Patch version number of the engine used by the cluster.
"l
engineVersion" WVersion number of the engine to be used for the cluster. Downgrades are not supported.
"�
finalSnapshotNameB" |Name of the final cluster snapshot to be created when this resource is deleted. If omitted, no final snapshot will be made.
"K
	kmsKeyArnB" 8ARN of the KMS key used to encrypt the cluster at rest.
"�
maintenanceWindow" �Specifies the weekly time range during which maintenance on the cluster is performed. Specify as a range in the format `ddd:hh24:mi-ddd:hh24:mi` (24H Clock UTC). The minimum maintenance window is a 60 minute period. Example: `sun:23:00-mon:01:30`.
"t
multiRegionClusterNameB" TThe multi region cluster identifier specified on `aws.memorydb.MultiRegionCluster`.
"{
name" oName of the cluster. If omitted, the provider will assign a random, unique name. Conflicts with `name_prefix`.
"d

namePrefix" RCreates a unique name beginning with the specified prefix. Conflicts with `name`.
"�
nodeType" �The compute and memory capacity of the nodes in the cluster. See AWS documentation on [supported node types](https://docs.aws.amazon.com/memorydb/latest/devguide/nodes.supportedtypes.html) as well as [vertical scaling](https://docs.aws.amazon.com/memorydb/latest/devguide/cluster-vertical-scaling.html).

The following arguments are optional:
"�
numReplicasPerShardB oThe number of replicas to apply to each shard, up to a maximum of 5. Defaults to `1` (i.e. 2 nodes per shard).
"I
	numShardsB 6The number of shards in the cluster. Defaults to `1`.
"W
parameterGroupName" =The name of the parameter group associated with the cluster.
"`
port TThe port number on which each of the nodes accepts connections. Defaults to `6379`.
"[
securityGroupIdsB*" ?Set of VPC Security Group ID-s to associate with this cluster.
"q
shardsF*D:B
@
memorydbClusterShard&aws:memorydb/ClusterShard:ClusterShardSet of shards in this cluster.
"�
snapshotArnsB*" �List of ARN-s that uniquely identify RDB snapshot files stored in S3. The snapshot files will be used to populate the new cluster. Object names in the ARN-s cannot contain any commas.
"^
snapshotNameB" HThe name of a snapshot from which to restore data into the new cluster.
"�
snapshotRetentionLimit �The number of days for which MemoryDB retains automatic snapshots before deleting them. When set to `0`, automatic backups are disabled. Defaults to `0`.
"�
snapshotWindow" zThe daily time range (in UTC) during which MemoryDB begins taking a daily snapshot of your shard. Example: `05:00-09:00`.
"S
snsTopicArnB" >ARN of the SNS topic to which cluster notifications are sent.
"�
subnetGroupName" wThe name of the subnet group to be used for the cluster. Defaults to a subnet group consisting of default VPC subnets.
"�
tagsB2" �A map of tags to assign to the resource. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
"�
tagsAll2" xA map of tags assigned to the resource, including those inherited from the provider `default_tags` configuration block.
"�

tlsEnabledB
 �A flag to enable in-transit encryption on the cluster. When set to `false`, the `acl_name` must be `open-access`. Defaults to `true`.
*�B
R
memorydbMultiRegionCluster2aws:memorydb/multiRegionCluster:MultiRegionCluster�,Provides a MemoryDB Multi Region Cluster.

More information about MemoryDB can be found in the [Developer Guide](https://docs.aws.amazon.com/memorydb/latest/devguide/what-is-memorydb-for-redis.html).

## Example Usage

<!--Start PulumiCodeChooser -->
```typescript
import * as pulumi from "@pulumi/pulumi";
import * as aws from "@pulumi/aws";

const example = new aws.memorydb.MultiRegionCluster("example", {
    multiRegionClusterNameSuffix: "example",
    nodeType: "db.r7g.xlarge",
});
const exampleCluster = new aws.memorydb.Cluster("example", {
    aclName: exampleAwsMemorydbAcl.id,
    autoMinorVersionUpgrade: false,
    name: "example",
    nodeType: "db.t4g.small",
    numShards: 2,
    securityGroupIds: [exampleAwsSecurityGroup.id],
    snapshotRetentionLimit: 7,
    subnetGroupName: exampleAwsMemorydbSubnetGroup.id,
    multiRegionClusterName: example.multiRegionClusterName,
});
```
```python
import pulumi
import pulumi_aws as aws

example = aws.memorydb.MultiRegionCluster("example",
    multi_region_cluster_name_suffix="example",
    node_type="db.r7g.xlarge")
example_cluster = aws.memorydb.Cluster("example",
    acl_name=example_aws_memorydb_acl["id"],
    auto_minor_version_upgrade=False,
    name="example",
    node_type="db.t4g.small",
    num_shards=2,
    security_group_ids=[example_aws_security_group["id"]],
    snapshot_retention_limit=7,
    subnet_group_name=example_aws_memorydb_subnet_group["id"],
    multi_region_cluster_name=example.multi_region_cluster_name)
```
```csharp
using System.Collections.Generic;
using System.Linq;
using Pulumi;
using Aws = Pulumi.Aws;

return await Deployment.RunAsync(() => 
{
    var example = new Aws.MemoryDb.MultiRegionCluster("example", new()
    {
        MultiRegionClusterNameSuffix = "example",
        NodeType = "db.r7g.xlarge",
    });

    var exampleCluster = new Aws.MemoryDb.Cluster("example", new()
    {
        AclName = exampleAwsMemorydbAcl.Id,
        AutoMinorVersionUpgrade = false,
        Name = "example",
        NodeType = "db.t4g.small",
        NumShards = 2,
        SecurityGroupIds = new[]
        {
            exampleAwsSecurityGroup.Id,
        },
        SnapshotRetentionLimit = 7,
        SubnetGroupName = exampleAwsMemorydbSubnetGroup.Id,
        MultiRegionClusterName = example.MultiRegionClusterName,
    });

});
```
```go
package main

import (
	"github.com/pulumi/pulumi-aws/sdk/v6/go/aws/memorydb"
	"github.com/pulumi/pulumi/sdk/v3/go/pulumi"
)

func main() {
	pulumi.Run(func(ctx *pulumi.Context) error {
		example, err := memorydb.NewMultiRegionCluster(ctx, "example", &memorydb.MultiRegionClusterArgs{
			MultiRegionClusterNameSuffix: pulumi.String("example"),
			NodeType:                     pulumi.String("db.r7g.xlarge"),
		})
		if err != nil {
			return err
		}
		_, err = memorydb.NewCluster(ctx, "example", &memorydb.ClusterArgs{
			AclName:                 pulumi.Any(exampleAwsMemorydbAcl.Id),
			AutoMinorVersionUpgrade: pulumi.Bool(false),
			Name:                    pulumi.String("example"),
			NodeType:                pulumi.String("db.t4g.small"),
			NumShards:               pulumi.Int(2),
			SecurityGroupIds: pulumi.StringArray{
				exampleAwsSecurityGroup.Id,
			},
			SnapshotRetentionLimit: pulumi.Int(7),
			SubnetGroupName:        pulumi.Any(exampleAwsMemorydbSubnetGroup.Id),
			MultiRegionClusterName: example.MultiRegionClusterName,
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
import com.pulumi.aws.memorydb.MultiRegionCluster;
import com.pulumi.aws.memorydb.MultiRegionClusterArgs;
import com.pulumi.aws.memorydb.Cluster;
import com.pulumi.aws.memorydb.ClusterArgs;
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
        var example = new MultiRegionCluster("example", MultiRegionClusterArgs.builder()
            .multiRegionClusterNameSuffix("example")
            .nodeType("db.r7g.xlarge")
            .build());

        var exampleCluster = new Cluster("exampleCluster", ClusterArgs.builder()
            .aclName(exampleAwsMemorydbAcl.id())
            .autoMinorVersionUpgrade(false)
            .name("example")
            .nodeType("db.t4g.small")
            .numShards(2)
            .securityGroupIds(exampleAwsSecurityGroup.id())
            .snapshotRetentionLimit(7)
            .subnetGroupName(exampleAwsMemorydbSubnetGroup.id())
            .multiRegionClusterName(example.multiRegionClusterName())
            .build());

    }
}
```
```yaml
resources:
  example:
    type: aws:memorydb:MultiRegionCluster
    properties:
      multiRegionClusterNameSuffix: example
      nodeType: db.r7g.xlarge
  exampleCluster:
    type: aws:memorydb:Cluster
    name: example
    properties:
      aclName: ${exampleAwsMemorydbAcl.id}
      autoMinorVersionUpgrade: false
      name: example
      nodeType: db.t4g.small
      numShards: 2
      securityGroupIds:
        - ${exampleAwsSecurityGroup.id}
      snapshotRetentionLimit: 7
      subnetGroupName: ${exampleAwsMemorydbSubnetGroup.id}
      multiRegionClusterName: ${example.multiRegionClusterName}
```
<!--End PulumiCodeChooser -->

## Import

Using `pulumi import`, import a cluster using the `multi_region_cluster_name`. For example:

```sh
$ pulumi import aws:memorydb/multiRegionCluster:MultiRegionCluster example virxk-example
```
?
descriptionB" *description for the multi-region cluster.
w
engineB" gThe name of the engine to be used for the multi-region cluster. Valid values are `redis` and `valkey`.
x
engineVersionB" aThe version of the engine to be used for the multi-region cluster. Downgrades are not supported.
�
multiRegionClusterNameSuffix" �A suffix to be added to the multi-region cluster name. An AWS generated prefix is automatically applied to the multi-region cluster name when it is created.
w
multiRegionParameterGroupNameB" PThe name of the multi-region parameter group to be associated with the cluster.
n
nodeType" ^The node type to be used for the multi-region cluster.

The following arguments are optional:
F
	numShardsB 3The number of shards for the multi-region cluster.
�
tagsB2" �A map of tags to assign to the resource. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
|
timeoutspBn:l
j
memorydbMultiRegionClusterTimeoutsBaws:memorydb/MultiRegionClusterTimeouts:MultiRegionClusterTimeoutsK

tlsEnabledB
 7A flag to enable in-transit encryption on the cluster.

updateStrategyB" "0
arn" %The ARN of the multi-region cluster.
"?
descriptionB" *description for the multi-region cluster.
"u
engine" gThe name of the engine to be used for the multi-region cluster. Valid values are `redis` and `valkey`.
"v
engineVersion" aThe version of the engine to be used for the multi-region cluster. Downgrades are not supported.
"D
multiRegionClusterName" &The name of the multi-region cluster.
"�
multiRegionClusterNameSuffix" �A suffix to be added to the multi-region cluster name. An AWS generated prefix is automatically applied to the multi-region cluster name when it is created.
"u
multiRegionParameterGroupName" PThe name of the multi-region parameter group to be associated with the cluster.
"n
nodeType" ^The node type to be used for the multi-region cluster.

The following arguments are optional:
"D
	numShards 3The number of shards for the multi-region cluster.
"
status" "�
tagsB2" �A map of tags to assign to the resource. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
"�
tagsAll2" xA map of tags assigned to the resource, including those inherited from the provider `default_tags` configuration block.
"|
timeoutspBn:l
j
memorydbMultiRegionClusterTimeoutsBaws:memorydb/MultiRegionClusterTimeouts:MultiRegionClusterTimeouts"I

tlsEnabled
 7A flag to enable in-transit encryption on the cluster.
"
updateStrategyB" *�*
F
memorydbParameterGroup*aws:memorydb/parameterGroup:ParameterGroup�Provides a MemoryDB Parameter Group.

More information about parameter groups can be found in the [MemoryDB User Guide](https://docs.aws.amazon.com/memorydb/latest/devguide/parametergroups.html).

## Example Usage

<!--Start PulumiCodeChooser -->
```typescript
import * as pulumi from "@pulumi/pulumi";
import * as aws from "@pulumi/aws";

const example = new aws.memorydb.ParameterGroup("example", {
    name: "my-parameter-group",
    family: "memorydb_redis6",
    parameters: [{
        name: "activedefrag",
        value: "yes",
    }],
});
```
```python
import pulumi
import pulumi_aws as aws

example = aws.memorydb.ParameterGroup("example",
    name="my-parameter-group",
    family="memorydb_redis6",
    parameters=[{
        "name": "activedefrag",
        "value": "yes",
    }])
```
```csharp
using System.Collections.Generic;
using System.Linq;
using Pulumi;
using Aws = Pulumi.Aws;

return await Deployment.RunAsync(() => 
{
    var example = new Aws.MemoryDb.ParameterGroup("example", new()
    {
        Name = "my-parameter-group",
        Family = "memorydb_redis6",
        Parameters = new[]
        {
            new Aws.MemoryDb.Inputs.ParameterGroupParameterArgs
            {
                Name = "activedefrag",
                Value = "yes",
            },
        },
    });

});
```
```go
package main

import (
	"github.com/pulumi/pulumi-aws/sdk/v6/go/aws/memorydb"
	"github.com/pulumi/pulumi/sdk/v3/go/pulumi"
)

func main() {
	pulumi.Run(func(ctx *pulumi.Context) error {
		_, err := memorydb.NewParameterGroup(ctx, "example", &memorydb.ParameterGroupArgs{
			Name:   pulumi.String("my-parameter-group"),
			Family: pulumi.String("memorydb_redis6"),
			Parameters: memorydb.ParameterGroupParameterArray{
				&memorydb.ParameterGroupParameterArgs{
					Name:  pulumi.String("activedefrag"),
					Value: pulumi.String("yes"),
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
import com.pulumi.aws.memorydb.ParameterGroup;
import com.pulumi.aws.memorydb.ParameterGroupArgs;
import com.pulumi.aws.memorydb.inputs.ParameterGroupParameterArgs;
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
        var example = new ParameterGroup("example", ParameterGroupArgs.builder()
            .name("my-parameter-group")
            .family("memorydb_redis6")
            .parameters(ParameterGroupParameterArgs.builder()
                .name("activedefrag")
                .value("yes")
                .build())
            .build());

    }
}
```
```yaml
resources:
  example:
    type: aws:memorydb:ParameterGroup
    properties:
      name: my-parameter-group
      family: memorydb_redis6
      parameters:
        - name: activedefrag
          value: yes
```
<!--End PulumiCodeChooser -->

## Import

Using `pulumi import`, import a parameter group using the `name`. For example:

```sh
$ pulumi import aws:memorydb/parameterGroup:ParameterGroup example my-parameter-group
```
]
descriptionB" HDescription for the parameter group. Defaults to `"Managed by Pulumi"`.
s
family" eThe engine version that the parameter group can be used with.

The following arguments are optional:
�
nameB" wName of the parameter group. If omitted, the provider will assign a random, unique name. Conflicts with `name_prefix`.
f

namePrefixB" RCreates a unique name beginning with the specified prefix. Conflicts with `name`.
�

parametersiBg*e:c
a
memorydbParameterGroupParameter<aws:memorydb/ParameterGroupParameter:ParameterGroupParameter{Set of MemoryDB parameters to apply. Any parameters not specified will fall back to their family defaults. Detailed below.
�
tagsB2" �A map of tags to assign to the resource. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
"+
arn"  The ARN of the parameter group.
"]
descriptionB" HDescription for the parameter group. Defaults to `"Managed by Pulumi"`.
"s
family" eThe engine version that the parameter group can be used with.

The following arguments are optional:
"�
name" wName of the parameter group. If omitted, the provider will assign a random, unique name. Conflicts with `name_prefix`.
"d

namePrefix" RCreates a unique name beginning with the specified prefix. Conflicts with `name`.
"�

parametersiBg*e:c
a
memorydbParameterGroupParameter<aws:memorydb/ParameterGroupParameter:ParameterGroupParameter{Set of MemoryDB parameters to apply. Any parameters not specified will fall back to their family defaults. Detailed below.
"�
tagsB2" �A map of tags to assign to the resource. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
"�
tagsAll2" xA map of tags assigned to the resource, including those inherited from the provider `default_tags` configuration block.
*� 
4
memorydbSnapshotaws:memorydb/snapshot:Snapshot�Provides a MemoryDB Snapshot.

More information about snapshot and restore can be found in the [MemoryDB User Guide](https://docs.aws.amazon.com/memorydb/latest/devguide/snapshots.html).

## Example Usage

<!--Start PulumiCodeChooser -->
```typescript
import * as pulumi from "@pulumi/pulumi";
import * as aws from "@pulumi/aws";

const example = new aws.memorydb.Snapshot("example", {
    clusterName: exampleAwsMemorydbCluster.name,
    name: "my-snapshot",
});
```
```python
import pulumi
import pulumi_aws as aws

example = aws.memorydb.Snapshot("example",
    cluster_name=example_aws_memorydb_cluster["name"],
    name="my-snapshot")
```
```csharp
using System.Collections.Generic;
using System.Linq;
using Pulumi;
using Aws = Pulumi.Aws;

return await Deployment.RunAsync(() => 
{
    var example = new Aws.MemoryDb.Snapshot("example", new()
    {
        ClusterName = exampleAwsMemorydbCluster.Name,
        Name = "my-snapshot",
    });

});
```
```go
package main

import (
	"github.com/pulumi/pulumi-aws/sdk/v6/go/aws/memorydb"
	"github.com/pulumi/pulumi/sdk/v3/go/pulumi"
)

func main() {
	pulumi.Run(func(ctx *pulumi.Context) error {
		_, err := memorydb.NewSnapshot(ctx, "example", &memorydb.SnapshotArgs{
			ClusterName: pulumi.Any(exampleAwsMemorydbCluster.Name),
			Name:        pulumi.String("my-snapshot"),
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
import com.pulumi.aws.memorydb.Snapshot;
import com.pulumi.aws.memorydb.SnapshotArgs;
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
        var example = new Snapshot("example", SnapshotArgs.builder()
            .clusterName(exampleAwsMemorydbCluster.name())
            .name("my-snapshot")
            .build());

    }
}
```
```yaml
resources:
  example:
    type: aws:memorydb:Snapshot
    properties:
      clusterName: ${exampleAwsMemorydbCluster.name}
      name: my-snapshot
```
<!--End PulumiCodeChooser -->

## Import

Using `pulumi import`, import a snapshot using the `name`. For example:

```sh
$ pulumi import aws:memorydb/snapshot:Snapshot example my-snapshot
```
G
clusterName" 4Name of the MemoryDB cluster to take a snapshot of.
L
	kmsKeyArnB" 9ARN of the KMS key used to encrypt the snapshot at rest.
~
nameB" pName of the snapshot. If omitted, the provider will assign a random, unique name. Conflicts with `name_prefix`.
f

namePrefixB" RCreates a unique name beginning with the specified prefix. Conflicts with `name`.
�
tagsB2" �A map of tags to assign to the resource. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
"$
arn" The ARN of the snapshot.
"�
clusterConfigurationsv*t:r
p
memorydbSnapshotClusterConfigurationFaws:memorydb/SnapshotClusterConfiguration:SnapshotClusterConfigurationDThe configuration of the cluster from which the snapshot was taken.
"G
clusterName" 4Name of the MemoryDB cluster to take a snapshot of.
"L
	kmsKeyArnB" 9ARN of the KMS key used to encrypt the snapshot at rest.
"|
name" pName of the snapshot. If omitted, the provider will assign a random, unique name. Conflicts with `name_prefix`.
"d

namePrefix" RCreates a unique name beginning with the specified prefix. Conflicts with `name`.
"{
source" mIndicates whether the snapshot is from an automatic backup (`automated`) or was created manually (`manual`).
"�
tagsB2" �A map of tags to assign to the resource. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
"�
tagsAll2" xA map of tags assigned to the resource, including those inherited from the provider `default_tags` configuration block.
*�/
=
memorydbSubnetGroup$aws:memorydb/subnetGroup:SubnetGroup�"Provides a MemoryDB Subnet Group.

More information about subnet groups can be found in the [MemoryDB User Guide](https://docs.aws.amazon.com/memorydb/latest/devguide/subnetgroups.html).

## Example Usage

<!--Start PulumiCodeChooser -->
```typescript
import * as pulumi from "@pulumi/pulumi";
import * as aws from "@pulumi/aws";

const example = new aws.ec2.Vpc("example", {cidrBlock: "10.0.0.0/16"});
const exampleSubnet = new aws.ec2.Subnet("example", {
    vpcId: example.id,
    cidrBlock: "10.0.0.0/24",
    availabilityZone: "us-west-2a",
});
const exampleSubnetGroup = new aws.memorydb.SubnetGroup("example", {
    name: "my-subnet-group",
    subnetIds: [exampleSubnet.id],
});
```
```python
import pulumi
import pulumi_aws as aws

example = aws.ec2.Vpc("example", cidr_block="10.0.0.0/16")
example_subnet = aws.ec2.Subnet("example",
    vpc_id=example.id,
    cidr_block="10.0.0.0/24",
    availability_zone="us-west-2a")
example_subnet_group = aws.memorydb.SubnetGroup("example",
    name="my-subnet-group",
    subnet_ids=[example_subnet.id])
```
```csharp
using System.Collections.Generic;
using System.Linq;
using Pulumi;
using Aws = Pulumi.Aws;

return await Deployment.RunAsync(() => 
{
    var example = new Aws.Ec2.Vpc("example", new()
    {
        CidrBlock = "10.0.0.0/16",
    });

    var exampleSubnet = new Aws.Ec2.Subnet("example", new()
    {
        VpcId = example.Id,
        CidrBlock = "10.0.0.0/24",
        AvailabilityZone = "us-west-2a",
    });

    var exampleSubnetGroup = new Aws.MemoryDb.SubnetGroup("example", new()
    {
        Name = "my-subnet-group",
        SubnetIds = new[]
        {
            exampleSubnet.Id,
        },
    });

});
```
```go
package main

import (
	"github.com/pulumi/pulumi-aws/sdk/v6/go/aws/ec2"
	"github.com/pulumi/pulumi-aws/sdk/v6/go/aws/memorydb"
	"github.com/pulumi/pulumi/sdk/v3/go/pulumi"
)

func main() {
	pulumi.Run(func(ctx *pulumi.Context) error {
		example, err := ec2.NewVpc(ctx, "example", &ec2.VpcArgs{
			CidrBlock: pulumi.String("10.0.0.0/16"),
		})
		if err != nil {
			return err
		}
		exampleSubnet, err := ec2.NewSubnet(ctx, "example", &ec2.SubnetArgs{
			VpcId:            example.ID(),
			CidrBlock:        pulumi.String("10.0.0.0/24"),
			AvailabilityZone: pulumi.String("us-west-2a"),
		})
		if err != nil {
			return err
		}
		_, err = memorydb.NewSubnetGroup(ctx, "example", &memorydb.SubnetGroupArgs{
			Name: pulumi.String("my-subnet-group"),
			SubnetIds: pulumi.StringArray{
				exampleSubnet.ID(),
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
import com.pulumi.aws.ec2.Vpc;
import com.pulumi.aws.ec2.VpcArgs;
import com.pulumi.aws.ec2.Subnet;
import com.pulumi.aws.ec2.SubnetArgs;
import com.pulumi.aws.memorydb.SubnetGroup;
import com.pulumi.aws.memorydb.SubnetGroupArgs;
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
        var example = new Vpc("example", VpcArgs.builder()
            .cidrBlock("10.0.0.0/16")
            .build());

        var exampleSubnet = new Subnet("exampleSubnet", SubnetArgs.builder()
            .vpcId(example.id())
            .cidrBlock("10.0.0.0/24")
            .availabilityZone("us-west-2a")
            .build());

        var exampleSubnetGroup = new SubnetGroup("exampleSubnetGroup", SubnetGroupArgs.builder()
            .name("my-subnet-group")
            .subnetIds(exampleSubnet.id())
            .build());

    }
}
```
```yaml
resources:
  example:
    type: aws:ec2:Vpc
    properties:
      cidrBlock: 10.0.0.0/16
  exampleSubnet:
    type: aws:ec2:Subnet
    name: example
    properties:
      vpcId: ${example.id}
      cidrBlock: 10.0.0.0/24
      availabilityZone: us-west-2a
  exampleSubnetGroup:
    type: aws:memorydb:SubnetGroup
    name: example
    properties:
      name: my-subnet-group
      subnetIds:
        - ${exampleSubnet.id}
```
<!--End PulumiCodeChooser -->

## Import

Using `pulumi import`, import a subnet group using its `name`. For example:

```sh
$ pulumi import aws:memorydb/subnetGroup:SubnetGroup example my-subnet-group
```
Z
descriptionB" EDescription for the subnet group. Defaults to `"Managed by Pulumi"`.
�
nameB" tName of the subnet group. If omitted, the provider will assign a random, unique name. Conflicts with `name_prefix`.
f

namePrefixB" RCreates a unique name beginning with the specified prefix. Conflicts with `name`.
�
	subnetIds*" zSet of VPC Subnet ID-s for the subnet group. At least one subnet must be provided.

The following arguments are optional:
�
tagsB2" �A map of tags to assign to the resource. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
"(
arn" The ARN of the subnet group.
"Z
descriptionB" EDescription for the subnet group. Defaults to `"Managed by Pulumi"`.
"�
name" tName of the subnet group. If omitted, the provider will assign a random, unique name. Conflicts with `name_prefix`.
"d

namePrefix" RCreates a unique name beginning with the specified prefix. Conflicts with `name`.
"�
	subnetIds*" zSet of VPC Subnet ID-s for the subnet group. At least one subnet must be provided.

The following arguments are optional:
"�
tagsB2" �A map of tags to assign to the resource. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
"�
tagsAll2" xA map of tags assigned to the resource, including those inherited from the provider `default_tags` configuration block.
"7
vpcId" *The VPC in which the subnet group exists.
*�,
(
memorydbUseraws:memorydb/user:User�!Provides a MemoryDB User.

More information about users and ACL-s can be found in the [MemoryDB User Guide](https://docs.aws.amazon.com/memorydb/latest/devguide/clusters.acls.html).

> **Note:** All arguments including the username and passwords will be stored in the raw state as plain-text.
## Example Usage

<!--Start PulumiCodeChooser -->
```typescript
import * as pulumi from "@pulumi/pulumi";
import * as aws from "@pulumi/aws";
import * as random from "@pulumi/random";

const example = new random.index.Password("example", {length: 16});
const exampleUser = new aws.memorydb.User("example", {
    userName: "my-user",
    accessString: "on ~* &* +@all",
    authenticationMode: {
        type: "password",
        passwords: [example.result],
    },
});
```
```python
import pulumi
import pulumi_aws as aws
import pulumi_random as random

example = random.index.Password("example", length=16)
example_user = aws.memorydb.User("example",
    user_name="my-user",
    access_string="on ~* &* +@all",
    authentication_mode={
        "type": "password",
        "passwords": [example["result"]],
    })
```
```csharp
using System.Collections.Generic;
using System.Linq;
using Pulumi;
using Aws = Pulumi.Aws;
using Random = Pulumi.Random;

return await Deployment.RunAsync(() => 
{
    var example = new Random.Index.Password("example", new()
    {
        Length = 16,
    });

    var exampleUser = new Aws.MemoryDb.User("example", new()
    {
        UserName = "my-user",
        AccessString = "on ~* &* +@all",
        AuthenticationMode = new Aws.MemoryDb.Inputs.UserAuthenticationModeArgs
        {
            Type = "password",
            Passwords = new[]
            {
                example.Result,
            },
        },
    });

});
```
```go
package main

import (
	"github.com/pulumi/pulumi-aws/sdk/v6/go/aws/memorydb"
	"github.com/pulumi/pulumi-random/sdk/v4/go/random"
	"github.com/pulumi/pulumi/sdk/v3/go/pulumi"
)

func main() {
	pulumi.Run(func(ctx *pulumi.Context) error {
		example, err := random.NewPassword(ctx, "example", &random.PasswordArgs{
			Length: 16,
		})
		if err != nil {
			return err
		}
		_, err = memorydb.NewUser(ctx, "example", &memorydb.UserArgs{
			UserName:     pulumi.String("my-user"),
			AccessString: pulumi.String("on ~* &* +@all"),
			AuthenticationMode: &memorydb.UserAuthenticationModeArgs{
				Type: pulumi.String("password"),
				Passwords: pulumi.StringArray{
					example.Result,
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
import com.pulumi.random.password;
import com.pulumi.random.PasswordArgs;
import com.pulumi.aws.memorydb.User;
import com.pulumi.aws.memorydb.UserArgs;
import com.pulumi.aws.memorydb.inputs.UserAuthenticationModeArgs;
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
        var example = new Password("example", PasswordArgs.builder()
            .length(16)
            .build());

        var exampleUser = new User("exampleUser", UserArgs.builder()
            .userName("my-user")
            .accessString("on ~* &* +@all")
            .authenticationMode(UserAuthenticationModeArgs.builder()
                .type("password")
                .passwords(example.result())
                .build())
            .build());

    }
}
```
```yaml
resources:
  example:
    type: random:password
    properties:
      length: 16
  exampleUser:
    type: aws:memorydb:User
    name: example
    properties:
      userName: my-user
      accessString: on ~* &* +@all
      authenticationMode:
        type: password
        passwords:
          - ${example.result}
```
<!--End PulumiCodeChooser -->

## Import

Using `pulumi import`, import a user using the `user_name`. For example:

```sh
$ pulumi import aws:memorydb/user:User example my-user
```
The `passwords` are not available for imported resources, as this information cannot be read back from the MemoryDB API.

B
accessString" .Access permissions string used for this user.
�
authenticationModeb:`
^
memorydbUserAuthenticationMode:aws:memorydb/UserAuthenticationMode:UserAuthenticationMode>Denotes the user's authentication properties. Detailed below.
�
tagsB2" �A map of tags to assign to the resource. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
g
userName" WName of the MemoryDB user. Up to 40 characters.

The following arguments are optional:
"B
accessString" .Access permissions string used for this user.
"
arn" ARN of the user.
"�
authenticationModeb:`
^
memorydbUserAuthenticationMode:aws:memorydb/UserAuthenticationMode:UserAuthenticationMode>Denotes the user's authentication properties. Detailed below.
"K
minimumEngineVersion" /Minimum engine version supported for the user.
"�
tagsB2" �A map of tags to assign to the resource. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
"�
tagsAll2" xA map of tags assigned to the resource, including those inherited from the provider `default_tags` configuration block.
"g
userName" WName of the MemoryDB user. Up to 40 characters.

The following arguments are optional:
*��
"
mqBrokeraws:mq/broker:Broker��Provides an Amazon MQ broker resource. This resources also manages users for the broker.

> For more information on Amazon MQ, see [Amazon MQ documentation](https://docs.aws.amazon.com/amazon-mq/latest/developer-guide/welcome.html).

> **NOTE:** Amazon MQ currently places limits on **RabbitMQ** brokers. For example, a RabbitMQ broker cannot have: instances with an associated IP address of an ENI attached to the broker, an associated LDAP server to authenticate and authorize broker connections, storage type `EFS`, or audit logging. Although this resource allows you to create RabbitMQ users, RabbitMQ users cannot have console access or groups. Also, Amazon MQ does not return information about RabbitMQ users so drift detection is not possible.

> **NOTE:** Changes to an MQ Broker can occur when you change a parameter, such as `configuration` or `user`, and are reflected in the next maintenance window. Because of this, the provider may report a difference in its planning phase because a modification has not yet taken place. You can use the `apply_immediately` flag to instruct the service to apply the change immediately (see documentation below). Using `apply_immediately` can result in a brief downtime as the broker reboots.


## Example Usage

### Basic Example

<!--Start PulumiCodeChooser -->
```typescript
import * as pulumi from "@pulumi/pulumi";
import * as aws from "@pulumi/aws";

const example = new aws.mq.Broker("example", {
    brokerName: "example",
    configuration: {
        id: test.id,
        revision: test.latestRevision,
    },
    engineType: "ActiveMQ",
    engineVersion: "5.17.6",
    hostInstanceType: "mq.t2.micro",
    securityGroups: [testAwsSecurityGroup.id],
    users: [{
        username: "ExampleUser",
        password: "MindTheGap",
    }],
});
```
```python
import pulumi
import pulumi_aws as aws

example = aws.mq.Broker("example",
    broker_name="example",
    configuration={
        "id": test["id"],
        "revision": test["latestRevision"],
    },
    engine_type="ActiveMQ",
    engine_version="5.17.6",
    host_instance_type="mq.t2.micro",
    security_groups=[test_aws_security_group["id"]],
    users=[{
        "username": "ExampleUser",
        "password": "MindTheGap",
    }])
```
```csharp
using System.Collections.Generic;
using System.Linq;
using Pulumi;
using Aws = Pulumi.Aws;

return await Deployment.RunAsync(() => 
{
    var example = new Aws.Mq.Broker("example", new()
    {
        BrokerName = "example",
        Configuration = new Aws.Mq.Inputs.BrokerConfigurationArgs
        {
            Id = test.Id,
            Revision = test.LatestRevision,
        },
        EngineType = "ActiveMQ",
        EngineVersion = "5.17.6",
        HostInstanceType = "mq.t2.micro",
        SecurityGroups = new[]
        {
            testAwsSecurityGroup.Id,
        },
        Users = new[]
        {
            new Aws.Mq.Inputs.BrokerUserArgs
            {
                Username = "ExampleUser",
                Password = "MindTheGap",
            },
        },
    });

});
```
```go
package main

import (
	"github.com/pulumi/pulumi-aws/sdk/v6/go/aws/mq"
	"github.com/pulumi/pulumi/sdk/v3/go/pulumi"
)

func main() {
	pulumi.Run(func(ctx *pulumi.Context) error {
		_, err := mq.NewBroker(ctx, "example", &mq.BrokerArgs{
			BrokerName: pulumi.String("example"),
			Configuration: &mq.BrokerConfigurationArgs{
				Id:       pulumi.Any(test.Id),
				Revision: pulumi.Any(test.LatestRevision),
			},
			EngineType:       pulumi.String("ActiveMQ"),
			EngineVersion:    pulumi.String("5.17.6"),
			HostInstanceType: pulumi.String("mq.t2.micro"),
			SecurityGroups: pulumi.StringArray{
				testAwsSecurityGroup.Id,
			},
			Users: mq.BrokerUserArray{
				&mq.BrokerUserArgs{
					Username: pulumi.String("ExampleUser"),
					Password: pulumi.String("MindTheGap"),
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
import com.pulumi.aws.mq.Broker;
import com.pulumi.aws.mq.BrokerArgs;
import com.pulumi.aws.mq.inputs.BrokerConfigurationArgs;
import com.pulumi.aws.mq.inputs.BrokerUserArgs;
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
        var example = new Broker("example", BrokerArgs.builder()
            .brokerName("example")
            .configuration(BrokerConfigurationArgs.builder()
                .id(test.id())
                .revision(test.latestRevision())
                .build())
            .engineType("ActiveMQ")
            .engineVersion("5.17.6")
            .hostInstanceType("mq.t2.micro")
            .securityGroups(testAwsSecurityGroup.id())
            .users(BrokerUserArgs.builder()
                .username("ExampleUser")
                .password("MindTheGap")
                .build())
            .build());

    }
}
```
```yaml
resources:
  example:
    type: aws:mq:Broker
    properties:
      brokerName: example
      configuration:
        id: ${test.id}
        revision: ${test.latestRevision}
      engineType: ActiveMQ
      engineVersion: 5.17.6
      hostInstanceType: mq.t2.micro
      securityGroups:
        - ${testAwsSecurityGroup.id}
      users:
        - username: ExampleUser
          password: MindTheGap
```
<!--End PulumiCodeChooser -->

### High-throughput Optimized Example

This example shows the use of EBS storage for high-throughput optimized performance.

<!--Start PulumiCodeChooser -->
```typescript
import * as pulumi from "@pulumi/pulumi";
import * as aws from "@pulumi/aws";

const example = new aws.mq.Broker("example", {
    brokerName: "example",
    configuration: {
        id: test.id,
        revision: test.latestRevision,
    },
    engineType: "ActiveMQ",
    engineVersion: "5.17.6",
    storageType: "ebs",
    hostInstanceType: "mq.m5.large",
    securityGroups: [testAwsSecurityGroup.id],
    users: [{
        username: "ExampleUser",
        password: "MindTheGap",
    }],
});
```
```python
import pulumi
import pulumi_aws as aws

example = aws.mq.Broker("example",
    broker_name="example",
    configuration={
        "id": test["id"],
        "revision": test["latestRevision"],
    },
    engine_type="ActiveMQ",
    engine_version="5.17.6",
    storage_type="ebs",
    host_instance_type="mq.m5.large",
    security_groups=[test_aws_security_group["id"]],
    users=[{
        "username": "ExampleUser",
        "password": "MindTheGap",
    }])
```
```csharp
using System.Collections.Generic;
using System.Linq;
using Pulumi;
using Aws = Pulumi.Aws;

return await Deployment.RunAsync(() => 
{
    var example = new Aws.Mq.Broker("example", new()
    {
        BrokerName = "example",
        Configuration = new Aws.Mq.Inputs.BrokerConfigurationArgs
        {
            Id = test.Id,
            Revision = test.LatestRevision,
        },
        EngineType = "ActiveMQ",
        EngineVersion = "5.17.6",
        StorageType = "ebs",
        HostInstanceType = "mq.m5.large",
        SecurityGroups = new[]
        {
            testAwsSecurityGroup.Id,
        },
        Users = new[]
        {
            new Aws.Mq.Inputs.BrokerUserArgs
            {
                Username = "ExampleUser",
                Password = "MindTheGap",
            },
        },
    });

});
```
```go
package main

import (
	"github.com/pulumi/pulumi-aws/sdk/v6/go/aws/mq"
	"github.com/pulumi/pulumi/sdk/v3/go/pulumi"
)

func main() {
	pulumi.Run(func(ctx *pulumi.Context) error {
		_, err := mq.NewBroker(ctx, "example", &mq.BrokerArgs{
			BrokerName: pulumi.String("example"),
			Configuration: &mq.BrokerConfigurationArgs{
				Id:       pulumi.Any(test.Id),
				Revision: pulumi.Any(test.LatestRevision),
			},
			EngineType:       pulumi.String("ActiveMQ"),
			EngineVersion:    pulumi.String("5.17.6"),
			StorageType:      pulumi.String("ebs"),
			HostInstanceType: pulumi.String("mq.m5.large"),
			SecurityGroups: pulumi.StringArray{
				testAwsSecurityGroup.Id,
			},
			Users: mq.BrokerUserArray{
				&mq.BrokerUserArgs{
					Username: pulumi.String("ExampleUser"),
					Password: pulumi.String("MindTheGap"),
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
import com.pulumi.aws.mq.Broker;
import com.pulumi.aws.mq.BrokerArgs;
import com.pulumi.aws.mq.inputs.BrokerConfigurationArgs;
import com.pulumi.aws.mq.inputs.BrokerUserArgs;
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
        var example = new Broker("example", BrokerArgs.builder()
            .brokerName("example")
            .configuration(BrokerConfigurationArgs.builder()
                .id(test.id())
                .revision(test.latestRevision())
                .build())
            .engineType("ActiveMQ")
            .engineVersion("5.17.6")
            .storageType("ebs")
            .hostInstanceType("mq.m5.large")
            .securityGroups(testAwsSecurityGroup.id())
            .users(BrokerUserArgs.builder()
                .username("ExampleUser")
                .password("MindTheGap")
                .build())
            .build());

    }
}
```
```yaml
resources:
  example:
    type: aws:mq:Broker
    properties:
      brokerName: example
      configuration:
        id: ${test.id}
        revision: ${test.latestRevision}
      engineType: ActiveMQ
      engineVersion: 5.17.6
      storageType: ebs
      hostInstanceType: mq.m5.large
      securityGroups:
        - ${testAwsSecurityGroup.id}
      users:
        - username: ExampleUser
          password: MindTheGap
```
<!--End PulumiCodeChooser -->

### Cross-Region Data Replication

<!--Start PulumiCodeChooser -->
```typescript
import * as pulumi from "@pulumi/pulumi";
import * as aws from "@pulumi/aws";

const examplePrimary = new aws.mq.Broker("example_primary", {
    applyImmediately: true,
    brokerName: "example_primary",
    engineType: "ActiveMQ",
    engineVersion: "5.17.6",
    hostInstanceType: "mq.m5.large",
    securityGroups: [examplePrimaryAwsSecurityGroup.id],
    deploymentMode: "ACTIVE_STANDBY_MULTI_AZ",
    users: [
        {
            username: "ExampleUser",
            password: "MindTheGap",
        },
        {
            username: "ExampleReplicationUser",
            password: "Example12345",
            replicationUser: true,
        },
    ],
});
const example = new aws.mq.Broker("example", {
    applyImmediately: true,
    brokerName: "example",
    engineType: "ActiveMQ",
    engineVersion: "5.17.6",
    hostInstanceType: "mq.m5.large",
    securityGroups: [exampleAwsSecurityGroup.id],
    deploymentMode: "ACTIVE_STANDBY_MULTI_AZ",
    dataReplicationMode: "CRDR",
    dataReplicationPrimaryBrokerArn: primary.arn,
    users: [
        {
            username: "ExampleUser",
            password: "MindTheGap",
        },
        {
            username: "ExampleReplicationUser",
            password: "Example12345",
            replicationUser: true,
        },
    ],
});
```
```python
import pulumi
import pulumi_aws as aws

example_primary = aws.mq.Broker("example_primary",
    apply_immediately=True,
    broker_name="example_primary",
    engine_type="ActiveMQ",
    engine_version="5.17.6",
    host_instance_type="mq.m5.large",
    security_groups=[example_primary_aws_security_group["id"]],
    deployment_mode="ACTIVE_STANDBY_MULTI_AZ",
    users=[
        {
            "username": "ExampleUser",
            "password": "MindTheGap",
        },
        {
            "username": "ExampleReplicationUser",
            "password": "Example12345",
            "replication_user": True,
        },
    ])
example = aws.mq.Broker("example",
    apply_immediately=True,
    broker_name="example",
    engine_type="ActiveMQ",
    engine_version="5.17.6",
    host_instance_type="mq.m5.large",
    security_groups=[example_aws_security_group["id"]],
    deployment_mode="ACTIVE_STANDBY_MULTI_AZ",
    data_replication_mode="CRDR",
    data_replication_primary_broker_arn=primary["arn"],
    users=[
        {
            "username": "ExampleUser",
            "password": "MindTheGap",
        },
        {
            "username": "ExampleReplicationUser",
            "password": "Example12345",
            "replication_user": True,
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
    var examplePrimary = new Aws.Mq.Broker("example_primary", new()
    {
        ApplyImmediately = true,
        BrokerName = "example_primary",
        EngineType = "ActiveMQ",
        EngineVersion = "5.17.6",
        HostInstanceType = "mq.m5.large",
        SecurityGroups = new[]
        {
            examplePrimaryAwsSecurityGroup.Id,
        },
        DeploymentMode = "ACTIVE_STANDBY_MULTI_AZ",
        Users = new[]
        {
            new Aws.Mq.Inputs.BrokerUserArgs
            {
                Username = "ExampleUser",
                Password = "MindTheGap",
            },
            new Aws.Mq.Inputs.BrokerUserArgs
            {
                Username = "ExampleReplicationUser",
                Password = "Example12345",
                ReplicationUser = true,
            },
        },
    });

    var example = new Aws.Mq.Broker("example", new()
    {
        ApplyImmediately = true,
        BrokerName = "example",
        EngineType = "ActiveMQ",
        EngineVersion = "5.17.6",
        HostInstanceType = "mq.m5.large",
        SecurityGroups = new[]
        {
            exampleAwsSecurityGroup.Id,
        },
        DeploymentMode = "ACTIVE_STANDBY_MULTI_AZ",
        DataReplicationMode = "CRDR",
        DataReplicationPrimaryBrokerArn = primary.Arn,
        Users = new[]
        {
            new Aws.Mq.Inputs.BrokerUserArgs
            {
                Username = "ExampleUser",
                Password = "MindTheGap",
            },
            new Aws.Mq.Inputs.BrokerUserArgs
            {
                Username = "ExampleReplicationUser",
                Password = "Example12345",
                ReplicationUser = true,
            },
        },
    });

});
```
```go
package main

import (
	"github.com/pulumi/pulumi-aws/sdk/v6/go/aws/mq"
	"github.com/pulumi/pulumi/sdk/v3/go/pulumi"
)

func main() {
	pulumi.Run(func(ctx *pulumi.Context) error {
		_, err := mq.NewBroker(ctx, "example_primary", &mq.BrokerArgs{
			ApplyImmediately: pulumi.Bool(true),
			BrokerName:       pulumi.String("example_primary"),
			EngineType:       pulumi.String("ActiveMQ"),
			EngineVersion:    pulumi.String("5.17.6"),
			HostInstanceType: pulumi.String("mq.m5.large"),
			SecurityGroups: pulumi.StringArray{
				examplePrimaryAwsSecurityGroup.Id,
			},
			DeploymentMode: pulumi.String("ACTIVE_STANDBY_MULTI_AZ"),
			Users: mq.BrokerUserArray{
				&mq.BrokerUserArgs{
					Username: pulumi.String("ExampleUser"),
					Password: pulumi.String("MindTheGap"),
				},
				&mq.BrokerUserArgs{
					Username:        pulumi.String("ExampleReplicationUser"),
					Password:        pulumi.String("Example12345"),
					ReplicationUser: pulumi.Bool(true),
				},
			},
		})
		if err != nil {
			return err
		}
		_, err = mq.NewBroker(ctx, "example", &mq.BrokerArgs{
			ApplyImmediately: pulumi.Bool(true),
			BrokerName:       pulumi.String("example"),
			EngineType:       pulumi.String("ActiveMQ"),
			EngineVersion:    pulumi.String("5.17.6"),
			HostInstanceType: pulumi.String("mq.m5.large"),
			SecurityGroups: pulumi.StringArray{
				exampleAwsSecurityGroup.Id,
			},
			DeploymentMode:                  pulumi.String("ACTIVE_STANDBY_MULTI_AZ"),
			DataReplicationMode:             pulumi.String("CRDR"),
			DataReplicationPrimaryBrokerArn: pulumi.Any(primary.Arn),
			Users: mq.BrokerUserArray{
				&mq.BrokerUserArgs{
					Username: pulumi.String("ExampleUser"),
					Password: pulumi.String("MindTheGap"),
				},
				&mq.BrokerUserArgs{
					Username:        pulumi.String("ExampleReplicationUser"),
					Password:        pulumi.String("Example12345"),
					ReplicationUser: pulumi.Bool(true),
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
import com.pulumi.aws.mq.Broker;
import com.pulumi.aws.mq.BrokerArgs;
import com.pulumi.aws.mq.inputs.BrokerUserArgs;
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
        var examplePrimary = new Broker("examplePrimary", BrokerArgs.builder()
            .applyImmediately(true)
            .brokerName("example_primary")
            .engineType("ActiveMQ")
            .engineVersion("5.17.6")
            .hostInstanceType("mq.m5.large")
            .securityGroups(examplePrimaryAwsSecurityGroup.id())
            .deploymentMode("ACTIVE_STANDBY_MULTI_AZ")
            .users(            
                BrokerUserArgs.builder()
                    .username("ExampleUser")
                    .password("MindTheGap")
                    .build(),
                BrokerUserArgs.builder()
                    .username("ExampleReplicationUser")
                    .password("Example12345")
                    .replicationUser(true)
                    .build())
            .build());

        var example = new Broker("example", BrokerArgs.builder()
            .applyImmediately(true)
            .brokerName("example")
            .engineType("ActiveMQ")
            .engineVersion("5.17.6")
            .hostInstanceType("mq.m5.large")
            .securityGroups(exampleAwsSecurityGroup.id())
            .deploymentMode("ACTIVE_STANDBY_MULTI_AZ")
            .dataReplicationMode("CRDR")
            .dataReplicationPrimaryBrokerArn(primary.arn())
            .users(            
                BrokerUserArgs.builder()
                    .username("ExampleUser")
                    .password("MindTheGap")
                    .build(),
                BrokerUserArgs.builder()
                    .username("ExampleReplicationUser")
                    .password("Example12345")
                    .replicationUser(true)
                    .build())
            .build());

    }
}
```
```yaml
resources:
  examplePrimary:
    type: aws:mq:Broker
    name: example_primary
    properties:
      applyImmediately: true
      brokerName: example_primary
      engineType: ActiveMQ
      engineVersion: 5.17.6
      hostInstanceType: mq.m5.large
      securityGroups:
        - ${examplePrimaryAwsSecurityGroup.id}
      deploymentMode: ACTIVE_STANDBY_MULTI_AZ
      users:
        - username: ExampleUser
          password: MindTheGap
        - username: ExampleReplicationUser
          password: Example12345
          replicationUser: true
  example:
    type: aws:mq:Broker
    properties:
      applyImmediately: true
      brokerName: example
      engineType: ActiveMQ
      engineVersion: 5.17.6
      hostInstanceType: mq.m5.large
      securityGroups:
        - ${exampleAwsSecurityGroup.id}
      deploymentMode: ACTIVE_STANDBY_MULTI_AZ
      dataReplicationMode: CRDR
      dataReplicationPrimaryBrokerArn: ${primary.arn}
      users:
        - username: ExampleUser
          password: MindTheGap
        - username: ExampleReplicationUser
          password: Example12345
          replicationUser: true
```
<!--End PulumiCodeChooser -->

See the [AWS MQ documentation](https://docs.aws.amazon.com/amazon-mq/latest/developer-guide/crdr-for-active-mq.html) on cross-region data replication for additional details.

## Import

Using `pulumi import`, import MQ Brokers using their broker id. For example:

```sh
$ pulumi import aws:mq/broker:Broker example a1b2c3d4-d5f6-7777-8888-9999aaaabbbbcccc
```
�
applyImmediatelyB
 Specifies whether any broker modifications are applied immediately, or during the next maintenance window. Default is `false`.
�
authenticationStrategyB" �Authentication strategy used to secure the broker. Valid values are `simple` and `ldap`. `ldap` is not supported for `engine_type` `RabbitMQ`.
�
autoMinorVersionUpgradeB
 iWhether to automatically upgrade to new minor versions of brokers as Amazon MQ makes releases available.
(

brokerNameB" Name of the broker.
�
configurationOBM:K
I
mqBrokerConfiguration.aws:mq/BrokerConfiguration:BrokerConfigurationzConfiguration block for broker configuration. Applies to `engine_type` of `ActiveMQ` and `RabbitMQ` only. Detailed below.
�
dataReplicationModeB" fDefines whether this broker is a part of a data replication pair. Valid values are `CRDR` and `NONE`.
�
dataReplicationPrimaryBrokerArnB" �The Amazon Resource Name (ARN) of the primary broker that is used to replicate data from in a data replication pair, and is applied to the replica broker. Must be set when `data_replication_mode` is `CRDR`.
�
deploymentModeB" �Deployment mode of the broker. Valid values are `SINGLE_INSTANCE`, `ACTIVE_STANDBY_MULTI_AZ`, and `CLUSTER_MULTI_AZ`. Default is `SINGLE_INSTANCE`.
�
encryptionOptions[BY:W
U
mqBrokerEncryptionOptions6aws:mq/BrokerEncryptionOptions:BrokerEncryptionOptionsCConfiguration block containing encryption options. Detailed below.
U

engineType" CType of broker engine. Valid values are `ActiveMQ` and `RabbitMQ`.
�
engineVersion" �Version of the broker engine. See the [AmazonMQ Broker Engine docs](https://docs.aws.amazon.com/amazon-mq/latest/developer-guide/broker-engine.html) for supported versions. For example, `5.17.6`.
[
hostInstanceType" CBroker's instance type. For example, `mq.t3.micro`, `mq.m5.large`.
�
ldapServerMetadata^B\:Z
X
mqBrokerLdapServerMetadata8aws:mq/BrokerLdapServerMetadata:BrokerLdapServerMetadata�Configuration block for the LDAP server used to authenticate and authorize connections to the broker. Not supported for `engine_type` `RabbitMQ`. Detailed below. (Currently, AWS may not process changes to LDAP server metadata.)
�
logs4B2:0
.
mq
BrokerLogsaws:mq/BrokerLogs:BrokerLogsQConfiguration block for the logging configuration of the broker. Detailed below.
�
maintenanceWindowStartTimevBt:r
p
mq BrokerMaintenanceWindowStartTimeHaws:mq/BrokerMaintenanceWindowStartTime:BrokerMaintenanceWindowStartTimeKConfiguration block for the maintenance window start time. Detailed below.
�
publiclyAccessibleB
 dWhether to enable connections from applications outside of the VPC that hosts the broker's subnets.
M
securityGroupsB*" 3List of security group IDs assigned to the broker.
�
storageTypeB" �Storage type of the broker. For `engine_type` `ActiveMQ`, the valid values are `efs` and `ebs`, and the AWS-default is `efs`. For `engine_type` `RabbitMQ`, only `ebs` is supported. When using `ebs`, only the `mq.m5` broker instance type family is supported.
�
	subnetIdsB*" �List of subnet IDs in which to launch the broker. A `SINGLE_INSTANCE` deployment requires one subnet. An `ACTIVE_STANDBY_MULTI_AZ` deployment requires multiple subnets.
�
tagsB2" �Map of tags to assign to the broker. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
�
users4*2:0
.
mq
BrokerUseraws:mq/BrokerUser:BrokerUser�Configuration block for broker users. For `engine_type` of `RabbitMQ`, Amazon MQ does not return broker users preventing this resource from making user updates and drift detection. Detailed below.

The following arguments are optional:
"�
applyImmediatelyB
 Specifies whether any broker modifications are applied immediately, or during the next maintenance window. Default is `false`.
"
arn" ARN of the broker.
"�
authenticationStrategy" �Authentication strategy used to secure the broker. Valid values are `simple` and `ldap`. `ldap` is not supported for `engine_type` `RabbitMQ`.
"�
autoMinorVersionUpgradeB
 iWhether to automatically upgrade to new minor versions of brokers as Amazon MQ makes releases available.
"&

brokerName" Name of the broker.
"�
configurationM:K
I
mqBrokerConfiguration.aws:mq/BrokerConfiguration:BrokerConfigurationzConfiguration block for broker configuration. Applies to `engine_type` of `ActiveMQ` and `RabbitMQ` only. Detailed below.
"�
dataReplicationMode" fDefines whether this broker is a part of a data replication pair. Valid values are `CRDR` and `NONE`.
"�
dataReplicationPrimaryBrokerArnB" �The Amazon Resource Name (ARN) of the primary broker that is used to replicate data from in a data replication pair, and is applied to the replica broker. Must be set when `data_replication_mode` is `CRDR`.
"�
deploymentModeB" �Deployment mode of the broker. Valid values are `SINGLE_INSTANCE`, `ACTIVE_STANDBY_MULTI_AZ`, and `CLUSTER_MULTI_AZ`. Default is `SINGLE_INSTANCE`.
"�
encryptionOptions[BY:W
U
mqBrokerEncryptionOptions6aws:mq/BrokerEncryptionOptions:BrokerEncryptionOptionsCConfiguration block containing encryption options. Detailed below.
"U

engineType" CType of broker engine. Valid values are `ActiveMQ` and `RabbitMQ`.
"�
engineVersion" �Version of the broker engine. See the [AmazonMQ Broker Engine docs](https://docs.aws.amazon.com/amazon-mq/latest/developer-guide/broker-engine.html) for supported versions. For example, `5.17.6`.
"[
hostInstanceType" CBroker's instance type. For example, `mq.t3.micro`, `mq.m5.large`.
"�
	instances@*>:<
:
mqBrokerInstance$aws:mq/BrokerInstance:BrokerInstanceEList of information about allocated brokers (both active & standby).
"�
ldapServerMetadata^B\:Z
X
mqBrokerLdapServerMetadata8aws:mq/BrokerLdapServerMetadata:BrokerLdapServerMetadata�Configuration block for the LDAP server used to authenticate and authorize connections to the broker. Not supported for `engine_type` `RabbitMQ`. Detailed below. (Currently, AWS may not process changes to LDAP server metadata.)
"�
logs4B2:0
.
mq
BrokerLogsaws:mq/BrokerLogs:BrokerLogsQConfiguration block for the logging configuration of the broker. Detailed below.
"�
maintenanceWindowStartTimet:r
p
mq BrokerMaintenanceWindowStartTimeHaws:mq/BrokerMaintenanceWindowStartTime:BrokerMaintenanceWindowStartTimeKConfiguration block for the maintenance window start time. Detailed below.
"j
pendingDataReplicationMode" H(Optional) The data replication mode that will be applied after reboot.
"�
publiclyAccessibleB
 dWhether to enable connections from applications outside of the VPC that hosts the broker's subnets.
"M
securityGroupsB*" 3List of security group IDs assigned to the broker.
"�
storageType" �Storage type of the broker. For `engine_type` `ActiveMQ`, the valid values are `efs` and `ebs`, and the AWS-default is `efs`. For `engine_type` `RabbitMQ`, only `ebs` is supported. When using `ebs`, only the `mq.m5` broker instance type family is supported.
"�
	subnetIds*" �List of subnet IDs in which to launch the broker. A `SINGLE_INSTANCE` deployment requires one subnet. An `ACTIVE_STANDBY_MULTI_AZ` deployment requires multiple subnets.
"�
tagsB2" �Map of tags to assign to the broker. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
"�
tagsAll2" xA map of tags assigned to the resource, including those inherited from the provider `default_tags` configuration block.
"�
users4*2:0
.
mq
BrokerUseraws:mq/BrokerUser:BrokerUser�Configuration block for broker users. For `engine_type` of `RabbitMQ`, Amazon MQ does not return broker users preventing this resource from making user updates and drift detection. Detailed below.

The following arguments are optional:
*�R
7
mqConfiguration"aws:mq/configuration:Configuration�@Provides an MQ Configuration Resource.

For more information on Amazon MQ, see [Amazon MQ documentation](https://docs.aws.amazon.com/amazon-mq/latest/developer-guide/welcome.html).

## Example Usage

### ActiveMQ

<!--Start PulumiCodeChooser -->
```typescript
import * as pulumi from "@pulumi/pulumi";
import * as aws from "@pulumi/aws";

const example = new aws.mq.Configuration("example", {
    description: "Example Configuration",
    name: "example",
    engineType: "ActiveMQ",
    engineVersion: "5.17.6",
    data: `<?xml version="1.0" encoding="UTF-8" standalone="yes"?>
<broker xmlns="http://activemq.apache.org/schema/core">
  <plugins>
    <forcePersistencyModeBrokerPlugin persistenceFlag="true"/>
    <statisticsBrokerPlugin/>
    <timeStampingBrokerPlugin ttlCeiling="86400000" zeroExpirationOverride="86400000"/>
  </plugins>
</broker>
`,
});
```
```python
import pulumi
import pulumi_aws as aws

example = aws.mq.Configuration("example",
    description="Example Configuration",
    name="example",
    engine_type="ActiveMQ",
    engine_version="5.17.6",
    data="""<?xml version="1.0" encoding="UTF-8" standalone="yes"?>
<broker xmlns="http://activemq.apache.org/schema/core">
  <plugins>
    <forcePersistencyModeBrokerPlugin persistenceFlag="true"/>
    <statisticsBrokerPlugin/>
    <timeStampingBrokerPlugin ttlCeiling="86400000" zeroExpirationOverride="86400000"/>
  </plugins>
</broker>
""")
```
```csharp
using System.Collections.Generic;
using System.Linq;
using Pulumi;
using Aws = Pulumi.Aws;

return await Deployment.RunAsync(() => 
{
    var example = new Aws.Mq.Configuration("example", new()
    {
        Description = "Example Configuration",
        Name = "example",
        EngineType = "ActiveMQ",
        EngineVersion = "5.17.6",
        Data = @"<?xml version=""1.0"" encoding=""UTF-8"" standalone=""yes""?>
<broker xmlns=""http://activemq.apache.org/schema/core"">
  <plugins>
    <forcePersistencyModeBrokerPlugin persistenceFlag=""true""/>
    <statisticsBrokerPlugin/>
    <timeStampingBrokerPlugin ttlCeiling=""86400000"" zeroExpirationOverride=""86400000""/>
  </plugins>
</broker>
",
    });

});
```
```go
package main

import (
	"github.com/pulumi/pulumi-aws/sdk/v6/go/aws/mq"
	"github.com/pulumi/pulumi/sdk/v3/go/pulumi"
)

func main() {
	pulumi.Run(func(ctx *pulumi.Context) error {
		_, err := mq.NewConfiguration(ctx, "example", &mq.ConfigurationArgs{
			Description:   pulumi.String("Example Configuration"),
			Name:          pulumi.String("example"),
			EngineType:    pulumi.String("ActiveMQ"),
			EngineVersion: pulumi.String("5.17.6"),
			Data: pulumi.String(`<?xml version="1.0" encoding="UTF-8" standalone="yes"?>
<broker xmlns="http://activemq.apache.org/schema/core">
  <plugins>
    <forcePersistencyModeBrokerPlugin persistenceFlag="true"/>
    <statisticsBrokerPlugin/>
    <timeStampingBrokerPlugin ttlCeiling="86400000" zeroExpirationOverride="86400000"/>
  </plugins>
</broker>
`),
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
import com.pulumi.aws.mq.Configuration;
import com.pulumi.aws.mq.ConfigurationArgs;
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
        var example = new Configuration("example", ConfigurationArgs.builder()
            .description("Example Configuration")
            .name("example")
            .engineType("ActiveMQ")
            .engineVersion("5.17.6")
            .data("""
<?xml version="1.0" encoding="UTF-8" standalone="yes"?>
<broker xmlns="http://activemq.apache.org/schema/core">
  <plugins>
    <forcePersistencyModeBrokerPlugin persistenceFlag="true"/>
    <statisticsBrokerPlugin/>
    <timeStampingBrokerPlugin ttlCeiling="86400000" zeroExpirationOverride="86400000"/>
  </plugins>
</broker>
            """)
            .build());

    }
}
```
```yaml
resources:
  example:
    type: aws:mq:Configuration
    properties:
      description: Example Configuration
      name: example
      engineType: ActiveMQ
      engineVersion: 5.17.6
      data: |
        <?xml version="1.0" encoding="UTF-8" standalone="yes"?>
        <broker xmlns="http://activemq.apache.org/schema/core">
          <plugins>
            <forcePersistencyModeBrokerPlugin persistenceFlag="true"/>
            <statisticsBrokerPlugin/>
            <timeStampingBrokerPlugin ttlCeiling="86400000" zeroExpirationOverride="86400000"/>
          </plugins>
        </broker>
```
<!--End PulumiCodeChooser -->

### RabbitMQ

<!--Start PulumiCodeChooser -->
```typescript
import * as pulumi from "@pulumi/pulumi";
import * as aws from "@pulumi/aws";

const example = new aws.mq.Configuration("example", {
    description: "Example Configuration",
    name: "example",
    engineType: "RabbitMQ",
    engineVersion: "3.11.20",
    data: `# Default RabbitMQ delivery acknowledgement timeout is 30 minutes in milliseconds
consumer_timeout = 1800000
`,
});
```
```python
import pulumi
import pulumi_aws as aws

example = aws.mq.Configuration("example",
    description="Example Configuration",
    name="example",
    engine_type="RabbitMQ",
    engine_version="3.11.20",
    data="""# Default RabbitMQ delivery acknowledgement timeout is 30 minutes in milliseconds
consumer_timeout = 1800000
""")
```
```csharp
using System.Collections.Generic;
using System.Linq;
using Pulumi;
using Aws = Pulumi.Aws;

return await Deployment.RunAsync(() => 
{
    var example = new Aws.Mq.Configuration("example", new()
    {
        Description = "Example Configuration",
        Name = "example",
        EngineType = "RabbitMQ",
        EngineVersion = "3.11.20",
        Data = @"# Default RabbitMQ delivery acknowledgement timeout is 30 minutes in milliseconds
consumer_timeout = 1800000
",
    });

});
```
```go
package main

import (
	"github.com/pulumi/pulumi-aws/sdk/v6/go/aws/mq"
	"github.com/pulumi/pulumi/sdk/v3/go/pulumi"
)

func main() {
	pulumi.Run(func(ctx *pulumi.Context) error {
		_, err := mq.NewConfiguration(ctx, "example", &mq.ConfigurationArgs{
			Description:   pulumi.String("Example Configuration"),
			Name:          pulumi.String("example"),
			EngineType:    pulumi.String("RabbitMQ"),
			EngineVersion: pulumi.String("3.11.20"),
			Data:          pulumi.String("# Default RabbitMQ delivery acknowledgement timeout is 30 minutes in milliseconds\nconsumer_timeout = 1800000\n"),
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
import com.pulumi.aws.mq.Configuration;
import com.pulumi.aws.mq.ConfigurationArgs;
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
        var example = new Configuration("example", ConfigurationArgs.builder()
            .description("Example Configuration")
            .name("example")
            .engineType("RabbitMQ")
            .engineVersion("3.11.20")
            .data("""
# Default RabbitMQ delivery acknowledgement timeout is 30 minutes in milliseconds
consumer_timeout = 1800000
            """)
            .build());

    }
}
```
```yaml
resources:
  example:
    type: aws:mq:Configuration
    properties:
      description: Example Configuration
      name: example
      engineType: RabbitMQ
      engineVersion: 3.11.20
      data: |
        # Default RabbitMQ delivery acknowledgement timeout is 30 minutes in milliseconds
        consumer_timeout = 1800000
```
<!--End PulumiCodeChooser -->

## Import

Using `pulumi import`, import MQ Configurations using the configuration ID. For example:

```sh
$ pulumi import aws:mq/configuration:Configuration example c-0187d1eb-88c8-475a-9b79-16ef5a10c94f
```
�
authenticationStrategyB" �Authentication strategy associated with the configuration. Valid values are `simple` and `ldap`. `ldap` is not supported for `engine_type` `RabbitMQ`.
�
data" �Broker configuration in XML format for `ActiveMQ` or [Cuttlefish](https://github.com/Kyorai/cuttlefish) format for `RabbitMQ`. See [official docs](https://docs.aws.amazon.com/amazon-mq/latest/developer-guide/amazon-mq-broker-configuration-parameters.html) for supported parameters and format of the XML.
7
descriptionB" "Description of the configuration.
U

engineType" CType of broker engine. Valid values are `ActiveMQ` and `RabbitMQ`.
3
engineVersion" Version of the broker engine.
P
nameB" BName of the configuration.

The following arguments are optional:
�
tagsB2" �Map of tags to assign to the resource. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
"%
arn" ARN of the configuration.
"�
authenticationStrategy" �Authentication strategy associated with the configuration. Valid values are `simple` and `ldap`. `ldap` is not supported for `engine_type` `RabbitMQ`.
"�
data" �Broker configuration in XML format for `ActiveMQ` or [Cuttlefish](https://github.com/Kyorai/cuttlefish) format for `RabbitMQ`. See [official docs](https://docs.aws.amazon.com/amazon-mq/latest/developer-guide/amazon-mq-broker-configuration-parameters.html) for supported parameters and format of the XML.
"7
descriptionB" "Description of the configuration.
"U

engineType" CType of broker engine. Valid values are `ActiveMQ` and `RabbitMQ`.
"3
engineVersion" Version of the broker engine.
"<
latestRevision &Latest revision of the configuration.
"N
name" BName of the configuration.

The following arguments are optional:
"�
tagsB2" �Map of tags to assign to the resource. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
"�
tagsAll2" xA map of tags assigned to the resource, including those inherited from the provider `default_tags` configuration block.
*��
'
mskClusteraws:msk/cluster:Cluster�Manages an Amazon MSK cluster.

> **Note:** This resource manages _provisioned_ clusters. To manage a _serverless_ Amazon MSK cluster, use the `aws.msk.ServerlessCluster` resource.

## Example Usage

### Basic

<!--Start PulumiCodeChooser -->
```typescript
import * as pulumi from "@pulumi/pulumi";
import * as aws from "@pulumi/aws";

const vpc = new aws.ec2.Vpc("vpc", {cidrBlock: "192.168.0.0/22"});
const azs = aws.getAvailabilityZones({
    state: "available",
});
const subnetAz1 = new aws.ec2.Subnet("subnet_az1", {
    availabilityZone: azs.then(azs => azs.names?.[0]),
    cidrBlock: "192.168.0.0/24",
    vpcId: vpc.id,
});
const subnetAz2 = new aws.ec2.Subnet("subnet_az2", {
    availabilityZone: azs.then(azs => azs.names?.[1]),
    cidrBlock: "192.168.1.0/24",
    vpcId: vpc.id,
});
const subnetAz3 = new aws.ec2.Subnet("subnet_az3", {
    availabilityZone: azs.then(azs => azs.names?.[2]),
    cidrBlock: "192.168.2.0/24",
    vpcId: vpc.id,
});
const sg = new aws.ec2.SecurityGroup("sg", {vpcId: vpc.id});
const kms = new aws.kms.Key("kms", {description: "example"});
const test = new aws.cloudwatch.LogGroup("test", {name: "msk_broker_logs"});
const bucket = new aws.s3.BucketV2("bucket", {bucket: "msk-broker-logs-bucket"});
const bucketAcl = new aws.s3.BucketAclV2("bucket_acl", {
    bucket: bucket.id,
    acl: "private",
});
const assumeRole = aws.iam.getPolicyDocument({
    statements: [{
        effect: "Allow",
        principals: [{
            type: "Service",
            identifiers: ["firehose.amazonaws.com"],
        }],
        actions: ["sts:AssumeRole"],
    }],
});
const firehoseRole = new aws.iam.Role("firehose_role", {
    name: "firehose_test_role",
    assumeRolePolicy: assumeRole.then(assumeRole => assumeRole.json),
});
const testStream = new aws.kinesis.FirehoseDeliveryStream("test_stream", {
    name: "kinesis-firehose-msk-broker-logs-stream",
    destination: "extended_s3",
    extendedS3Configuration: {
        roleArn: firehoseRole.arn,
        bucketArn: bucket.arn,
    },
    tags: {
        LogDeliveryEnabled: "placeholder",
    },
});
const example = new aws.msk.Cluster("example", {
    clusterName: "example",
    kafkaVersion: "3.2.0",
    numberOfBrokerNodes: 3,
    brokerNodeGroupInfo: {
        instanceType: "kafka.m5.large",
        clientSubnets: [
            subnetAz1.id,
            subnetAz2.id,
            subnetAz3.id,
        ],
        storageInfo: {
            ebsStorageInfo: {
                volumeSize: 1000,
            },
        },
        securityGroups: [sg.id],
    },
    encryptionInfo: {
        encryptionAtRestKmsKeyArn: kms.arn,
    },
    openMonitoring: {
        prometheus: {
            jmxExporter: {
                enabledInBroker: true,
            },
            nodeExporter: {
                enabledInBroker: true,
            },
        },
    },
    loggingInfo: {
        brokerLogs: {
            cloudwatchLogs: {
                enabled: true,
                logGroup: test.name,
            },
            firehose: {
                enabled: true,
                deliveryStream: testStream.name,
            },
            s3: {
                enabled: true,
                bucket: bucket.id,
                prefix: "logs/msk-",
            },
        },
    },
    tags: {
        foo: "bar",
    },
});
export const zookeeperConnectString = example.zookeeperConnectString;
export const bootstrapBrokersTls = example.bootstrapBrokersTls;
```
```python
import pulumi
import pulumi_aws as aws

vpc = aws.ec2.Vpc("vpc", cidr_block="192.168.0.0/22")
azs = aws.get_availability_zones(state="available")
subnet_az1 = aws.ec2.Subnet("subnet_az1",
    availability_zone=azs.names[0],
    cidr_block="192.168.0.0/24",
    vpc_id=vpc.id)
subnet_az2 = aws.ec2.Subnet("subnet_az2",
    availability_zone=azs.names[1],
    cidr_block="192.168.1.0/24",
    vpc_id=vpc.id)
subnet_az3 = aws.ec2.Subnet("subnet_az3",
    availability_zone=azs.names[2],
    cidr_block="192.168.2.0/24",
    vpc_id=vpc.id)
sg = aws.ec2.SecurityGroup("sg", vpc_id=vpc.id)
kms = aws.kms.Key("kms", description="example")
test = aws.cloudwatch.LogGroup("test", name="msk_broker_logs")
bucket = aws.s3.BucketV2("bucket", bucket="msk-broker-logs-bucket")
bucket_acl = aws.s3.BucketAclV2("bucket_acl",
    bucket=bucket.id,
    acl="private")
assume_role = aws.iam.get_policy_document(statements=[{
    "effect": "Allow",
    "principals": [{
        "type": "Service",
        "identifiers": ["firehose.amazonaws.com"],
    }],
    "actions": ["sts:AssumeRole"],
}])
firehose_role = aws.iam.Role("firehose_role",
    name="firehose_test_role",
    assume_role_policy=assume_role.json)
test_stream = aws.kinesis.FirehoseDeliveryStream("test_stream",
    name="kinesis-firehose-msk-broker-logs-stream",
    destination="extended_s3",
    extended_s3_configuration={
        "role_arn": firehose_role.arn,
        "bucket_arn": bucket.arn,
    },
    tags={
        "LogDeliveryEnabled": "placeholder",
    })
example = aws.msk.Cluster("example",
    cluster_name="example",
    kafka_version="3.2.0",
    number_of_broker_nodes=3,
    broker_node_group_info={
        "instance_type": "kafka.m5.large",
        "client_subnets": [
            subnet_az1.id,
            subnet_az2.id,
            subnet_az3.id,
        ],
        "storage_info": {
            "ebs_storage_info": {
                "volume_size": 1000,
            },
        },
        "security_groups": [sg.id],
    },
    encryption_info={
        "encryption_at_rest_kms_key_arn": kms.arn,
    },
    open_monitoring={
        "prometheus": {
            "jmx_exporter": {
                "enabled_in_broker": True,
            },
            "node_exporter": {
                "enabled_in_broker": True,
            },
        },
    },
    logging_info={
        "broker_logs": {
            "cloudwatch_logs": {
                "enabled": True,
                "log_group": test.name,
            },
            "firehose": {
                "enabled": True,
                "delivery_stream": test_stream.name,
            },
            "s3": {
                "enabled": True,
                "bucket": bucket.id,
                "prefix": "logs/msk-",
            },
        },
    },
    tags={
        "foo": "bar",
    })
pulumi.export("zookeeperConnectString", example.zookeeper_connect_string)
pulumi.export("bootstrapBrokersTls", example.bootstrap_brokers_tls)
```
```csharp
using System.Collections.Generic;
using System.Linq;
using Pulumi;
using Aws = Pulumi.Aws;

return await Deployment.RunAsync(() => 
{
    var vpc = new Aws.Ec2.Vpc("vpc", new()
    {
        CidrBlock = "192.168.0.0/22",
    });

    var azs = Aws.GetAvailabilityZones.Invoke(new()
    {
        State = "available",
    });

    var subnetAz1 = new Aws.Ec2.Subnet("subnet_az1", new()
    {
        AvailabilityZone = azs.Apply(getAvailabilityZonesResult => getAvailabilityZonesResult.Names[0]),
        CidrBlock = "192.168.0.0/24",
        VpcId = vpc.Id,
    });

    var subnetAz2 = new Aws.Ec2.Subnet("subnet_az2", new()
    {
        AvailabilityZone = azs.Apply(getAvailabilityZonesResult => getAvailabilityZonesResult.Names[1]),
        CidrBlock = "192.168.1.0/24",
        VpcId = vpc.Id,
    });

    var subnetAz3 = new Aws.Ec2.Subnet("subnet_az3", new()
    {
        AvailabilityZone = azs.Apply(getAvailabilityZonesResult => getAvailabilityZonesResult.Names[2]),
        CidrBlock = "192.168.2.0/24",
        VpcId = vpc.Id,
    });

    var sg = new Aws.Ec2.SecurityGroup("sg", new()
    {
        VpcId = vpc.Id,
    });

    var kms = new Aws.Kms.Key("kms", new()
    {
        Description = "example",
    });

    var test = new Aws.CloudWatch.LogGroup("test", new()
    {
        Name = "msk_broker_logs",
    });

    var bucket = new Aws.S3.BucketV2("bucket", new()
    {
        Bucket = "msk-broker-logs-bucket",
    });

    var bucketAcl = new Aws.S3.BucketAclV2("bucket_acl", new()
    {
        Bucket = bucket.Id,
        Acl = "private",
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
                            "firehose.amazonaws.com",
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

    var firehoseRole = new Aws.Iam.Role("firehose_role", new()
    {
        Name = "firehose_test_role",
        AssumeRolePolicy = assumeRole.Apply(getPolicyDocumentResult => getPolicyDocumentResult.Json),
    });

    var testStream = new Aws.Kinesis.FirehoseDeliveryStream("test_stream", new()
    {
        Name = "kinesis-firehose-msk-broker-logs-stream",
        Destination = "extended_s3",
        ExtendedS3Configuration = new Aws.Kinesis.Inputs.FirehoseDeliveryStreamExtendedS3ConfigurationArgs
        {
            RoleArn = firehoseRole.Arn,
            BucketArn = bucket.Arn,
        },
        Tags = 
        {
            { "LogDeliveryEnabled", "placeholder" },
        },
    });

    var example = new Aws.Msk.Cluster("example", new()
    {
        ClusterName = "example",
        KafkaVersion = "3.2.0",
        NumberOfBrokerNodes = 3,
        BrokerNodeGroupInfo = new Aws.Msk.Inputs.ClusterBrokerNodeGroupInfoArgs
        {
            InstanceType = "kafka.m5.large",
            ClientSubnets = new[]
            {
                subnetAz1.Id,
                subnetAz2.Id,
                subnetAz3.Id,
            },
            StorageInfo = new Aws.Msk.Inputs.ClusterBrokerNodeGroupInfoStorageInfoArgs
            {
                EbsStorageInfo = new Aws.Msk.Inputs.ClusterBrokerNodeGroupInfoStorageInfoEbsStorageInfoArgs
                {
                    VolumeSize = 1000,
                },
            },
            SecurityGroups = new[]
            {
                sg.Id,
            },
        },
        EncryptionInfo = new Aws.Msk.Inputs.ClusterEncryptionInfoArgs
        {
            EncryptionAtRestKmsKeyArn = kms.Arn,
        },
        OpenMonitoring = new Aws.Msk.Inputs.ClusterOpenMonitoringArgs
        {
            Prometheus = new Aws.Msk.Inputs.ClusterOpenMonitoringPrometheusArgs
            {
                JmxExporter = new Aws.Msk.Inputs.ClusterOpenMonitoringPrometheusJmxExporterArgs
                {
                    EnabledInBroker = true,
                },
                NodeExporter = new Aws.Msk.Inputs.ClusterOpenMonitoringPrometheusNodeExporterArgs
                {
                    EnabledInBroker = true,
                },
            },
        },
        LoggingInfo = new Aws.Msk.Inputs.ClusterLoggingInfoArgs
        {
            BrokerLogs = new Aws.Msk.Inputs.ClusterLoggingInfoBrokerLogsArgs
            {
                CloudwatchLogs = new Aws.Msk.Inputs.ClusterLoggingInfoBrokerLogsCloudwatchLogsArgs
                {
                    Enabled = true,
                    LogGroup = test.Name,
                },
                Firehose = new Aws.Msk.Inputs.ClusterLoggingInfoBrokerLogsFirehoseArgs
                {
                    Enabled = true,
                    DeliveryStream = testStream.Name,
                },
                S3 = new Aws.Msk.Inputs.ClusterLoggingInfoBrokerLogsS3Args
                {
                    Enabled = true,
                    Bucket = bucket.Id,
                    Prefix = "logs/msk-",
                },
            },
        },
        Tags = 
        {
            { "foo", "bar" },
        },
    });

    return new Dictionary<string, object?>
    {
        ["zookeeperConnectString"] = example.ZookeeperConnectString,
        ["bootstrapBrokersTls"] = example.BootstrapBrokersTls,
    };
});
```
```go
package main

import (
	"github.com/pulumi/pulumi-aws/sdk/v6/go/aws"
	"github.com/pulumi/pulumi-aws/sdk/v6/go/aws/cloudwatch"
	"github.com/pulumi/pulumi-aws/sdk/v6/go/aws/ec2"
	"github.com/pulumi/pulumi-aws/sdk/v6/go/aws/iam"
	"github.com/pulumi/pulumi-aws/sdk/v6/go/aws/kinesis"
	"github.com/pulumi/pulumi-aws/sdk/v6/go/aws/kms"
	"github.com/pulumi/pulumi-aws/sdk/v6/go/aws/msk"
	"github.com/pulumi/pulumi-aws/sdk/v6/go/aws/s3"
	"github.com/pulumi/pulumi/sdk/v3/go/pulumi"
)

func main() {
	pulumi.Run(func(ctx *pulumi.Context) error {
		vpc, err := ec2.NewVpc(ctx, "vpc", &ec2.VpcArgs{
			CidrBlock: pulumi.String("192.168.0.0/22"),
		})
		if err != nil {
			return err
		}
		azs, err := aws.GetAvailabilityZones(ctx, &aws.GetAvailabilityZonesArgs{
			State: pulumi.StringRef("available"),
		}, nil)
		if err != nil {
			return err
		}
		subnetAz1, err := ec2.NewSubnet(ctx, "subnet_az1", &ec2.SubnetArgs{
			AvailabilityZone: pulumi.String(azs.Names[0]),
			CidrBlock:        pulumi.String("192.168.0.0/24"),
			VpcId:            vpc.ID(),
		})
		if err != nil {
			return err
		}
		subnetAz2, err := ec2.NewSubnet(ctx, "subnet_az2", &ec2.SubnetArgs{
			AvailabilityZone: pulumi.String(azs.Names[1]),
			CidrBlock:        pulumi.String("192.168.1.0/24"),
			VpcId:            vpc.ID(),
		})
		if err != nil {
			return err
		}
		subnetAz3, err := ec2.NewSubnet(ctx, "subnet_az3", &ec2.SubnetArgs{
			AvailabilityZone: pulumi.String(azs.Names[2]),
			CidrBlock:        pulumi.String("192.168.2.0/24"),
			VpcId:            vpc.ID(),
		})
		if err != nil {
			return err
		}
		sg, err := ec2.NewSecurityGroup(ctx, "sg", &ec2.SecurityGroupArgs{
			VpcId: vpc.ID(),
		})
		if err != nil {
			return err
		}
		kms, err := kms.NewKey(ctx, "kms", &kms.KeyArgs{
			Description: pulumi.String("example"),
		})
		if err != nil {
			return err
		}
		test, err := cloudwatch.NewLogGroup(ctx, "test", &cloudwatch.LogGroupArgs{
			Name: pulumi.String("msk_broker_logs"),
		})
		if err != nil {
			return err
		}
		bucket, err := s3.NewBucketV2(ctx, "bucket", &s3.BucketV2Args{
			Bucket: pulumi.String("msk-broker-logs-bucket"),
		})
		if err != nil {
			return err
		}
		_, err = s3.NewBucketAclV2(ctx, "bucket_acl", &s3.BucketAclV2Args{
			Bucket: bucket.ID(),
			Acl:    pulumi.String("private"),
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
								"firehose.amazonaws.com",
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
		firehoseRole, err := iam.NewRole(ctx, "firehose_role", &iam.RoleArgs{
			Name:             pulumi.String("firehose_test_role"),
			AssumeRolePolicy: pulumi.String(assumeRole.Json),
		})
		if err != nil {
			return err
		}
		testStream, err := kinesis.NewFirehoseDeliveryStream(ctx, "test_stream", &kinesis.FirehoseDeliveryStreamArgs{
			Name:        pulumi.String("kinesis-firehose-msk-broker-logs-stream"),
			Destination: pulumi.String("extended_s3"),
			ExtendedS3Configuration: &kinesis.FirehoseDeliveryStreamExtendedS3ConfigurationArgs{
				RoleArn:   firehoseRole.Arn,
				BucketArn: bucket.Arn,
			},
			Tags: pulumi.StringMap{
				"LogDeliveryEnabled": pulumi.String("placeholder"),
			},
		})
		if err != nil {
			return err
		}
		example, err := msk.NewCluster(ctx, "example", &msk.ClusterArgs{
			ClusterName:         pulumi.String("example"),
			KafkaVersion:        pulumi.String("3.2.0"),
			NumberOfBrokerNodes: pulumi.Int(3),
			BrokerNodeGroupInfo: &msk.ClusterBrokerNodeGroupInfoArgs{
				InstanceType: pulumi.String("kafka.m5.large"),
				ClientSubnets: pulumi.StringArray{
					subnetAz1.ID(),
					subnetAz2.ID(),
					subnetAz3.ID(),
				},
				StorageInfo: &msk.ClusterBrokerNodeGroupInfoStorageInfoArgs{
					EbsStorageInfo: &msk.ClusterBrokerNodeGroupInfoStorageInfoEbsStorageInfoArgs{
						VolumeSize: pulumi.Int(1000),
					},
				},
				SecurityGroups: pulumi.StringArray{
					sg.ID(),
				},
			},
			EncryptionInfo: &msk.ClusterEncryptionInfoArgs{
				EncryptionAtRestKmsKeyArn: kms.Arn,
			},
			OpenMonitoring: &msk.ClusterOpenMonitoringArgs{
				Prometheus: &msk.ClusterOpenMonitoringPrometheusArgs{
					JmxExporter: &msk.ClusterOpenMonitoringPrometheusJmxExporterArgs{
						EnabledInBroker: pulumi.Bool(true),
					},
					NodeExporter: &msk.ClusterOpenMonitoringPrometheusNodeExporterArgs{
						EnabledInBroker: pulumi.Bool(true),
					},
				},
			},
			LoggingInfo: &msk.ClusterLoggingInfoArgs{
				BrokerLogs: &msk.ClusterLoggingInfoBrokerLogsArgs{
					CloudwatchLogs: &msk.ClusterLoggingInfoBrokerLogsCloudwatchLogsArgs{
						Enabled:  pulumi.Bool(true),
						LogGroup: test.Name,
					},
					Firehose: &msk.ClusterLoggingInfoBrokerLogsFirehoseArgs{
						Enabled:        pulumi.Bool(true),
						DeliveryStream: testStream.Name,
					},
					S3: &msk.ClusterLoggingInfoBrokerLogsS3Args{
						Enabled: pulumi.Bool(true),
						Bucket:  bucket.ID(),
						Prefix:  pulumi.String("logs/msk-"),
					},
				},
			},
			Tags: pulumi.StringMap{
				"foo": pulumi.String("bar"),
			},
		})
		if err != nil {
			return err
		}
		ctx.Export("zookeeperConnectString", example.ZookeeperConnectString)
		ctx.Export("bootstrapBrokersTls", example.BootstrapBrokersTls)
		return nil
	})
}
```
```java
package generated_program;

import com.pulumi.Context;
import com.pulumi.Pulumi;
import com.pulumi.core.Output;
import com.pulumi.aws.ec2.Vpc;
import com.pulumi.aws.ec2.VpcArgs;
import com.pulumi.aws.AwsFunctions;
import com.pulumi.aws.inputs.GetAvailabilityZonesArgs;
import com.pulumi.aws.ec2.Subnet;
import com.pulumi.aws.ec2.SubnetArgs;
import com.pulumi.aws.ec2.SecurityGroup;
import com.pulumi.aws.ec2.SecurityGroupArgs;
import com.pulumi.aws.kms.Key;
import com.pulumi.aws.kms.KeyArgs;
import com.pulumi.aws.cloudwatch.LogGroup;
import com.pulumi.aws.cloudwatch.LogGroupArgs;
import com.pulumi.aws.s3.BucketV2;
import com.pulumi.aws.s3.BucketV2Args;
import com.pulumi.aws.s3.BucketAclV2;
import com.pulumi.aws.s3.BucketAclV2Args;
import com.pulumi.aws.iam.IamFunctions;
import com.pulumi.aws.iam.inputs.GetPolicyDocumentArgs;
import com.pulumi.aws.iam.Role;
import com.pulumi.aws.iam.RoleArgs;
import com.pulumi.aws.kinesis.FirehoseDeliveryStream;
import com.pulumi.aws.kinesis.FirehoseDeliveryStreamArgs;
import com.pulumi.aws.kinesis.inputs.FirehoseDeliveryStreamExtendedS3ConfigurationArgs;
import com.pulumi.aws.msk.Cluster;
import com.pulumi.aws.msk.ClusterArgs;
import com.pulumi.aws.msk.inputs.ClusterBrokerNodeGroupInfoArgs;
import com.pulumi.aws.msk.inputs.ClusterBrokerNodeGroupInfoStorageInfoArgs;
import com.pulumi.aws.msk.inputs.ClusterBrokerNodeGroupInfoStorageInfoEbsStorageInfoArgs;
import com.pulumi.aws.msk.inputs.ClusterEncryptionInfoArgs;
import com.pulumi.aws.msk.inputs.ClusterOpenMonitoringArgs;
import com.pulumi.aws.msk.inputs.ClusterOpenMonitoringPrometheusArgs;
import com.pulumi.aws.msk.inputs.ClusterOpenMonitoringPrometheusJmxExporterArgs;
import com.pulumi.aws.msk.inputs.ClusterOpenMonitoringPrometheusNodeExporterArgs;
import com.pulumi.aws.msk.inputs.ClusterLoggingInfoArgs;
import com.pulumi.aws.msk.inputs.ClusterLoggingInfoBrokerLogsArgs;
import com.pulumi.aws.msk.inputs.ClusterLoggingInfoBrokerLogsCloudwatchLogsArgs;
import com.pulumi.aws.msk.inputs.ClusterLoggingInfoBrokerLogsFirehoseArgs;
import com.pulumi.aws.msk.inputs.ClusterLoggingInfoBrokerLogsS3Args;
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
        var vpc = new Vpc("vpc", VpcArgs.builder()
            .cidrBlock("192.168.0.0/22")
            .build());

        final var azs = AwsFunctions.getAvailabilityZones(GetAvailabilityZonesArgs.builder()
            .state("available")
            .build());

        var subnetAz1 = new Subnet("subnetAz1", SubnetArgs.builder()
            .availabilityZone(azs.applyValue(getAvailabilityZonesResult -> getAvailabilityZonesResult.names()[0]))
            .cidrBlock("192.168.0.0/24")
            .vpcId(vpc.id())
            .build());

        var subnetAz2 = new Subnet("subnetAz2", SubnetArgs.builder()
            .availabilityZone(azs.applyValue(getAvailabilityZonesResult -> getAvailabilityZonesResult.names()[1]))
            .cidrBlock("192.168.1.0/24")
            .vpcId(vpc.id())
            .build());

        var subnetAz3 = new Subnet("subnetAz3", SubnetArgs.builder()
            .availabilityZone(azs.applyValue(getAvailabilityZonesResult -> getAvailabilityZonesResult.names()[2]))
            .cidrBlock("192.168.2.0/24")
            .vpcId(vpc.id())
            .build());

        var sg = new SecurityGroup("sg", SecurityGroupArgs.builder()
            .vpcId(vpc.id())
            .build());

        var kms = new Key("kms", KeyArgs.builder()
            .description("example")
            .build());

        var test = new LogGroup("test", LogGroupArgs.builder()
            .name("msk_broker_logs")
            .build());

        var bucket = new BucketV2("bucket", BucketV2Args.builder()
            .bucket("msk-broker-logs-bucket")
            .build());

        var bucketAcl = new BucketAclV2("bucketAcl", BucketAclV2Args.builder()
            .bucket(bucket.id())
            .acl("private")
            .build());

        final var assumeRole = IamFunctions.getPolicyDocument(GetPolicyDocumentArgs.builder()
            .statements(GetPolicyDocumentStatementArgs.builder()
                .effect("Allow")
                .principals(GetPolicyDocumentStatementPrincipalArgs.builder()
                    .type("Service")
                    .identifiers("firehose.amazonaws.com")
                    .build())
                .actions("sts:AssumeRole")
                .build())
            .build());

        var firehoseRole = new Role("firehoseRole", RoleArgs.builder()
            .name("firehose_test_role")
            .assumeRolePolicy(assumeRole.applyValue(getPolicyDocumentResult -> getPolicyDocumentResult.json()))
            .build());

        var testStream = new FirehoseDeliveryStream("testStream", FirehoseDeliveryStreamArgs.builder()
            .name("kinesis-firehose-msk-broker-logs-stream")
            .destination("extended_s3")
            .extendedS3Configuration(FirehoseDeliveryStreamExtendedS3ConfigurationArgs.builder()
                .roleArn(firehoseRole.arn())
                .bucketArn(bucket.arn())
                .build())
            .tags(Map.of("LogDeliveryEnabled", "placeholder"))
            .build());

        var example = new Cluster("example", ClusterArgs.builder()
            .clusterName("example")
            .kafkaVersion("3.2.0")
            .numberOfBrokerNodes(3)
            .brokerNodeGroupInfo(ClusterBrokerNodeGroupInfoArgs.builder()
                .instanceType("kafka.m5.large")
                .clientSubnets(                
                    subnetAz1.id(),
                    subnetAz2.id(),
                    subnetAz3.id())
                .storageInfo(ClusterBrokerNodeGroupInfoStorageInfoArgs.builder()
                    .ebsStorageInfo(ClusterBrokerNodeGroupInfoStorageInfoEbsStorageInfoArgs.builder()
                        .volumeSize(1000)
                        .build())
                    .build())
                .securityGroups(sg.id())
                .build())
            .encryptionInfo(ClusterEncryptionInfoArgs.builder()
                .encryptionAtRestKmsKeyArn(kms.arn())
                .build())
            .openMonitoring(ClusterOpenMonitoringArgs.builder()
                .prometheus(ClusterOpenMonitoringPrometheusArgs.builder()
                    .jmxExporter(ClusterOpenMonitoringPrometheusJmxExporterArgs.builder()
                        .enabledInBroker(true)
                        .build())
                    .nodeExporter(ClusterOpenMonitoringPrometheusNodeExporterArgs.builder()
                        .enabledInBroker(true)
                        .build())
                    .build())
                .build())
            .loggingInfo(ClusterLoggingInfoArgs.builder()
                .brokerLogs(ClusterLoggingInfoBrokerLogsArgs.builder()
                    .cloudwatchLogs(ClusterLoggingInfoBrokerLogsCloudwatchLogsArgs.builder()
                        .enabled(true)
                        .logGroup(test.name())
                        .build())
                    .firehose(ClusterLoggingInfoBrokerLogsFirehoseArgs.builder()
                        .enabled(true)
                        .deliveryStream(testStream.name())
                        .build())
                    .s3(ClusterLoggingInfoBrokerLogsS3Args.builder()
                        .enabled(true)
                        .bucket(bucket.id())
                        .prefix("logs/msk-")
                        .build())
                    .build())
                .build())
            .tags(Map.of("foo", "bar"))
            .build());

        ctx.export("zookeeperConnectString", example.zookeeperConnectString());
        ctx.export("bootstrapBrokersTls", example.bootstrapBrokersTls());
    }
}
```
```yaml
resources:
  vpc:
    type: aws:ec2:Vpc
    properties:
      cidrBlock: 192.168.0.0/22
  subnetAz1:
    type: aws:ec2:Subnet
    name: subnet_az1
    properties:
      availabilityZone: ${azs.names[0]}
      cidrBlock: 192.168.0.0/24
      vpcId: ${vpc.id}
  subnetAz2:
    type: aws:ec2:Subnet
    name: subnet_az2
    properties:
      availabilityZone: ${azs.names[1]}
      cidrBlock: 192.168.1.0/24
      vpcId: ${vpc.id}
  subnetAz3:
    type: aws:ec2:Subnet
    name: subnet_az3
    properties:
      availabilityZone: ${azs.names[2]}
      cidrBlock: 192.168.2.0/24
      vpcId: ${vpc.id}
  sg:
    type: aws:ec2:SecurityGroup
    properties:
      vpcId: ${vpc.id}
  kms:
    type: aws:kms:Key
    properties:
      description: example
  test:
    type: aws:cloudwatch:LogGroup
    properties:
      name: msk_broker_logs
  bucket:
    type: aws:s3:BucketV2
    properties:
      bucket: msk-broker-logs-bucket
  bucketAcl:
    type: aws:s3:BucketAclV2
    name: bucket_acl
    properties:
      bucket: ${bucket.id}
      acl: private
  firehoseRole:
    type: aws:iam:Role
    name: firehose_role
    properties:
      name: firehose_test_role
      assumeRolePolicy: ${assumeRole.json}
  testStream:
    type: aws:kinesis:FirehoseDeliveryStream
    name: test_stream
    properties:
      name: kinesis-firehose-msk-broker-logs-stream
      destination: extended_s3
      extendedS3Configuration:
        roleArn: ${firehoseRole.arn}
        bucketArn: ${bucket.arn}
      tags:
        LogDeliveryEnabled: placeholder
  example:
    type: aws:msk:Cluster
    properties:
      clusterName: example
      kafkaVersion: 3.2.0
      numberOfBrokerNodes: 3
      brokerNodeGroupInfo:
        instanceType: kafka.m5.large
        clientSubnets:
          - ${subnetAz1.id}
          - ${subnetAz2.id}
          - ${subnetAz3.id}
        storageInfo:
          ebsStorageInfo:
            volumeSize: 1000
        securityGroups:
          - ${sg.id}
      encryptionInfo:
        encryptionAtRestKmsKeyArn: ${kms.arn}
      openMonitoring:
        prometheus:
          jmxExporter:
            enabledInBroker: true
          nodeExporter:
            enabledInBroker: true
      loggingInfo:
        brokerLogs:
          cloudwatchLogs:
            enabled: true
            logGroup: ${test.name}
          firehose:
            enabled: true
            deliveryStream: ${testStream.name}
          s3:
            enabled: true
            bucket: ${bucket.id}
            prefix: logs/msk-
      tags:
        foo: bar
variables:
  azs:
    fn::invoke:
      function: aws:getAvailabilityZones
      arguments:
        state: available
  assumeRole:
    fn::invoke:
      function: aws:iam:getPolicyDocument
      arguments:
        statements:
          - effect: Allow
            principals:
              - type: Service
                identifiers:
                  - firehose.amazonaws.com
            actions:
              - sts:AssumeRole
outputs:
  zookeeperConnectString: ${example.zookeeperConnectString}
  bootstrapBrokersTls: ${example.bootstrapBrokersTls}
```
<!--End PulumiCodeChooser -->

### With volume_throughput argument

<!--Start PulumiCodeChooser -->
```typescript
import * as pulumi from "@pulumi/pulumi";
import * as aws from "@pulumi/aws";

const example = new aws.msk.Cluster("example", {
    clusterName: "example",
    kafkaVersion: "2.7.1",
    numberOfBrokerNodes: 3,
    brokerNodeGroupInfo: {
        instanceType: "kafka.m5.4xlarge",
        clientSubnets: [
            subnetAz1.id,
            subnetAz2.id,
            subnetAz3.id,
        ],
        storageInfo: {
            ebsStorageInfo: {
                provisionedThroughput: {
                    enabled: true,
                    volumeThroughput: 250,
                },
                volumeSize: 1000,
            },
        },
        securityGroups: [sg.id],
    },
});
```
```python
import pulumi
import pulumi_aws as aws

example = aws.msk.Cluster("example",
    cluster_name="example",
    kafka_version="2.7.1",
    number_of_broker_nodes=3,
    broker_node_group_info={
        "instance_type": "kafka.m5.4xlarge",
        "client_subnets": [
            subnet_az1["id"],
            subnet_az2["id"],
            subnet_az3["id"],
        ],
        "storage_info": {
            "ebs_storage_info": {
                "provisioned_throughput": {
                    "enabled": True,
                    "volume_throughput": 250,
                },
                "volume_size": 1000,
            },
        },
        "security_groups": [sg["id"]],
    })
```
```csharp
using System.Collections.Generic;
using System.Linq;
using Pulumi;
using Aws = Pulumi.Aws;

return await Deployment.RunAsync(() => 
{
    var example = new Aws.Msk.Cluster("example", new()
    {
        ClusterName = "example",
        KafkaVersion = "2.7.1",
        NumberOfBrokerNodes = 3,
        BrokerNodeGroupInfo = new Aws.Msk.Inputs.ClusterBrokerNodeGroupInfoArgs
        {
            InstanceType = "kafka.m5.4xlarge",
            ClientSubnets = new[]
            {
                subnetAz1.Id,
                subnetAz2.Id,
                subnetAz3.Id,
            },
            StorageInfo = new Aws.Msk.Inputs.ClusterBrokerNodeGroupInfoStorageInfoArgs
            {
                EbsStorageInfo = new Aws.Msk.Inputs.ClusterBrokerNodeGroupInfoStorageInfoEbsStorageInfoArgs
                {
                    ProvisionedThroughput = new Aws.Msk.Inputs.ClusterBrokerNodeGroupInfoStorageInfoEbsStorageInfoProvisionedThroughputArgs
                    {
                        Enabled = true,
                        VolumeThroughput = 250,
                    },
                    VolumeSize = 1000,
                },
            },
            SecurityGroups = new[]
            {
                sg.Id,
            },
        },
    });

});
```
```go
package main

import (
	"github.com/pulumi/pulumi-aws/sdk/v6/go/aws/msk"
	"github.com/pulumi/pulumi/sdk/v3/go/pulumi"
)

func main() {
	pulumi.Run(func(ctx *pulumi.Context) error {
		_, err := msk.NewCluster(ctx, "example", &msk.ClusterArgs{
			ClusterName:         pulumi.String("example"),
			KafkaVersion:        pulumi.String("2.7.1"),
			NumberOfBrokerNodes: pulumi.Int(3),
			BrokerNodeGroupInfo: &msk.ClusterBrokerNodeGroupInfoArgs{
				InstanceType: pulumi.String("kafka.m5.4xlarge"),
				ClientSubnets: pulumi.StringArray{
					subnetAz1.Id,
					subnetAz2.Id,
					subnetAz3.Id,
				},
				StorageInfo: &msk.ClusterBrokerNodeGroupInfoStorageInfoArgs{
					EbsStorageInfo: &msk.ClusterBrokerNodeGroupInfoStorageInfoEbsStorageInfoArgs{
						ProvisionedThroughput: &msk.ClusterBrokerNodeGroupInfoStorageInfoEbsStorageInfoProvisionedThroughputArgs{
							Enabled:          pulumi.Bool(true),
							VolumeThroughput: pulumi.Int(250),
						},
						VolumeSize: pulumi.Int(1000),
					},
				},
				SecurityGroups: pulumi.StringArray{
					sg.Id,
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
import com.pulumi.aws.msk.Cluster;
import com.pulumi.aws.msk.ClusterArgs;
import com.pulumi.aws.msk.inputs.ClusterBrokerNodeGroupInfoArgs;
import com.pulumi.aws.msk.inputs.ClusterBrokerNodeGroupInfoStorageInfoArgs;
import com.pulumi.aws.msk.inputs.ClusterBrokerNodeGroupInfoStorageInfoEbsStorageInfoArgs;
import com.pulumi.aws.msk.inputs.ClusterBrokerNodeGroupInfoStorageInfoEbsStorageInfoProvisionedThroughputArgs;
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
        var example = new Cluster("example", ClusterArgs.builder()
            .clusterName("example")
            .kafkaVersion("2.7.1")
            .numberOfBrokerNodes(3)
            .brokerNodeGroupInfo(ClusterBrokerNodeGroupInfoArgs.builder()
                .instanceType("kafka.m5.4xlarge")
                .clientSubnets(                
                    subnetAz1.id(),
                    subnetAz2.id(),
                    subnetAz3.id())
                .storageInfo(ClusterBrokerNodeGroupInfoStorageInfoArgs.builder()
                    .ebsStorageInfo(ClusterBrokerNodeGroupInfoStorageInfoEbsStorageInfoArgs.builder()
                        .provisionedThroughput(ClusterBrokerNodeGroupInfoStorageInfoEbsStorageInfoProvisionedThroughputArgs.builder()
                            .enabled(true)
                            .volumeThroughput(250)
                            .build())
                        .volumeSize(1000)
                        .build())
                    .build())
                .securityGroups(sg.id())
                .build())
            .build());

    }
}
```
```yaml
resources:
  example:
    type: aws:msk:Cluster
    properties:
      clusterName: example
      kafkaVersion: 2.7.1
      numberOfBrokerNodes: 3
      brokerNodeGroupInfo:
        instanceType: kafka.m5.4xlarge
        clientSubnets:
          - ${subnetAz1.id}
          - ${subnetAz2.id}
          - ${subnetAz3.id}
        storageInfo:
          ebsStorageInfo:
            provisionedThroughput:
              enabled: true
              volumeThroughput: 250
            volumeSize: 1000
        securityGroups:
          - ${sg.id}
```
<!--End PulumiCodeChooser -->

## Import

Using `pulumi import`, import MSK clusters using the cluster `arn`. For example:

```sh
$ pulumi import aws:msk/cluster:Cluster example arn:aws:kafka:us-west-2:123456789012:cluster/example/279c0212-d057-4dba-9aa9-1c4e5a25bfc7-3
```
�
brokerNodeGroupInfod:b
`
mskClusterBrokerNodeGroupInfo=aws:msk/ClusterBrokerNodeGroupInfo:ClusterBrokerNodeGroupInfo?Configuration block for the broker nodes of the Kafka cluster.
�
clientAuthenticationiBg:e
c
mskClusterClientAuthentication?aws:msk/ClusterClientAuthentication:ClusterClientAuthenticationGConfiguration block for specifying a client authentication. See below.
.
clusterNameB" Name of the MSK cluster.
�
configurationInfo`B^:\
Z
mskClusterConfigurationInfo9aws:msk/ClusterConfigurationInfo:ClusterConfigurationInfo^Configuration block for specifying a MSK Configuration to attach to Kafka brokers. See below.
�
encryptionInfoWBU:S
Q
mskClusterEncryptionInfo3aws:msk/ClusterEncryptionInfo:ClusterEncryptionInfo:Configuration block for specifying encryption. See below.
�
enhancedMonitoringB" �Specify the desired enhanced MSK CloudWatch monitoring level. See [Monitoring Amazon MSK with Amazon CloudWatch](https://docs.aws.amazon.com/msk/latest/developerguide/monitoring.html)
@
kafkaVersion" ,Specify the desired Kafka software version.
�
loggingInfoNBL:J
H
mskClusterLoggingInfo-aws:msk/ClusterLoggingInfo:ClusterLoggingInfo\Configuration block for streaming broker logs to Cloudwatch/S3/Kinesis Firehose. See below.
�
numberOfBrokerNodes �The desired total number of broker nodes in the kafka cluster.  It must be a multiple of the number of specified client subnets.
�
openMonitoringWBU:S
Q
mskClusterOpenMonitoring3aws:msk/ClusterOpenMonitoring:ClusterOpenMonitoringPConfiguration block for JMX and Node monitoring for the MSK cluster. See below.
o
storageModeB" ZControls storage mode for supported storage tiers. Valid values are: `LOCAL` or `TIERED`.
�
tagsB2" �A map of tags to assign to the resource. .If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
":
arn" /Amazon Resource Name (ARN) of the MSK cluster.
"�
bootstrapBrokers" �Comma separated list of one or more hostname:port pairs of kafka brokers suitable to bootstrap connectivity to the kafka cluster. Contains a value if `encryption_info.0.encryption_in_transit.0.client_broker` is set to `PLAINTEXT` or `TLS_PLAINTEXT`. The resource sorts values alphabetically. AWS may not always return all endpoints so this value is not guaranteed to be stable across applies.
"�
bootstrapBrokersPublicSaslIam" �One or more DNS names (or IP addresses) and SASL IAM port pairs. For example, `b-1-public.exampleClusterName.abcde.c2.kafka.us-east-1.amazonaws.com:9198,b-2-public.exampleClusterName.abcde.c2.kafka.us-east-1.amazonaws.com:9198,b-3-public.exampleClusterName.abcde.c2.kafka.us-east-1.amazonaws.com:9198`. This attribute will have a value if `encryption_info.0.encryption_in_transit.0.client_broker` is set to `TLS_PLAINTEXT` or `TLS` and `client_authentication.0.sasl.0.iam` is set to `true` and `broker_node_group_info.0.connectivity_info.0.public_access.0.type` is set to `SERVICE_PROVIDED_EIPS` and the cluster fulfill all other requirements for public access. The resource sorts the list alphabetically. AWS may not always return all endpoints so the values may not be stable across applies.
"�
bootstrapBrokersPublicSaslScram" �One or more DNS names (or IP addresses) and SASL SCRAM port pairs. For example, `b-1-public.exampleClusterName.abcde.c2.kafka.us-east-1.amazonaws.com:9196,b-2-public.exampleClusterName.abcde.c2.kafka.us-east-1.amazonaws.com:9196,b-3-public.exampleClusterName.abcde.c2.kafka.us-east-1.amazonaws.com:9196`. This attribute will have a value if `encryption_info.0.encryption_in_transit.0.client_broker` is set to `TLS_PLAINTEXT` or `TLS` and `client_authentication.0.sasl.0.scram` is set to `true` and `broker_node_group_info.0.connectivity_info.0.public_access.0.type` is set to `SERVICE_PROVIDED_EIPS` and the cluster fulfill all other requirements for public access. The resource sorts the list alphabetically. AWS may not always return all endpoints so the values may not be stable across applies.
"�
bootstrapBrokersPublicTls" �One or more DNS names (or IP addresses) and TLS port pairs. For example, `b-1-public.exampleClusterName.abcde.c2.kafka.us-east-1.amazonaws.com:9194,b-2-public.exampleClusterName.abcde.c2.kafka.us-east-1.amazonaws.com:9194,b-3-public.exampleClusterName.abcde.c2.kafka.us-east-1.amazonaws.com:9194`. This attribute will have a value if `encryption_info.0.encryption_in_transit.0.client_broker` is set to `TLS_PLAINTEXT` or `TLS` and `broker_node_group_info.0.connectivity_info.0.public_access.0.type` is set to `SERVICE_PROVIDED_EIPS` and the cluster fulfill all other requirements for public access. The resource sorts the list alphabetically. AWS may not always return all endpoints so the values may not be stable across applies.
"�
bootstrapBrokersSaslIam" �One or more DNS names (or IP addresses) and SASL IAM port pairs. For example, `b-1.exampleClusterName.abcde.c2.kafka.us-east-1.amazonaws.com:9098,b-2.exampleClusterName.abcde.c2.kafka.us-east-1.amazonaws.com:9098,b-3.exampleClusterName.abcde.c2.kafka.us-east-1.amazonaws.com:9098`. This attribute will have a value if `encryption_info.0.encryption_in_transit.0.client_broker` is set to `TLS_PLAINTEXT` or `TLS` and `client_authentication.0.sasl.0.iam` is set to `true`. The resource sorts the list alphabetically. AWS may not always return all endpoints so the values may not be stable across applies.
"�
bootstrapBrokersSaslScram" �One or more DNS names (or IP addresses) and SASL SCRAM port pairs. For example, `b-1.exampleClusterName.abcde.c2.kafka.us-east-1.amazonaws.com:9096,b-2.exampleClusterName.abcde.c2.kafka.us-east-1.amazonaws.com:9096,b-3.exampleClusterName.abcde.c2.kafka.us-east-1.amazonaws.com:9096`. This attribute will have a value if `encryption_info.0.encryption_in_transit.0.client_broker` is set to `TLS_PLAINTEXT` or `TLS` and `client_authentication.0.sasl.0.scram` is set to `true`. The resource sorts the list alphabetically. AWS may not always return all endpoints so the values may not be stable across applies.
"�
bootstrapBrokersTls" �One or more DNS names (or IP addresses) and TLS port pairs. For example, `b-1.exampleClusterName.abcde.c2.kafka.us-east-1.amazonaws.com:9094,b-2.exampleClusterName.abcde.c2.kafka.us-east-1.amazonaws.com:9094,b-3.exampleClusterName.abcde.c2.kafka.us-east-1.amazonaws.com:9094`. This attribute will have a value if `encryption_info.0.encryption_in_transit.0.client_broker` is set to `TLS_PLAINTEXT` or `TLS`. The resource sorts the list alphabetically. AWS may not always return all endpoints so the values may not be stable across applies.
"�
&bootstrapBrokersVpcConnectivitySaslIam" �A string containing one or more DNS names (or IP addresses) and SASL IAM port pairs for VPC connectivity. AWS may not always return all endpoints so the values may not be stable across applies.
"�
(bootstrapBrokersVpcConnectivitySaslScram" �A string containing one or more DNS names (or IP addresses) and SASL SCRAM port pairs for VPC connectivity. AWS may not always return all endpoints so the values may not be stable across applies.
"�
"bootstrapBrokersVpcConnectivityTls" �A string containing one or more DNS names (or IP addresses) and TLS port pairs for VPC connectivity. AWS may not always return all endpoints so the values may not be stable across applies.
"�
brokerNodeGroupInfod:b
`
mskClusterBrokerNodeGroupInfo=aws:msk/ClusterBrokerNodeGroupInfo:ClusterBrokerNodeGroupInfo?Configuration block for the broker nodes of the Kafka cluster.
"�
clientAuthenticationiBg:e
c
mskClusterClientAuthentication?aws:msk/ClusterClientAuthentication:ClusterClientAuthenticationGConfiguration block for specifying a client authentication. See below.
",
clusterName" Name of the MSK cluster.
"E
clusterUuid" 2UUID of the MSK cluster, for use in IAM policies.
"�
configurationInfo`B^:\
Z
mskClusterConfigurationInfo9aws:msk/ClusterConfigurationInfo:ClusterConfigurationInfo^Configuration block for specifying a MSK Configuration to attach to Kafka brokers. See below.
"b
currentVersion" LCurrent version of the MSK Cluster used for updates, e.g., `K13V1IB3VIYZZH`
"�
encryptionInfoWBU:S
Q
mskClusterEncryptionInfo3aws:msk/ClusterEncryptionInfo:ClusterEncryptionInfo:Configuration block for specifying encryption. See below.
"�
enhancedMonitoringB" �Specify the desired enhanced MSK CloudWatch monitoring level. See [Monitoring Amazon MSK with Amazon CloudWatch](https://docs.aws.amazon.com/msk/latest/developerguide/monitoring.html)
"@
kafkaVersion" ,Specify the desired Kafka software version.
"�
loggingInfoNBL:J
H
mskClusterLoggingInfo-aws:msk/ClusterLoggingInfo:ClusterLoggingInfo\Configuration block for streaming broker logs to Cloudwatch/S3/Kinesis Firehose. See below.
"�
numberOfBrokerNodes �The desired total number of broker nodes in the kafka cluster.  It must be a multiple of the number of specified client subnets.
"�
openMonitoringWBU:S
Q
mskClusterOpenMonitoring3aws:msk/ClusterOpenMonitoring:ClusterOpenMonitoringPConfiguration block for JMX and Node monitoring for the MSK cluster. See below.
"m
storageMode" ZControls storage mode for supported storage tiers. Valid values are: `LOCAL` or `TIERED`.
"�
tagsB2" �A map of tags to assign to the resource. .If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
"�
tagsAll2" xA map of tags assigned to the resource, including those inherited from the provider `default_tags` configuration block.
"�
zookeeperConnectString" �A comma separated list of one or more hostname:port pairs to use to connect to the Apache Zookeeper cluster. The returned values are sorted alphabetically. The AWS API may not return all endpoints, so this value is not guaranteed to be stable across applies.
"�
zookeeperConnectStringTls" �A comma separated list of one or more hostname:port pairs to use to connect to the Apache Zookeeper cluster via TLS. The returned values are sorted alphabetically. The AWS API may not return all endpoints, so this value is not guaranteed to be stable across applies.
*�B
9
mskClusterPolicy#aws:msk/clusterPolicy:ClusterPolicy�?Resource for managing an AWS Managed Streaming for Kafka Cluster Policy.

## Example Usage

### Basic Usage

<!--Start PulumiCodeChooser -->
```typescript
import * as pulumi from "@pulumi/pulumi";
import * as aws from "@pulumi/aws";

const current = aws.getCallerIdentity({});
const currentGetPartition = aws.getPartition({});
const example = new aws.msk.ClusterPolicy("example", {
    clusterArn: exampleAwsMskCluster.arn,
    policy: JSON.stringify({
        Version: "2012-10-17",
        Statement: [{
            Sid: "ExampleMskClusterPolicy",
            Effect: "Allow",
            Principal: {
                AWS: Promise.all([currentGetPartition, current]).then(([currentGetPartition, current]) => `arn:${currentGetPartition.partition}:iam::${current.accountId}:root`),
            },
            Action: [
                "kafka:Describe*",
                "kafka:Get*",
                "kafka:CreateVpcConnection",
                "kafka:GetBootstrapBrokers",
            ],
            Resource: exampleAwsMskCluster.arn,
        }],
    }),
});
```
```python
import pulumi
import json
import pulumi_aws as aws

current = aws.get_caller_identity()
current_get_partition = aws.get_partition()
example = aws.msk.ClusterPolicy("example",
    cluster_arn=example_aws_msk_cluster["arn"],
    policy=json.dumps({
        "Version": "2012-10-17",
        "Statement": [{
            "Sid": "ExampleMskClusterPolicy",
            "Effect": "Allow",
            "Principal": {
                "AWS": f"arn:{current_get_partition.partition}:iam::{current.account_id}:root",
            },
            "Action": [
                "kafka:Describe*",
                "kafka:Get*",
                "kafka:CreateVpcConnection",
                "kafka:GetBootstrapBrokers",
            ],
            "Resource": example_aws_msk_cluster["arn"],
        }],
    }))
```
```csharp
using System.Collections.Generic;
using System.Linq;
using System.Text.Json;
using Pulumi;
using Aws = Pulumi.Aws;

return await Deployment.RunAsync(() => 
{
    var current = Aws.GetCallerIdentity.Invoke();

    var currentGetPartition = Aws.GetPartition.Invoke();

    var example = new Aws.Msk.ClusterPolicy("example", new()
    {
        ClusterArn = exampleAwsMskCluster.Arn,
        Policy = JsonSerializer.Serialize(new Dictionary<string, object?>
        {
            ["Version"] = "2012-10-17",
            ["Statement"] = new[]
            {
                new Dictionary<string, object?>
                {
                    ["Sid"] = "ExampleMskClusterPolicy",
                    ["Effect"] = "Allow",
                    ["Principal"] = new Dictionary<string, object?>
                    {
                        ["AWS"] = Output.Tuple(currentGetPartition, current).Apply(values =>
                        {
                            var currentGetPartition = values.Item1;
                            var current = values.Item2;
                            return $"arn:{currentGetPartition.Apply(getPartitionResult => getPartitionResult.Partition)}:iam::{current.Apply(getCallerIdentityResult => getCallerIdentityResult.AccountId)}:root";
                        }),
                    },
                    ["Action"] = new[]
                    {
                        "kafka:Describe*",
                        "kafka:Get*",
                        "kafka:CreateVpcConnection",
                        "kafka:GetBootstrapBrokers",
                    },
                    ["Resource"] = exampleAwsMskCluster.Arn,
                },
            },
        }),
    });

});
```
```go
package main

import (
	"encoding/json"
	"fmt"

	"github.com/pulumi/pulumi-aws/sdk/v6/go/aws"
	"github.com/pulumi/pulumi-aws/sdk/v6/go/aws/msk"
	"github.com/pulumi/pulumi/sdk/v3/go/pulumi"
)

func main() {
	pulumi.Run(func(ctx *pulumi.Context) error {
		current, err := aws.GetCallerIdentity(ctx, &aws.GetCallerIdentityArgs{}, nil)
		if err != nil {
			return err
		}
		currentGetPartition, err := aws.GetPartition(ctx, &aws.GetPartitionArgs{}, nil)
		if err != nil {
			return err
		}
		tmpJSON0, err := json.Marshal(map[string]interface{}{
			"Version": "2012-10-17",
			"Statement": []map[string]interface{}{
				map[string]interface{}{
					"Sid":    "ExampleMskClusterPolicy",
					"Effect": "Allow",
					"Principal": map[string]interface{}{
						"AWS": fmt.Sprintf("arn:%v:iam::%v:root", currentGetPartition.Partition, current.AccountId),
					},
					"Action": []string{
						"kafka:Describe*",
						"kafka:Get*",
						"kafka:CreateVpcConnection",
						"kafka:GetBootstrapBrokers",
					},
					"Resource": exampleAwsMskCluster.Arn,
				},
			},
		})
		if err != nil {
			return err
		}
		json0 := string(tmpJSON0)
		_, err = msk.NewClusterPolicy(ctx, "example", &msk.ClusterPolicyArgs{
			ClusterArn: pulumi.Any(exampleAwsMskCluster.Arn),
			Policy:     pulumi.String(json0),
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
import com.pulumi.aws.inputs.GetCallerIdentityArgs;
import com.pulumi.aws.inputs.GetPartitionArgs;
import com.pulumi.aws.msk.ClusterPolicy;
import com.pulumi.aws.msk.ClusterPolicyArgs;
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
        final var current = AwsFunctions.getCallerIdentity();

        final var currentGetPartition = AwsFunctions.getPartition();

        var example = new ClusterPolicy("example", ClusterPolicyArgs.builder()
            .clusterArn(exampleAwsMskCluster.arn())
            .policy(serializeJson(
                jsonObject(
                    jsonProperty("Version", "2012-10-17"),
                    jsonProperty("Statement", jsonArray(jsonObject(
                        jsonProperty("Sid", "ExampleMskClusterPolicy"),
                        jsonProperty("Effect", "Allow"),
                        jsonProperty("Principal", jsonObject(
                            jsonProperty("AWS", String.format("arn:%s:iam::%s:root", currentGetPartition.applyValue(getPartitionResult -> getPartitionResult.partition()),current.applyValue(getCallerIdentityResult -> getCallerIdentityResult.accountId())))
                        )),
                        jsonProperty("Action", jsonArray(
                            "kafka:Describe*", 
                            "kafka:Get*", 
                            "kafka:CreateVpcConnection", 
                            "kafka:GetBootstrapBrokers"
                        )),
                        jsonProperty("Resource", exampleAwsMskCluster.arn())
                    )))
                )))
            .build());

    }
}
```
```yaml
resources:
  example:
    type: aws:msk:ClusterPolicy
    properties:
      clusterArn: ${exampleAwsMskCluster.arn}
      policy:
        fn::toJSON:
          Version: 2012-10-17
          Statement:
            - Sid: ExampleMskClusterPolicy
              Effect: Allow
              Principal:
                AWS: arn:${currentGetPartition.partition}:iam::${current.accountId}:root
              Action:
                - kafka:Describe*
                - kafka:Get*
                - kafka:CreateVpcConnection
                - kafka:GetBootstrapBrokers
              Resource: ${exampleAwsMskCluster.arn}
variables:
  current:
    fn::invoke:
      function: aws:getCallerIdentity
      arguments: {}
  currentGetPartition:
    fn::invoke:
      function: aws:getPartition
      arguments: {}
```
<!--End PulumiCodeChooser -->

## Import

Using `pulumi import`, import Managed Streaming for Kafka Cluster Policy using the `cluster_arn`. For example:

```sh
$ pulumi import aws:msk/clusterPolicy:ClusterPolicy example arn:aws:kafka:us-west-2:123456789012:cluster/example/279c0212-d057-4dba-9aa9-1c4e5a25bfc7-3
```
W

clusterArn" EThe Amazon Resource Name (ARN) that uniquely identifies the cluster.
+
policy" Resource policy for cluster.
"W

clusterArn" EThe Amazon Resource Name (ARN) that uniquely identifies the cluster.
"
currentVersion" "+
policy" Resource policy for cluster.
*�
9
mskConfiguration#aws:msk/configuration:Configuration�Manages an Amazon Managed Streaming for Kafka configuration. More information can be found on the [MSK Developer Guide](https://docs.aws.amazon.com/msk/latest/developerguide/msk-configuration.html).

## Example Usage

<!--Start PulumiCodeChooser -->
```typescript
import * as pulumi from "@pulumi/pulumi";
import * as aws from "@pulumi/aws";

const example = new aws.msk.Configuration("example", {
    kafkaVersions: ["2.1.0"],
    name: "example",
    serverProperties: `auto.create.topics.enable = true
delete.topic.enable = true
`,
});
```
```python
import pulumi
import pulumi_aws as aws

example = aws.msk.Configuration("example",
    kafka_versions=["2.1.0"],
    name="example",
    server_properties="""auto.create.topics.enable = true
delete.topic.enable = true
""")
```
```csharp
using System.Collections.Generic;
using System.Linq;
using Pulumi;
using Aws = Pulumi.Aws;

return await Deployment.RunAsync(() => 
{
    var example = new Aws.Msk.Configuration("example", new()
    {
        KafkaVersions = new[]
        {
            "2.1.0",
        },
        Name = "example",
        ServerProperties = @"auto.create.topics.enable = true
delete.topic.enable = true
",
    });

});
```
```go
package main

import (
	"github.com/pulumi/pulumi-aws/sdk/v6/go/aws/msk"
	"github.com/pulumi/pulumi/sdk/v3/go/pulumi"
)

func main() {
	pulumi.Run(func(ctx *pulumi.Context) error {
		_, err := msk.NewConfiguration(ctx, "example", &msk.ConfigurationArgs{
			KafkaVersions: pulumi.StringArray{
				pulumi.String("2.1.0"),
			},
			Name:             pulumi.String("example"),
			ServerProperties: pulumi.String("auto.create.topics.enable = true\ndelete.topic.enable = true\n"),
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
import com.pulumi.aws.msk.Configuration;
import com.pulumi.aws.msk.ConfigurationArgs;
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
        var example = new Configuration("example", ConfigurationArgs.builder()
            .kafkaVersions("2.1.0")
            .name("example")
            .serverProperties("""
auto.create.topics.enable = true
delete.topic.enable = true
            """)
            .build());

    }
}
```
```yaml
resources:
  example:
    type: aws:msk:Configuration
    properties:
      kafkaVersions:
        - 2.1.0
      name: example
      serverProperties: |
        auto.create.topics.enable = true
        delete.topic.enable = true
```
<!--End PulumiCodeChooser -->

## Import

Using `pulumi import`, import MSK configurations using the configuration ARN. For example:

```sh
$ pulumi import aws:msk/configuration:Configuration example arn:aws:kafka:us-west-2:123456789012:configuration/example/279c0212-d057-4dba-9aa9-1c4e5a25bfc7-3
```
7
descriptionB" "Description of the configuration.
Y
kafkaVersionsB*" @List of Apache Kafka versions which can use this configuration.
)
nameB" Name of the configuration.
�
serverProperties" �Contents of the server.properties file. Supported properties are documented in the [MSK Developer Guide](https://docs.aws.amazon.com/msk/latest/developerguide/msk-configuration-properties.html).
"<
arn" 1Amazon Resource Name (ARN) of the configuration.
"7
descriptionB" "Description of the configuration.
"Y
kafkaVersionsB*" @List of Apache Kafka versions which can use this configuration.
"<
latestRevision &Latest revision of the configuration.
"'
name" Name of the configuration.
"�
serverProperties" �Contents of the server.properties file. Supported properties are documented in the [MSK Developer Guide](https://docs.aws.amazon.com/msk/latest/developerguide/msk-configuration-properties.html).
*�)
0
msk
Replicatoraws:msk/replicator:Replicator�Resource for managing an AWS Managed Streaming for Kafka Replicator.

## Example Usage

### Basic Usage

<!--Start PulumiCodeChooser -->
```java
package generated_program;

import com.pulumi.Context;
import com.pulumi.Pulumi;
import com.pulumi.core.Output;
import com.pulumi.aws.msk.Replicator;
import com.pulumi.aws.msk.ReplicatorArgs;
import com.pulumi.aws.msk.inputs.ReplicatorKafkaClusterArgs;
import com.pulumi.aws.msk.inputs.ReplicatorKafkaClusterAmazonMskClusterArgs;
import com.pulumi.aws.msk.inputs.ReplicatorKafkaClusterVpcConfigArgs;
import com.pulumi.aws.msk.inputs.ReplicatorReplicationInfoListArgs;
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
        var test = new Replicator("test", ReplicatorArgs.builder()
            .replicatorName("test-name")
            .description("test-description")
            .serviceExecutionRoleArn(sourceAwsIamRole.arn())
            .kafkaClusters(            
                ReplicatorKafkaClusterArgs.builder()
                    .amazonMskCluster(ReplicatorKafkaClusterAmazonMskClusterArgs.builder()
                        .mskClusterArn(source.arn())
                        .build())
                    .vpcConfig(ReplicatorKafkaClusterVpcConfigArgs.builder()
                        .subnetIds(sourceAwsSubnet.stream().map(element -> element.id()).collect(toList()))
                        .securityGroupsIds(sourceAwsSecurityGroup.id())
                        .build())
                    .build(),
                ReplicatorKafkaClusterArgs.builder()
                    .amazonMskCluster(ReplicatorKafkaClusterAmazonMskClusterArgs.builder()
                        .mskClusterArn(target.arn())
                        .build())
                    .vpcConfig(ReplicatorKafkaClusterVpcConfigArgs.builder()
                        .subnetIds(targetAwsSubnet.stream().map(element -> element.id()).collect(toList()))
                        .securityGroupsIds(targetAwsSecurityGroup.id())
                        .build())
                    .build())
            .replicationInfoList(ReplicatorReplicationInfoListArgs.builder()
                .sourceKafkaClusterArn(source.arn())
                .targetKafkaClusterArn(target.arn())
                .targetCompressionType("NONE")
                .topicReplications(ReplicatorReplicationInfoListTopicReplicationArgs.builder()
                    .topicConfigurationName(%!v(PANIC=Format method: runtime error: invalid memory address or nil pointer dereference))
                    .topicsToReplicates(".*")
                    .startingPosition(ReplicatorReplicationInfoListTopicReplicationStartingPositionArgs.builder()
                        .type("LATEST")
                        .build())
                    .build())
                .consumerGroupReplications(ReplicatorReplicationInfoListConsumerGroupReplicationArgs.builder()
                    .consumerGroupsToReplicates(".*")
                    .build())
                .build())
            .build());

    }
}
```
<!--End PulumiCodeChooser -->

## Import

Using `pulumi import`, import MSK replicators using the replicator ARN. For example:

```sh
$ pulumi import aws:msk/replicator:Replicator example arn:aws:kafka:us-west-2:123456789012:configuration/example/279c0212-d057-4dba-9aa9-1c4e5a25bfc7-3
```
>
descriptionB" )A summary description of the replicator.
�
kafkaClustersZ*X:V
T
mskReplicatorKafkaCluster5aws:msk/ReplicatorKafkaCluster:ReplicatorKafkaCluster>A list of Kafka clusters which are targets of the replicator.
�
replicationInfoListm:k
i
mskReplicatorReplicationInfoListCaws:msk/ReplicatorReplicationInfoList:ReplicatorReplicationInfoList�A list of replication configurations, where each configuration targets a given source cluster to target cluster replication flow.
2
replicatorName" The name of the replicator.
�
serviceExecutionRoleArn" The ARN of the IAM role used by the replicator to access resources in the customer's account (e.g source and target clusters).

tagsB2" "�
arn" �ARN of the Replicator. Do not begin the description with "An", "The", "Defines", "Indicates", or "Specifies," as these are verbose. In other words, "Indicates the amount of storage," can be rewritten as "Amount of storage," without losing any information.
"
currentVersion" ">
descriptionB" )A summary description of the replicator.
"�
kafkaClustersZ*X:V
T
mskReplicatorKafkaCluster5aws:msk/ReplicatorKafkaCluster:ReplicatorKafkaCluster>A list of Kafka clusters which are targets of the replicator.
"�
replicationInfoListm:k
i
mskReplicatorReplicationInfoListCaws:msk/ReplicatorReplicationInfoList:ReplicatorReplicationInfoList�A list of replication configurations, where each configuration targets a given source cluster to target cluster replication flow.
"2
replicatorName" The name of the replicator.
"�
serviceExecutionRoleArn" The ARN of the IAM role used by the replicator to access resources in the customer's account (e.g source and target clusters).
"
tagsB2" "
tagsAll2" *�}
T
mskScramSecretAssociation5aws:msk/scramSecretAssociation:ScramSecretAssociation�zAssociates SCRAM secrets stored in the Secrets Manager service with a Managed Streaming for Kafka (MSK) cluster.

!> This resource takes exclusive ownership over SCRAM secrets associated with a cluster. This includes removal of SCRAM secrets which are not explicitly configured. To prevent persistent drift, ensure any `aws.msk.SingleScramSecretAssociation` resources managed alongside this resource are included in the `secret_arn_list` argument.

> **Note:** The following assumes the MSK cluster has SASL/SCRAM authentication enabled. See below for example usage or refer to the [Username/Password Authentication](https://docs.aws.amazon.com/msk/latest/developerguide/msk-password.html) section of the MSK Developer Guide for more details.

To set up username and password authentication for a cluster, create an `aws.secretsmanager.Secret` resource and associate
a username and password with the secret with an `aws.secretsmanager.SecretVersion` resource. When creating a secret for the cluster,
the `name` must have the prefix `AmazonMSK_` and you must either use an existing custom AWS KMS key or create a new
custom AWS KMS key for your secret with the `aws.kms.Key` resource. It is important to note that a policy is required for the `aws.secretsmanager.Secret`
resource in order for Kafka to be able to read it. This policy is attached automatically when the `aws.msk.ScramSecretAssociation` is used,
however, this policy will not be in the state and as such, will present a diff on plan/apply. For that reason, you must use the `aws.secretsmanager.SecretPolicy`
resource](/docs/providers/aws/r/secretsmanager_secret_policy.html) as shown below in order to ensure that the state is in a clean state after the creation of secret and the association to the cluster.

## Example Usage

<!--Start PulumiCodeChooser -->
```typescript
import * as pulumi from "@pulumi/pulumi";
import * as aws from "@pulumi/aws";

const exampleCluster = new aws.msk.Cluster("example", {
    clusterName: "example",
    clientAuthentication: {
        sasl: {
            scram: true,
        },
    },
});
const exampleKey = new aws.kms.Key("example", {description: "Example Key for MSK Cluster Scram Secret Association"});
const exampleSecret = new aws.secretsmanager.Secret("example", {
    name: "AmazonMSK_example",
    kmsKeyId: exampleKey.keyId,
});
const exampleSecretVersion = new aws.secretsmanager.SecretVersion("example", {
    secretId: exampleSecret.id,
    secretString: JSON.stringify({
        username: "user",
        password: "pass",
    }),
});
const exampleScramSecretAssociation = new aws.msk.ScramSecretAssociation("example", {
    clusterArn: exampleCluster.arn,
    secretArnLists: [exampleSecret.arn],
}, {
    dependsOn: [exampleSecretVersion],
});
const example = aws.iam.getPolicyDocumentOutput({
    statements: [{
        sid: "AWSKafkaResourcePolicy",
        effect: "Allow",
        principals: [{
            type: "Service",
            identifiers: ["kafka.amazonaws.com"],
        }],
        actions: ["secretsmanager:getSecretValue"],
        resources: [exampleSecret.arn],
    }],
});
const exampleSecretPolicy = new aws.secretsmanager.SecretPolicy("example", {
    secretArn: exampleSecret.arn,
    policy: example.apply(example => example.json),
});
```
```python
import pulumi
import json
import pulumi_aws as aws

example_cluster = aws.msk.Cluster("example",
    cluster_name="example",
    client_authentication={
        "sasl": {
            "scram": True,
        },
    })
example_key = aws.kms.Key("example", description="Example Key for MSK Cluster Scram Secret Association")
example_secret = aws.secretsmanager.Secret("example",
    name="AmazonMSK_example",
    kms_key_id=example_key.key_id)
example_secret_version = aws.secretsmanager.SecretVersion("example",
    secret_id=example_secret.id,
    secret_string=json.dumps({
        "username": "user",
        "password": "pass",
    }))
example_scram_secret_association = aws.msk.ScramSecretAssociation("example",
    cluster_arn=example_cluster.arn,
    secret_arn_lists=[example_secret.arn],
    opts = pulumi.ResourceOptions(depends_on=[example_secret_version]))
example = aws.iam.get_policy_document_output(statements=[{
    "sid": "AWSKafkaResourcePolicy",
    "effect": "Allow",
    "principals": [{
        "type": "Service",
        "identifiers": ["kafka.amazonaws.com"],
    }],
    "actions": ["secretsmanager:getSecretValue"],
    "resources": [example_secret.arn],
}])
example_secret_policy = aws.secretsmanager.SecretPolicy("example",
    secret_arn=example_secret.arn,
    policy=example.json)
```
```csharp
using System.Collections.Generic;
using System.Linq;
using System.Text.Json;
using Pulumi;
using Aws = Pulumi.Aws;

return await Deployment.RunAsync(() => 
{
    var exampleCluster = new Aws.Msk.Cluster("example", new()
    {
        ClusterName = "example",
        ClientAuthentication = new Aws.Msk.Inputs.ClusterClientAuthenticationArgs
        {
            Sasl = new Aws.Msk.Inputs.ClusterClientAuthenticationSaslArgs
            {
                Scram = true,
            },
        },
    });

    var exampleKey = new Aws.Kms.Key("example", new()
    {
        Description = "Example Key for MSK Cluster Scram Secret Association",
    });

    var exampleSecret = new Aws.SecretsManager.Secret("example", new()
    {
        Name = "AmazonMSK_example",
        KmsKeyId = exampleKey.KeyId,
    });

    var exampleSecretVersion = new Aws.SecretsManager.SecretVersion("example", new()
    {
        SecretId = exampleSecret.Id,
        SecretString = JsonSerializer.Serialize(new Dictionary<string, object?>
        {
            ["username"] = "user",
            ["password"] = "pass",
        }),
    });

    var exampleScramSecretAssociation = new Aws.Msk.ScramSecretAssociation("example", new()
    {
        ClusterArn = exampleCluster.Arn,
        SecretArnLists = new[]
        {
            exampleSecret.Arn,
        },
    }, new CustomResourceOptions
    {
        DependsOn =
        {
            exampleSecretVersion,
        },
    });

    var example = Aws.Iam.GetPolicyDocument.Invoke(new()
    {
        Statements = new[]
        {
            new Aws.Iam.Inputs.GetPolicyDocumentStatementInputArgs
            {
                Sid = "AWSKafkaResourcePolicy",
                Effect = "Allow",
                Principals = new[]
                {
                    new Aws.Iam.Inputs.GetPolicyDocumentStatementPrincipalInputArgs
                    {
                        Type = "Service",
                        Identifiers = new[]
                        {
                            "kafka.amazonaws.com",
                        },
                    },
                },
                Actions = new[]
                {
                    "secretsmanager:getSecretValue",
                },
                Resources = new[]
                {
                    exampleSecret.Arn,
                },
            },
        },
    });

    var exampleSecretPolicy = new Aws.SecretsManager.SecretPolicy("example", new()
    {
        SecretArn = exampleSecret.Arn,
        Policy = example.Apply(getPolicyDocumentResult => getPolicyDocumentResult.Json),
    });

});
```
```go
package main

import (
	"encoding/json"

	"github.com/pulumi/pulumi-aws/sdk/v6/go/aws/iam"
	"github.com/pulumi/pulumi-aws/sdk/v6/go/aws/kms"
	"github.com/pulumi/pulumi-aws/sdk/v6/go/aws/msk"
	"github.com/pulumi/pulumi-aws/sdk/v6/go/aws/secretsmanager"
	"github.com/pulumi/pulumi/sdk/v3/go/pulumi"
)

func main() {
	pulumi.Run(func(ctx *pulumi.Context) error {
		exampleCluster, err := msk.NewCluster(ctx, "example", &msk.ClusterArgs{
			ClusterName: pulumi.String("example"),
			ClientAuthentication: &msk.ClusterClientAuthenticationArgs{
				Sasl: &msk.ClusterClientAuthenticationSaslArgs{
					Scram: pulumi.Bool(true),
				},
			},
		})
		if err != nil {
			return err
		}
		exampleKey, err := kms.NewKey(ctx, "example", &kms.KeyArgs{
			Description: pulumi.String("Example Key for MSK Cluster Scram Secret Association"),
		})
		if err != nil {
			return err
		}
		exampleSecret, err := secretsmanager.NewSecret(ctx, "example", &secretsmanager.SecretArgs{
			Name:     pulumi.String("AmazonMSK_example"),
			KmsKeyId: exampleKey.KeyId,
		})
		if err != nil {
			return err
		}
		tmpJSON0, err := json.Marshal(map[string]interface{}{
			"username": "user",
			"password": "pass",
		})
		if err != nil {
			return err
		}
		json0 := string(tmpJSON0)
		exampleSecretVersion, err := secretsmanager.NewSecretVersion(ctx, "example", &secretsmanager.SecretVersionArgs{
			SecretId:     exampleSecret.ID(),
			SecretString: pulumi.String(json0),
		})
		if err != nil {
			return err
		}
		_, err = msk.NewScramSecretAssociation(ctx, "example", &msk.ScramSecretAssociationArgs{
			ClusterArn: exampleCluster.Arn,
			SecretArnLists: pulumi.StringArray{
				exampleSecret.Arn,
			},
		}, pulumi.DependsOn([]pulumi.Resource{
			exampleSecretVersion,
		}))
		if err != nil {
			return err
		}
		example := iam.GetPolicyDocumentOutput(ctx, iam.GetPolicyDocumentOutputArgs{
			Statements: iam.GetPolicyDocumentStatementArray{
				&iam.GetPolicyDocumentStatementArgs{
					Sid:    pulumi.String("AWSKafkaResourcePolicy"),
					Effect: pulumi.String("Allow"),
					Principals: iam.GetPolicyDocumentStatementPrincipalArray{
						&iam.GetPolicyDocumentStatementPrincipalArgs{
							Type: pulumi.String("Service"),
							Identifiers: pulumi.StringArray{
								pulumi.String("kafka.amazonaws.com"),
							},
						},
					},
					Actions: pulumi.StringArray{
						pulumi.String("secretsmanager:getSecretValue"),
					},
					Resources: pulumi.StringArray{
						exampleSecret.Arn,
					},
				},
			},
		}, nil)
		_, err = secretsmanager.NewSecretPolicy(ctx, "example", &secretsmanager.SecretPolicyArgs{
			SecretArn: exampleSecret.Arn,
			Policy: pulumi.String(example.ApplyT(func(example iam.GetPolicyDocumentResult) (*string, error) {
				return &example.Json, nil
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
import com.pulumi.aws.msk.Cluster;
import com.pulumi.aws.msk.ClusterArgs;
import com.pulumi.aws.msk.inputs.ClusterClientAuthenticationArgs;
import com.pulumi.aws.msk.inputs.ClusterClientAuthenticationSaslArgs;
import com.pulumi.aws.kms.Key;
import com.pulumi.aws.kms.KeyArgs;
import com.pulumi.aws.secretsmanager.Secret;
import com.pulumi.aws.secretsmanager.SecretArgs;
import com.pulumi.aws.secretsmanager.SecretVersion;
import com.pulumi.aws.secretsmanager.SecretVersionArgs;
import com.pulumi.aws.msk.ScramSecretAssociation;
import com.pulumi.aws.msk.ScramSecretAssociationArgs;
import com.pulumi.aws.iam.IamFunctions;
import com.pulumi.aws.iam.inputs.GetPolicyDocumentArgs;
import com.pulumi.aws.secretsmanager.SecretPolicy;
import com.pulumi.aws.secretsmanager.SecretPolicyArgs;
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
        var exampleCluster = new Cluster("exampleCluster", ClusterArgs.builder()
            .clusterName("example")
            .clientAuthentication(ClusterClientAuthenticationArgs.builder()
                .sasl(ClusterClientAuthenticationSaslArgs.builder()
                    .scram(true)
                    .build())
                .build())
            .build());

        var exampleKey = new Key("exampleKey", KeyArgs.builder()
            .description("Example Key for MSK Cluster Scram Secret Association")
            .build());

        var exampleSecret = new Secret("exampleSecret", SecretArgs.builder()
            .name("AmazonMSK_example")
            .kmsKeyId(exampleKey.keyId())
            .build());

        var exampleSecretVersion = new SecretVersion("exampleSecretVersion", SecretVersionArgs.builder()
            .secretId(exampleSecret.id())
            .secretString(serializeJson(
                jsonObject(
                    jsonProperty("username", "user"),
                    jsonProperty("password", "pass")
                )))
            .build());

        var exampleScramSecretAssociation = new ScramSecretAssociation("exampleScramSecretAssociation", ScramSecretAssociationArgs.builder()
            .clusterArn(exampleCluster.arn())
            .secretArnLists(exampleSecret.arn())
            .build(), CustomResourceOptions.builder()
                .dependsOn(exampleSecretVersion)
                .build());

        final var example = IamFunctions.getPolicyDocument(GetPolicyDocumentArgs.builder()
            .statements(GetPolicyDocumentStatementArgs.builder()
                .sid("AWSKafkaResourcePolicy")
                .effect("Allow")
                .principals(GetPolicyDocumentStatementPrincipalArgs.builder()
                    .type("Service")
                    .identifiers("kafka.amazonaws.com")
                    .build())
                .actions("secretsmanager:getSecretValue")
                .resources(exampleSecret.arn())
                .build())
            .build());

        var exampleSecretPolicy = new SecretPolicy("exampleSecretPolicy", SecretPolicyArgs.builder()
            .secretArn(exampleSecret.arn())
            .policy(example.applyValue(getPolicyDocumentResult -> getPolicyDocumentResult).applyValue(example -> example.applyValue(getPolicyDocumentResult -> getPolicyDocumentResult.json())))
            .build());

    }
}
```
```yaml
resources:
  exampleScramSecretAssociation:
    type: aws:msk:ScramSecretAssociation
    name: example
    properties:
      clusterArn: ${exampleCluster.arn}
      secretArnLists:
        - ${exampleSecret.arn}
    options:
      dependsOn:
        - ${exampleSecretVersion}
  exampleCluster:
    type: aws:msk:Cluster
    name: example
    properties:
      clusterName: example
      clientAuthentication:
        sasl:
          scram: true
  exampleSecret:
    type: aws:secretsmanager:Secret
    name: example
    properties:
      name: AmazonMSK_example
      kmsKeyId: ${exampleKey.keyId}
  exampleKey:
    type: aws:kms:Key
    name: example
    properties:
      description: Example Key for MSK Cluster Scram Secret Association
  exampleSecretVersion:
    type: aws:secretsmanager:SecretVersion
    name: example
    properties:
      secretId: ${exampleSecret.id}
      secretString:
        fn::toJSON:
          username: user
          password: pass
  exampleSecretPolicy:
    type: aws:secretsmanager:SecretPolicy
    name: example
    properties:
      secretArn: ${exampleSecret.arn}
      policy: ${example.json}
variables:
  example:
    fn::invoke:
      function: aws:iam:getPolicyDocument
      arguments:
        statements:
          - sid: AWSKafkaResourcePolicy
            effect: Allow
            principals:
              - type: Service
                identifiers:
                  - kafka.amazonaws.com
            actions:
              - secretsmanager:getSecretValue
            resources:
              - ${exampleSecret.arn}
```
<!--End PulumiCodeChooser -->

## Import

Using `pulumi import`, import MSK SCRAM Secret Associations using the `id`. For example:

```sh
$ pulumi import aws:msk/scramSecretAssociation:ScramSecretAssociation example arn:aws:kafka:us-west-2:123456789012:cluster/example/279c0212-d057-4dba-9aa9-1c4e5a25bfc7-3
```
A

clusterArn" /Amazon Resource Name (ARN) of the MSK cluster.
A
secretArnLists*" )List of AWS Secrets Manager secret ARNs.
"A

clusterArn" /Amazon Resource Name (ARN) of the MSK cluster.
"A
secretArnLists*" )List of AWS Secrets Manager secret ARNs.
*�3
E
mskServerlessCluster+aws:msk/serverlessCluster:ServerlessCluster�&Manages an Amazon MSK Serverless cluster.

> **Note:** To manage a _provisioned_ Amazon MSK cluster, use the `aws.msk.Cluster` resource.

## Example Usage

<!--Start PulumiCodeChooser -->
```typescript
import * as pulumi from "@pulumi/pulumi";
import * as aws from "@pulumi/aws";

const example = new aws.msk.ServerlessCluster("example", {
    clusterName: "Example",
    vpcConfigs: [{
        subnetIds: exampleAwsSubnet.map(__item => __item.id),
        securityGroupIds: [exampleAwsSecurityGroup.id],
    }],
    clientAuthentication: {
        sasl: {
            iam: {
                enabled: true,
            },
        },
    },
});
```
```python
import pulumi
import pulumi_aws as aws

example = aws.msk.ServerlessCluster("example",
    cluster_name="Example",
    vpc_configs=[{
        "subnet_ids": [__item["id"] for __item in example_aws_subnet],
        "security_group_ids": [example_aws_security_group["id"]],
    }],
    client_authentication={
        "sasl": {
            "iam": {
                "enabled": True,
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
    var example = new Aws.Msk.ServerlessCluster("example", new()
    {
        ClusterName = "Example",
        VpcConfigs = new[]
        {
            new Aws.Msk.Inputs.ServerlessClusterVpcConfigArgs
            {
                SubnetIds = exampleAwsSubnet.Select(__item => __item.Id).ToList(),
                SecurityGroupIds = new[]
                {
                    exampleAwsSecurityGroup.Id,
                },
            },
        },
        ClientAuthentication = new Aws.Msk.Inputs.ServerlessClusterClientAuthenticationArgs
        {
            Sasl = new Aws.Msk.Inputs.ServerlessClusterClientAuthenticationSaslArgs
            {
                Iam = new Aws.Msk.Inputs.ServerlessClusterClientAuthenticationSaslIamArgs
                {
                    Enabled = true,
                },
            },
        },
    });

});
```
```go
package main

import (
	"github.com/pulumi/pulumi-aws/sdk/v6/go/aws/msk"
	"github.com/pulumi/pulumi/sdk/v3/go/pulumi"
)
func main() {
pulumi.Run(func(ctx *pulumi.Context) error {
_, err := msk.NewServerlessCluster(ctx, "example", &msk.ServerlessClusterArgs{
ClusterName: pulumi.String("Example"),
VpcConfigs: msk.ServerlessClusterVpcConfigArray{
&msk.ServerlessClusterVpcConfigArgs{
SubnetIds: []pulumi.String(%!v(PANIC=Format method: fatal: A failure has occurred: unlowered splat expression @ example.pp:3,24-46)),
SecurityGroupIds: pulumi.StringArray{
exampleAwsSecurityGroup.Id,
},
},
},
ClientAuthentication: &msk.ServerlessClusterClientAuthenticationArgs{
Sasl: &msk.ServerlessClusterClientAuthenticationSaslArgs{
Iam: &msk.ServerlessClusterClientAuthenticationSaslIamArgs{
Enabled: pulumi.Bool(true),
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
import com.pulumi.aws.msk.ServerlessCluster;
import com.pulumi.aws.msk.ServerlessClusterArgs;
import com.pulumi.aws.msk.inputs.ServerlessClusterVpcConfigArgs;
import com.pulumi.aws.msk.inputs.ServerlessClusterClientAuthenticationArgs;
import com.pulumi.aws.msk.inputs.ServerlessClusterClientAuthenticationSaslArgs;
import com.pulumi.aws.msk.inputs.ServerlessClusterClientAuthenticationSaslIamArgs;
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
        var example = new ServerlessCluster("example", ServerlessClusterArgs.builder()
            .clusterName("Example")
            .vpcConfigs(ServerlessClusterVpcConfigArgs.builder()
                .subnetIds(exampleAwsSubnet.stream().map(element -> element.id()).collect(toList()))
                .securityGroupIds(exampleAwsSecurityGroup.id())
                .build())
            .clientAuthentication(ServerlessClusterClientAuthenticationArgs.builder()
                .sasl(ServerlessClusterClientAuthenticationSaslArgs.builder()
                    .iam(ServerlessClusterClientAuthenticationSaslIamArgs.builder()
                        .enabled(true)
                        .build())
                    .build())
                .build())
            .build());

    }
}
```
<!--End PulumiCodeChooser -->

## Import

Using `pulumi import`, import MSK serverless clusters using the cluster `arn`. For example:

```sh
$ pulumi import aws:msk/serverlessCluster:ServerlessCluster example arn:aws:kafka:us-west-2:123456789012:cluster/example/279c0212-d057-4dba-9aa9-1c4e5a25bfc7-3
```
�
clientAuthentication�:�
�
msk%ServerlessClusterClientAuthenticationSaws:msk/ServerlessClusterClientAuthentication:ServerlessClusterClientAuthenticationSSpecifies client authentication information for the serverless cluster. See below.
9
clusterNameB" $The name of the serverless cluster.
�
tagsB2" �A map of tags to assign to the resource. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
�

vpcConfigsf*d:b
`
mskServerlessClusterVpcConfig=aws:msk/ServerlessClusterVpcConfig:ServerlessClusterVpcConfig*VPC configuration information. See below.
".
arn" #The ARN of the serverless cluster.
"�
clientAuthentication�:�
�
msk%ServerlessClusterClientAuthenticationSaws:msk/ServerlessClusterClientAuthentication:ServerlessClusterClientAuthenticationSSpecifies client authentication information for the serverless cluster. See below.
"7
clusterName" $The name of the serverless cluster.
"L
clusterUuid" 9UUID of the serverless cluster, for use in IAM policies.
"�
tagsB2" �A map of tags to assign to the resource. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
"�
tagsAll2" xA map of tags assigned to the resource, including those inherited from the provider `default_tags` configuration block.
"�

vpcConfigsf*d:b
`
mskServerlessClusterVpcConfig=aws:msk/ServerlessClusterVpcConfig:ServerlessClusterVpcConfig*VPC configuration information. See below.
*�
f
mskSingleScramSecretAssociationAaws:msk/singleScramSecretAssociation:SingleScramSecretAssociation�Associates a single SCRAM secret with a Managed Streaming for Kafka (MSK) cluster.

## Example Usage

<!--Start PulumiCodeChooser -->
```typescript
import * as pulumi from "@pulumi/pulumi";
import * as aws from "@pulumi/aws";

const example = new aws.msk.SingleScramSecretAssociation("example", {
    clusterArn: exampleAwsMskCluster.arn,
    secretArn: exampleAwsSecretsmanagerSecret.arn,
});
```
```python
import pulumi
import pulumi_aws as aws

example = aws.msk.SingleScramSecretAssociation("example",
    cluster_arn=example_aws_msk_cluster["arn"],
    secret_arn=example_aws_secretsmanager_secret["arn"])
```
```csharp
using System.Collections.Generic;
using System.Linq;
using Pulumi;
using Aws = Pulumi.Aws;

return await Deployment.RunAsync(() => 
{
    var example = new Aws.Msk.SingleScramSecretAssociation("example", new()
    {
        ClusterArn = exampleAwsMskCluster.Arn,
        SecretArn = exampleAwsSecretsmanagerSecret.Arn,
    });

});
```
```go
package main

import (
	"github.com/pulumi/pulumi-aws/sdk/v6/go/aws/msk"
	"github.com/pulumi/pulumi/sdk/v3/go/pulumi"
)

func main() {
	pulumi.Run(func(ctx *pulumi.Context) error {
		_, err := msk.NewSingleScramSecretAssociation(ctx, "example", &msk.SingleScramSecretAssociationArgs{
			ClusterArn: pulumi.Any(exampleAwsMskCluster.Arn),
			SecretArn:  pulumi.Any(exampleAwsSecretsmanagerSecret.Arn),
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
import com.pulumi.aws.msk.SingleScramSecretAssociation;
import com.pulumi.aws.msk.SingleScramSecretAssociationArgs;
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
        var example = new SingleScramSecretAssociation("example", SingleScramSecretAssociationArgs.builder()
            .clusterArn(exampleAwsMskCluster.arn())
            .secretArn(exampleAwsSecretsmanagerSecret.arn())
            .build());

    }
}
```
```yaml
resources:
  example:
    type: aws:msk:SingleScramSecretAssociation
    properties:
      clusterArn: ${exampleAwsMskCluster.arn}
      secretArn: ${exampleAwsSecretsmanagerSecret.arn}
```
<!--End PulumiCodeChooser -->

## Import

Using `pulumi import`, import an MSK SCRAM Secret Association using the `cluster_arn` and `secret_arn`. For example:

```sh
$ pulumi import aws:msk/singleScramSecretAssociation:SingleScramSecretAssociation example arn:aws:kafka:us-west-2:123456789012:cluster/example/279c0212-d057-4dba-9aa9-1c4e5a25bfc7-3,arn:aws:secretsmanager:us-east-1:123456789012:secret:example-123456
```
A

clusterArn" /Amazon Resource Name (ARN) of the MSK cluster.
1
	secretArn"  AWS Secrets Manager secret ARN.
"A

clusterArn" /Amazon Resource Name (ARN) of the MSK cluster.
"1
	secretArn"  AWS Secrets Manager secret ARN.
*�&
9
mskVpcConnection#aws:msk/vpcConnection:VpcConnection�Resource for managing an AWS Managed Streaming for Kafka VPC Connection.

## Example Usage

<!--Start PulumiCodeChooser -->
```typescript
import * as pulumi from "@pulumi/pulumi";
import * as aws from "@pulumi/aws";

const test = new aws.msk.VpcConnection("test", {
    authentication: "SASL_IAM",
    targetClusterArn: "aws_msk_cluster.arn",
    vpcId: testAwsVpc.id,
    clientSubnets: testAwsSubnet.map(__item => __item.id),
    securityGroups: [testAwsSecurityGroup.id],
});
```
```python
import pulumi
import pulumi_aws as aws

test = aws.msk.VpcConnection("test",
    authentication="SASL_IAM",
    target_cluster_arn="aws_msk_cluster.arn",
    vpc_id=test_aws_vpc["id"],
    client_subnets=[__item["id"] for __item in test_aws_subnet],
    security_groups=[test_aws_security_group["id"]])
```
```csharp
using System.Collections.Generic;
using System.Linq;
using Pulumi;
using Aws = Pulumi.Aws;

return await Deployment.RunAsync(() => 
{
    var test = new Aws.Msk.VpcConnection("test", new()
    {
        Authentication = "SASL_IAM",
        TargetClusterArn = "aws_msk_cluster.arn",
        VpcId = testAwsVpc.Id,
        ClientSubnets = testAwsSubnet.Select(__item => __item.Id).ToList(),
        SecurityGroups = new[]
        {
            testAwsSecurityGroup.Id,
        },
    });

});
```
```go
package main

import (
	"github.com/pulumi/pulumi-aws/sdk/v6/go/aws/msk"
	"github.com/pulumi/pulumi/sdk/v3/go/pulumi"
)
func main() {
pulumi.Run(func(ctx *pulumi.Context) error {
var splat0 []interface{}
for _, val0 := range testAwsSubnet {
splat0 = append(splat0, val0.Id)
}
_, err := msk.NewVpcConnection(ctx, "test", &msk.VpcConnectionArgs{
Authentication: pulumi.String("SASL_IAM"),
TargetClusterArn: pulumi.String("aws_msk_cluster.arn"),
VpcId: pulumi.Any(testAwsVpc.Id),
ClientSubnets: toPulumiArray(splat0),
SecurityGroups: pulumi.StringArray{
testAwsSecurityGroup.Id,
},
})
if err != nil {
return err
}
return nil
})
}
func toPulumiArray(arr []) pulumi.Array {
var pulumiArr pulumi.Array
for _, v := range arr {
pulumiArr = append(pulumiArr, pulumi.(v))
}
return pulumiArr
}
```
```java
package generated_program;

import com.pulumi.Context;
import com.pulumi.Pulumi;
import com.pulumi.core.Output;
import com.pulumi.aws.msk.VpcConnection;
import com.pulumi.aws.msk.VpcConnectionArgs;
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
        var test = new VpcConnection("test", VpcConnectionArgs.builder()
            .authentication("SASL_IAM")
            .targetClusterArn("aws_msk_cluster.arn")
            .vpcId(testAwsVpc.id())
            .clientSubnets(testAwsSubnet.stream().map(element -> element.id()).collect(toList()))
            .securityGroups(testAwsSecurityGroup.id())
            .build());

    }
}
```
<!--End PulumiCodeChooser -->

## Import

Using `pulumi import`, import MSK configurations using the configuration ARN. For example:

```sh
$ pulumi import aws:msk/vpcConnection:VpcConnection example arn:aws:kafka:eu-west-2:123456789012:vpc-connection/123456789012/example/38173259-79cd-4ee8-87f3-682ea6023f48-2
```
�
authentication" }The authentication type for the client VPC connection. Specify one of these auth type strings: SASL_IAM, SASL_SCRAM, or TLS.
L
clientSubnets*" 5The list of subnets in the client VPC to connect to.
X
securityGroups*" @The security groups to attach to the ENIs for the broker nodes.
�
tagsB2" �A map of tags to assign to the resource. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
G
targetClusterArn" /The Amazon Resource Name (ARN) of the cluster.
.
vpcId" !The VPC ID of the remote client.
"=
arn" 2Amazon Resource Name (ARN) of the VPC connection.
"�
authentication" }The authentication type for the client VPC connection. Specify one of these auth type strings: SASL_IAM, SASL_SCRAM, or TLS.
"L
clientSubnets*" 5The list of subnets in the client VPC to connect to.
"X
securityGroups*" @The security groups to attach to the ENIs for the broker nodes.
"�
tagsB2" �A map of tags to assign to the resource. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
"�
tagsAll2" xA map of tags assigned to the resource, including those inherited from the provider `default_tags` configuration block.
"G
targetClusterArn" /The Amazon Resource Name (ARN) of the cluster.
".
vpcId" !The VPC ID of the remote client.
*՗
;

mskconnect	Connector"aws:mskconnect/connector:Connector�kProvides an Amazon MSK Connect Connector resource.

## Example Usage

### Basic configuration

<!--Start PulumiCodeChooser -->
```typescript
import * as pulumi from "@pulumi/pulumi";
import * as aws from "@pulumi/aws";

const example = new aws.mskconnect.Connector("example", {
    name: "example",
    kafkaconnectVersion: "2.7.1",
    capacity: {
        autoscaling: {
            mcuCount: 1,
            minWorkerCount: 1,
            maxWorkerCount: 2,
            scaleInPolicy: {
                cpuUtilizationPercentage: 20,
            },
            scaleOutPolicy: {
                cpuUtilizationPercentage: 80,
            },
        },
    },
    connectorConfiguration: {
        "connector.class": "com.github.jcustenborder.kafka.connect.simulator.SimulatorSinkConnector",
        "tasks.max": "1",
        topics: "example",
    },
    kafkaCluster: {
        apacheKafkaCluster: {
            bootstrapServers: exampleAwsMskCluster.bootstrapBrokersTls,
            vpc: {
                securityGroups: [exampleAwsSecurityGroup.id],
                subnets: [
                    example1.id,
                    example2.id,
                    example3.id,
                ],
            },
        },
    },
    kafkaClusterClientAuthentication: {
        authenticationType: "NONE",
    },
    kafkaClusterEncryptionInTransit: {
        encryptionType: "TLS",
    },
    plugins: [{
        customPlugin: {
            arn: exampleAwsMskconnectCustomPlugin.arn,
            revision: exampleAwsMskconnectCustomPlugin.latestRevision,
        },
    }],
    serviceExecutionRoleArn: exampleAwsIamRole.arn,
});
```
```python
import pulumi
import pulumi_aws as aws

example = aws.mskconnect.Connector("example",
    name="example",
    kafkaconnect_version="2.7.1",
    capacity={
        "autoscaling": {
            "mcu_count": 1,
            "min_worker_count": 1,
            "max_worker_count": 2,
            "scale_in_policy": {
                "cpu_utilization_percentage": 20,
            },
            "scale_out_policy": {
                "cpu_utilization_percentage": 80,
            },
        },
    },
    connector_configuration={
        "connector.class": "com.github.jcustenborder.kafka.connect.simulator.SimulatorSinkConnector",
        "tasks.max": "1",
        "topics": "example",
    },
    kafka_cluster={
        "apache_kafka_cluster": {
            "bootstrap_servers": example_aws_msk_cluster["bootstrapBrokersTls"],
            "vpc": {
                "security_groups": [example_aws_security_group["id"]],
                "subnets": [
                    example1["id"],
                    example2["id"],
                    example3["id"],
                ],
            },
        },
    },
    kafka_cluster_client_authentication={
        "authentication_type": "NONE",
    },
    kafka_cluster_encryption_in_transit={
        "encryption_type": "TLS",
    },
    plugins=[{
        "custom_plugin": {
            "arn": example_aws_mskconnect_custom_plugin["arn"],
            "revision": example_aws_mskconnect_custom_plugin["latestRevision"],
        },
    }],
    service_execution_role_arn=example_aws_iam_role["arn"])
```
```csharp
using System.Collections.Generic;
using System.Linq;
using Pulumi;
using Aws = Pulumi.Aws;

return await Deployment.RunAsync(() => 
{
    var example = new Aws.MskConnect.Connector("example", new()
    {
        Name = "example",
        KafkaconnectVersion = "2.7.1",
        Capacity = new Aws.MskConnect.Inputs.ConnectorCapacityArgs
        {
            Autoscaling = new Aws.MskConnect.Inputs.ConnectorCapacityAutoscalingArgs
            {
                McuCount = 1,
                MinWorkerCount = 1,
                MaxWorkerCount = 2,
                ScaleInPolicy = new Aws.MskConnect.Inputs.ConnectorCapacityAutoscalingScaleInPolicyArgs
                {
                    CpuUtilizationPercentage = 20,
                },
                ScaleOutPolicy = new Aws.MskConnect.Inputs.ConnectorCapacityAutoscalingScaleOutPolicyArgs
                {
                    CpuUtilizationPercentage = 80,
                },
            },
        },
        ConnectorConfiguration = 
        {
            { "connector.class", "com.github.jcustenborder.kafka.connect.simulator.SimulatorSinkConnector" },
            { "tasks.max", "1" },
            { "topics", "example" },
        },
        KafkaCluster = new Aws.MskConnect.Inputs.ConnectorKafkaClusterArgs
        {
            ApacheKafkaCluster = new Aws.MskConnect.Inputs.ConnectorKafkaClusterApacheKafkaClusterArgs
            {
                BootstrapServers = exampleAwsMskCluster.BootstrapBrokersTls,
                Vpc = new Aws.MskConnect.Inputs.ConnectorKafkaClusterApacheKafkaClusterVpcArgs
                {
                    SecurityGroups = new[]
                    {
                        exampleAwsSecurityGroup.Id,
                    },
                    Subnets = new[]
                    {
                        example1.Id,
                        example2.Id,
                        example3.Id,
                    },
                },
            },
        },
        KafkaClusterClientAuthentication = new Aws.MskConnect.Inputs.ConnectorKafkaClusterClientAuthenticationArgs
        {
            AuthenticationType = "NONE",
        },
        KafkaClusterEncryptionInTransit = new Aws.MskConnect.Inputs.ConnectorKafkaClusterEncryptionInTransitArgs
        {
            EncryptionType = "TLS",
        },
        Plugins = new[]
        {
            new Aws.MskConnect.Inputs.ConnectorPluginArgs
            {
                CustomPlugin = new Aws.MskConnect.Inputs.ConnectorPluginCustomPluginArgs
                {
                    Arn = exampleAwsMskconnectCustomPlugin.Arn,
                    Revision = exampleAwsMskconnectCustomPlugin.LatestRevision,
                },
            },
        },
        ServiceExecutionRoleArn = exampleAwsIamRole.Arn,
    });

});
```
```go
package main

import (
	"github.com/pulumi/pulumi-aws/sdk/v6/go/aws/mskconnect"
	"github.com/pulumi/pulumi/sdk/v3/go/pulumi"
)

func main() {
	pulumi.Run(func(ctx *pulumi.Context) error {
		_, err := mskconnect.NewConnector(ctx, "example", &mskconnect.ConnectorArgs{
			Name:                pulumi.String("example"),
			KafkaconnectVersion: pulumi.String("2.7.1"),
			Capacity: &mskconnect.ConnectorCapacityArgs{
				Autoscaling: &mskconnect.ConnectorCapacityAutoscalingArgs{
					McuCount:       pulumi.Int(1),
					MinWorkerCount: pulumi.Int(1),
					MaxWorkerCount: pulumi.Int(2),
					ScaleInPolicy: &mskconnect.ConnectorCapacityAutoscalingScaleInPolicyArgs{
						CpuUtilizationPercentage: pulumi.Int(20),
					},
					ScaleOutPolicy: &mskconnect.ConnectorCapacityAutoscalingScaleOutPolicyArgs{
						CpuUtilizationPercentage: pulumi.Int(80),
					},
				},
			},
			ConnectorConfiguration: pulumi.StringMap{
				"connector.class": pulumi.String("com.github.jcustenborder.kafka.connect.simulator.SimulatorSinkConnector"),
				"tasks.max":       pulumi.String("1"),
				"topics":          pulumi.String("example"),
			},
			KafkaCluster: &mskconnect.ConnectorKafkaClusterArgs{
				ApacheKafkaCluster: &mskconnect.ConnectorKafkaClusterApacheKafkaClusterArgs{
					BootstrapServers: pulumi.Any(exampleAwsMskCluster.BootstrapBrokersTls),
					Vpc: &mskconnect.ConnectorKafkaClusterApacheKafkaClusterVpcArgs{
						SecurityGroups: pulumi.StringArray{
							exampleAwsSecurityGroup.Id,
						},
						Subnets: pulumi.StringArray{
							example1.Id,
							example2.Id,
							example3.Id,
						},
					},
				},
			},
			KafkaClusterClientAuthentication: &mskconnect.ConnectorKafkaClusterClientAuthenticationArgs{
				AuthenticationType: pulumi.String("NONE"),
			},
			KafkaClusterEncryptionInTransit: &mskconnect.ConnectorKafkaClusterEncryptionInTransitArgs{
				EncryptionType: pulumi.String("TLS"),
			},
			Plugins: mskconnect.ConnectorPluginArray{
				&mskconnect.ConnectorPluginArgs{
					CustomPlugin: &mskconnect.ConnectorPluginCustomPluginArgs{
						Arn:      pulumi.Any(exampleAwsMskconnectCustomPlugin.Arn),
						Revision: pulumi.Any(exampleAwsMskconnectCustomPlugin.LatestRevision),
					},
				},
			},
			ServiceExecutionRoleArn: pulumi.Any(exampleAwsIamRole.Arn),
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
import com.pulumi.aws.mskconnect.Connector;
import com.pulumi.aws.mskconnect.ConnectorArgs;
import com.pulumi.aws.mskconnect.inputs.ConnectorCapacityArgs;
import com.pulumi.aws.mskconnect.inputs.ConnectorCapacityAutoscalingArgs;
import com.pulumi.aws.mskconnect.inputs.ConnectorCapacityAutoscalingScaleInPolicyArgs;
import com.pulumi.aws.mskconnect.inputs.ConnectorCapacityAutoscalingScaleOutPolicyArgs;
import com.pulumi.aws.mskconnect.inputs.ConnectorKafkaClusterArgs;
import com.pulumi.aws.mskconnect.inputs.ConnectorKafkaClusterApacheKafkaClusterArgs;
import com.pulumi.aws.mskconnect.inputs.ConnectorKafkaClusterApacheKafkaClusterVpcArgs;
import com.pulumi.aws.mskconnect.inputs.ConnectorKafkaClusterClientAuthenticationArgs;
import com.pulumi.aws.mskconnect.inputs.ConnectorKafkaClusterEncryptionInTransitArgs;
import com.pulumi.aws.mskconnect.inputs.ConnectorPluginArgs;
import com.pulumi.aws.mskconnect.inputs.ConnectorPluginCustomPluginArgs;
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
        var example = new Connector("example", ConnectorArgs.builder()
            .name("example")
            .kafkaconnectVersion("2.7.1")
            .capacity(ConnectorCapacityArgs.builder()
                .autoscaling(ConnectorCapacityAutoscalingArgs.builder()
                    .mcuCount(1)
                    .minWorkerCount(1)
                    .maxWorkerCount(2)
                    .scaleInPolicy(ConnectorCapacityAutoscalingScaleInPolicyArgs.builder()
                        .cpuUtilizationPercentage(20)
                        .build())
                    .scaleOutPolicy(ConnectorCapacityAutoscalingScaleOutPolicyArgs.builder()
                        .cpuUtilizationPercentage(80)
                        .build())
                    .build())
                .build())
            .connectorConfiguration(Map.ofEntries(
                Map.entry("connector.class", "com.github.jcustenborder.kafka.connect.simulator.SimulatorSinkConnector"),
                Map.entry("tasks.max", "1"),
                Map.entry("topics", "example")
            ))
            .kafkaCluster(ConnectorKafkaClusterArgs.builder()
                .apacheKafkaCluster(ConnectorKafkaClusterApacheKafkaClusterArgs.builder()
                    .bootstrapServers(exampleAwsMskCluster.bootstrapBrokersTls())
                    .vpc(ConnectorKafkaClusterApacheKafkaClusterVpcArgs.builder()
                        .securityGroups(exampleAwsSecurityGroup.id())
                        .subnets(                        
                            example1.id(),
                            example2.id(),
                            example3.id())
                        .build())
                    .build())
                .build())
            .kafkaClusterClientAuthentication(ConnectorKafkaClusterClientAuthenticationArgs.builder()
                .authenticationType("NONE")
                .build())
            .kafkaClusterEncryptionInTransit(ConnectorKafkaClusterEncryptionInTransitArgs.builder()
                .encryptionType("TLS")
                .build())
            .plugins(ConnectorPluginArgs.builder()
                .customPlugin(ConnectorPluginCustomPluginArgs.builder()
                    .arn(exampleAwsMskconnectCustomPlugin.arn())
                    .revision(exampleAwsMskconnectCustomPlugin.latestRevision())
                    .build())
                .build())
            .serviceExecutionRoleArn(exampleAwsIamRole.arn())
            .build());

    }
}
```
```yaml
resources:
  example:
    type: aws:mskconnect:Connector
    properties:
      name: example
      kafkaconnectVersion: 2.7.1
      capacity:
        autoscaling:
          mcuCount: 1
          minWorkerCount: 1
          maxWorkerCount: 2
          scaleInPolicy:
            cpuUtilizationPercentage: 20
          scaleOutPolicy:
            cpuUtilizationPercentage: 80
      connectorConfiguration:
        connector.class: com.github.jcustenborder.kafka.connect.simulator.SimulatorSinkConnector
        tasks.max: '1'
        topics: example
      kafkaCluster:
        apacheKafkaCluster:
          bootstrapServers: ${exampleAwsMskCluster.bootstrapBrokersTls}
          vpc:
            securityGroups:
              - ${exampleAwsSecurityGroup.id}
            subnets:
              - ${example1.id}
              - ${example2.id}
              - ${example3.id}
      kafkaClusterClientAuthentication:
        authenticationType: NONE
      kafkaClusterEncryptionInTransit:
        encryptionType: TLS
      plugins:
        - customPlugin:
            arn: ${exampleAwsMskconnectCustomPlugin.arn}
            revision: ${exampleAwsMskconnectCustomPlugin.latestRevision}
      serviceExecutionRoleArn: ${exampleAwsIamRole.arn}
```
<!--End PulumiCodeChooser -->

## Import

Using `pulumi import`, import MSK Connect Connector using the connector's `arn`. For example:

```sh
$ pulumi import aws:mskconnect/connector:Connector example 'arn:aws:kafkaconnect:eu-central-1:123456789012:connector/example/264edee4-17a3-412e-bd76-6681cfc93805-3'
```
�
capacityW:U
S

mskconnectConnectorCapacity2aws:mskconnect/ConnectorCapacity:ConnectorCapacity]Information about the capacity allocated to the connector. See `capacity` Block for details.
l
connectorConfiguration2" LA map of keys to values that represent the configuration for the connector.
=
descriptionB" (A summary description of the connector.
�
kafkaClusterc:a
_

mskconnectConnectorKafkaCluster:aws:mskconnect/ConnectorKafkaCluster:ConnectorKafkaCluster[Specifies which Apache Kafka cluster to connect to. See `kafka_cluster` Block for details.
�
 kafkaClusterClientAuthentication�:�
�

mskconnect)ConnectorKafkaClusterClientAuthenticationbaws:mskconnect/ConnectorKafkaClusterClientAuthentication:ConnectorKafkaClusterClientAuthentication�Details of the client authentication used by the Apache Kafka cluster. See `kafka_cluster_client_authentication` Block for details.
�
kafkaClusterEncryptionInTransit�:�
�

mskconnect(ConnectorKafkaClusterEncryptionInTransit`aws:mskconnect/ConnectorKafkaClusterEncryptionInTransit:ConnectorKafkaClusterEncryptionInTransit{Details of encryption in transit to the Apache Kafka cluster. See `kafka_cluster_encryption_in_transit` Block for details.
�
kafkaconnectVersion" tThe version of Kafka Connect. It has to be compatible with both the Apache Kafka cluster's version and the plugins.
�
logDeliverybB`:^
\

mskconnectConnectorLogDelivery8aws:mskconnect/ConnectorLogDelivery:ConnectorLogDeliveryBDetails about log delivery. See `log_delivery` Block for details.
)
nameB" The name of the connector.
�
pluginsS*Q:O
M

mskconnectConnectorPlugin.aws:mskconnect/ConnectorPlugin:ConnectorPluginRSpecifies which plugins to use for the connector. See `plugin` Block for details.
�
serviceExecutionRoleArn" �The Amazon Resource Name (ARN) of the IAM role used by the connector to access the Amazon Web Services resources that it needs. The types of resources depends on the logic of the connector. For example, a connector that has Amazon S3 as a destination must have permissions that allow it to write to the S3 destination bucket.

The following arguments are optional:
�
tagsB2" �A map of tags to assign to the resource. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
�
workerConfigurationzBx:v
t

mskconnectConnectorWorkerConfigurationHaws:mskconnect/ConnectorWorkerConfiguration:ConnectorWorkerConfigurationnSpecifies which worker configuration to use with the connector. See `worker_configuration` Block for details.
"<
arn" 1The Amazon Resource Name (ARN) of the connector.
"�
capacityW:U
S

mskconnectConnectorCapacity2aws:mskconnect/ConnectorCapacity:ConnectorCapacity]Information about the capacity allocated to the connector. See `capacity` Block for details.
"l
connectorConfiguration2" LA map of keys to values that represent the configuration for the connector.
"=
descriptionB" (A summary description of the connector.
"�
kafkaClusterc:a
_

mskconnectConnectorKafkaCluster:aws:mskconnect/ConnectorKafkaCluster:ConnectorKafkaCluster[Specifies which Apache Kafka cluster to connect to. See `kafka_cluster` Block for details.
"�
 kafkaClusterClientAuthentication�:�
�

mskconnect)ConnectorKafkaClusterClientAuthenticationbaws:mskconnect/ConnectorKafkaClusterClientAuthentication:ConnectorKafkaClusterClientAuthentication�Details of the client authentication used by the Apache Kafka cluster. See `kafka_cluster_client_authentication` Block for details.
"�
kafkaClusterEncryptionInTransit�:�
�

mskconnect(ConnectorKafkaClusterEncryptionInTransit`aws:mskconnect/ConnectorKafkaClusterEncryptionInTransit:ConnectorKafkaClusterEncryptionInTransit{Details of encryption in transit to the Apache Kafka cluster. See `kafka_cluster_encryption_in_transit` Block for details.
"�
kafkaconnectVersion" tThe version of Kafka Connect. It has to be compatible with both the Apache Kafka cluster's version and the plugins.
"�
logDeliverybB`:^
\

mskconnectConnectorLogDelivery8aws:mskconnect/ConnectorLogDelivery:ConnectorLogDeliveryBDetails about log delivery. See `log_delivery` Block for details.
"'
name" The name of the connector.
"�
pluginsS*Q:O
M

mskconnectConnectorPlugin.aws:mskconnect/ConnectorPlugin:ConnectorPluginRSpecifies which plugins to use for the connector. See `plugin` Block for details.
"�
serviceExecutionRoleArn" �The Amazon Resource Name (ARN) of the IAM role used by the connector to access the Amazon Web Services resources that it needs. The types of resources depends on the logic of the connector. For example, a connector that has Amazon S3 as a destination must have permissions that allow it to write to the S3 destination bucket.

The following arguments are optional:
"�
tagsB2" �A map of tags to assign to the resource. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
"�
tagsAll2" xA map of tags assigned to the resource, including those inherited from the provider `default_tags` configuration block.
"5
version" &The current version of the connector.
"�
workerConfigurationzBx:v
t

mskconnectConnectorWorkerConfigurationHaws:mskconnect/ConnectorWorkerConfiguration:ConnectorWorkerConfigurationnSpecifies which worker configuration to use with the connector. See `worker_configuration` Block for details.
*�:
D

mskconnectCustomPlugin(aws:mskconnect/customPlugin:CustomPlugin�-Provides an Amazon MSK Connect Custom Plugin Resource.

## Example Usage

### Basic configuration

<!--Start PulumiCodeChooser -->
```typescript
import * as pulumi from "@pulumi/pulumi";
import * as aws from "@pulumi/aws";

const example = new aws.s3.BucketV2("example", {bucket: "example"});
const exampleBucketObjectv2 = new aws.s3.BucketObjectv2("example", {
    bucket: example.id,
    key: "debezium.zip",
    source: new pulumi.asset.FileAsset("debezium.zip"),
});
const exampleCustomPlugin = new aws.mskconnect.CustomPlugin("example", {
    name: "debezium-example",
    contentType: "ZIP",
    location: {
        s3: {
            bucketArn: example.arn,
            fileKey: exampleBucketObjectv2.key,
        },
    },
});
```
```python
import pulumi
import pulumi_aws as aws

example = aws.s3.BucketV2("example", bucket="example")
example_bucket_objectv2 = aws.s3.BucketObjectv2("example",
    bucket=example.id,
    key="debezium.zip",
    source=pulumi.FileAsset("debezium.zip"))
example_custom_plugin = aws.mskconnect.CustomPlugin("example",
    name="debezium-example",
    content_type="ZIP",
    location={
        "s3": {
            "bucket_arn": example.arn,
            "file_key": example_bucket_objectv2.key,
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
    var example = new Aws.S3.BucketV2("example", new()
    {
        Bucket = "example",
    });

    var exampleBucketObjectv2 = new Aws.S3.BucketObjectv2("example", new()
    {
        Bucket = example.Id,
        Key = "debezium.zip",
        Source = new FileAsset("debezium.zip"),
    });

    var exampleCustomPlugin = new Aws.MskConnect.CustomPlugin("example", new()
    {
        Name = "debezium-example",
        ContentType = "ZIP",
        Location = new Aws.MskConnect.Inputs.CustomPluginLocationArgs
        {
            S3 = new Aws.MskConnect.Inputs.CustomPluginLocationS3Args
            {
                BucketArn = example.Arn,
                FileKey = exampleBucketObjectv2.Key,
            },
        },
    });

});
```
```go
package main

import (
	"github.com/pulumi/pulumi-aws/sdk/v6/go/aws/mskconnect"
	"github.com/pulumi/pulumi-aws/sdk/v6/go/aws/s3"
	"github.com/pulumi/pulumi/sdk/v3/go/pulumi"
)

func main() {
	pulumi.Run(func(ctx *pulumi.Context) error {
		example, err := s3.NewBucketV2(ctx, "example", &s3.BucketV2Args{
			Bucket: pulumi.String("example"),
		})
		if err != nil {
			return err
		}
		exampleBucketObjectv2, err := s3.NewBucketObjectv2(ctx, "example", &s3.BucketObjectv2Args{
			Bucket: example.ID(),
			Key:    pulumi.String("debezium.zip"),
			Source: pulumi.NewFileAsset("debezium.zip"),
		})
		if err != nil {
			return err
		}
		_, err = mskconnect.NewCustomPlugin(ctx, "example", &mskconnect.CustomPluginArgs{
			Name:        pulumi.String("debezium-example"),
			ContentType: pulumi.String("ZIP"),
			Location: &mskconnect.CustomPluginLocationArgs{
				S3: &mskconnect.CustomPluginLocationS3Args{
					BucketArn: example.Arn,
					FileKey:   exampleBucketObjectv2.Key,
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
import com.pulumi.aws.s3.BucketV2;
import com.pulumi.aws.s3.BucketV2Args;
import com.pulumi.aws.s3.BucketObjectv2;
import com.pulumi.aws.s3.BucketObjectv2Args;
import com.pulumi.aws.mskconnect.CustomPlugin;
import com.pulumi.aws.mskconnect.CustomPluginArgs;
import com.pulumi.aws.mskconnect.inputs.CustomPluginLocationArgs;
import com.pulumi.aws.mskconnect.inputs.CustomPluginLocationS3Args;
import com.pulumi.asset.FileAsset;
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
        var example = new BucketV2("example", BucketV2Args.builder()
            .bucket("example")
            .build());

        var exampleBucketObjectv2 = new BucketObjectv2("exampleBucketObjectv2", BucketObjectv2Args.builder()
            .bucket(example.id())
            .key("debezium.zip")
            .source(new FileAsset("debezium.zip"))
            .build());

        var exampleCustomPlugin = new CustomPlugin("exampleCustomPlugin", CustomPluginArgs.builder()
            .name("debezium-example")
            .contentType("ZIP")
            .location(CustomPluginLocationArgs.builder()
                .s3(CustomPluginLocationS3Args.builder()
                    .bucketArn(example.arn())
                    .fileKey(exampleBucketObjectv2.key())
                    .build())
                .build())
            .build());

    }
}
```
```yaml
resources:
  example:
    type: aws:s3:BucketV2
    properties:
      bucket: example
  exampleBucketObjectv2:
    type: aws:s3:BucketObjectv2
    name: example
    properties:
      bucket: ${example.id}
      key: debezium.zip
      source:
        fn::FileAsset: debezium.zip
  exampleCustomPlugin:
    type: aws:mskconnect:CustomPlugin
    name: example
    properties:
      name: debezium-example
      contentType: ZIP
      location:
        s3:
          bucketArn: ${example.arn}
          fileKey: ${exampleBucketObjectv2.key}
```
<!--End PulumiCodeChooser -->

## Import

Using `pulumi import`, import MSK Connect Custom Plugin using the plugin's `arn`. For example:

```sh
$ pulumi import aws:mskconnect/customPlugin:CustomPlugin example 'arn:aws:kafkaconnect:eu-central-1:123456789012:custom-plugin/debezium-example/abcdefgh-1234-5678-9abc-defghijklmno-4'
```
T
contentType" AThe type of the plugin file. Allowed values are `ZIP` and `JAR`.
A
descriptionB" ,A summary description of the custom plugin.
�
location`:^
\

mskconnectCustomPluginLocation8aws:mskconnect/CustomPluginLocation:CustomPluginLocationUInformation about the location of a custom plugin. See `location` Block for details.
.
nameB"  The name of the custom plugin..
�
tagsB2" �A map of tags to assign to the resource. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.

The following arguments are optional:
"@
arn" 5the Amazon Resource Name (ARN) of the custom plugin.
"T
contentType" AThe type of the plugin file. Allowed values are `ZIP` and `JAR`.
"A
descriptionB" ,A summary description of the custom plugin.
"^
latestRevision Han ID of the latest successfully created revision of the custom plugin.
"�
location`:^
\

mskconnectCustomPluginLocation8aws:mskconnect/CustomPluginLocation:CustomPluginLocationUInformation about the location of a custom plugin. See `location` Block for details.
",
name"  The name of the custom plugin..
"-
state"  the state of the custom plugin.
"�
tagsB2" �A map of tags to assign to the resource. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.

The following arguments are optional:
"�
tagsAll2" xA map of tags assigned to the resource, including those inherited from the provider `default_tags` configuration block.
*�%
Y

mskconnectWorkerConfiguration6aws:mskconnect/workerConfiguration:WorkerConfiguration�Provides an Amazon MSK Connect Worker Configuration Resource.

## Example Usage

### Basic configuration

<!--Start PulumiCodeChooser -->
```typescript
import * as pulumi from "@pulumi/pulumi";
import * as aws from "@pulumi/aws";

const example = new aws.mskconnect.WorkerConfiguration("example", {
    name: "example",
    propertiesFileContent: `key.converter=org.apache.kafka.connect.storage.StringConverter
value.converter=org.apache.kafka.connect.storage.StringConverter
`,
});
```
```python
import pulumi
import pulumi_aws as aws

example = aws.mskconnect.WorkerConfiguration("example",
    name="example",
    properties_file_content="""key.converter=org.apache.kafka.connect.storage.StringConverter
value.converter=org.apache.kafka.connect.storage.StringConverter
""")
```
```csharp
using System.Collections.Generic;
using System.Linq;
using Pulumi;
using Aws = Pulumi.Aws;

return await Deployment.RunAsync(() => 
{
    var example = new Aws.MskConnect.WorkerConfiguration("example", new()
    {
        Name = "example",
        PropertiesFileContent = @"key.converter=org.apache.kafka.connect.storage.StringConverter
value.converter=org.apache.kafka.connect.storage.StringConverter
",
    });

});
```
```go
package main

import (
	"github.com/pulumi/pulumi-aws/sdk/v6/go/aws/mskconnect"
	"github.com/pulumi/pulumi/sdk/v3/go/pulumi"
)

func main() {
	pulumi.Run(func(ctx *pulumi.Context) error {
		_, err := mskconnect.NewWorkerConfiguration(ctx, "example", &mskconnect.WorkerConfigurationArgs{
			Name:                  pulumi.String("example"),
			PropertiesFileContent: pulumi.String("key.converter=org.apache.kafka.connect.storage.StringConverter\nvalue.converter=org.apache.kafka.connect.storage.StringConverter\n"),
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
import com.pulumi.aws.mskconnect.WorkerConfiguration;
import com.pulumi.aws.mskconnect.WorkerConfigurationArgs;
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
        var example = new WorkerConfiguration("example", WorkerConfigurationArgs.builder()
            .name("example")
            .propertiesFileContent("""
key.converter=org.apache.kafka.connect.storage.StringConverter
value.converter=org.apache.kafka.connect.storage.StringConverter
            """)
            .build());

    }
}
```
```yaml
resources:
  example:
    type: aws:mskconnect:WorkerConfiguration
    properties:
      name: example
      propertiesFileContent: |
        key.converter=org.apache.kafka.connect.storage.StringConverter
        value.converter=org.apache.kafka.connect.storage.StringConverter
```
<!--End PulumiCodeChooser -->

## Import

Using `pulumi import`, import MSK Connect Worker Configuration using the plugin's `arn`. For example:

```sh
$ pulumi import aws:mskconnect/workerConfiguration:WorkerConfiguration example 'arn:aws:kafkaconnect:eu-central-1:123456789012:worker-configuration/example/8848493b-7fcc-478c-a646-4a52634e3378-4'
```
H
descriptionB" 3A summary description of the worker configuration.
4
nameB" &The name of the worker configuration.
�
propertiesFileContent" �Contents of connect-distributed.properties file. The value can be either base64 encoded or in raw format.

The following arguments are optional:
�
tagsB2" �A map of tags to assign to the resource. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
"G
arn" <the Amazon Resource Name (ARN) of the worker configuration.
"H
descriptionB" 3A summary description of the worker configuration.
"e
latestRevision Oan ID of the latest successfully created revision of the worker configuration.
"2
name" &The name of the worker configuration.
"�
propertiesFileContent" �Contents of connect-distributed.properties file. The value can be either base64 encoded or in raw format.

The following arguments are optional:
"�
tagsB2" �A map of tags to assign to the resource. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
"�
tagsAll2" xA map of tags assigned to the resource, including those inherited from the provider `default_tags` configuration block.
2�
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
2�
<
mediaconvertgetQueue"aws:mediaconvert/getQueue:getQueue�Retrieve information about a AWS Elemental MediaConvert Queue.

## Example Usage

<!--Start PulumiCodeChooser -->
```typescript
import * as pulumi from "@pulumi/pulumi";
import * as aws from "@pulumi/aws";

const example = aws.mediaconvert.getQueue({
    id: "tf-example-queue",
});
```
```python
import pulumi
import pulumi_aws as aws

example = aws.mediaconvert.get_queue(id="tf-example-queue")
```
```csharp
using System.Collections.Generic;
using System.Linq;
using Pulumi;
using Aws = Pulumi.Aws;

return await Deployment.RunAsync(() => 
{
    var example = Aws.MediaConvert.GetQueue.Invoke(new()
    {
        Id = "tf-example-queue",
    });

});
```
```go
package main

import (
	"github.com/pulumi/pulumi-aws/sdk/v6/go/aws/mediaconvert"
	"github.com/pulumi/pulumi/sdk/v3/go/pulumi"
)

func main() {
	pulumi.Run(func(ctx *pulumi.Context) error {
		_, err := mediaconvert.LookupQueue(ctx, &mediaconvert.LookupQueueArgs{
			Id: "tf-example-queue",
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
import com.pulumi.aws.mediaconvert.MediaconvertFunctions;
import com.pulumi.aws.mediaconvert.inputs.GetQueueArgs;
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
        final var example = MediaconvertFunctions.getQueue(GetQueueArgs.builder()
            .id("tf-example-queue")
            .build());

    }
}
```
```yaml
variables:
  example:
    fn::invoke:
      function: aws:mediaconvert:getQueue
      arguments:
        id: tf-example-queue
```
<!--End PulumiCodeChooser -->
>
id" 4Unique identifier of the queue. The same as `name`.
�
tagsB2" xA map of tags assigned to the resource, including those inherited from the provider `default_tags` configuration block.
"!
arn" The Arn of the queue.
"
id" "
name" The same as `id`.
"'
status" The status of the queue.
"�
tags2" xA map of tags assigned to the resource, including those inherited from the provider `default_tags` configuration block.
2�
6
	medialivegetInputaws:medialive/getInput:getInput�Data source for managing an AWS Elemental MediaLive Input.

## Example Usage

### Basic Usage

<!--Start PulumiCodeChooser -->
```typescript
import * as pulumi from "@pulumi/pulumi";
import * as aws from "@pulumi/aws";

const example = aws.medialive.getInput({
    id: exampleAwsMedialiveInput.id,
});
```
```python
import pulumi
import pulumi_aws as aws

example = aws.medialive.get_input(id=example_aws_medialive_input["id"])
```
```csharp
using System.Collections.Generic;
using System.Linq;
using Pulumi;
using Aws = Pulumi.Aws;

return await Deployment.RunAsync(() => 
{
    var example = Aws.MediaLive.GetInput.Invoke(new()
    {
        Id = exampleAwsMedialiveInput.Id,
    });

});
```
```go
package main

import (
	"github.com/pulumi/pulumi-aws/sdk/v6/go/aws/medialive"
	"github.com/pulumi/pulumi/sdk/v3/go/pulumi"
)

func main() {
	pulumi.Run(func(ctx *pulumi.Context) error {
		_, err := medialive.LookupInput(ctx, &medialive.LookupInputArgs{
			Id: exampleAwsMedialiveInput.Id,
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
import com.pulumi.aws.medialive.MedialiveFunctions;
import com.pulumi.aws.medialive.inputs.GetInputArgs;
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
        final var example = MedialiveFunctions.getInput(GetInputArgs.builder()
            .id(exampleAwsMedialiveInput.id())
            .build());

    }
}
```
```yaml
variables:
  example:
    fn::invoke:
      function: aws:medialive:getInput
      arguments:
        id: ${exampleAwsMedialiveInput.id}
```
<!--End PulumiCodeChooser -->

id" The ID of the Input.
"
arn" ARN of the Input.
"6
attachedChannels*" Channels attached to Input.
"m
destinations]*[:Y
W
	medialivegetInputDestination5aws:medialive/getInputDestination:getInputDestination"
id" "#

inputClass" The input class.
"�
inputDevices]*[:Y
W
	medialivegetInputInputDevice5aws:medialive/getInputInputDevice:getInputInputDeviceSettings for the devices.
"V
inputPartnerIds*" =A list of IDs for all Inputs which are partners of this one.
"1
inputSourceType" Source type of the input.
"�
mediaConnectFlowsl*j:h
f
	medialivegetInputMediaConnectFlow?aws:medialive/getInputMediaConnectFlow:getInputMediaConnectFlow"A list of the MediaConnect Flows.
"
name" Name of the input.
"Q
roleArn" BThe ARN of the role this input assumes during and after creation.
"7
securityGroups*" List of input security groups.
"�
sourcesN*L:J
H
	medialivegetInputSource+aws:medialive/getInputSource:getInputSource'The source URLs for a PULL-type input.
"%
state" The state of the input.
"3
tags2" %A map of tags assigned to the Input.
"#
type" The type of the input.
2�
.
memorydbgetAclaws:memorydb/getAcl:getAcl�Provides information about a MemoryDB ACL.

## Example Usage

<!--Start PulumiCodeChooser -->
```typescript
import * as pulumi from "@pulumi/pulumi";
import * as aws from "@pulumi/aws";

const example = aws.memorydb.getAcl({
    name: "my-acl",
});
```
```python
import pulumi
import pulumi_aws as aws

example = aws.memorydb.get_acl(name="my-acl")
```
```csharp
using System.Collections.Generic;
using System.Linq;
using Pulumi;
using Aws = Pulumi.Aws;

return await Deployment.RunAsync(() => 
{
    var example = Aws.MemoryDb.GetAcl.Invoke(new()
    {
        Name = "my-acl",
    });

});
```
```go
package main

import (
	"github.com/pulumi/pulumi-aws/sdk/v6/go/aws/memorydb"
	"github.com/pulumi/pulumi/sdk/v3/go/pulumi"
)

func main() {
	pulumi.Run(func(ctx *pulumi.Context) error {
		_, err := memorydb.LookupAcl(ctx, &memorydb.LookupAclArgs{
			Name: "my-acl",
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
import com.pulumi.aws.memorydb.MemorydbFunctions;
import com.pulumi.aws.memorydb.inputs.GetAclArgs;
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
        final var example = MemorydbFunctions.getAcl(GetAclArgs.builder()
            .name("my-acl")
            .build());

    }
}
```
```yaml
variables:
  example:
    fn::invoke:
      function: aws:memorydb:getAcl
      arguments:
        name: my-acl
```
<!--End PulumiCodeChooser -->

name" Name of the ACL.
1
tagsB2" !Map of tags assigned to the ACL.
"
arn" ARN of the ACL.
"E
id" ;The provider-assigned unique ID for this managed resource.
"M
minimumEngineVersion" 1The minimum engine version supported by the ACL.
"

name" "/
tags2" !Map of tags assigned to the ACL.
"D
	userNames*" 1Set of MemoryDB user names included in this ACL.
2�!
:
memorydb
getCluster"aws:memorydb/getCluster:getCluster�Provides information about a MemoryDB Cluster.

## Example Usage

<!--Start PulumiCodeChooser -->
```typescript
import * as pulumi from "@pulumi/pulumi";
import * as aws from "@pulumi/aws";

const example = aws.memorydb.getCluster({
    name: "my-cluster",
});
```
```python
import pulumi
import pulumi_aws as aws

example = aws.memorydb.get_cluster(name="my-cluster")
```
```csharp
using System.Collections.Generic;
using System.Linq;
using Pulumi;
using Aws = Pulumi.Aws;

return await Deployment.RunAsync(() => 
{
    var example = Aws.MemoryDb.GetCluster.Invoke(new()
    {
        Name = "my-cluster",
    });

});
```
```go
package main

import (
	"github.com/pulumi/pulumi-aws/sdk/v6/go/aws/memorydb"
	"github.com/pulumi/pulumi/sdk/v3/go/pulumi"
)

func main() {
	pulumi.Run(func(ctx *pulumi.Context) error {
		_, err := memorydb.LookupCluster(ctx, &memorydb.LookupClusterArgs{
			Name: "my-cluster",
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
import com.pulumi.aws.memorydb.MemorydbFunctions;
import com.pulumi.aws.memorydb.inputs.GetClusterArgs;
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
        final var example = MemorydbFunctions.getCluster(GetClusterArgs.builder()
            .name("my-cluster")
            .build());

    }
}
```
```yaml
variables:
  example:
    fn::invoke:
      function: aws:memorydb:getCluster
      arguments:
        name: my-cluster
```
<!--End PulumiCodeChooser -->
!
name" Name of the cluster.
5
tagsB2" %Map of tags assigned to the cluster.
"L
aclName" =Name of the Access Control List associated with the cluster.
"
arn" ARN of the cluster.
"^
autoMinorVersionUpgrade
 ?True when the cluster allows automatic minor version upgrades.
"�
clusterEndpointsm*k:i
g
memorydbgetClusterClusterEndpoint@aws:memorydb/getClusterClusterEndpoint:getClusterClusterEndpoint"6
dataTiering
 #True when data tiering is enabled.
"0
description" Description for the cluster.
"5
engine" 'Engine that will run on cluster nodes.
"R
enginePatchVersion" 8Patch version number of the engine used by the cluster.
"G
engineVersion" 2Version number of the engine used by the cluster.
"�
finalSnapshotName" |Name of the final cluster snapshot to be created when this resource is deleted. If omitted, no final snapshot will be made.
"E
id" ;The provider-assigned unique ID for this managed resource.
"I
	kmsKeyArn" 8ARN of the KMS key used to encrypt the cluster at rest.
"�
maintenanceWindow" �Weekly time range during which maintenance on the cluster is performed. Specify as a range in the format `ddd:hh24:mi-ddd:hh24:mi` (24H Clock UTC). Example: `sun:23:00-mon:01:30`.
"
name" Name of this node.
"I
nodeType" 9Compute and memory capacity of the nodes in the cluster.
"J
numReplicasPerShard /The number of replicas to apply to each shard.
"2
	numShards !Number of shards in the cluster.
"W
parameterGroupName" =The name of the parameter group associated with the cluster.
"8
port ,Port number that this node is listening on.
"W
securityGroupIds*" =Set of VPC Security Group ID-s associated with this cluster.
"z
shardsO*M:K
I
memorydbgetClusterShard,aws:memorydb/getClusterShard:getClusterShardSet of shards in this cluster.
"�
snapshotRetentionLimit �The number of days for which MemoryDB retains automatic snapshots before deleting them. When set to `0`, automatic backups are disabled.
"�
snapshotWindow" vDaily time range (in UTC) during which MemoryDB begins taking a daily snapshot of your shard. Example: `05:00-09:00`.
"Q
snsTopicArn" >ARN of the SNS topic to which cluster notifications are sent.
"J
subnetGroupName" 3The name of the subnet group used for the cluster.
"3
tags2" %Map of tags assigned to the cluster.
"O

tlsEnabled
 =When true, in-transit encryption is enabled for the cluster.
2�
O
memorydbgetParameterGroup0aws:memorydb/getParameterGroup:getParameterGroup�Provides information about a MemoryDB Parameter Group.

## Example Usage

<!--Start PulumiCodeChooser -->
```typescript
import * as pulumi from "@pulumi/pulumi";
import * as aws from "@pulumi/aws";

const example = aws.memorydb.getParameterGroup({
    name: "my-parameter-group",
});
```
```python
import pulumi
import pulumi_aws as aws

example = aws.memorydb.get_parameter_group(name="my-parameter-group")
```
```csharp
using System.Collections.Generic;
using System.Linq;
using Pulumi;
using Aws = Pulumi.Aws;

return await Deployment.RunAsync(() => 
{
    var example = Aws.MemoryDb.GetParameterGroup.Invoke(new()
    {
        Name = "my-parameter-group",
    });

});
```
```go
package main

import (
	"github.com/pulumi/pulumi-aws/sdk/v6/go/aws/memorydb"
	"github.com/pulumi/pulumi/sdk/v3/go/pulumi"
)

func main() {
	pulumi.Run(func(ctx *pulumi.Context) error {
		_, err := memorydb.LookupParameterGroup(ctx, &memorydb.LookupParameterGroupArgs{
			Name: "my-parameter-group",
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
import com.pulumi.aws.memorydb.MemorydbFunctions;
import com.pulumi.aws.memorydb.inputs.GetParameterGroupArgs;
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
        final var example = MemorydbFunctions.getParameterGroup(GetParameterGroupArgs.builder()
            .name("my-parameter-group")
            .build());

    }
}
```
```yaml
variables:
  example:
    fn::invoke:
      function: aws:memorydb:getParameterGroup
      arguments:
        name: my-parameter-group
```
<!--End PulumiCodeChooser -->
)
name" Name of the parameter group.
=
tagsB2" -Map of tags assigned to the parameter group.
"'
arn" ARN of the parameter group.
"7
description" $Description of the parameter group.
"H
family" :Engine version that the parameter group can be used with.
"E
id" ;The provider-assigned unique ID for this managed resource.
"#
name" Name of the parameter.
"�

parametersp*n:l
j
memorydbgetParameterGroupParameterBaws:memorydb/getParameterGroupParameter:getParameterGroupParameterHSet of user-defined MemoryDB parameters applied by the parameter group.
";
tags2" -Map of tags assigned to the parameter group.
2�
=
memorydbgetSnapshot$aws:memorydb/getSnapshot:getSnapshot�Provides information about a MemoryDB Snapshot.

## Example Usage

<!--Start PulumiCodeChooser -->
```typescript
import * as pulumi from "@pulumi/pulumi";
import * as aws from "@pulumi/aws";

const example = aws.memorydb.getSnapshot({
    name: "my-snapshot",
});
```
```python
import pulumi
import pulumi_aws as aws

example = aws.memorydb.get_snapshot(name="my-snapshot")
```
```csharp
using System.Collections.Generic;
using System.Linq;
using Pulumi;
using Aws = Pulumi.Aws;

return await Deployment.RunAsync(() => 
{
    var example = Aws.MemoryDb.GetSnapshot.Invoke(new()
    {
        Name = "my-snapshot",
    });

});
```
```go
package main

import (
	"github.com/pulumi/pulumi-aws/sdk/v6/go/aws/memorydb"
	"github.com/pulumi/pulumi/sdk/v3/go/pulumi"
)

func main() {
	pulumi.Run(func(ctx *pulumi.Context) error {
		_, err := memorydb.LookupSnapshot(ctx, &memorydb.LookupSnapshotArgs{
			Name: "my-snapshot",
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
import com.pulumi.aws.memorydb.MemorydbFunctions;
import com.pulumi.aws.memorydb.inputs.GetSnapshotArgs;
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
        final var example = MemorydbFunctions.getSnapshot(GetSnapshotArgs.builder()
            .name("my-snapshot")
            .build());

    }
}
```
```yaml
variables:
  example:
    fn::invoke:
      function: aws:memorydb:getSnapshot
      arguments:
        name: my-snapshot
```
<!--End PulumiCodeChooser -->
"
name" Name of the snapshot.
6
tagsB2" &Map of tags assigned to the snapshot.
" 
arn" ARN of the snapshot.
"�
clusterConfigurations*}:{
y
memorydbgetSnapshotClusterConfigurationLaws:memorydb/getSnapshotClusterConfiguration:getSnapshotClusterConfigurationDThe configuration of the cluster from which the snapshot was taken.
"S
clusterName" @Name of the MemoryDB cluster that this snapshot was taken from.
"E
id" ;The provider-assigned unique ID for this managed resource.
"J
	kmsKeyArn" 9ARN of the KMS key used to encrypt the snapshot at rest.
"!
name" Name of the cluster.
"q
source" cWhether the snapshot is from an automatic backup (`automated`) or was created manually (`manual`).
"4
tags2" &Map of tags assigned to the snapshot.
2�
F
memorydbgetSubnetGroup*aws:memorydb/getSubnetGroup:getSubnetGroup�Provides information about a MemoryDB Subnet Group.

## Example Usage

<!--Start PulumiCodeChooser -->
```typescript
import * as pulumi from "@pulumi/pulumi";
import * as aws from "@pulumi/aws";

const example = aws.memorydb.getSubnetGroup({
    name: "my-subnet-group",
});
```
```python
import pulumi
import pulumi_aws as aws

example = aws.memorydb.get_subnet_group(name="my-subnet-group")
```
```csharp
using System.Collections.Generic;
using System.Linq;
using Pulumi;
using Aws = Pulumi.Aws;

return await Deployment.RunAsync(() => 
{
    var example = Aws.MemoryDb.GetSubnetGroup.Invoke(new()
    {
        Name = "my-subnet-group",
    });

});
```
```go
package main

import (
	"github.com/pulumi/pulumi-aws/sdk/v6/go/aws/memorydb"
	"github.com/pulumi/pulumi/sdk/v3/go/pulumi"
)

func main() {
	pulumi.Run(func(ctx *pulumi.Context) error {
		_, err := memorydb.LookupSubnetGroup(ctx, &memorydb.LookupSubnetGroupArgs{
			Name: "my-subnet-group",
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
import com.pulumi.aws.memorydb.MemorydbFunctions;
import com.pulumi.aws.memorydb.inputs.GetSubnetGroupArgs;
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
        final var example = MemorydbFunctions.getSubnetGroup(GetSubnetGroupArgs.builder()
            .name("my-subnet-group")
            .build());

    }
}
```
```yaml
variables:
  example:
    fn::invoke:
      function: aws:memorydb:getSubnetGroup
      arguments:
        name: my-subnet-group
```
<!--End PulumiCodeChooser -->
&
name" Name of the subnet group.
:
tagsB2" *Map of tags assigned to the subnet group.
"$
arn" ARN of the subnet group.
"4
description" !Description of the subnet group.
"E
id" ;The provider-assigned unique ID for this managed resource.
"

name" "?
	subnetIds*" ,Set of VPC Subnet ID-s of the subnet group.
"8
tags2" *Map of tags assigned to the subnet group.
"3
vpcId" &VPC in which the subnet group exists.
2�
1
memorydbgetUseraws:memorydb/getUser:getUser�Provides information about a MemoryDB User.

## Example Usage

<!--Start PulumiCodeChooser -->
```typescript
import * as pulumi from "@pulumi/pulumi";
import * as aws from "@pulumi/aws";

const example = aws.memorydb.getUser({
    userName: "my-user",
});
```
```python
import pulumi
import pulumi_aws as aws

example = aws.memorydb.get_user(user_name="my-user")
```
```csharp
using System.Collections.Generic;
using System.Linq;
using Pulumi;
using Aws = Pulumi.Aws;

return await Deployment.RunAsync(() => 
{
    var example = Aws.MemoryDb.GetUser.Invoke(new()
    {
        UserName = "my-user",
    });

});
```
```go
package main

import (
	"github.com/pulumi/pulumi-aws/sdk/v6/go/aws/memorydb"
	"github.com/pulumi/pulumi/sdk/v3/go/pulumi"
)

func main() {
	pulumi.Run(func(ctx *pulumi.Context) error {
		_, err := memorydb.LookupUser(ctx, &memorydb.LookupUserArgs{
			UserName: "my-user",
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
import com.pulumi.aws.memorydb.MemorydbFunctions;
import com.pulumi.aws.memorydb.inputs.GetUserArgs;
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
        final var example = MemorydbFunctions.getUser(GetUserArgs.builder()
            .userName("my-user")
            .build());

    }
}
```
```yaml
variables:
  example:
    fn::invoke:
      function: aws:memorydb:getUser
      arguments:
        userName: my-user
```
<!--End PulumiCodeChooser -->
2
tagsB2" "Map of tags assigned to the user.
"
userName" Name of the user.
"B
accessString" .Access permissions string used for this user.
"
arn" ARN of the user.
"�
authenticationModesm*k:i
g
memorydbgetUserAuthenticationMode@aws:memorydb/getUserAuthenticationMode:getUserAuthenticationMode.Denotes the user's authentication properties.
"E
id" ;The provider-assigned unique ID for this managed resource.
"K
minimumEngineVersion" /Minimum engine version supported for the user.
"0
tags2" "Map of tags assigned to the user.
"
userName" 2�$
+
mq	getBrokeraws:mq/getBroker:getBroker�Provides information about a MQ Broker.

## Example Usage

<!--Start PulumiCodeChooser -->
```typescript
import * as pulumi from "@pulumi/pulumi";
import * as aws from "@pulumi/aws";

const config = new pulumi.Config();
const brokerId = config.get("brokerId") || "";
const brokerName = config.get("brokerName") || "";
const byId = aws.mq.getBroker({
    brokerId: brokerId,
});
const byName = aws.mq.getBroker({
    brokerName: brokerName,
});
```
```python
import pulumi
import pulumi_aws as aws

config = pulumi.Config()
broker_id = config.get("brokerId")
if broker_id is None:
    broker_id = ""
broker_name = config.get("brokerName")
if broker_name is None:
    broker_name = ""
by_id = aws.mq.get_broker(broker_id=broker_id)
by_name = aws.mq.get_broker(broker_name=broker_name)
```
```csharp
using System.Collections.Generic;
using System.Linq;
using Pulumi;
using Aws = Pulumi.Aws;

return await Deployment.RunAsync(() => 
{
    var config = new Config();
    var brokerId = config.Get("brokerId") ?? "";
    var brokerName = config.Get("brokerName") ?? "";
    var byId = Aws.Mq.GetBroker.Invoke(new()
    {
        BrokerId = brokerId,
    });

    var byName = Aws.Mq.GetBroker.Invoke(new()
    {
        BrokerName = brokerName,
    });

});
```
```go
package main

import (
	"github.com/pulumi/pulumi-aws/sdk/v6/go/aws/mq"
	"github.com/pulumi/pulumi/sdk/v3/go/pulumi"
	"github.com/pulumi/pulumi/sdk/v3/go/pulumi/config"
)

func main() {
	pulumi.Run(func(ctx *pulumi.Context) error {
		cfg := config.New(ctx, "")
		brokerId := ""
		if param := cfg.Get("brokerId"); param != "" {
			brokerId = param
		}
		brokerName := ""
		if param := cfg.Get("brokerName"); param != "" {
			brokerName = param
		}
		_, err := mq.LookupBroker(ctx, &mq.LookupBrokerArgs{
			BrokerId: pulumi.StringRef(brokerId),
		}, nil)
		if err != nil {
			return err
		}
		_, err = mq.LookupBroker(ctx, &mq.LookupBrokerArgs{
			BrokerName: pulumi.StringRef(brokerName),
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
import com.pulumi.aws.mq.MqFunctions;
import com.pulumi.aws.mq.inputs.GetBrokerArgs;
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
        final var brokerId = config.get("brokerId").orElse("");
        final var brokerName = config.get("brokerName").orElse("");
        final var byId = MqFunctions.getBroker(GetBrokerArgs.builder()
            .brokerId(brokerId)
            .build());

        final var byName = MqFunctions.getBroker(GetBrokerArgs.builder()
            .brokerName(brokerName)
            .build());

    }
}
```
```yaml
configuration:
  brokerId:
    type: string
    default: ""
  brokerName:
    type: string
    default: ""
variables:
  byId:
    fn::invoke:
      function: aws:mq:getBroker
      arguments:
        brokerId: ${brokerId}
  byName:
    fn::invoke:
      function: aws:mq:getBroker
      arguments:
        brokerName: ${brokerName}
```
<!--End PulumiCodeChooser -->
.
brokerIdB" Unique id of the mq broker.
2

brokerNameB" Unique name of the mq broker.

tagsB2" "	
arn" "
authenticationStrategy" "
autoMinorVersionUpgrade
 "
brokerId" "

brokerName" "g
configurationV:T
R
mqgetBrokerConfiguration4aws:mq/getBrokerConfiguration:getBrokerConfiguration"
deploymentMode" "v
encryptionOptionsa*_:]
[
mqgetBrokerEncryptionOption:aws:mq/getBrokerEncryptionOption:getBrokerEncryptionOption"

engineType" "
engineVersion" "
hostInstanceType" "E
id" ;The provider-assigned unique ID for this managed resource.
"V
	instancesI*G:E
C
mqgetBrokerInstance*aws:mq/getBrokerInstance:getBrokerInstance"~
ldapServerMetadatasg*e:c
a
mqgetBrokerLdapServerMetadata>aws:mq/getBrokerLdapServerMetadata:getBrokerLdapServerMetadata"C
logs;:9
7
mqgetBrokerLogs"aws:mq/getBrokerLogs:getBrokerLogs"�
maintenanceWindowStartTime}:{
y
mq#getBrokerMaintenanceWindowStartTimeNaws:mq/getBrokerMaintenanceWindowStartTime:getBrokerMaintenanceWindowStartTime"
publiclyAccessible
 "
securityGroups*" "
storageType" "
	subnetIds*" "
tags2" "F
users=*;:9
7
mqgetBrokerUser"aws:mq/getBrokerUser:getBrokerUser2�
L
mqgetBrokerEngineTypes0aws:mq/getBrokerEngineTypes:getBrokerEngineTypes�Retrieve information about available broker engines.

## Example Usage

### Basic Usage

<!--Start PulumiCodeChooser -->
```typescript
import * as pulumi from "@pulumi/pulumi";
import * as aws from "@pulumi/aws";

const example = aws.mq.getBrokerEngineTypes({
    engineType: "ACTIVEMQ",
});
```
```python
import pulumi
import pulumi_aws as aws

example = aws.mq.get_broker_engine_types(engine_type="ACTIVEMQ")
```
```csharp
using System.Collections.Generic;
using System.Linq;
using Pulumi;
using Aws = Pulumi.Aws;

return await Deployment.RunAsync(() => 
{
    var example = Aws.Mq.GetBrokerEngineTypes.Invoke(new()
    {
        EngineType = "ACTIVEMQ",
    });

});
```
```go
package main

import (
	"github.com/pulumi/pulumi-aws/sdk/v6/go/aws/mq"
	"github.com/pulumi/pulumi/sdk/v3/go/pulumi"
)

func main() {
	pulumi.Run(func(ctx *pulumi.Context) error {
		_, err := mq.GetBrokerEngineTypes(ctx, &mq.GetBrokerEngineTypesArgs{
			EngineType: pulumi.StringRef("ACTIVEMQ"),
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
import com.pulumi.aws.mq.MqFunctions;
import com.pulumi.aws.mq.inputs.GetBrokerEngineTypesArgs;
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
        final var example = MqFunctions.getBrokerEngineTypes(GetBrokerEngineTypesArgs.builder()
            .engineType("ACTIVEMQ")
            .build());

    }
}
```
```yaml
variables:
  example:
    fn::invoke:
      function: aws:mq:getBrokerEngineTypes
      arguments:
        engineType: ACTIVEMQ
```
<!--End PulumiCodeChooser -->
F

engineTypeB" 2The MQ engine type to return version details for.
"�
brokerEngineTypes�*�:~
|
mq$getBrokerEngineTypesBrokerEngineTypePaws:mq/getBrokerEngineTypesBrokerEngineType:getBrokerEngineTypesBrokerEngineTypeAA list of available engine types and versions. See Engine Types.
".

engineTypeB" The broker's engine type.
"E
id" ;The provider-assigned unique ID for this managed resource.
2�,
X
mqgetInstanceTypeOfferings8aws:mq/getInstanceTypeOfferings:getInstanceTypeOfferings�&Provides information about a MQ Broker Instance Offerings.

## Example Usage

<!--Start PulumiCodeChooser -->
```typescript
import * as pulumi from "@pulumi/pulumi";
import * as aws from "@pulumi/aws";

const empty = aws.mq.getInstanceTypeOfferings({});
const engine = aws.mq.getInstanceTypeOfferings({
    engineType: "ACTIVEMQ",
});
const storage = aws.mq.getInstanceTypeOfferings({
    storageType: "EBS",
});
const instance = aws.mq.getInstanceTypeOfferings({
    hostInstanceType: "mq.m5.large",
});
const all = aws.mq.getInstanceTypeOfferings({
    hostInstanceType: "mq.m5.large",
    storageType: "EBS",
    engineType: "ACTIVEMQ",
});
```
```python
import pulumi
import pulumi_aws as aws

empty = aws.mq.get_instance_type_offerings()
engine = aws.mq.get_instance_type_offerings(engine_type="ACTIVEMQ")
storage = aws.mq.get_instance_type_offerings(storage_type="EBS")
instance = aws.mq.get_instance_type_offerings(host_instance_type="mq.m5.large")
all = aws.mq.get_instance_type_offerings(host_instance_type="mq.m5.large",
    storage_type="EBS",
    engine_type="ACTIVEMQ")
```
```csharp
using System.Collections.Generic;
using System.Linq;
using Pulumi;
using Aws = Pulumi.Aws;

return await Deployment.RunAsync(() => 
{
    var empty = Aws.Mq.GetInstanceTypeOfferings.Invoke();

    var engine = Aws.Mq.GetInstanceTypeOfferings.Invoke(new()
    {
        EngineType = "ACTIVEMQ",
    });

    var storage = Aws.Mq.GetInstanceTypeOfferings.Invoke(new()
    {
        StorageType = "EBS",
    });

    var instance = Aws.Mq.GetInstanceTypeOfferings.Invoke(new()
    {
        HostInstanceType = "mq.m5.large",
    });

    var all = Aws.Mq.GetInstanceTypeOfferings.Invoke(new()
    {
        HostInstanceType = "mq.m5.large",
        StorageType = "EBS",
        EngineType = "ACTIVEMQ",
    });

});
```
```go
package main

import (
	"github.com/pulumi/pulumi-aws/sdk/v6/go/aws/mq"
	"github.com/pulumi/pulumi/sdk/v3/go/pulumi"
)

func main() {
	pulumi.Run(func(ctx *pulumi.Context) error {
		_, err := mq.GetInstanceTypeOfferings(ctx, &mq.GetInstanceTypeOfferingsArgs{}, nil)
		if err != nil {
			return err
		}
		_, err = mq.GetInstanceTypeOfferings(ctx, &mq.GetInstanceTypeOfferingsArgs{
			EngineType: pulumi.StringRef("ACTIVEMQ"),
		}, nil)
		if err != nil {
			return err
		}
		_, err = mq.GetInstanceTypeOfferings(ctx, &mq.GetInstanceTypeOfferingsArgs{
			StorageType: pulumi.StringRef("EBS"),
		}, nil)
		if err != nil {
			return err
		}
		_, err = mq.GetInstanceTypeOfferings(ctx, &mq.GetInstanceTypeOfferingsArgs{
			HostInstanceType: pulumi.StringRef("mq.m5.large"),
		}, nil)
		if err != nil {
			return err
		}
		_, err = mq.GetInstanceTypeOfferings(ctx, &mq.GetInstanceTypeOfferingsArgs{
			HostInstanceType: pulumi.StringRef("mq.m5.large"),
			StorageType:      pulumi.StringRef("EBS"),
			EngineType:       pulumi.StringRef("ACTIVEMQ"),
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
import com.pulumi.aws.mq.MqFunctions;
import com.pulumi.aws.mq.inputs.GetInstanceTypeOfferingsArgs;
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
        final var empty = MqFunctions.getInstanceTypeOfferings();

        final var engine = MqFunctions.getInstanceTypeOfferings(GetInstanceTypeOfferingsArgs.builder()
            .engineType("ACTIVEMQ")
            .build());

        final var storage = MqFunctions.getInstanceTypeOfferings(GetInstanceTypeOfferingsArgs.builder()
            .storageType("EBS")
            .build());

        final var instance = MqFunctions.getInstanceTypeOfferings(GetInstanceTypeOfferingsArgs.builder()
            .hostInstanceType("mq.m5.large")
            .build());

        final var all = MqFunctions.getInstanceTypeOfferings(GetInstanceTypeOfferingsArgs.builder()
            .hostInstanceType("mq.m5.large")
            .storageType("EBS")
            .engineType("ACTIVEMQ")
            .build());

    }
}
```
```yaml
variables:
  empty:
    fn::invoke:
      function: aws:mq:getInstanceTypeOfferings
      arguments: {}
  engine:
    fn::invoke:
      function: aws:mq:getInstanceTypeOfferings
      arguments:
        engineType: ACTIVEMQ
  storage:
    fn::invoke:
      function: aws:mq:getInstanceTypeOfferings
      arguments:
        storageType: EBS
  instance:
    fn::invoke:
      function: aws:mq:getInstanceTypeOfferings
      arguments:
        hostInstanceType: mq.m5.large
  all:
    fn::invoke:
      function: aws:mq:getInstanceTypeOfferings
      arguments:
        hostInstanceType: mq.m5.large
        storageType: EBS
        engineType: ACTIVEMQ
```
<!--End PulumiCodeChooser -->
4

engineTypeB"  Filter response by engine type.
A
hostInstanceTypeB" 'Filter response by host instance type.
6
storageTypeB" !Filter response by storage type.
"�
brokerInstanceOptions�*�:�
�
mq,getInstanceTypeOfferingsBrokerInstanceOption`aws:mq/getInstanceTypeOfferingsBrokerInstanceOption:getInstanceTypeOfferingsBrokerInstanceOptionBOption for host instance type. See Broker Instance Options below.
"*

engineTypeB" Broker's engine type.
"2
hostInstanceTypeB" Broker's instance type.
"E
id" ;The provider-assigned unique ID for this managed resource.
",
storageTypeB" Broker's storage type.
2�
K
mskgetBootstrapBrokers/aws:msk/getBootstrapBrokers:getBootstrapBrokers�Get a list of brokers that a client application can use to bootstrap.

## Example Usage

<!--Start PulumiCodeChooser -->
```typescript
import * as pulumi from "@pulumi/pulumi";
import * as aws from "@pulumi/aws";

const example = aws.msk.getBootstrapBrokers({
    clusterArn: exampleAwsMskCluster.arn,
});
```
```python
import pulumi
import pulumi_aws as aws

example = aws.msk.get_bootstrap_brokers(cluster_arn=example_aws_msk_cluster["arn"])
```
```csharp
using System.Collections.Generic;
using System.Linq;
using Pulumi;
using Aws = Pulumi.Aws;

return await Deployment.RunAsync(() => 
{
    var example = Aws.Msk.GetBootstrapBrokers.Invoke(new()
    {
        ClusterArn = exampleAwsMskCluster.Arn,
    });

});
```
```go
package main

import (
	"github.com/pulumi/pulumi-aws/sdk/v6/go/aws/msk"
	"github.com/pulumi/pulumi/sdk/v3/go/pulumi"
)

func main() {
	pulumi.Run(func(ctx *pulumi.Context) error {
		_, err := msk.GetBootstrapBrokers(ctx, &msk.GetBootstrapBrokersArgs{
			ClusterArn: exampleAwsMskCluster.Arn,
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
import com.pulumi.aws.msk.MskFunctions;
import com.pulumi.aws.msk.inputs.GetBootstrapBrokersArgs;
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
        final var example = MskFunctions.getBootstrapBrokers(GetBootstrapBrokersArgs.builder()
            .clusterArn(exampleAwsMskCluster.arn())
            .build());

    }
}
```
```yaml
variables:
  example:
    fn::invoke:
      function: aws:msk:getBootstrapBrokers
      arguments:
        clusterArn: ${exampleAwsMskCluster.arn}
```
<!--End PulumiCodeChooser -->
:

clusterArn" (ARN of the cluster the nodes belong to.
"�
bootstrapBrokers" �Comma separated list of one or more hostname:port pairs of kafka brokers suitable to bootstrap connectivity to the kafka cluster.
"f
bootstrapBrokersPublicSaslIam" AOne or more DNS names (or IP addresses) and SASL IAM port pairs.
"j
bootstrapBrokersPublicSaslScram" COne or more DNS names (or IP addresses) and SASL SCRAM port pairs.
"]
bootstrapBrokersPublicTls" <One or more DNS names (or IP addresses) and TLS port pairs.
"`
bootstrapBrokersSaslIam" AOne or more DNS names (or IP addresses) and SASL IAM port pairs.
"d
bootstrapBrokersSaslScram" COne or more DNS names (or IP addresses) and SASL SCRAM port pairs.
"W
bootstrapBrokersTls" <One or more DNS names (or IP addresses) and TLS port pairs.
"�
&bootstrapBrokersVpcConnectivitySaslIam" jA string containing one or more DNS names (or IP addresses) and SASL IAM port pairs for VPC connectivity.
"�
(bootstrapBrokersVpcConnectivitySaslScram" lA string containing one or more DNS names (or IP addresses) and SASL SCRAM port pairs for VPC connectivity.
"�
"bootstrapBrokersVpcConnectivityTls" eA string containing one or more DNS names (or IP addresses) and TLS port pairs for VPC connectivity.
"

clusterArn" "E
id" ;The provider-assigned unique ID for this managed resource.
2�
<
mskgetBrokerNodes%aws:msk/getBrokerNodes:getBrokerNodes�Get information on an Amazon MSK Broker Nodes.

## Example Usage

<!--Start PulumiCodeChooser -->
```typescript
import * as pulumi from "@pulumi/pulumi";
import * as aws from "@pulumi/aws";

const example = aws.msk.getBrokerNodes({
    clusterArn: exampleAwsMskCluster.arn,
});
```
```python
import pulumi
import pulumi_aws as aws

example = aws.msk.get_broker_nodes(cluster_arn=example_aws_msk_cluster["arn"])
```
```csharp
using System.Collections.Generic;
using System.Linq;
using Pulumi;
using Aws = Pulumi.Aws;

return await Deployment.RunAsync(() => 
{
    var example = Aws.Msk.GetBrokerNodes.Invoke(new()
    {
        ClusterArn = exampleAwsMskCluster.Arn,
    });

});
```
```go
package main

import (
	"github.com/pulumi/pulumi-aws/sdk/v6/go/aws/msk"
	"github.com/pulumi/pulumi/sdk/v3/go/pulumi"
)

func main() {
	pulumi.Run(func(ctx *pulumi.Context) error {
		_, err := msk.GetBrokerNodes(ctx, &msk.GetBrokerNodesArgs{
			ClusterArn: exampleAwsMskCluster.Arn,
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
import com.pulumi.aws.msk.MskFunctions;
import com.pulumi.aws.msk.inputs.GetBrokerNodesArgs;
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
        final var example = MskFunctions.getBrokerNodes(GetBrokerNodesArgs.builder()
            .clusterArn(exampleAwsMskCluster.arn())
            .build());

    }
}
```
```yaml
variables:
  example:
    fn::invoke:
      function: aws:msk:getBrokerNodes
      arguments:
        clusterArn: ${exampleAwsMskCluster.arn}
```
<!--End PulumiCodeChooser -->
:

clusterArn" (ARN of the cluster the nodes belong to.
"

clusterArn" "E
id" ;The provider-assigned unique ID for this managed resource.
"w
nodeInfoListsf*d:b
`
mskgetBrokerNodesNodeInfoList=aws:msk/getBrokerNodesNodeInfoList:getBrokerNodesNodeInfoList2�=
0
msk
getClusteraws:msk/getCluster:getCluster�Get information on an Amazon MSK Cluster.

> **Note:** This data sources returns information on _provisioned_ clusters.

## Example Usage

<!--Start PulumiCodeChooser -->
```typescript
import * as pulumi from "@pulumi/pulumi";
import * as aws from "@pulumi/aws";

const example = aws.msk.getCluster({
    clusterName: "example",
});
```
```python
import pulumi
import pulumi_aws as aws

example = aws.msk.get_cluster(cluster_name="example")
```
```csharp
using System.Collections.Generic;
using System.Linq;
using Pulumi;
using Aws = Pulumi.Aws;

return await Deployment.RunAsync(() => 
{
    var example = Aws.Msk.GetCluster.Invoke(new()
    {
        ClusterName = "example",
    });

});
```
```go
package main

import (
	"github.com/pulumi/pulumi-aws/sdk/v6/go/aws/msk"
	"github.com/pulumi/pulumi/sdk/v3/go/pulumi"
)

func main() {
	pulumi.Run(func(ctx *pulumi.Context) error {
		_, err := msk.LookupCluster(ctx, &msk.LookupClusterArgs{
			ClusterName: "example",
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
import com.pulumi.aws.msk.MskFunctions;
import com.pulumi.aws.msk.inputs.GetClusterArgs;
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
        final var example = MskFunctions.getCluster(GetClusterArgs.builder()
            .clusterName("example")
            .build());

    }
}
```
```yaml
variables:
  example:
    fn::invoke:
      function: aws:msk:getCluster
      arguments:
        clusterName: example
```
<!--End PulumiCodeChooser -->
(
clusterName" Name of the cluster.
@
tagsB2" 0Map of key-value pairs assigned to the cluster.
"#
arn" ARN of the MSK cluster.
"�
bootstrapBrokers" �Comma separated list of one or more hostname:port pairs of kafka brokers suitable to bootstrap connectivity to the kafka cluster. Contains a value if `encryption_info.0.encryption_in_transit.0.client_broker` is set to `PLAINTEXT` or `TLS_PLAINTEXT`. The resource sorts values alphabetically. AWS may not always return all endpoints so this value is not guaranteed to be stable across applies.
"�
bootstrapBrokersPublicSaslIam" �One or more DNS names (or IP addresses) and SASL IAM port pairs. For example, `b-1-public.exampleClusterName.abcde.c2.kafka.us-east-1.amazonaws.com:9198,b-2-public.exampleClusterName.abcde.c2.kafka.us-east-1.amazonaws.com:9198,b-3-public.exampleClusterName.abcde.c2.kafka.us-east-1.amazonaws.com:9198`. This attribute will have a value if `encryption_info.0.encryption_in_transit.0.client_broker` is set to `TLS_PLAINTEXT` or `TLS` and `client_authentication.0.sasl.0.iam` is set to `true` and `broker_node_group_info.0.connectivity_info.0.public_access.0.type` is set to `SERVICE_PROVIDED_EIPS` and the cluster fulfill all other requirements for public access. The resource sorts the list alphabetically. AWS may not always return all endpoints so the values may not be stable across applies.
"�
bootstrapBrokersPublicSaslScram" �One or more DNS names (or IP addresses) and SASL SCRAM port pairs. For example, `b-1-public.exampleClusterName.abcde.c2.kafka.us-east-1.amazonaws.com:9196,b-2-public.exampleClusterName.abcde.c2.kafka.us-east-1.amazonaws.com:9196,b-3-public.exampleClusterName.abcde.c2.kafka.us-east-1.amazonaws.com:9196`. This attribute will have a value if `encryption_info.0.encryption_in_transit.0.client_broker` is set to `TLS_PLAINTEXT` or `TLS` and `client_authentication.0.sasl.0.scram` is set to `true` and `broker_node_group_info.0.connectivity_info.0.public_access.0.type` is set to `SERVICE_PROVIDED_EIPS` and the cluster fulfill all other requirements for public access. The resource sorts the list alphabetically. AWS may not always return all endpoints so the values may not be stable across applies.
"�
bootstrapBrokersPublicTls" �One or more DNS names (or IP addresses) and TLS port pairs. For example, `b-1-public.exampleClusterName.abcde.c2.kafka.us-east-1.amazonaws.com:9194,b-2-public.exampleClusterName.abcde.c2.kafka.us-east-1.amazonaws.com:9194,b-3-public.exampleClusterName.abcde.c2.kafka.us-east-1.amazonaws.com:9194`. This attribute will have a value if `encryption_info.0.encryption_in_transit.0.client_broker` is set to `TLS_PLAINTEXT` or `TLS` and `broker_node_group_info.0.connectivity_info.0.public_access.0.type` is set to `SERVICE_PROVIDED_EIPS` and the cluster fulfill all other requirements for public access. The resource sorts the list alphabetically. AWS may not always return all endpoints so the values may not be stable across applies.
"�
bootstrapBrokersSaslIam" �One or more DNS names (or IP addresses) and SASL IAM port pairs. For example, `b-1.exampleClusterName.abcde.c2.kafka.us-east-1.amazonaws.com:9098,b-2.exampleClusterName.abcde.c2.kafka.us-east-1.amazonaws.com:9098,b-3.exampleClusterName.abcde.c2.kafka.us-east-1.amazonaws.com:9098`. This attribute will have a value if `encryption_info.0.encryption_in_transit.0.client_broker` is set to `TLS_PLAINTEXT` or `TLS` and `client_authentication.0.sasl.0.iam` is set to `true`. The resource sorts the list alphabetically. AWS may not always return all endpoints so the values may not be stable across applies.
"�
bootstrapBrokersSaslScram" �One or more DNS names (or IP addresses) and SASL SCRAM port pairs. For example, `b-1.exampleClusterName.abcde.c2.kafka.us-east-1.amazonaws.com:9096,b-2.exampleClusterName.abcde.c2.kafka.us-east-1.amazonaws.com:9096,b-3.exampleClusterName.abcde.c2.kafka.us-east-1.amazonaws.com:9096`. This attribute will have a value if `encryption_info.0.encryption_in_transit.0.client_broker` is set to `TLS_PLAINTEXT` or `TLS` and `client_authentication.0.sasl.0.scram` is set to `true`. The resource sorts the list alphabetically. AWS may not always return all endpoints so the values may not be stable across applies.
"�
bootstrapBrokersTls" �One or more DNS names (or IP addresses) and TLS port pairs. For example, `b-1.exampleClusterName.abcde.c2.kafka.us-east-1.amazonaws.com:9094,b-2.exampleClusterName.abcde.c2.kafka.us-east-1.amazonaws.com:9094,b-3.exampleClusterName.abcde.c2.kafka.us-east-1.amazonaws.com:9094`. This attribute will have a value if `encryption_info.0.encryption_in_transit.0.client_broker` is set to `TLS_PLAINTEXT` or `TLS`. The resource sorts the list alphabetically. AWS may not always return all endpoints so the values may not be stable across applies.
"�
brokerNodeGroupInfoso*m:k
i
mskgetClusterBrokerNodeGroupInfoCaws:msk/getClusterBrokerNodeGroupInfo:getClusterBrokerNodeGroupInfo?Configuration block for the broker nodes of the Kafka cluster.
"
clusterName" "E
clusterUuid" 2UUID of the MSK cluster, for use in IAM policies.
"E
id" ;The provider-assigned unique ID for this managed resource.
"*
kafkaVersion" Apache Kafka version.
"B
numberOfBrokerNodes 'Number of broker nodes in the cluster.
">
tags2" 0Map of key-value pairs assigned to the cluster.
"�
zookeeperConnectString" �A comma separated list of one or more hostname:port pairs to use to connect to the Apache Zookeeper cluster. The returned values are sorted alphbetically. The AWS API may not return all endpoints, so this value is not guaranteed to be stable across applies.
"�
zookeeperConnectStringTls" �A comma separated list of one or more hostname:port pairs to use to connect to the Apache Zookeeper cluster via TLS. The returned values are sorted alphabetically. The AWS API may not return all endpoints, so this value is not guaranteed to be stable across applies.
2�
B
mskgetConfiguration)aws:msk/getConfiguration:getConfiguration�Get information on an Amazon MSK Configuration.

## Example Usage

<!--Start PulumiCodeChooser -->
```typescript
import * as pulumi from "@pulumi/pulumi";
import * as aws from "@pulumi/aws";

const example = aws.msk.getConfiguration({
    name: "example",
});
```
```python
import pulumi
import pulumi_aws as aws

example = aws.msk.get_configuration(name="example")
```
```csharp
using System.Collections.Generic;
using System.Linq;
using Pulumi;
using Aws = Pulumi.Aws;

return await Deployment.RunAsync(() => 
{
    var example = Aws.Msk.GetConfiguration.Invoke(new()
    {
        Name = "example",
    });

});
```
```go
package main

import (
	"github.com/pulumi/pulumi-aws/sdk/v6/go/aws/msk"
	"github.com/pulumi/pulumi/sdk/v3/go/pulumi"
)

func main() {
	pulumi.Run(func(ctx *pulumi.Context) error {
		_, err := msk.LookupConfiguration(ctx, &msk.LookupConfigurationArgs{
			Name: "example",
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
import com.pulumi.aws.msk.MskFunctions;
import com.pulumi.aws.msk.inputs.GetConfigurationArgs;
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
        final var example = MskFunctions.getConfiguration(GetConfigurationArgs.builder()
            .name("example")
            .build());

    }
}
```
```yaml
variables:
  example:
    fn::invoke:
      function: aws:msk:getConfiguration
      arguments:
        name: example
```
<!--End PulumiCodeChooser -->
'
name" Name of the configuration.
"%
arn" ARN of the configuration.
"5
description" "Description of the configuration.
"E
id" ;The provider-assigned unique ID for this managed resource.
"W
kafkaVersions*" @List of Apache Kafka versions which can use this configuration.
"<
latestRevision &Latest revision of the configuration.
"

name" "@
serverProperties" (Contents of the server.properties file.
2�
?
mskgetKafkaVersion'aws:msk/getKafkaVersion:getKafkaVersion�Get information on a Amazon MSK Kafka Version

## Example Usage

<!--Start PulumiCodeChooser -->
```typescript
import * as pulumi from "@pulumi/pulumi";
import * as aws from "@pulumi/aws";

const preferred = aws.msk.getKafkaVersion({
    preferredVersions: [
        "2.4.1.1",
        "2.4.1",
        "2.2.1",
    ],
});
const example = aws.msk.getKafkaVersion({
    version: "2.8.0",
});
```
```python
import pulumi
import pulumi_aws as aws

preferred = aws.msk.get_kafka_version(preferred_versions=[
    "2.4.1.1",
    "2.4.1",
    "2.2.1",
])
example = aws.msk.get_kafka_version(version="2.8.0")
```
```csharp
using System.Collections.Generic;
using System.Linq;
using Pulumi;
using Aws = Pulumi.Aws;

return await Deployment.RunAsync(() => 
{
    var preferred = Aws.Msk.GetKafkaVersion.Invoke(new()
    {
        PreferredVersions = new[]
        {
            "2.4.1.1",
            "2.4.1",
            "2.2.1",
        },
    });

    var example = Aws.Msk.GetKafkaVersion.Invoke(new()
    {
        Version = "2.8.0",
    });

});
```
```go
package main

import (
	"github.com/pulumi/pulumi-aws/sdk/v6/go/aws/msk"
	"github.com/pulumi/pulumi/sdk/v3/go/pulumi"
)

func main() {
	pulumi.Run(func(ctx *pulumi.Context) error {
		_, err := msk.GetKafkaVersion(ctx, &msk.GetKafkaVersionArgs{
			PreferredVersions: []string{
				"2.4.1.1",
				"2.4.1",
				"2.2.1",
			},
		}, nil)
		if err != nil {
			return err
		}
		_, err = msk.GetKafkaVersion(ctx, &msk.GetKafkaVersionArgs{
			Version: pulumi.StringRef("2.8.0"),
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
import com.pulumi.aws.msk.MskFunctions;
import com.pulumi.aws.msk.inputs.GetKafkaVersionArgs;
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
        final var preferred = MskFunctions.getKafkaVersion(GetKafkaVersionArgs.builder()
            .preferredVersions(            
                "2.4.1.1",
                "2.4.1",
                "2.2.1")
            .build());

        final var example = MskFunctions.getKafkaVersion(GetKafkaVersionArgs.builder()
            .version("2.8.0")
            .build());

    }
}
```
```yaml
variables:
  preferred:
    fn::invoke:
      function: aws:msk:getKafkaVersion
      arguments:
        preferredVersions:
          - 2.4.1.1
          - 2.4.1
          - 2.2.1
  example:
    fn::invoke:
      function: aws:msk:getKafkaVersion
      arguments:
        version: 2.8.0
```
<!--End PulumiCodeChooser -->
�
preferredVersionsB*" �Ordered list of preferred Kafka versions. The first match in this list will be returned. Either `preferred_versions` or `version` must be set.
�
versionB" pVersion of MSK Kafka. For example 2.4.1.1 or "2.2.1" etc. Either `preferred_versions` or `version` must be set.
"E
id" ;The provider-assigned unique ID for this managed resource.
"
preferredVersionsB*" "L
status" >Status of the MSK Kafka version eg. `ACTIVE` or `DEPRECATED`.
"
version" 2�
B
mskgetVpcConnection)aws:msk/getVpcConnection:getVpcConnection�Get information on an Amazon MSK VPC Connection.

## Example Usage

<!--Start PulumiCodeChooser -->
```typescript
import * as pulumi from "@pulumi/pulumi";
import * as aws from "@pulumi/aws";

const example = aws.msk.getVpcConnection({
    arn: exampleAwsMskVpcConnection.arn,
});
```
```python
import pulumi
import pulumi_aws as aws

example = aws.msk.get_vpc_connection(arn=example_aws_msk_vpc_connection["arn"])
```
```csharp
using System.Collections.Generic;
using System.Linq;
using Pulumi;
using Aws = Pulumi.Aws;

return await Deployment.RunAsync(() => 
{
    var example = Aws.Msk.GetVpcConnection.Invoke(new()
    {
        Arn = exampleAwsMskVpcConnection.Arn,
    });

});
```
```go
package main

import (
	"github.com/pulumi/pulumi-aws/sdk/v6/go/aws/msk"
	"github.com/pulumi/pulumi/sdk/v3/go/pulumi"
)

func main() {
	pulumi.Run(func(ctx *pulumi.Context) error {
		_, err := msk.LookupVpcConnection(ctx, &msk.LookupVpcConnectionArgs{
			Arn: exampleAwsMskVpcConnection.Arn,
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
import com.pulumi.aws.msk.MskFunctions;
import com.pulumi.aws.msk.inputs.GetVpcConnectionArgs;
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
        final var example = MskFunctions.getVpcConnection(GetVpcConnectionArgs.builder()
            .arn(exampleAwsMskVpcConnection.arn())
            .build());

    }
}
```
```yaml
variables:
  example:
    fn::invoke:
      function: aws:msk:getVpcConnection
      arguments:
        arn: ${exampleAwsMskVpcConnection.arn}
```
<!--End PulumiCodeChooser -->
&
arn" ARN of the VPC Connection.
G
tagsB2" 7Map of key-value pairs assigned to the VPC Connection.
"	
arn" "M
authentication" 7The authentication type for the client VPC Connection.
">
clientSubnets*" 'The list of subnets in the client VPC.
"E
id" ;The provider-assigned unique ID for this managed resource.
"W
securityGroups*" ?The security groups attached to the ENIs for the broker nodes.
"E
tags2" 7Map of key-value pairs assigned to the VPC Connection.
"G
targetClusterArn" /The Amazon Resource Name (ARN) of the cluster.
".
vpcId" !The VPC ID of the remote client.
2�
D

mskconnectgetConnector(aws:mskconnect/getConnector:getConnector�Get information on an Amazon MSK Connect Connector.

## Example Usage

<!--Start PulumiCodeChooser -->
```typescript
import * as pulumi from "@pulumi/pulumi";
import * as aws from "@pulumi/aws";

const example = aws.mskconnect.getConnector({
    name: "example-mskconnector",
});
```
```python
import pulumi
import pulumi_aws as aws

example = aws.mskconnect.get_connector(name="example-mskconnector")
```
```csharp
using System.Collections.Generic;
using System.Linq;
using Pulumi;
using Aws = Pulumi.Aws;

return await Deployment.RunAsync(() => 
{
    var example = Aws.MskConnect.GetConnector.Invoke(new()
    {
        Name = "example-mskconnector",
    });

});
```
```go
package main

import (
	"github.com/pulumi/pulumi-aws/sdk/v6/go/aws/mskconnect"
	"github.com/pulumi/pulumi/sdk/v3/go/pulumi"
)

func main() {
	pulumi.Run(func(ctx *pulumi.Context) error {
		_, err := mskconnect.LookupConnector(ctx, &mskconnect.LookupConnectorArgs{
			Name: "example-mskconnector",
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
import com.pulumi.aws.mskconnect.MskconnectFunctions;
import com.pulumi.aws.mskconnect.inputs.GetConnectorArgs;
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
        final var example = MskconnectFunctions.getConnector(GetConnectorArgs.builder()
            .name("example-mskconnector")
            .build());

    }
}
```
```yaml
variables:
  example:
    fn::invoke:
      function: aws:mskconnect:getConnector
      arguments:
        name: example-mskconnector
```
<!--End PulumiCodeChooser -->
#
name" Name of the connector.
8
tagsB2" (A map of tags assigned to the resource.
"!
arn" ARN of the connector.
"9
description" &Summary description of the connector.
"E
id" ;The provider-assigned unique ID for this managed resource.
"

name" "6
tags2" (A map of tags assigned to the resource.
"1
version" "Current version of the connector.
2�
M

mskconnectgetCustomPlugin.aws:mskconnect/getCustomPlugin:getCustomPlugin�Get information on an Amazon MSK Connect custom plugin.

## Example Usage

<!--Start PulumiCodeChooser -->
```typescript
import * as pulumi from "@pulumi/pulumi";
import * as aws from "@pulumi/aws";

const example = aws.mskconnect.getCustomPlugin({
    name: "example-debezium-1",
});
```
```python
import pulumi
import pulumi_aws as aws

example = aws.mskconnect.get_custom_plugin(name="example-debezium-1")
```
```csharp
using System.Collections.Generic;
using System.Linq;
using Pulumi;
using Aws = Pulumi.Aws;

return await Deployment.RunAsync(() => 
{
    var example = Aws.MskConnect.GetCustomPlugin.Invoke(new()
    {
        Name = "example-debezium-1",
    });

});
```
```go
package main

import (
	"github.com/pulumi/pulumi-aws/sdk/v6/go/aws/mskconnect"
	"github.com/pulumi/pulumi/sdk/v3/go/pulumi"
)

func main() {
	pulumi.Run(func(ctx *pulumi.Context) error {
		_, err := mskconnect.LookupCustomPlugin(ctx, &mskconnect.LookupCustomPluginArgs{
			Name: "example-debezium-1",
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
import com.pulumi.aws.mskconnect.MskconnectFunctions;
import com.pulumi.aws.mskconnect.inputs.GetCustomPluginArgs;
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
        final var example = MskconnectFunctions.getCustomPlugin(GetCustomPluginArgs.builder()
            .name("example-debezium-1")
            .build());

    }
}
```
```yaml
variables:
  example:
    fn::invoke:
      function: aws:mskconnect:getCustomPlugin
      arguments:
        name: example-debezium-1
```
<!--End PulumiCodeChooser -->
'
name" Name of the custom plugin.
8
tagsB2" (A map of tags assigned to the resource.
")
arn" the ARN of the custom plugin.
"?
description" ,a summary description of the custom plugin.
"E
id" ;The provider-assigned unique ID for this managed resource.
"^
latestRevision Han ID of the latest successfully created revision of the custom plugin.
"

name" "-
state"  the state of the custom plugin.
"6
tags2" (A map of tags assigned to the resource.
2�
b

mskconnectgetWorkerConfiguration<aws:mskconnect/getWorkerConfiguration:getWorkerConfiguration�Get information on an Amazon MSK Connect Worker Configuration.

## Example Usage

<!--Start PulumiCodeChooser -->
```typescript
import * as pulumi from "@pulumi/pulumi";
import * as aws from "@pulumi/aws";

const example = aws.mskconnect.getWorkerConfiguration({
    name: "example",
});
```
```python
import pulumi
import pulumi_aws as aws

example = aws.mskconnect.get_worker_configuration(name="example")
```
```csharp
using System.Collections.Generic;
using System.Linq;
using Pulumi;
using Aws = Pulumi.Aws;

return await Deployment.RunAsync(() => 
{
    var example = Aws.MskConnect.GetWorkerConfiguration.Invoke(new()
    {
        Name = "example",
    });

});
```
```go
package main

import (
	"github.com/pulumi/pulumi-aws/sdk/v6/go/aws/mskconnect"
	"github.com/pulumi/pulumi/sdk/v3/go/pulumi"
)

func main() {
	pulumi.Run(func(ctx *pulumi.Context) error {
		_, err := mskconnect.LookupWorkerConfiguration(ctx, &mskconnect.LookupWorkerConfigurationArgs{
			Name: "example",
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
import com.pulumi.aws.mskconnect.MskconnectFunctions;
import com.pulumi.aws.mskconnect.inputs.GetWorkerConfigurationArgs;
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
        final var example = MskconnectFunctions.getWorkerConfiguration(GetWorkerConfigurationArgs.builder()
            .name("example")
            .build());

    }
}
```
```yaml
variables:
  example:
    fn::invoke:
      function: aws:mskconnect:getWorkerConfiguration
      arguments:
        name: example
```
<!--End PulumiCodeChooser -->
.
name" "Name of the worker configuration.
8
tagsB2" (A map of tags assigned to the resource.
"0
arn" %the ARN of the worker configuration.
"F
description" 3a summary description of the worker configuration.
"E
id" ;The provider-assigned unique ID for this managed resource.
"e
latestRevision Oan ID of the latest successfully created revision of the worker configuration.
"

name" "N
propertiesFileContent" 1contents of connect-distributed.properties file.
"6
tags2" (A map of tags assigned to the resource.
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
:�
m
macieFindingsFilterFindingCriteriaEaws:macie/FindingsFilterFindingCriteria:FindingsFilterFindingCriteria�
��

criterions�B�*�:�
�
macie&FindingsFilterFindingCriteriaCriterionWaws:macie/FindingsFilterFindingCriteriaCriterion:FindingsFilterFindingCriteriaCriterion|A condition that specifies the property, operator, and one or more values to use to filter the results.  (documented below)
:�
�
macie&FindingsFilterFindingCriteriaCriterionWaws:macie/FindingsFilterFindingCriteriaCriterion:FindingsFilterFindingCriteriaCriterion�
��
eqExactMatchesB*" �The value for the property exclusively matches (equals an exact match for) all the specified values. If you specify multiple values, Amazon Macie uses AND logic to join the values.
�
eqsB*" �The value for the property matches (equals) the specified value. If you specify multiple values, Amazon Macie uses OR logic to join the values.
4
field" 'The name of the field to be evaluated.
L
gtB" @The value for the property is greater than the specified value.
Y
gteB" LThe value for the property is greater than or equal to the specified value.
I
ltB" =The value for the property is less than the specified value.
V
lteB" IThe value for the property is less than or equal to the specified value.
�
neqsB*" �The value for the property doesn't match (doesn't equal) the specified value. If you specify multiple values, Amazon Macie uses OR logic to join the values.
:�
�
macie2.ClassificationExportConfigurationS3Destinationhaws:macie2/ClassificationExportConfigurationS3Destination:ClassificationExportConfigurationS3Destination�
�k

bucketName" YThe Amazon S3 bucket name in which Amazon Macie exports the data classification results.
p
	keyPrefixB" ]The object key for the bucket in which Amazon Macie exports the data classification results.
�
	kmsKeyArn" �Amazon Resource Name (ARN) of the KMS key to be used to encrypt the data.

Additional information can be found in the [Storing and retaining sensitive data discovery results with Amazon Macie for AWS Macie documentation](https://docs.aws.amazon.com/macie/latest/user/discovery-results-repository-s3.html).
:�	
x
macie2 ClassificationJobS3JobDefinitionLaws:macie2/ClassificationJobS3JobDefinition:ClassificationJobS3JobDefinition�
��
bucketCriteria�B�:�
�
macie2.ClassificationJobS3JobDefinitionBucketCriteriahaws:macie2/ClassificationJobS3JobDefinitionBucketCriteria:ClassificationJobS3JobDefinitionBucketCriteria�The property- and tag-based conditions that determine which S3 buckets to include or exclude from the analysis. Conflicts with `bucket_definitions`. (documented below)
�
bucketDefinitions�B�*�:�
�
macie20ClassificationJobS3JobDefinitionBucketDefinitionlaws:macie2/ClassificationJobS3JobDefinitionBucketDefinition:ClassificationJobS3JobDefinitionBucketDefinition�An array of objects, one for each AWS account that owns buckets to analyze. Each object specifies the account ID for an account and one or more buckets to analyze for the account. Conflicts with `bucket_criteria`. (documented below)
�
scoping�B�:�
�
macie2'ClassificationJobS3JobDefinitionScopingZaws:macie2/ClassificationJobS3JobDefinitionScoping:ClassificationJobS3JobDefinitionScoping�The property- and tag-based conditions that determine which objects to include or exclude from the analysis. (documented below)
:�
�
macie2.ClassificationJobS3JobDefinitionBucketCriteriahaws:macie2/ClassificationJobS3JobDefinitionBucketCriteria:ClassificationJobS3JobDefinitionBucketCriteria�
��
excludes�B�:�
�
macie26ClassificationJobS3JobDefinitionBucketCriteriaExcludesxaws:macie2/ClassificationJobS3JobDefinitionBucketCriteriaExcludes:ClassificationJobS3JobDefinitionBucketCriteriaExcludeswThe property- or tag-based conditions that determine which S3 buckets to exclude from the analysis. (documented below)
�
includes�B�:�
�
macie26ClassificationJobS3JobDefinitionBucketCriteriaIncludesxaws:macie2/ClassificationJobS3JobDefinitionBucketCriteriaIncludes:ClassificationJobS3JobDefinitionBucketCriteriaIncludesuThe property- or tag-based conditions that determine which S3 buckets to include in the analysis. (documented below)
:�
�
macie26ClassificationJobS3JobDefinitionBucketCriteriaExcludesxaws:macie2/ClassificationJobS3JobDefinitionBucketCriteriaExcludes:ClassificationJobS3JobDefinitionBucketCriteriaExcludes�
��
ands�B�*�:�
�
macie29ClassificationJobS3JobDefinitionBucketCriteriaExcludesAnd~aws:macie2/ClassificationJobS3JobDefinitionBucketCriteriaExcludesAnd:ClassificationJobS3JobDefinitionBucketCriteriaExcludesAnd�An array of conditions, one for each condition that determines which objects to include or exclude from the job. (documented below)
:�
�
macie29ClassificationJobS3JobDefinitionBucketCriteriaExcludesAnd~aws:macie2/ClassificationJobS3JobDefinitionBucketCriteriaExcludesAnd:ClassificationJobS3JobDefinitionBucketCriteriaExcludesAnd�
��
simpleCriterion�B�:�
�
macie2HClassificationJobS3JobDefinitionBucketCriteriaExcludesAndSimpleCriterion�aws:macie2/ClassificationJobS3JobDefinitionBucketCriteriaExcludesAndSimpleCriterion:ClassificationJobS3JobDefinitionBucketCriteriaExcludesAndSimpleCriterion�A property-based condition that defines a property, operator, and one or more values for including or excluding an S3 buckets from the job. (documented below)
�
tagCriterion�B�:�
�
macie2EClassificationJobS3JobDefinitionBucketCriteriaExcludesAndTagCriterion�aws:macie2/ClassificationJobS3JobDefinitionBucketCriteriaExcludesAndTagCriterion:ClassificationJobS3JobDefinitionBucketCriteriaExcludesAndTagCriterion�A tag-based condition that defines the operator and tag keys or tag key and value pairs for including or excluding an S3 buckets from the job. (documented below)
:�
�
macie2HClassificationJobS3JobDefinitionBucketCriteriaExcludesAndSimpleCriterion�aws:macie2/ClassificationJobS3JobDefinitionBucketCriteriaExcludesAndSimpleCriterion:ClassificationJobS3JobDefinitionBucketCriteriaExcludesAndSimpleCriterion�
��

comparatorB" �The operator to use in a condition. Valid combination of values are available in the [AWS Documentation](https://docs.aws.amazon.com/macie/latest/APIReference/jobs.html#jobs-model-jobcomparator)
�
keyB" �The object property to use in the condition. Valid combination of values are available in the [AWS Documentation](https://docs.aws.amazon.com/macie/latest/APIReference/jobs.html#jobs-model-simplecriterionkeyforjob)
�
valuesB*" �An array that lists the values to use in the condition. Valid combination of values are available in the [AWS Documentation](https://docs.aws.amazon.com/macie/latest/APIReference/jobs.html#jobs-model-simplecriterionforjob)
:�
�
macie2EClassificationJobS3JobDefinitionBucketCriteriaExcludesAndTagCriterion�aws:macie2/ClassificationJobS3JobDefinitionBucketCriteriaExcludesAndTagCriterion:ClassificationJobS3JobDefinitionBucketCriteriaExcludesAndTagCriterion�
��

comparatorB" �The operator to use in the condition. Valid combination and values are available in the [AWS Documentation](https://docs.aws.amazon.com/macie/latest/APIReference/jobs.html#jobs-model-jobcomparator)
�
	tagValues�B�*�:�
�
macie2MClassificationJobS3JobDefinitionBucketCriteriaExcludesAndTagCriterionTagValue�aws:macie2/ClassificationJobS3JobDefinitionBucketCriteriaExcludesAndTagCriterionTagValue:ClassificationJobS3JobDefinitionBucketCriteriaExcludesAndTagCriterionTagValueiThe  tag key and value pairs to use in the condition. One or more blocks are allowed. (documented below)
:�
�
macie2MClassificationJobS3JobDefinitionBucketCriteriaExcludesAndTagCriterionTagValue�aws:macie2/ClassificationJobS3JobDefinitionBucketCriteriaExcludesAndTagCriterionTagValue:ClassificationJobS3JobDefinitionBucketCriteriaExcludesAndTagCriterionTagValue>
<
keyB" The tag key.

valueB" The tag value.
:�
�
macie26ClassificationJobS3JobDefinitionBucketCriteriaIncludesxaws:macie2/ClassificationJobS3JobDefinitionBucketCriteriaIncludes:ClassificationJobS3JobDefinitionBucketCriteriaIncludes�
��
ands�B�*�:�
�
macie29ClassificationJobS3JobDefinitionBucketCriteriaIncludesAnd~aws:macie2/ClassificationJobS3JobDefinitionBucketCriteriaIncludesAnd:ClassificationJobS3JobDefinitionBucketCriteriaIncludesAnd�An array of conditions, one for each condition that determines which objects to include or exclude from the job. (documented below)
:�
�
macie29ClassificationJobS3JobDefinitionBucketCriteriaIncludesAnd~aws:macie2/ClassificationJobS3JobDefinitionBucketCriteriaIncludesAnd:ClassificationJobS3JobDefinitionBucketCriteriaIncludesAnd�
��
simpleCriterion�B�:�
�
macie2HClassificationJobS3JobDefinitionBucketCriteriaIncludesAndSimpleCriterion�aws:macie2/ClassificationJobS3JobDefinitionBucketCriteriaIncludesAndSimpleCriterion:ClassificationJobS3JobDefinitionBucketCriteriaIncludesAndSimpleCriterion�A property-based condition that defines a property, operator, and one or more values for including or excluding an S3 buckets from the job. (documented below)
�
tagCriterion�B�:�
�
macie2EClassificationJobS3JobDefinitionBucketCriteriaIncludesAndTagCriterion�aws:macie2/ClassificationJobS3JobDefinitionBucketCriteriaIncludesAndTagCriterion:ClassificationJobS3JobDefinitionBucketCriteriaIncludesAndTagCriterion�A tag-based condition that defines the operator and tag keys or tag key and value pairs for including or excluding an S3 buckets from the job. (documented below)
:�
�
macie2HClassificationJobS3JobDefinitionBucketCriteriaIncludesAndSimpleCriterion�aws:macie2/ClassificationJobS3JobDefinitionBucketCriteriaIncludesAndSimpleCriterion:ClassificationJobS3JobDefinitionBucketCriteriaIncludesAndSimpleCriterion�
��

comparatorB" �The operator to use in a condition. Valid combination of values are available in the [AWS Documentation](https://docs.aws.amazon.com/macie/latest/APIReference/jobs.html#jobs-model-jobcomparator)
�
keyB" �The object property to use in the condition. Valid combination of values are available in the [AWS Documentation](https://docs.aws.amazon.com/macie/latest/APIReference/jobs.html#jobs-model-simplecriterionkeyforjob)
�
valuesB*" �An array that lists the values to use in the condition. Valid combination of values are available in the [AWS Documentation](https://docs.aws.amazon.com/macie/latest/APIReference/jobs.html#jobs-model-simplecriterionforjob)
:�
�
macie2EClassificationJobS3JobDefinitionBucketCriteriaIncludesAndTagCriterion�aws:macie2/ClassificationJobS3JobDefinitionBucketCriteriaIncludesAndTagCriterion:ClassificationJobS3JobDefinitionBucketCriteriaIncludesAndTagCriterion�
��

comparatorB" �The operator to use in the condition. Valid combination and values are available in the [AWS Documentation](https://docs.aws.amazon.com/macie/latest/APIReference/jobs.html#jobs-model-jobcomparator)
�
	tagValues�B�*�:�
�
macie2MClassificationJobS3JobDefinitionBucketCriteriaIncludesAndTagCriterionTagValue�aws:macie2/ClassificationJobS3JobDefinitionBucketCriteriaIncludesAndTagCriterionTagValue:ClassificationJobS3JobDefinitionBucketCriteriaIncludesAndTagCriterionTagValueiThe  tag key and value pairs to use in the condition. One or more blocks are allowed. (documented below)
:�
�
macie2MClassificationJobS3JobDefinitionBucketCriteriaIncludesAndTagCriterionTagValue�aws:macie2/ClassificationJobS3JobDefinitionBucketCriteriaIncludesAndTagCriterionTagValue:ClassificationJobS3JobDefinitionBucketCriteriaIncludesAndTagCriterionTagValue>
<
keyB" The tag key.

valueB" The tag value.
:�
�
macie20ClassificationJobS3JobDefinitionBucketDefinitionlaws:macie2/ClassificationJobS3JobDefinitionBucketDefinition:ClassificationJobS3JobDefinitionBucketDefinition�
�R
	accountId" AThe unique identifier for the AWS account that owns the buckets.
?
buckets*" .An array that lists the names of the buckets.
:�
�
macie2'ClassificationJobS3JobDefinitionScopingZaws:macie2/ClassificationJobS3JobDefinitionScoping:ClassificationJobS3JobDefinitionScoping�
��
excludes�B�:�
�
macie2/ClassificationJobS3JobDefinitionScopingExcludesjaws:macie2/ClassificationJobS3JobDefinitionScopingExcludes:ClassificationJobS3JobDefinitionScopingExcludestThe property- or tag-based conditions that determine which objects to exclude from the analysis. (documented below)
�
includes�B�:�
�
macie2/ClassificationJobS3JobDefinitionScopingIncludesjaws:macie2/ClassificationJobS3JobDefinitionScopingIncludes:ClassificationJobS3JobDefinitionScopingIncludesrThe property- or tag-based conditions that determine which objects to include in the analysis. (documented below)
:�
�
macie2/ClassificationJobS3JobDefinitionScopingExcludesjaws:macie2/ClassificationJobS3JobDefinitionScopingExcludes:ClassificationJobS3JobDefinitionScopingExcludes�
��
ands�B�*�:�
�
macie22ClassificationJobS3JobDefinitionScopingExcludesAndpaws:macie2/ClassificationJobS3JobDefinitionScopingExcludesAnd:ClassificationJobS3JobDefinitionScopingExcludesAnd�An array of conditions, one for each condition that determines which objects to include or exclude from the job. (documented below)
:�
�
macie22ClassificationJobS3JobDefinitionScopingExcludesAndpaws:macie2/ClassificationJobS3JobDefinitionScopingExcludesAnd:ClassificationJobS3JobDefinitionScopingExcludesAnd�
��
simpleScopeTerm�B�:�
�
macie2AClassificationJobS3JobDefinitionScopingExcludesAndSimpleScopeTerm�aws:macie2/ClassificationJobS3JobDefinitionScopingExcludesAndSimpleScopeTerm:ClassificationJobS3JobDefinitionScopingExcludesAndSimpleScopeTerm�A property-based condition that defines a property, operator, and one or more values for including or excluding an object from the job. (documented below)
�
tagScopeTerm�B�:�
�
macie2>ClassificationJobS3JobDefinitionScopingExcludesAndTagScopeTerm�aws:macie2/ClassificationJobS3JobDefinitionScopingExcludesAndTagScopeTerm:ClassificationJobS3JobDefinitionScopingExcludesAndTagScopeTerm�A tag-based condition that defines the operator and tag keys or tag key and value pairs for including or excluding an object from the job. (documented below)
:�
�
macie2AClassificationJobS3JobDefinitionScopingExcludesAndSimpleScopeTerm�aws:macie2/ClassificationJobS3JobDefinitionScopingExcludesAndSimpleScopeTerm:ClassificationJobS3JobDefinitionScopingExcludesAndSimpleScopeTerm�
��

comparatorB" vThe operator to use in a condition. Valid values are: `EQ`, `GT`, `GTE`, `LT`, `LTE`, `NE`, `CONTAINS`, `STARTS_WITH`
:
keyB" -The object property to use in the condition.
J
valuesB*" 8An array that lists the values to use in the condition.
:�
�
macie2>ClassificationJobS3JobDefinitionScopingExcludesAndTagScopeTerm�aws:macie2/ClassificationJobS3JobDefinitionScopingExcludesAndTagScopeTerm:ClassificationJobS3JobDefinitionScopingExcludesAndTagScopeTerm�
�:

comparatorB" &The operator to use in the condition.
Q
keyB" DThe tag key to use in the condition. The only valid value is `TAG`.
�
	tagValues�B�*�:�
�
macie2FClassificationJobS3JobDefinitionScopingExcludesAndTagScopeTermTagValue�aws:macie2/ClassificationJobS3JobDefinitionScopingExcludesAndTagScopeTermTagValue:ClassificationJobS3JobDefinitionScopingExcludesAndTagScopeTermTagValueAThe tag keys or tag key and value pairs to use in the condition.
c
targetB" SThe type of object to apply the condition to. The only valid value is `S3_OBJECT`.
:�
�
macie2FClassificationJobS3JobDefinitionScopingExcludesAndTagScopeTermTagValue�aws:macie2/ClassificationJobS3JobDefinitionScopingExcludesAndTagScopeTermTagValue:ClassificationJobS3JobDefinitionScopingExcludesAndTagScopeTermTagValue>
<
keyB" The tag key.

valueB" The tag value.
:�
�
macie2/ClassificationJobS3JobDefinitionScopingIncludesjaws:macie2/ClassificationJobS3JobDefinitionScopingIncludes:ClassificationJobS3JobDefinitionScopingIncludes�
��
ands�B�*�:�
�
macie22ClassificationJobS3JobDefinitionScopingIncludesAndpaws:macie2/ClassificationJobS3JobDefinitionScopingIncludesAnd:ClassificationJobS3JobDefinitionScopingIncludesAnd�An array of conditions, one for each condition that determines which objects to include or exclude from the job. (documented below)
:�
�
macie22ClassificationJobS3JobDefinitionScopingIncludesAndpaws:macie2/ClassificationJobS3JobDefinitionScopingIncludesAnd:ClassificationJobS3JobDefinitionScopingIncludesAnd�
��
simpleScopeTerm�B�:�
�
macie2AClassificationJobS3JobDefinitionScopingIncludesAndSimpleScopeTerm�aws:macie2/ClassificationJobS3JobDefinitionScopingIncludesAndSimpleScopeTerm:ClassificationJobS3JobDefinitionScopingIncludesAndSimpleScopeTerm�A property-based condition that defines a property, operator, and one or more values for including or excluding an object from the job. (documented below)
�
tagScopeTerm�B�:�
�
macie2>ClassificationJobS3JobDefinitionScopingIncludesAndTagScopeTerm�aws:macie2/ClassificationJobS3JobDefinitionScopingIncludesAndTagScopeTerm:ClassificationJobS3JobDefinitionScopingIncludesAndTagScopeTerm�A tag-based condition that defines the operator and tag keys or tag key and value pairs for including or excluding an object from the job. (documented below)
:�
�
macie2AClassificationJobS3JobDefinitionScopingIncludesAndSimpleScopeTerm�aws:macie2/ClassificationJobS3JobDefinitionScopingIncludesAndSimpleScopeTerm:ClassificationJobS3JobDefinitionScopingIncludesAndSimpleScopeTerm�
��

comparatorB" vThe operator to use in a condition. Valid values are: `EQ`, `GT`, `GTE`, `LT`, `LTE`, `NE`, `CONTAINS`, `STARTS_WITH`
:
keyB" -The object property to use in the condition.
J
valuesB*" 8An array that lists the values to use in the condition.
:�
�
macie2>ClassificationJobS3JobDefinitionScopingIncludesAndTagScopeTerm�aws:macie2/ClassificationJobS3JobDefinitionScopingIncludesAndTagScopeTerm:ClassificationJobS3JobDefinitionScopingIncludesAndTagScopeTerm�
�:

comparatorB" &The operator to use in the condition.
Q
keyB" DThe tag key to use in the condition. The only valid value is `TAG`.
�
	tagValues�B�*�:�
�
macie2FClassificationJobS3JobDefinitionScopingIncludesAndTagScopeTermTagValue�aws:macie2/ClassificationJobS3JobDefinitionScopingIncludesAndTagScopeTermTagValue:ClassificationJobS3JobDefinitionScopingIncludesAndTagScopeTermTagValueAThe tag keys or tag key and value pairs to use in the condition.
c
targetB" SThe type of object to apply the condition to. The only valid value is `S3_OBJECT`.
:�
�
macie2FClassificationJobS3JobDefinitionScopingIncludesAndTagScopeTermTagValue�aws:macie2/ClassificationJobS3JobDefinitionScopingIncludesAndTagScopeTermTagValue:ClassificationJobS3JobDefinitionScopingIncludesAndTagScopeTermTagValue>
<
keyB" The tag key.

valueB" The tag value.
:�
~
macie2"ClassificationJobScheduleFrequencyPaws:macie2/ClassificationJobScheduleFrequency:ClassificationJobScheduleFrequency�
�Q
dailyScheduleB
 :Specifies a daily recurrence pattern for running the job.
U
monthlyScheduleB <Specifies a monthly recurrence pattern for running the job.
S
weeklyScheduleB" ;Specifies a weekly recurrence pattern for running the job.
:�
{
macie2!ClassificationJobUserPausedDetailNaws:macie2/ClassificationJobUserPausedDetail:ClassificationJobUserPausedDetailZ
X
jobExpiresAtB" +
#jobImminentExpirationHealthEventArnB" 
jobPausedAtB" :�
x
mediaconvertQueueReservationPlanSettingsJaws:mediaconvert/QueueReservationPlanSettings:QueueReservationPlanSettings�
�t

commitment" bThe length of the term of your reserved queue pricing plan commitment. Valid value is `ONE_YEAR`.
~
renewalType" kSpecifies whether the term of your reserved queue pricing plan. Valid values are `AUTO_RENEW` or `EXPIRE`.
W
reservedSlots BSpecifies the number of reserved transcode slots (RTS) for queue.
:�
r
	medialiveChannelCdiInputSpecificationGaws:medialive/ChannelCdiInputSpecification:ChannelCdiInputSpecification4
20

resolution" Maximum CDI input resolution.
:�
T
	medialiveChannelDestination3aws:medialive/ChannelDestination:ChannelDestination�
�J
id" @User-specified id. Ths is used in an output group or an output.
�
mediaPackageSettings�B�*�:�
�
	medialive%ChannelDestinationMediaPackageSettingYaws:medialive/ChannelDestinationMediaPackageSetting:ChannelDestinationMediaPackageSetting�Destination settings for a MediaPackage output; one destination for both encoders. See Media Package Settings for more details.
�
multiplexSettings�B�:�
�
	medialive#ChannelDestinationMultiplexSettingsUaws:medialive/ChannelDestinationMultiplexSettings:ChannelDestinationMultiplexSettingsyDestination settings for a Multiplex output; one destination for both encoders. See Multiplex Settings for more details.
�
settingsqBo*m:k
i
	medialiveChannelDestinationSettingAaws:medialive/ChannelDestinationSetting:ChannelDestinationSettingwDestination settings for a standard output; one destination for each redundant encoder. See Settings for more details.
:�
�
	medialive%ChannelDestinationMediaPackageSettingYaws:medialive/ChannelDestinationMediaPackageSetting:ChannelDestinationMediaPackageSettingf
db
	channelId" QID of the channel in MediaPackage that is the destination for this output group.
:�
�
	medialive#ChannelDestinationMultiplexSettingsUaws:medialive/ChannelDestinationMultiplexSettings:ChannelDestinationMultiplexSettings�
�T
multiplexId" AThe ID of the Multiplex that the encoder is providing output to.
f
programName" SThe program name of the Multiplex program that the encoder is providing output to.
:�
i
	medialiveChannelDestinationSettingAaws:medialive/ChannelDestinationSetting:ChannelDestinationSetting�
�R
passwordParamB" ;Key used to extract the password from EC2 Parameter store.
I

streamNameB" 5Stream name RTMP destinations (URLs of type rtmp://)
-
urlB"  A URL specifying a destination.
,
usernameB" Username for destination.
:�
`
	medialiveChannelEncoderSettings;aws:medialive/ChannelEncoderSettings:ChannelEncoderSettings�
��
audioDescriptions�B�*�:�
�
	medialive&ChannelEncoderSettingsAudioDescription[aws:medialive/ChannelEncoderSettingsAudioDescription:ChannelEncoderSettingsAudioDescriptionMAudio descriptions for the channel. See Audio Descriptions for more details.
�
availBlanking�B�:�
�
	medialive#ChannelEncoderSettingsAvailBlankingUaws:medialive/ChannelEncoderSettingsAvailBlanking:ChannelEncoderSettingsAvailBlankingESettings for ad avail blanking. See Avail Blanking for more details.
�
captionDescriptions�B�*�:�
�
	medialive(ChannelEncoderSettingsCaptionDescription_aws:medialive/ChannelEncoderSettingsCaptionDescription:ChannelEncoderSettingsCaptionDescriptionACaption Descriptions. See Caption Descriptions for more details.
�
globalConfiguration�B�:�
�
	medialive)ChannelEncoderSettingsGlobalConfigurationaaws:medialive/ChannelEncoderSettingsGlobalConfiguration:ChannelEncoderSettingsGlobalConfigurationfConfiguration settings that apply to the event as a whole. See Global Configuration for more details.
�
motionGraphicsConfiguration�B�:�
�
	medialive1ChannelEncoderSettingsMotionGraphicsConfigurationqaws:medialive/ChannelEncoderSettingsMotionGraphicsConfiguration:ChannelEncoderSettingsMotionGraphicsConfigurationRSettings for motion graphics. See Motion Graphics Configuration for more details.
�
nielsenConfiguration�B�:�
�
	medialive*ChannelEncoderSettingsNielsenConfigurationcaws:medialive/ChannelEncoderSettingsNielsenConfiguration:ChannelEncoderSettingsNielsenConfigurationLNielsen configuration settings. See Nielsen Configuration for more details.
�
outputGroups�*�:�
�
	medialive!ChannelEncoderSettingsOutputGroupQaws:medialive/ChannelEncoderSettingsOutputGroup:ChannelEncoderSettingsOutputGroupCOutput groups for the channel. See Output Groups for more details.
�
timecodeConfig�:�
�
	medialive$ChannelEncoderSettingsTimecodeConfigWaws:medialive/ChannelEncoderSettingsTimecodeConfig:ChannelEncoderSettingsTimecodeConfiguContains settings used to acquire and adjust timecode information from inputs. See Timecode Config for more details.
�
videoDescriptions�B�*�:�
�
	medialive&ChannelEncoderSettingsVideoDescription[aws:medialive/ChannelEncoderSettingsVideoDescription:ChannelEncoderSettingsVideoDescription=Video Descriptions. See Video Descriptions for more details.
:�
�
	medialive&ChannelEncoderSettingsAudioDescription[aws:medialive/ChannelEncoderSettingsAudioDescription:ChannelEncoderSettingsAudioDescription�
��
audioNormalizationSettings�B�:�
�
	medialive@ChannelEncoderSettingsAudioDescriptionAudioNormalizationSettings�aws:medialive/ChannelEncoderSettingsAudioDescriptionAudioNormalizationSettings:ChannelEncoderSettingsAudioDescriptionAudioNormalizationSettingsZAdvanced audio normalization settings. See Audio Normalization Settings for more details.
f
audioSelectorName" MThe name of the audio selector used as the source for this AudioDescription.

	audioTypeB" lApplies only if audioTypeControl is useConfigured. The values for audioType are defined in ISO-IEC 13818-1.
C
audioTypeControlB" )Determined how audio type is determined.
�
audioWatermarkSettings�B�:�
�
	medialive<ChannelEncoderSettingsAudioDescriptionAudioWatermarkSettings�aws:medialive/ChannelEncoderSettingsAudioDescriptionAudioWatermarkSettings:ChannelEncoderSettingsAudioDescriptionAudioWatermarkSettings�Settings to configure one or more solutions that insert audio watermarks in the audio encode. See Audio Watermark Settings for more details.
�
codecSettings�B�:�
�
	medialive3ChannelEncoderSettingsAudioDescriptionCodecSettingsuaws:medialive/ChannelEncoderSettingsAudioDescriptionCodecSettings:ChannelEncoderSettingsAudioDescriptionCodecSettingsAAudio codec settings. See Audio Codec Settings for more details.

languageCodeB" 
languageCodeControlB" 0
name" $The name of this audio description.
�
remixSettings�B�:�
�
	medialive3ChannelEncoderSettingsAudioDescriptionRemixSettingsuaws:medialive/ChannelEncoderSettingsAudioDescriptionRemixSettings:ChannelEncoderSettingsAudioDescriptionRemixSettingsI

streamNameB" 5Stream name RTMP destinations (URLs of type rtmp://)
:�
�
	medialive@ChannelEncoderSettingsAudioDescriptionAudioNormalizationSettings�aws:medialive/ChannelEncoderSettingsAudioDescriptionAudioNormalizationSettings:ChannelEncoderSettingsAudioDescriptionAudioNormalizationSettings�
��
	algorithmB" �Audio normalization algorithm to use. itu17701 conforms to the CALM Act specification, itu17702 to the EBU R-128 specification.
G
algorithmControlB" -Algorithm control for the audio description.
@

targetLkfsB ,Target LKFS (loudness) to adjust volume to.
:�
�
	medialive<ChannelEncoderSettingsAudioDescriptionAudioWatermarkSettings�aws:medialive/ChannelEncoderSettingsAudioDescriptionAudioWatermarkSettings:ChannelEncoderSettingsAudioDescriptionAudioWatermarkSettings�
��
nielsenWatermarksSettings�B�:�
�
	medialiveUChannelEncoderSettingsAudioDescriptionAudioWatermarkSettingsNielsenWatermarksSettings�aws:medialive/ChannelEncoderSettingsAudioDescriptionAudioWatermarkSettingsNielsenWatermarksSettings:ChannelEncoderSettingsAudioDescriptionAudioWatermarkSettingsNielsenWatermarksSettings:�
�
	medialiveUChannelEncoderSettingsAudioDescriptionAudioWatermarkSettingsNielsenWatermarksSettings�aws:medialive/ChannelEncoderSettingsAudioDescriptionAudioWatermarkSettingsNielsenWatermarksSettings:ChannelEncoderSettingsAudioDescriptionAudioWatermarkSettingsNielsenWatermarksSettings�
��
nielsenCbetSettings�B�:�
�
	medialivehChannelEncoderSettingsAudioDescriptionAudioWatermarkSettingsNielsenWatermarksSettingsNielsenCbetSettings�aws:medialive/ChannelEncoderSettingsAudioDescriptionAudioWatermarkSettingsNielsenWatermarksSettingsNielsenCbetSettings:ChannelEncoderSettingsAudioDescriptionAudioWatermarkSettingsNielsenWatermarksSettingsNielsenCbetSettings\Used to insert watermarks of type Nielsen CBET. See Nielsen CBET Settings for more details.
�
nielsenDistributionTypeB" gDistribution types to assign to the watermarks. Options are `PROGRAM_CONTENT` and `FINAL_DISTRIBUTOR`.
�
nielsenNaesIiNwSettings�B�*�:�
�
	medialivekChannelEncoderSettingsAudioDescriptionAudioWatermarkSettingsNielsenWatermarksSettingsNielsenNaesIiNwSetting�aws:medialive/ChannelEncoderSettingsAudioDescriptionAudioWatermarkSettingsNielsenWatermarksSettingsNielsenNaesIiNwSetting:ChannelEncoderSettingsAudioDescriptionAudioWatermarkSettingsNielsenWatermarksSettingsNielsenNaesIiNwSetting�Used to insert watermarks of type Nielsen NAES, II (N2) and Nielsen NAES VI (NW). See Nielsen NAES II NW Settings for more details.
:�
�
	medialivehChannelEncoderSettingsAudioDescriptionAudioWatermarkSettingsNielsenWatermarksSettingsNielsenCbetSettings�aws:medialive/ChannelEncoderSettingsAudioDescriptionAudioWatermarkSettingsNielsenWatermarksSettingsNielsenCbetSettings:ChannelEncoderSettingsAudioDescriptionAudioWatermarkSettingsNielsenWatermarksSettingsNielsenCbetSettings�
�
cbetCheckDigitString" u
cbetStepaside" `Determines the method of CBET insertion mode when prior encoding is detected on the same layer.
4
csid" (CBET source ID to use in the watermark.
:�
�
	medialivekChannelEncoderSettingsAudioDescriptionAudioWatermarkSettingsNielsenWatermarksSettingsNielsenNaesIiNwSetting�aws:medialive/ChannelEncoderSettingsAudioDescriptionAudioWatermarkSettingsNielsenWatermarksSettingsNielsenNaesIiNwSetting:ChannelEncoderSettingsAudioDescriptionAudioWatermarkSettingsNielsenWatermarksSettingsNielsenNaesIiNwSettingZ
X
checkDigitString" >
sid 3The Nielsen Source ID to include in the watermark.
:�
�
	medialive3ChannelEncoderSettingsAudioDescriptionCodecSettingsuaws:medialive/ChannelEncoderSettingsAudioDescriptionCodecSettings:ChannelEncoderSettingsAudioDescriptionCodecSettings�
��
aacSettings�B�:�
�
	medialive>ChannelEncoderSettingsAudioDescriptionCodecSettingsAacSettings�aws:medialive/ChannelEncoderSettingsAudioDescriptionCodecSettingsAacSettings:ChannelEncoderSettingsAudioDescriptionCodecSettingsAacSettings1Aac Settings. See AAC Settings for more details.
�
ac3Settings�B�:�
�
	medialive>ChannelEncoderSettingsAudioDescriptionCodecSettingsAc3Settings�aws:medialive/ChannelEncoderSettingsAudioDescriptionCodecSettingsAc3Settings:ChannelEncoderSettingsAudioDescriptionCodecSettingsAc3Settings1Ac3 Settings. See AC3 Settings for more details.
�
eac3AtmosSettings�B�:�
�
	medialiveDChannelEncoderSettingsAudioDescriptionCodecSettingsEac3AtmosSettings�aws:medialive/ChannelEncoderSettingsAudioDescriptionCodecSettingsEac3AtmosSettings:ChannelEncoderSettingsAudioDescriptionCodecSettingsEac3AtmosSettings-Eac3 Atmos Settings. See EAC3 Atmos Settings
�
eac3Settings�B�:�
�
	medialive?ChannelEncoderSettingsAudioDescriptionCodecSettingsEac3Settings�aws:medialive/ChannelEncoderSettingsAudioDescriptionCodecSettingsEac3Settings:ChannelEncoderSettingsAudioDescriptionCodecSettingsEac3Settings!Eac3 Settings. See EAC3 Settings
�
mp2Settings�B�:�
�
	medialive>ChannelEncoderSettingsAudioDescriptionCodecSettingsMp2Settings�aws:medialive/ChannelEncoderSettingsAudioDescriptionCodecSettingsMp2Settings:ChannelEncoderSettingsAudioDescriptionCodecSettingsMp2Settings�
passThroughSettings�B�:�
�
	medialiveFChannelEncoderSettingsAudioDescriptionCodecSettingsPassThroughSettings�aws:medialive/ChannelEncoderSettingsAudioDescriptionCodecSettingsPassThroughSettings:ChannelEncoderSettingsAudioDescriptionCodecSettingsPassThroughSettings�
wavSettings�B�:�
�
	medialive>ChannelEncoderSettingsAudioDescriptionCodecSettingsWavSettings�aws:medialive/ChannelEncoderSettingsAudioDescriptionCodecSettingsWavSettings:ChannelEncoderSettingsAudioDescriptionCodecSettingsWavSettings:�
�
	medialive>ChannelEncoderSettingsAudioDescriptionCodecSettingsAacSettings�aws:medialive/ChannelEncoderSettingsAudioDescriptionCodecSettingsAacSettings:ChannelEncoderSettingsAudioDescriptionCodecSettingsAacSettings�
�1
bitrateB  Average bitrate in bits/second.
9

codingModeB" %Mono, Stereo, or 5.1 channel layout.
{
	inputTypeB" hSet to "broadcasterMixedAd" when input contains pre-mixed main audio + AD (narration) as a stereo pair.

profileB" AAC profile.
0
rateControlModeB" The rate control mode.
A
	rawFormatB" .Sets LATM/LOAS AAC output for raw containers.
'

sampleRateB Sample rate in Hz.
n
specB" `Use MPEG-2 AAC audio instead of MPEG-4 AAC audio for raw or MPEG-2 Transport Stream containers.
M

vbrQualityB" 9VBR Quality Level - Only used if rateControlMode is VBR.
:�
�
	medialive>ChannelEncoderSettingsAudioDescriptionCodecSettingsAc3Settings�aws:medialive/ChannelEncoderSettingsAudioDescriptionCodecSettingsAc3Settings:ChannelEncoderSettingsAudioDescriptionCodecSettingsAc3Settings�
�1
bitrateB  Average bitrate in bits/second.
Y
bitstreamModeB" BSpecifies the bitstream mode (bsmod) for the emitted AC-3 stream.
/

codingModeB" Dolby Digital coding mode.
3
dialnormB !Sets the dialnorm of the output.
�

drcProfileB" �If set to filmStandard, adds dynamic range compression signaling to the output bitstream as defined in the Dolby Digital specification.
m
	lfeFilterB" ZWhen set to enabled, applies a 120Hz lowpass filter to the LFE channel prior to encoding.
+
metadataControlB" Metadata control.
:�
�
	medialiveDChannelEncoderSettingsAudioDescriptionCodecSettingsEac3AtmosSettings�aws:medialive/ChannelEncoderSettingsAudioDescriptionCodecSettingsEac3AtmosSettings:ChannelEncoderSettingsAudioDescriptionCodecSettingsEac3AtmosSettings�
�1
bitrateB  Average bitrate in bits/second.
E

codingModeB" 1Dolby Digital Plus with Dolby Atmos coding mode.
4
dialnormB "Sets the dialnorm for the output.
C
drcLineB" 2Sets the Dolby dynamic range compression profile.
K
drcRfB" <Sets the profile for heavy Dolby dynamic range compression.
-

heightTrimB Height dimensional trim.
1
surroundTrimB Surround dimensional trim.
:�
�
	medialive?ChannelEncoderSettingsAudioDescriptionCodecSettingsEac3Settings�aws:medialive/ChannelEncoderSettingsAudioDescriptionCodecSettingsEac3Settings:ChannelEncoderSettingsAudioDescriptionCodecSettingsEac3Settings�
�:
attenuationControlB" Sets the attenuation control.
1
bitrateB  Average bitrate in bits/second.
Y
bitstreamModeB" BSpecifies the bitstream mode (bsmod) for the emitted AC-3 stream.
4

codingModeB"  Dolby Digital Plus coding mode.

dcFilterB" 
dialnormB C
drcLineB" 2Sets the Dolby dynamic range compression profile.
K
drcRfB" <Sets the profile for heavy Dolby dynamic range compression.


lfeControlB" m
	lfeFilterB" ZWhen set to enabled, applies a 120Hz lowpass filter to the LFE channel prior to encoding.

loRoCenterMixLevelB 
loRoSurroundMixLevelB 
ltRtCenterMixLevelB 
ltRtSurroundMixLevelB +
metadataControlB" Metadata control.

passthroughControlB" 
phaseControlB" 
stereoDownmixB" 
surroundExModeB" 
surroundModeB" :�
�
	medialive>ChannelEncoderSettingsAudioDescriptionCodecSettingsMp2Settings�aws:medialive/ChannelEncoderSettingsAudioDescriptionCodecSettingsMp2Settings:ChannelEncoderSettingsAudioDescriptionCodecSettingsMp2SettingsP
N
bitrateB 

codingModeB" '

sampleRateB Sample rate in Hz.
:�
�
	medialiveFChannelEncoderSettingsAudioDescriptionCodecSettingsPassThroughSettings�aws:medialive/ChannelEncoderSettingsAudioDescriptionCodecSettingsPassThroughSettings:ChannelEncoderSettingsAudioDescriptionCodecSettingsPassThroughSettings
 :�
�
	medialive>ChannelEncoderSettingsAudioDescriptionCodecSettingsWavSettings�aws:medialive/ChannelEncoderSettingsAudioDescriptionCodecSettingsWavSettings:ChannelEncoderSettingsAudioDescriptionCodecSettingsWavSettingsQ
O
bitDepthB 

codingModeB" '

sampleRateB Sample rate in Hz.
:�
�
	medialive3ChannelEncoderSettingsAudioDescriptionRemixSettingsuaws:medialive/ChannelEncoderSettingsAudioDescriptionRemixSettings:ChannelEncoderSettingsAudioDescriptionRemixSettings�
��
channelMappings�*�:�
�
	medialiveAChannelEncoderSettingsAudioDescriptionRemixSettingsChannelMapping�aws:medialive/ChannelEncoderSettingsAudioDescriptionRemixSettingsChannelMapping:ChannelEncoderSettingsAudioDescriptionRemixSettingsChannelMapping

channelsInB 
channelsOutB :�
�
	medialiveAChannelEncoderSettingsAudioDescriptionRemixSettingsChannelMapping�aws:medialive/ChannelEncoderSettingsAudioDescriptionRemixSettingsChannelMapping:ChannelEncoderSettingsAudioDescriptionRemixSettingsChannelMapping�
��
inputChannelLevels�*�:�
�
	medialiveRChannelEncoderSettingsAudioDescriptionRemixSettingsChannelMappingInputChannelLevel�aws:medialive/ChannelEncoderSettingsAudioDescriptionRemixSettingsChannelMappingInputChannelLevel:ChannelEncoderSettingsAudioDescriptionRemixSettingsChannelMappingInputChannelLevel
outputChannel :�
�
	medialiveRChannelEncoderSettingsAudioDescriptionRemixSettingsChannelMappingInputChannelLevel�aws:medialive/ChannelEncoderSettingsAudioDescriptionRemixSettingsChannelMappingInputChannelLevel:ChannelEncoderSettingsAudioDescriptionRemixSettingsChannelMappingInputChannelLevel"
 

gain 
inputChannel :�
�
	medialive#ChannelEncoderSettingsAvailBlankingUaws:medialive/ChannelEncoderSettingsAvailBlanking:ChannelEncoderSettingsAvailBlanking�
��
availBlankingImage�B�:�
�
	medialive5ChannelEncoderSettingsAvailBlankingAvailBlankingImageyaws:medialive/ChannelEncoderSettingsAvailBlankingAvailBlankingImage:ChannelEncoderSettingsAvailBlankingAvailBlankingImageFBlanking image to be used. See Avail Blanking Image for more details.
u
stateB" fWhen set to enabled, causes video, audio and captions to be blanked when insertion metadata is added.
:�
�
	medialive5ChannelEncoderSettingsAvailBlankingAvailBlankingImageyaws:medialive/ChannelEncoderSettingsAvailBlankingAvailBlankingImage:ChannelEncoderSettingsAvailBlankingAvailBlankingImage�
�R
passwordParamB" ;Key used to extract the password from EC2 Parameter store.
9
uri" .Path to a file accessible to the live stream.
)
usernameB" . Username to be used.
:�	
�
	medialive(ChannelEncoderSettingsCaptionDescription_aws:medialive/ChannelEncoderSettingsCaptionDescription:ChannelEncoderSettingsCaptionDescription�
��
accessibilityB" �Indicates whether the caption track implements accessibility features such as written descriptions of spoken dialog, music, and sounds.
�
captionSelectorName" �Specifies which input caption selector to use as a caption source when generating output captions. This field should match a captionSelector name.
�
destinationSettings�B�:�
�
	medialive;ChannelEncoderSettingsCaptionDescriptionDestinationSettings�aws:medialive/ChannelEncoderSettingsCaptionDescriptionDestinationSettings:ChannelEncoderSettingsCaptionDescriptionDestinationSettings}Additional settings for captions destination that depend on the destination type. See Destination Settings for more details.
2
languageCodeB" ISO 639-2 three-digit code.
~
languageDescriptionB" aHuman readable information to indicate captions available for players (eg. English, or Spanish).
�
name" Name of the caption description. Used to associate a caption description with an output. Names must be unique within an event.
:�)
�
	medialive;ChannelEncoderSettingsCaptionDescriptionDestinationSettings�aws:medialive/ChannelEncoderSettingsCaptionDescriptionDestinationSettings:ChannelEncoderSettingsCaptionDescriptionDestinationSettings�'
�'�
aribDestinationSettings�B�:�
�
	medialiveRChannelEncoderSettingsCaptionDescriptionDestinationSettingsAribDestinationSettings�aws:medialive/ChannelEncoderSettingsCaptionDescriptionDestinationSettingsAribDestinationSettings:ChannelEncoderSettingsCaptionDescriptionDestinationSettingsAribDestinationSettingsARIB Destination Settings.
�
burnInDestinationSettings�B�:�
�
	medialiveTChannelEncoderSettingsCaptionDescriptionDestinationSettingsBurnInDestinationSettings�aws:medialive/ChannelEncoderSettingsCaptionDescriptionDestinationSettingsBurnInDestinationSettings:ChannelEncoderSettingsCaptionDescriptionDestinationSettingsBurnInDestinationSettingsQBurn In Destination Settings. See Burn In Destination Settings for more details.
�
dvbSubDestinationSettings�B�:�
�
	medialiveTChannelEncoderSettingsCaptionDescriptionDestinationSettingsDvbSubDestinationSettings�aws:medialive/ChannelEncoderSettingsCaptionDescriptionDestinationSettingsDvbSubDestinationSettings:ChannelEncoderSettingsCaptionDescriptionDestinationSettingsDvbSubDestinationSettingsQDVB Sub Destination Settings. See DVB Sub Destination Settings for more details.
�
ebuTtDDestinationSettings�B�:�
�
	medialiveTChannelEncoderSettingsCaptionDescriptionDestinationSettingsEbuTtDDestinationSettings�aws:medialive/ChannelEncoderSettingsCaptionDescriptionDestinationSettingsEbuTtDDestinationSettings:ChannelEncoderSettingsCaptionDescriptionDestinationSettingsEbuTtDDestinationSettingsSEBU TT D Destination Settings. See EBU TT D Destination Settings for more details.
�
embeddedDestinationSettings�B�:�
�
	medialiveVChannelEncoderSettingsCaptionDescriptionDestinationSettingsEmbeddedDestinationSettings�aws:medialive/ChannelEncoderSettingsCaptionDescriptionDestinationSettingsEmbeddedDestinationSettings:ChannelEncoderSettingsCaptionDescriptionDestinationSettingsEmbeddedDestinationSettingsEmbedded Destination Settings.
�
%embeddedPlusScte20DestinationSettings�B�:�
�
	medialive`ChannelEncoderSettingsCaptionDescriptionDestinationSettingsEmbeddedPlusScte20DestinationSettings�aws:medialive/ChannelEncoderSettingsCaptionDescriptionDestinationSettingsEmbeddedPlusScte20DestinationSettings:ChannelEncoderSettingsCaptionDescriptionDestinationSettingsEmbeddedPlusScte20DestinationSettings+Embedded Plus SCTE20 Destination Settings.
�
"rtmpCaptionInfoDestinationSettings�B�:�
�
	medialive]ChannelEncoderSettingsCaptionDescriptionDestinationSettingsRtmpCaptionInfoDestinationSettings�aws:medialive/ChannelEncoderSettingsCaptionDescriptionDestinationSettingsRtmpCaptionInfoDestinationSettings:ChannelEncoderSettingsCaptionDescriptionDestinationSettingsRtmpCaptionInfoDestinationSettings(RTMP Caption Info Destination Settings.
�
%scte20PlusEmbeddedDestinationSettings�B�:�
�
	medialive`ChannelEncoderSettingsCaptionDescriptionDestinationSettingsScte20PlusEmbeddedDestinationSettings�aws:medialive/ChannelEncoderSettingsCaptionDescriptionDestinationSettingsScte20PlusEmbeddedDestinationSettings:ChannelEncoderSettingsCaptionDescriptionDestinationSettingsScte20PlusEmbeddedDestinationSettings+SCTE20 Plus Embedded Destination Settings.
�
scte27DestinationSettings�B�:�
�
	medialiveTChannelEncoderSettingsCaptionDescriptionDestinationSettingsScte27DestinationSettings�aws:medialive/ChannelEncoderSettingsCaptionDescriptionDestinationSettingsScte27DestinationSettings:ChannelEncoderSettingsCaptionDescriptionDestinationSettingsScte27DestinationSettingsSCTE27 Destination Settings.
�
smpteTtDestinationSettings�B�:�
�
	medialiveUChannelEncoderSettingsCaptionDescriptionDestinationSettingsSmpteTtDestinationSettings�aws:medialive/ChannelEncoderSettingsCaptionDescriptionDestinationSettingsSmpteTtDestinationSettings:ChannelEncoderSettingsCaptionDescriptionDestinationSettingsSmpteTtDestinationSettingsSMPTE TT Destination Settings.
�
teletextDestinationSettings�B�:�
�
	medialiveVChannelEncoderSettingsCaptionDescriptionDestinationSettingsTeletextDestinationSettings�aws:medialive/ChannelEncoderSettingsCaptionDescriptionDestinationSettingsTeletextDestinationSettings:ChannelEncoderSettingsCaptionDescriptionDestinationSettingsTeletextDestinationSettingsTeletext Destination Settings.
�
ttmlDestinationSettings�B�:�
�
	medialiveRChannelEncoderSettingsCaptionDescriptionDestinationSettingsTtmlDestinationSettings�aws:medialive/ChannelEncoderSettingsCaptionDescriptionDestinationSettingsTtmlDestinationSettings:ChannelEncoderSettingsCaptionDescriptionDestinationSettingsTtmlDestinationSettingsKTTML Destination Settings. See TTML Destination Settings for more details.
�
webvttDestinationSettings�B�:�
�
	medialiveTChannelEncoderSettingsCaptionDescriptionDestinationSettingsWebvttDestinationSettings�aws:medialive/ChannelEncoderSettingsCaptionDescriptionDestinationSettingsWebvttDestinationSettings:ChannelEncoderSettingsCaptionDescriptionDestinationSettingsWebvttDestinationSettingsOWebVTT Destination Settings. See WebVTT Destination Settings for more details.
:�
�
	medialiveRChannelEncoderSettingsCaptionDescriptionDestinationSettingsAribDestinationSettings�aws:medialive/ChannelEncoderSettingsCaptionDescriptionDestinationSettingsAribDestinationSettings:ChannelEncoderSettingsCaptionDescriptionDestinationSettingsAribDestinationSettings
 :�'
�
	medialiveTChannelEncoderSettingsCaptionDescriptionDestinationSettingsBurnInDestinationSettings�aws:medialive/ChannelEncoderSettingsCaptionDescriptionDestinationSettingsBurnInDestinationSettings:ChannelEncoderSettingsCaptionDescriptionDestinationSettingsBurnInDestinationSettings�%
�%�
	alignmentB" �If no explicit xPosition or yPosition is provided, setting alignment to centered will place the captions at the bottom center of the output. Similarly, setting a left alignment will align captions to the bottom left of the output. If x and y positions are given in conjunction with the alignment parameter, the font will be justified (either left or centered) relative to those coordinates. Selecting “smart” justification will left-justify live subtitles and center-justify pre-recorded subtitles. All burn-in and DVB-Sub font settings must match.
�
backgroundColorB" lSpecifies the color of the rectangle behind the captions. All burn-in and DVB-Sub font settings must match.
�
backgroundOpacityB �Specifies the opacity of the background rectangle. 255 is opaque; 0 is transparent. Leaving this parameter out is equivalent to setting it to 0 (transparent). All burn-in and DVB-Sub font settings must match.
�
font�B�:�
�
	medialiveXChannelEncoderSettingsCaptionDescriptionDestinationSettingsBurnInDestinationSettingsFont�aws:medialive/ChannelEncoderSettingsCaptionDescriptionDestinationSettingsBurnInDestinationSettingsFont:ChannelEncoderSettingsCaptionDescriptionDestinationSettingsBurnInDestinationSettingsFont�External font file used for caption burn-in. File extension must be ‘ttf’ or ‘tte’. Although the user can select output fonts for many different types of input captions, embedded, STL and teletext sources use a strict grid system. Using external fonts with these caption sources could cause unexpected display of proportional fonts. All burn-in and DVB-Sub font settings must match. See Font for more details.
�
	fontColorB" �Specifies the color of the burned-in captions. This option is not valid for source captions that are STL, 608/embedded or teletext. These source settings are already pre-defined by the caption stream. All burn-in and DVB-Sub font settings must match.
�
fontOpacityB �Specifies the opacity of the burned-in captions. 255 is opaque; 0 is transparent. All burn-in and DVB-Sub font settings must match.
�
fontResolutionB mFont resolution in DPI (dots per inch); default is 96 dpi. All burn-in and DVB-Sub font settings must match.
�
fontSizeB" �When set to ‘auto’ fontSize will scale depending on the size of the output. Giving a positive integer will specify the exact font size in points. All burn-in and DVB-Sub font settings must match.
�
outlineColor" �Specifies font outline color. This option is not valid for source captions that are either 608/embedded or teletext. These source settings are already pre-defined by the caption stream. All burn-in and DVB-Sub font settings must match.
�
outlineSizeB �Specifies font outline size in pixels. This option is not valid for source captions that are either 608/embedded or teletext. These source settings are already pre-defined by the caption stream. All burn-in and DVB-Sub font settings must match.

shadowColorB" jSpecifies the color of the shadow cast by the captions. All burn-in and DVB-Sub font settings must match.
�
shadowOpacityB �Specifies the opacity of the shadow. 255 is opaque; 0 is transparent. Leaving this parameter out is equivalent to setting it to 0 (transparent). All burn-in and DVB-Sub font settings must match.
�
shadowXOffsetB �Specifies the horizontal offset of the shadow relative to the captions in pixels. A value of -2 would result in a shadow offset 2 pixels to the left. All burn-in and DVB-Sub font settings must match.
�
shadowYOffsetB �Specifies the vertical offset of the shadow relative to the captions in pixels. A value of -2 would result in a shadow offset 2 pixels above the text. All burn-in and DVB-Sub font settings must match.
�
teletextGridControl" �Controls whether a fixed grid size will be used to generate the output subtitles bitmap. Only applicable for Teletext inputs and DVB-Sub/Burn-in outputs.
�
	xPositionB �Specifies the horizontal position of the caption relative to the left side of the output in pixels. A value of 10 would result in the captions starting 10 pixels from the left of the output. If no explicit xPosition is provided, the horizontal caption position will be determined by the alignment parameter. All burn-in and DVB-Sub font settings must match.
�
	yPositionB �Specifies the vertical position of the caption relative to the top of the output in pixels. A value of 10 would result in the captions starting 10 pixels from the top of the output. If no explicit yPosition is provided, the caption will be positioned towards the bottom of the output. All burn-in and DVB-Sub font settings must match.
:�
�
	medialiveXChannelEncoderSettingsCaptionDescriptionDestinationSettingsBurnInDestinationSettingsFont�aws:medialive/ChannelEncoderSettingsCaptionDescriptionDestinationSettingsBurnInDestinationSettingsFont:ChannelEncoderSettingsCaptionDescriptionDestinationSettingsBurnInDestinationSettingsFont�
�R
passwordParamB" ;Key used to extract the password from EC2 Parameter store.
9
uri" .Path to a file accessible to the live stream.
'
usernameB" Username to be used.
:�*
�
	medialiveTChannelEncoderSettingsCaptionDescriptionDestinationSettingsDvbSubDestinationSettings�aws:medialive/ChannelEncoderSettingsCaptionDescriptionDestinationSettingsDvbSubDestinationSettings:ChannelEncoderSettingsCaptionDescriptionDestinationSettingsDvbSubDestinationSettings�(
�(�
	alignmentB" �If no explicit xPosition or yPosition is provided, setting alignment to centered will place the captions at the bottom center of the output. Similarly, setting a left alignment will align captions to the bottom left of the output. If x and y positions are given in conjunction with the alignment parameter, the font will be justified (either left or centered) relative to those coordinates. Selecting “smart” justification will left-justify live subtitles and center-justify pre-recorded subtitles. This option is not valid for source captions that are STL or 608/embedded. These source settings are already pre-defined by the caption stream. All burn-in and DVB-Sub font settings must match.
�
backgroundColorB" lSpecifies the color of the rectangle behind the captions. All burn-in and DVB-Sub font settings must match.
�
backgroundOpacityB �Specifies the opacity of the background rectangle. 255 is opaque; 0 is transparent. Leaving this parameter blank is equivalent to setting it to 0 (transparent). All burn-in and DVB-Sub font settings must match.
�
font�B�:�
�
	medialiveXChannelEncoderSettingsCaptionDescriptionDestinationSettingsDvbSubDestinationSettingsFont�aws:medialive/ChannelEncoderSettingsCaptionDescriptionDestinationSettingsDvbSubDestinationSettingsFont:ChannelEncoderSettingsCaptionDescriptionDestinationSettingsDvbSubDestinationSettingsFont�External font file used for caption burn-in. File extension must be ‘ttf’ or ‘tte’. Although the user can select output fonts for many different types of input captions, embedded, STL and teletext sources use a strict grid system. Using external fonts with these caption sources could cause unexpected display of proportional fonts. All burn-in and DVB-Sub font settings must match. See Font for more details.
�
	fontColorB" �Specifies the color of the burned-in captions. This option is not valid for source captions that are STL, 608/embedded or teletext. These source settings are already pre-defined by the caption stream. All burn-in and DVB-Sub font settings must match.
�
fontOpacityB �Specifies the opacity of the burned-in captions. 255 is opaque; 0 is transparent. All burn-in and DVB-Sub font settings must match.
�
fontResolutionB mFont resolution in DPI (dots per inch); default is 96 dpi. All burn-in and DVB-Sub font settings must match.
�
fontSizeB" �When set to auto fontSize will scale depending on the size of the output. Giving a positive integer will specify the exact font size in points. All burn-in and DVB-Sub font settings must match.
�
outlineColorB" �Specifies font outline color. This option is not valid for source captions that are either 608/embedded or teletext. These source settings are already pre-defined by the caption stream. All burn-in and DVB-Sub font settings must match.
�
outlineSizeB �Specifies font outline size in pixels. This option is not valid for source captions that are either 608/embedded or teletext. These source settings are already pre-defined by the caption stream. All burn-in and DVB-Sub font settings must match.

shadowColorB" jSpecifies the color of the shadow cast by the captions. All burn-in and DVB-Sub font settings must match.
�
shadowOpacityB �Specifies the opacity of the shadow. 255 is opaque; 0 is transparent. Leaving this parameter blank is equivalent to setting it to 0 (transparent). All burn-in and DVB-Sub font settings must match.
�
shadowXOffsetB �Specifies the horizontal offset of the shadow relative to the captions in pixels. A value of -2 would result in a shadow offset 2 pixels to the left. All burn-in and DVB-Sub font settings must match.
�
shadowYOffsetB �Specifies the vertical offset of the shadow relative to the captions in pixels. A value of -2 would result in a shadow offset 2 pixels above the text. All burn-in and DVB-Sub font settings must match.
�
teletextGridControlB" �Controls whether a fixed grid size will be used to generate the output subtitles bitmap. Only applicable for Teletext inputs and DVB-Sub/Burn-in outputs.
�
	xPositionB �Specifies the horizontal position of the caption relative to the left side of the output in pixels. A value of 10 would result in the captions starting 10 pixels from the left of the output. If no explicit xPosition is provided, the horizontal caption position will be determined by the alignment parameter. This option is not valid for source captions that are STL, 608/embedded or teletext. These source settings are already pre-defined by the caption stream. All burn-in and DVB-Sub font settings must match.
�
	yPositionB �Specifies the vertical position of the caption relative to the top of the output in pixels. A value of 10 would result in the captions starting 10 pixels from the top of the output. If no explicit yPosition is provided, the caption will be positioned towards the bottom of the output. This option is not valid for source captions that are STL, 608/embedded or teletext. These source settings are already pre-defined by the caption stream. All burn-in and DVB-Sub font settings must match.
:�
�
	medialiveXChannelEncoderSettingsCaptionDescriptionDestinationSettingsDvbSubDestinationSettingsFont�aws:medialive/ChannelEncoderSettingsCaptionDescriptionDestinationSettingsDvbSubDestinationSettingsFont:ChannelEncoderSettingsCaptionDescriptionDestinationSettingsDvbSubDestinationSettingsFont�
�R
passwordParamB" ;Key used to extract the password from EC2 Parameter store.
9
uri" .Path to a file accessible to the live stream.
'
usernameB" Username to be used.
:�
�
	medialiveTChannelEncoderSettingsCaptionDescriptionDestinationSettingsEbuTtDDestinationSettings�aws:medialive/ChannelEncoderSettingsCaptionDescriptionDestinationSettingsEbuTtDDestinationSettings:ChannelEncoderSettingsCaptionDescriptionDestinationSettingsEbuTtDDestinationSettings�
��
copyrightHolderB" {Complete this field if you want to include the name of the copyright holder in the copyright tag in the captions metadata.
�
fillLineGapB" �Specifies how to handle the gap between the lines (in multi-line captions). - enabled: Fill with the captions background color (as specified in the input captions). - disabled: Leave the gap unfilled.
�

fontFamilyB" �Specifies the font family to include in the font data attached to the EBU-TT captions. Valid only if styleControl is set to include. If you leave this field empty, the font family is set to “monospaced”. (If styleControl is set to exclude, the font family is always set to “monospaced”.) You specify only the font family. All other style information (color, bold, position and so on) is copied from the input captions. The size is always set to 100% to allow the downstream player to choose the size. - Enter a list of font families, as a comma-separated list of font names, in order of preference. The name can be a font family (such as “Arial”), or a generic font family (such as “serif”), or “default” (to let the downstream player choose the font). - Leave blank to set the family to “monospace”.
�
styleControlB" �Specifies the style information (font color, font position, and so on) to include in the font data that is attached to the EBU-TT captions. - include: Take the style information (font color, font position, and so on) from the source captions and include that information in the font data attached to the EBU-TT captions. This option is valid only if the source captions are Embedded or Teletext. - exclude: In the font data attached to the EBU-TT captions, set the font family to “monospaced”. Do not include any other style information.
:�
�
	medialiveVChannelEncoderSettingsCaptionDescriptionDestinationSettingsEmbeddedDestinationSettings�aws:medialive/ChannelEncoderSettingsCaptionDescriptionDestinationSettingsEmbeddedDestinationSettings:ChannelEncoderSettingsCaptionDescriptionDestinationSettingsEmbeddedDestinationSettings
 :�
�
	medialive`ChannelEncoderSettingsCaptionDescriptionDestinationSettingsEmbeddedPlusScte20DestinationSettings�aws:medialive/ChannelEncoderSettingsCaptionDescriptionDestinationSettingsEmbeddedPlusScte20DestinationSettings:ChannelEncoderSettingsCaptionDescriptionDestinationSettingsEmbeddedPlusScte20DestinationSettings
 :�
�
	medialive]ChannelEncoderSettingsCaptionDescriptionDestinationSettingsRtmpCaptionInfoDestinationSettings�aws:medialive/ChannelEncoderSettingsCaptionDescriptionDestinationSettingsRtmpCaptionInfoDestinationSettings:ChannelEncoderSettingsCaptionDescriptionDestinationSettingsRtmpCaptionInfoDestinationSettings
 :�
�
	medialive`ChannelEncoderSettingsCaptionDescriptionDestinationSettingsScte20PlusEmbeddedDestinationSettings�aws:medialive/ChannelEncoderSettingsCaptionDescriptionDestinationSettingsScte20PlusEmbeddedDestinationSettings:ChannelEncoderSettingsCaptionDescriptionDestinationSettingsScte20PlusEmbeddedDestinationSettings
 :�
�
	medialiveTChannelEncoderSettingsCaptionDescriptionDestinationSettingsScte27DestinationSettings�aws:medialive/ChannelEncoderSettingsCaptionDescriptionDestinationSettingsScte27DestinationSettings:ChannelEncoderSettingsCaptionDescriptionDestinationSettingsScte27DestinationSettings
 :�
�
	medialiveUChannelEncoderSettingsCaptionDescriptionDestinationSettingsSmpteTtDestinationSettings�aws:medialive/ChannelEncoderSettingsCaptionDescriptionDestinationSettingsSmpteTtDestinationSettings:ChannelEncoderSettingsCaptionDescriptionDestinationSettingsSmpteTtDestinationSettings
 :�
�
	medialiveVChannelEncoderSettingsCaptionDescriptionDestinationSettingsTeletextDestinationSettings�aws:medialive/ChannelEncoderSettingsCaptionDescriptionDestinationSettingsTeletextDestinationSettings:ChannelEncoderSettingsCaptionDescriptionDestinationSettingsTeletextDestinationSettings
 :�
�
	medialiveRChannelEncoderSettingsCaptionDescriptionDestinationSettingsTtmlDestinationSettings�aws:medialive/ChannelEncoderSettingsCaptionDescriptionDestinationSettingsTtmlDestinationSettings:ChannelEncoderSettingsCaptionDescriptionDestinationSettingsTtmlDestinationSettings
}{
styleControl" gThis field is not currently supported and will not affect the output styling. Leave the default value.
:�
�
	medialiveTChannelEncoderSettingsCaptionDescriptionDestinationSettingsWebvttDestinationSettings�aws:medialive/ChannelEncoderSettingsCaptionDescriptionDestinationSettingsWebvttDestinationSettings:ChannelEncoderSettingsCaptionDescriptionDestinationSettingsWebvttDestinationSettings�
��
styleControl" �Controls whether the color and position of the source captions is passed through to the WebVTT output captions. PASSTHROUGH - Valid only if the source captions are EMBEDDED or TELETEXT. NO\_STYLE\_DATA - Don’t pass through the style. The output captions will not contain any font styling information.
:�
�
	medialive)ChannelEncoderSettingsGlobalConfigurationaaws:medialive/ChannelEncoderSettingsGlobalConfiguration:ChannelEncoderSettingsGlobalConfiguration�
�R
initialAudioGainB 8Value to set the initial audio gain for the Live Event.
�
inputEndActionB" �Indicates the action to take when the current input completes (e.g. end-of-file). When switchAndLoopInputs is configured the encoder will restart at the beginning of the first input. When “none” is configured the encoder will transcode either black, a solid color, or a user specified slate images per the “Input Loss Behavior” configuration until the next input switch occurs (which is controlled through the Channel Schedule API).
�
inputLossBehavior�B�:�
�
	medialive:ChannelEncoderSettingsGlobalConfigurationInputLossBehavior�aws:medialive/ChannelEncoderSettingsGlobalConfigurationInputLossBehavior:ChannelEncoderSettingsGlobalConfigurationInputLossBehaviorZSettings for system actions when input is lost. See Input Loss Behavior for more details.
�
outputLockingModeB" �Indicates how MediaLive pipelines are synchronized. PIPELINE\_LOCKING - MediaLive will attempt to synchronize the output of each pipeline to the other. EPOCH\_LOCKING - MediaLive will attempt to synchronize the output of each pipeline to the Unix epoch.
�
outputTimingSourceB" �Indicates whether the rate of frames emitted by the Live encoder should be paced by its system clock (which optionally may be locked to another source via NTP) or should be locked to the clock of the source that is providing the input stream.
�
supportLowFramerateInputsB" �Adjusts video input buffer for streams with very low video framerates. This is commonly set to enabled for music channels with less than one video frame per second.
:�
�
	medialive:ChannelEncoderSettingsGlobalConfigurationInputLossBehavior�aws:medialive/ChannelEncoderSettingsGlobalConfigurationInputLossBehavior:ChannelEncoderSettingsGlobalConfigurationInputLossBehavior�
�
blackFrameMsecB 
inputLossImageColorB" �
inputLossImageSlate�B�:�
�
	medialiveMChannelEncoderSettingsGlobalConfigurationInputLossBehaviorInputLossImageSlate�aws:medialive/ChannelEncoderSettingsGlobalConfigurationInputLossBehaviorInputLossImageSlate:ChannelEncoderSettingsGlobalConfigurationInputLossBehaviorInputLossImageSlate
inputLossImageTypeB" 
repeatFrameMsecB :�
�
	medialiveMChannelEncoderSettingsGlobalConfigurationInputLossBehaviorInputLossImageSlate�aws:medialive/ChannelEncoderSettingsGlobalConfigurationInputLossBehaviorInputLossImageSlate:ChannelEncoderSettingsGlobalConfigurationInputLossBehaviorInputLossImageSlate6
4
passwordParamB" 	
uri" 
usernameB" :�
�
	medialive1ChannelEncoderSettingsMotionGraphicsConfigurationqaws:medialive/ChannelEncoderSettingsMotionGraphicsConfiguration:ChannelEncoderSettingsMotionGraphicsConfiguration�
�<
motionGraphicsInsertionB" Motion Graphics Insertion.
�
motionGraphicsSettings�:�
�
	medialiveGChannelEncoderSettingsMotionGraphicsConfigurationMotionGraphicsSettings�aws:medialive/ChannelEncoderSettingsMotionGraphicsConfigurationMotionGraphicsSettings:ChannelEncoderSettingsMotionGraphicsConfigurationMotionGraphicsSettingsIMotion Graphics Settings. See Motion Graphics Settings for more details.
:�
�
	medialiveGChannelEncoderSettingsMotionGraphicsConfigurationMotionGraphicsSettings�aws:medialive/ChannelEncoderSettingsMotionGraphicsConfigurationMotionGraphicsSettings:ChannelEncoderSettingsMotionGraphicsConfigurationMotionGraphicsSettings�
��
htmlMotionGraphicsSettings�B�:�
�
	medialiveaChannelEncoderSettingsMotionGraphicsConfigurationMotionGraphicsSettingsHtmlMotionGraphicsSettings�aws:medialive/ChannelEncoderSettingsMotionGraphicsConfigurationMotionGraphicsSettingsHtmlMotionGraphicsSettings:ChannelEncoderSettingsMotionGraphicsConfigurationMotionGraphicsSettingsHtmlMotionGraphicsSettingsHtml Motion Graphics Settings.
:�
�
	medialiveaChannelEncoderSettingsMotionGraphicsConfigurationMotionGraphicsSettingsHtmlMotionGraphicsSettings�aws:medialive/ChannelEncoderSettingsMotionGraphicsConfigurationMotionGraphicsSettingsHtmlMotionGraphicsSettings:ChannelEncoderSettingsMotionGraphicsConfigurationMotionGraphicsSettingsHtmlMotionGraphicsSettings
 :�
�
	medialive*ChannelEncoderSettingsNielsenConfigurationcaws:medialive/ChannelEncoderSettingsNielsenConfiguration:ChannelEncoderSettingsNielsenConfiguration�
�Z
distributorIdB" CEnter the Distributor ID assigned to your organization by Nielsen.
D
nielsenPcmToId3TaggingB" $Enables Nielsen PCM to ID3 tagging.
:�
�
	medialive!ChannelEncoderSettingsOutputGroupQaws:medialive/ChannelEncoderSettingsOutputGroup:ChannelEncoderSettingsOutputGroup�
�<
nameB" .Custom output group name defined by the user.
�
outputGroupSettings�:�
�
	medialive4ChannelEncoderSettingsOutputGroupOutputGroupSettingswaws:medialive/ChannelEncoderSettingsOutputGroupOutputGroupSettings:ChannelEncoderSettingsOutputGroupOutputGroupSettingsWSettings associated with the output group. See Output Group Settings for more details.
�
outputs�*�:�
�
	medialive'ChannelEncoderSettingsOutputGroupOutput]aws:medialive/ChannelEncoderSettingsOutputGroupOutput:ChannelEncoderSettingsOutputGroupOutput/List of outputs. See Outputs for more details.
:�
�
	medialive'ChannelEncoderSettingsOutputGroupOutput]aws:medialive/ChannelEncoderSettingsOutputGroupOutput:ChannelEncoderSettingsOutputGroupOutput�
�k
audioDescriptionNamesB*" JThe names of the audio descriptions used as audio sources for the output.
q
captionDescriptionNamesB*" NThe names of the caption descriptions used as caption sources for the output.
9

outputNameB" %The name used to identify an output.
�
outputSettings�:�
�
	medialive5ChannelEncoderSettingsOutputGroupOutputOutputSettingsyaws:medialive/ChannelEncoderSettingsOutputGroupOutputOutputSettings:ChannelEncoderSettingsOutputGroupOutputOutputSettings;Settings for output. See Output Settings for more details.
e
videoDescriptionNameB" GThe name of the video description used as video source for the output.
:�
�
	medialive4ChannelEncoderSettingsOutputGroupOutputGroupSettingswaws:medialive/ChannelEncoderSettingsOutputGroupOutputGroupSettings:ChannelEncoderSettingsOutputGroupOutputGroupSettings�
��
archiveGroupSettings�B�*�:�
�
	medialiveGChannelEncoderSettingsOutputGroupOutputGroupSettingsArchiveGroupSetting�aws:medialive/ChannelEncoderSettingsOutputGroupOutputGroupSettingsArchiveGroupSetting:ChannelEncoderSettingsOutputGroupOutputGroupSettingsArchiveGroupSettingEArchive group settings. See Archive Group Settings for more details.
�
frameCaptureGroupSettings�B�:�
�
	medialiveMChannelEncoderSettingsOutputGroupOutputGroupSettingsFrameCaptureGroupSettings�aws:medialive/ChannelEncoderSettingsOutputGroupOutputGroupSettingsFrameCaptureGroupSettings:ChannelEncoderSettingsOutputGroupOutputGroupSettingsFrameCaptureGroupSettings�
hlsGroupSettings�B�:�
�
	medialiveDChannelEncoderSettingsOutputGroupOutputGroupSettingsHlsGroupSettings�aws:medialive/ChannelEncoderSettingsOutputGroupOutputGroupSettingsHlsGroupSettings:ChannelEncoderSettingsOutputGroupOutputGroupSettingsHlsGroupSettings�
mediaPackageGroupSettings�B�:�
�
	medialiveMChannelEncoderSettingsOutputGroupOutputGroupSettingsMediaPackageGroupSettings�aws:medialive/ChannelEncoderSettingsOutputGroupOutputGroupSettingsMediaPackageGroupSettings:ChannelEncoderSettingsOutputGroupOutputGroupSettingsMediaPackageGroupSettingsQMedia package group settings. See Media Package Group Settings for more details.
�
msSmoothGroupSettings�B�:�
�
	medialiveIChannelEncoderSettingsOutputGroupOutputGroupSettingsMsSmoothGroupSettings�aws:medialive/ChannelEncoderSettingsOutputGroupOutputGroupSettingsMsSmoothGroupSettings:ChannelEncoderSettingsOutputGroupOutputGroupSettingsMsSmoothGroupSettings�
multiplexGroupSettings�B�:�
�
	medialiveJChannelEncoderSettingsOutputGroupOutputGroupSettingsMultiplexGroupSettings�aws:medialive/ChannelEncoderSettingsOutputGroupOutputGroupSettingsMultiplexGroupSettings:ChannelEncoderSettingsOutputGroupOutputGroupSettingsMultiplexGroupSettings�
rtmpGroupSettings�B�:�
�
	medialiveEChannelEncoderSettingsOutputGroupOutputGroupSettingsRtmpGroupSettings�aws:medialive/ChannelEncoderSettingsOutputGroupOutputGroupSettingsRtmpGroupSettings:ChannelEncoderSettingsOutputGroupOutputGroupSettingsRtmpGroupSettings?RTMP group settings. See RTMP Group Settings for more details.
�
udpGroupSettings�B�:�
�
	medialiveDChannelEncoderSettingsOutputGroupOutputGroupSettingsUdpGroupSettings�aws:medialive/ChannelEncoderSettingsOutputGroupOutputGroupSettingsUdpGroupSettings:ChannelEncoderSettingsOutputGroupOutputGroupSettingsUdpGroupSettings:�	
�
	medialiveGChannelEncoderSettingsOutputGroupOutputGroupSettingsArchiveGroupSetting�aws:medialive/ChannelEncoderSettingsOutputGroupOutputGroupSettingsArchiveGroupSetting:ChannelEncoderSettingsOutputGroupOutputGroupSettingsArchiveGroupSetting�
��
archiveCdnSettings�B�:�
�
	medialiveYChannelEncoderSettingsOutputGroupOutputGroupSettingsArchiveGroupSettingArchiveCdnSettings�aws:medialive/ChannelEncoderSettingsOutputGroupOutputGroupSettingsArchiveGroupSettingArchiveCdnSettings:ChannelEncoderSettingsOutputGroupOutputGroupSettingsArchiveGroupSettingArchiveCdnSettingsbParameters that control the interactions with the CDN. See Archive CDN Settings for more details.
�
destination�:�
�
	medialiveRChannelEncoderSettingsOutputGroupOutputGroupSettingsArchiveGroupSettingDestination�aws:medialive/ChannelEncoderSettingsOutputGroupOutputGroupSettingsArchiveGroupSettingDestination:ChannelEncoderSettingsOutputGroupOutputGroupSettingsArchiveGroupSettingDestinationfA director and base filename where archive files should be written. See Destination for more details.
l
rolloverIntervalB RNumber of seconds to write to archive file before closing and starting a new one.
:�
�
	medialiveYChannelEncoderSettingsOutputGroupOutputGroupSettingsArchiveGroupSettingArchiveCdnSettings�aws:medialive/ChannelEncoderSettingsOutputGroupOutputGroupSettingsArchiveGroupSettingArchiveCdnSettings:ChannelEncoderSettingsOutputGroupOutputGroupSettingsArchiveGroupSettingArchiveCdnSettings�
��
archiveS3Settings�B�:�
�
	medialivejChannelEncoderSettingsOutputGroupOutputGroupSettingsArchiveGroupSettingArchiveCdnSettingsArchiveS3Settings�aws:medialive/ChannelEncoderSettingsOutputGroupOutputGroupSettingsArchiveGroupSettingArchiveCdnSettingsArchiveS3Settings:ChannelEncoderSettingsOutputGroupOutputGroupSettingsArchiveGroupSettingArchiveCdnSettingsArchiveS3Settings?Archive S3 Settings. See Archive S3 Settings for more details.
:�
�
	medialivejChannelEncoderSettingsOutputGroupOutputGroupSettingsArchiveGroupSettingArchiveCdnSettingsArchiveS3Settings�aws:medialive/ChannelEncoderSettingsOutputGroupOutputGroupSettingsArchiveGroupSettingArchiveCdnSettingsArchiveS3Settings:ChannelEncoderSettingsOutputGroupOutputGroupSettingsArchiveGroupSettingArchiveCdnSettingsArchiveS3SettingsK
IG
	cannedAclB" 4Specify the canned ACL to apply to each S3 request.
:�
�
	medialiveRChannelEncoderSettingsOutputGroupOutputGroupSettingsArchiveGroupSettingDestination�aws:medialive/ChannelEncoderSettingsOutputGroupOutputGroupSettingsArchiveGroupSettingDestination:ChannelEncoderSettingsOutputGroupOutputGroupSettingsArchiveGroupSettingDestination>
<:
destinationRefId" "Reference ID for the destination.
:�
�
	medialiveMChannelEncoderSettingsOutputGroupOutputGroupSettingsFrameCaptureGroupSettings�aws:medialive/ChannelEncoderSettingsOutputGroupOutputGroupSettingsFrameCaptureGroupSettings:ChannelEncoderSettingsOutputGroupOutputGroupSettingsFrameCaptureGroupSettings�
��
destination�:�
�
	medialiveXChannelEncoderSettingsOutputGroupOutputGroupSettingsFrameCaptureGroupSettingsDestination�aws:medialive/ChannelEncoderSettingsOutputGroupOutputGroupSettingsFrameCaptureGroupSettingsDestination:ChannelEncoderSettingsOutputGroupOutputGroupSettingsFrameCaptureGroupSettingsDestination�
frameCaptureCdnSettings�B�:�
�
	medialivedChannelEncoderSettingsOutputGroupOutputGroupSettingsFrameCaptureGroupSettingsFrameCaptureCdnSettings�aws:medialive/ChannelEncoderSettingsOutputGroupOutputGroupSettingsFrameCaptureGroupSettingsFrameCaptureCdnSettings:ChannelEncoderSettingsOutputGroupOutputGroupSettingsFrameCaptureGroupSettingsFrameCaptureCdnSettings:�
�
	medialiveXChannelEncoderSettingsOutputGroupOutputGroupSettingsFrameCaptureGroupSettingsDestination�aws:medialive/ChannelEncoderSettingsOutputGroupOutputGroupSettingsFrameCaptureGroupSettingsDestination:ChannelEncoderSettingsOutputGroupOutputGroupSettingsFrameCaptureGroupSettingsDestination>
<:
destinationRefId" "Reference ID for the destination.
:�
�
	medialivedChannelEncoderSettingsOutputGroupOutputGroupSettingsFrameCaptureGroupSettingsFrameCaptureCdnSettings�aws:medialive/ChannelEncoderSettingsOutputGroupOutputGroupSettingsFrameCaptureGroupSettingsFrameCaptureCdnSettings:ChannelEncoderSettingsOutputGroupOutputGroupSettingsFrameCaptureGroupSettingsFrameCaptureCdnSettings�
��
frameCaptureS3Settings�B�:�
�
	medialivezChannelEncoderSettingsOutputGroupOutputGroupSettingsFrameCaptureGroupSettingsFrameCaptureCdnSettingsFrameCaptureS3Settings�aws:medialive/ChannelEncoderSettingsOutputGroupOutputGroupSettingsFrameCaptureGroupSettingsFrameCaptureCdnSettingsFrameCaptureS3Settings:ChannelEncoderSettingsOutputGroupOutputGroupSettingsFrameCaptureGroupSettingsFrameCaptureCdnSettingsFrameCaptureS3Settings:�
�
	medialivezChannelEncoderSettingsOutputGroupOutputGroupSettingsFrameCaptureGroupSettingsFrameCaptureCdnSettingsFrameCaptureS3Settings�aws:medialive/ChannelEncoderSettingsOutputGroupOutputGroupSettingsFrameCaptureGroupSettingsFrameCaptureCdnSettingsFrameCaptureS3Settings:ChannelEncoderSettingsOutputGroupOutputGroupSettingsFrameCaptureGroupSettingsFrameCaptureCdnSettingsFrameCaptureS3SettingsK
IG
	cannedAclB" 4Specify the canned ACL to apply to each S3 request.
:�
�
	medialiveDChannelEncoderSettingsOutputGroupOutputGroupSettingsHlsGroupSettings�aws:medialive/ChannelEncoderSettingsOutputGroupOutputGroupSettingsHlsGroupSettings:ChannelEncoderSettingsOutputGroupOutputGroupSettingsHlsGroupSettings�
�?
	adMarkersB*" *The ad marker type for this output group.

baseUrlContentB" 
baseUrlContent1B" 
baseUrlManifestB" 
baseUrlManifest1B" �
captionLanguageMappings�B�*�:�
�
	medialiveZChannelEncoderSettingsOutputGroupOutputGroupSettingsHlsGroupSettingsCaptionLanguageMapping�aws:medialive/ChannelEncoderSettingsOutputGroupOutputGroupSettingsHlsGroupSettingsCaptionLanguageMapping:ChannelEncoderSettingsOutputGroupOutputGroupSettingsHlsGroupSettingsCaptionLanguageMapping
captionLanguageSettingB" 
clientCacheB" 
codecSpecificationB" 

constantIvB" �
destination�:�
�
	medialiveOChannelEncoderSettingsOutputGroupOutputGroupSettingsHlsGroupSettingsDestination�aws:medialive/ChannelEncoderSettingsOutputGroupOutputGroupSettingsHlsGroupSettingsDestination:ChannelEncoderSettingsOutputGroupOutputGroupSettingsHlsGroupSettingsDestination
directoryStructureB" 
discontinuityTagsB" 
encryptionTypeB" �
hlsCdnSettings�B�*�:�
�
	medialiveQChannelEncoderSettingsOutputGroupOutputGroupSettingsHlsGroupSettingsHlsCdnSetting�aws:medialive/ChannelEncoderSettingsOutputGroupOutputGroupSettingsHlsGroupSettingsHlsCdnSetting:ChannelEncoderSettingsOutputGroupOutputGroupSettingsHlsGroupSettingsHlsCdnSetting
hlsId3SegmentTaggingB" 
iframeOnlyPlaylistsB" !
incompleteSegmentBehaviorB" 
indexNSegmentsB 
inputLossActionB" 
ivInManifestB" 
ivSourceB" 
keepSegmentsB 
	keyFormatB" 
keyFormatVersionsB" �
keyProviderSettings�B�:�
�
	medialiveWChannelEncoderSettingsOutputGroupOutputGroupSettingsHlsGroupSettingsKeyProviderSettings�aws:medialive/ChannelEncoderSettingsOutputGroupOutputGroupSettingsHlsGroupSettingsKeyProviderSettings:ChannelEncoderSettingsOutputGroupOutputGroupSettingsHlsGroupSettingsKeyProviderSettings
manifestCompressionB" 
manifestDurationFormatB" 
minSegmentLengthB 
modeB" 
outputSelectionB" 
programDateTimeB" 
programDateTimeClockB" 
programDateTimePeriodB 
redundantManifestB" 
segmentLengthB 
segmentsPerSubdirectoryB 
streamInfResolutionB" J
timedMetadataId3FrameB" +Indicates ID3 frame that has the timecode.

timedMetadataId3PeriodB "
timestampDeltaMillisecondsB 

tsFileModeB" :�
�
	medialiveZChannelEncoderSettingsOutputGroupOutputGroupSettingsHlsGroupSettingsCaptionLanguageMapping�aws:medialive/ChannelEncoderSettingsOutputGroupOutputGroupSettingsHlsGroupSettingsCaptionLanguageMapping:ChannelEncoderSettingsOutputGroupOutputGroupSettingsHlsGroupSettingsCaptionLanguageMapping�
�
captionChannel 
languageCode" |
languageDescription" aHuman readable information to indicate captions available for players (eg. English, or Spanish).
:�
�
	medialiveOChannelEncoderSettingsOutputGroupOutputGroupSettingsHlsGroupSettingsDestination�aws:medialive/ChannelEncoderSettingsOutputGroupOutputGroupSettingsHlsGroupSettingsDestination:ChannelEncoderSettingsOutputGroupOutputGroupSettingsHlsGroupSettingsDestination>
<:
destinationRefId" "Reference ID for the destination.
:�
�
	medialiveQChannelEncoderSettingsOutputGroupOutputGroupSettingsHlsGroupSettingsHlsCdnSetting�aws:medialive/ChannelEncoderSettingsOutputGroupOutputGroupSettingsHlsGroupSettingsHlsCdnSetting:ChannelEncoderSettingsOutputGroupOutputGroupSettingsHlsGroupSettingsHlsCdnSetting�
��
hlsAkamaiSettings�B�:�
�
	medialivebChannelEncoderSettingsOutputGroupOutputGroupSettingsHlsGroupSettingsHlsCdnSettingHlsAkamaiSettings�aws:medialive/ChannelEncoderSettingsOutputGroupOutputGroupSettingsHlsGroupSettingsHlsCdnSettingHlsAkamaiSettings:ChannelEncoderSettingsOutputGroupOutputGroupSettingsHlsGroupSettingsHlsCdnSettingHlsAkamaiSettings�
hlsBasicPutSettings�B�:�
�
	medialivedChannelEncoderSettingsOutputGroupOutputGroupSettingsHlsGroupSettingsHlsCdnSettingHlsBasicPutSettings�aws:medialive/ChannelEncoderSettingsOutputGroupOutputGroupSettingsHlsGroupSettingsHlsCdnSettingHlsBasicPutSettings:ChannelEncoderSettingsOutputGroupOutputGroupSettingsHlsGroupSettingsHlsCdnSettingHlsBasicPutSettings�
hlsMediaStoreSettings�B�:�
�
	medialivefChannelEncoderSettingsOutputGroupOutputGroupSettingsHlsGroupSettingsHlsCdnSettingHlsMediaStoreSettings�aws:medialive/ChannelEncoderSettingsOutputGroupOutputGroupSettingsHlsGroupSettingsHlsCdnSettingHlsMediaStoreSettings:ChannelEncoderSettingsOutputGroupOutputGroupSettingsHlsGroupSettingsHlsCdnSettingHlsMediaStoreSettings�
hlsS3Settings�B�:�
�
	medialive^ChannelEncoderSettingsOutputGroupOutputGroupSettingsHlsGroupSettingsHlsCdnSettingHlsS3Settings�aws:medialive/ChannelEncoderSettingsOutputGroupOutputGroupSettingsHlsGroupSettingsHlsCdnSettingHlsS3Settings:ChannelEncoderSettingsOutputGroupOutputGroupSettingsHlsGroupSettingsHlsCdnSettingHlsS3Settings�
hlsWebdavSettings�B�:�
�
	medialivebChannelEncoderSettingsOutputGroupOutputGroupSettingsHlsGroupSettingsHlsCdnSettingHlsWebdavSettings�aws:medialive/ChannelEncoderSettingsOutputGroupOutputGroupSettingsHlsGroupSettingsHlsCdnSettingHlsWebdavSettings:ChannelEncoderSettingsOutputGroupOutputGroupSettingsHlsGroupSettingsHlsCdnSettingHlsWebdavSettings:�
�
	medialivebChannelEncoderSettingsOutputGroupOutputGroupSettingsHlsGroupSettingsHlsCdnSettingHlsAkamaiSettings�aws:medialive/ChannelEncoderSettingsOutputGroupOutputGroupSettingsHlsGroupSettingsHlsCdnSettingHlsAkamaiSettings:ChannelEncoderSettingsOutputGroupOutputGroupSettingsHlsGroupSettingsHlsCdnSettingHlsAkamaiSettings�
��
connectionRetryIntervalB jNumber of seconds to wait before retrying connection to the flash media server if the connection is lost.

filecacheDurationB 
httpTransferModeB" .

numRetriesB Number of retry attempts.
N
restartDelayB 8Number of seconds to wait until a restart is initiated.

saltB" 
tokenB" :�
�
	medialivedChannelEncoderSettingsOutputGroupOutputGroupSettingsHlsGroupSettingsHlsCdnSettingHlsBasicPutSettings�aws:medialive/ChannelEncoderSettingsOutputGroupOutputGroupSettingsHlsGroupSettingsHlsCdnSettingHlsBasicPutSettings:ChannelEncoderSettingsOutputGroupOutputGroupSettingsHlsGroupSettingsHlsCdnSettingHlsBasicPutSettings�
��
connectionRetryIntervalB jNumber of seconds to wait before retrying connection to the flash media server if the connection is lost.

filecacheDurationB .

numRetriesB Number of retry attempts.
N
restartDelayB 8Number of seconds to wait until a restart is initiated.
:�
�
	medialivefChannelEncoderSettingsOutputGroupOutputGroupSettingsHlsGroupSettingsHlsCdnSettingHlsMediaStoreSettings�aws:medialive/ChannelEncoderSettingsOutputGroupOutputGroupSettingsHlsGroupSettingsHlsCdnSettingHlsMediaStoreSettings:ChannelEncoderSettingsOutputGroupOutputGroupSettingsHlsGroupSettingsHlsCdnSettingHlsMediaStoreSettings�
��
connectionRetryIntervalB jNumber of seconds to wait before retrying connection to the flash media server if the connection is lost.

filecacheDurationB 
mediaStoreStorageClassB" .

numRetriesB Number of retry attempts.
N
restartDelayB 8Number of seconds to wait until a restart is initiated.
:�
�
	medialive^ChannelEncoderSettingsOutputGroupOutputGroupSettingsHlsGroupSettingsHlsCdnSettingHlsS3Settings�aws:medialive/ChannelEncoderSettingsOutputGroupOutputGroupSettingsHlsGroupSettingsHlsCdnSettingHlsS3Settings:ChannelEncoderSettingsOutputGroupOutputGroupSettingsHlsGroupSettingsHlsCdnSettingHlsS3SettingsK
IG
	cannedAclB" 4Specify the canned ACL to apply to each S3 request.
:�
�
	medialivebChannelEncoderSettingsOutputGroupOutputGroupSettingsHlsGroupSettingsHlsCdnSettingHlsWebdavSettings�aws:medialive/ChannelEncoderSettingsOutputGroupOutputGroupSettingsHlsGroupSettingsHlsCdnSettingHlsWebdavSettings:ChannelEncoderSettingsOutputGroupOutputGroupSettingsHlsGroupSettingsHlsCdnSettingHlsWebdavSettings�
��
connectionRetryIntervalB jNumber of seconds to wait before retrying connection to the flash media server if the connection is lost.

filecacheDurationB 
httpTransferModeB" .

numRetriesB Number of retry attempts.
N
restartDelayB 8Number of seconds to wait until a restart is initiated.
:�
�
	medialiveWChannelEncoderSettingsOutputGroupOutputGroupSettingsHlsGroupSettingsKeyProviderSettings�aws:medialive/ChannelEncoderSettingsOutputGroupOutputGroupSettingsHlsGroupSettingsKeyProviderSettings:ChannelEncoderSettingsOutputGroupOutputGroupSettingsHlsGroupSettingsKeyProviderSettings�
��
staticKeySettings�B�*�:�
�
	medialivegChannelEncoderSettingsOutputGroupOutputGroupSettingsHlsGroupSettingsKeyProviderSettingsStaticKeySetting�aws:medialive/ChannelEncoderSettingsOutputGroupOutputGroupSettingsHlsGroupSettingsKeyProviderSettingsStaticKeySetting:ChannelEncoderSettingsOutputGroupOutputGroupSettingsHlsGroupSettingsKeyProviderSettingsStaticKeySetting:�
�
	medialivegChannelEncoderSettingsOutputGroupOutputGroupSettingsHlsGroupSettingsKeyProviderSettingsStaticKeySetting�aws:medialive/ChannelEncoderSettingsOutputGroupOutputGroupSettingsHlsGroupSettingsKeyProviderSettingsStaticKeySetting:ChannelEncoderSettingsOutputGroupOutputGroupSettingsHlsGroupSettingsKeyProviderSettingsStaticKeySetting�
��
keyProviderServer�B�:�
�
	medialivexChannelEncoderSettingsOutputGroupOutputGroupSettingsHlsGroupSettingsKeyProviderSettingsStaticKeySettingKeyProviderServer�aws:medialive/ChannelEncoderSettingsOutputGroupOutputGroupSettingsHlsGroupSettingsKeyProviderSettingsStaticKeySettingKeyProviderServer:ChannelEncoderSettingsOutputGroupOutputGroupSettingsHlsGroupSettingsKeyProviderSettingsStaticKeySettingKeyProviderServer
staticKeyValue" :�
�
	medialivexChannelEncoderSettingsOutputGroupOutputGroupSettingsHlsGroupSettingsKeyProviderSettingsStaticKeySettingKeyProviderServer�aws:medialive/ChannelEncoderSettingsOutputGroupOutputGroupSettingsHlsGroupSettingsKeyProviderSettingsStaticKeySettingKeyProviderServer:ChannelEncoderSettingsOutputGroupOutputGroupSettingsHlsGroupSettingsKeyProviderSettingsStaticKeySettingKeyProviderServer6
4
passwordParamB" 	
uri" 
usernameB" :�
�
	medialiveMChannelEncoderSettingsOutputGroupOutputGroupSettingsMediaPackageGroupSettings�aws:medialive/ChannelEncoderSettingsOutputGroupOutputGroupSettingsMediaPackageGroupSettings:ChannelEncoderSettingsOutputGroupOutputGroupSettingsMediaPackageGroupSettings�
��
destination�:�
�
	medialiveXChannelEncoderSettingsOutputGroupOutputGroupSettingsMediaPackageGroupSettingsDestination�aws:medialive/ChannelEncoderSettingsOutputGroupOutputGroupSettingsMediaPackageGroupSettingsDestination:ChannelEncoderSettingsOutputGroupOutputGroupSettingsMediaPackageGroupSettingsDestinationfA director and base filename where archive files should be written. See Destination for more details.
:�
�
	medialiveXChannelEncoderSettingsOutputGroupOutputGroupSettingsMediaPackageGroupSettingsDestination�aws:medialive/ChannelEncoderSettingsOutputGroupOutputGroupSettingsMediaPackageGroupSettingsDestination:ChannelEncoderSettingsOutputGroupOutputGroupSettingsMediaPackageGroupSettingsDestination>
<:
destinationRefId" "Reference ID for the destination.
:�

�
	medialiveIChannelEncoderSettingsOutputGroupOutputGroupSettingsMsSmoothGroupSettings�aws:medialive/ChannelEncoderSettingsOutputGroupOutputGroupSettingsMsSmoothGroupSettings:ChannelEncoderSettingsOutputGroupOutputGroupSettingsMsSmoothGroupSettings�
�
acquisitionPointIdB"  
audioOnlyTimecodeControlB" U
certificateModeB" <Setting to allow self signed or verified RTMP certificates.
�
connectionRetryIntervalB jNumber of seconds to wait before retrying connection to the flash media server if the connection is lost.
�
destination�:�
�
	medialiveTChannelEncoderSettingsOutputGroupOutputGroupSettingsMsSmoothGroupSettingsDestination�aws:medialive/ChannelEncoderSettingsOutputGroupOutputGroupSettingsMsSmoothGroupSettingsDestination:ChannelEncoderSettingsOutputGroupOutputGroupSettingsMsSmoothGroupSettingsDestination
eventIdB" 
eventIdModeB" 
eventStopBehaviorB" 
filecacheDurationB 
fragmentLengthB 
inputLossActionB" .

numRetriesB Number of retry attempts.
N
restartDelayB 8Number of seconds to wait until a restart is initiated.

segmentationModeB" 
sendDelayMsB 
sparseTrackTypeB" 
streamManifestBehaviorB" 
timestampOffsetB" 
timestampOffsetModeB" :�
�
	medialiveTChannelEncoderSettingsOutputGroupOutputGroupSettingsMsSmoothGroupSettingsDestination�aws:medialive/ChannelEncoderSettingsOutputGroupOutputGroupSettingsMsSmoothGroupSettingsDestination:ChannelEncoderSettingsOutputGroupOutputGroupSettingsMsSmoothGroupSettingsDestination>
<:
destinationRefId" "Reference ID for the destination.
:�
�
	medialiveJChannelEncoderSettingsOutputGroupOutputGroupSettingsMultiplexGroupSettings�aws:medialive/ChannelEncoderSettingsOutputGroupOutputGroupSettingsMultiplexGroupSettings:ChannelEncoderSettingsOutputGroupOutputGroupSettingsMultiplexGroupSettings
 :�
�
	medialiveEChannelEncoderSettingsOutputGroupOutputGroupSettingsRtmpGroupSettings�aws:medialive/ChannelEncoderSettingsOutputGroupOutputGroupSettingsRtmpGroupSettings:ChannelEncoderSettingsOutputGroupOutputGroupSettingsRtmpGroupSettings�
�?
	adMarkersB*" *The ad marker type for this output group.
U
authenticationSchemeB" 7Authentication scheme to use when connecting with CDN.
J
cacheFullBehaviorB" /Controls behavior when content cache fills up.
P
cacheLengthB ;Cache length in seconds, is used to calculate buffer size.
V
captionDataB" AControls the types of data that passes to onCaptionInfo outputs.
_
inputLossActionB" FControls the behavior of the RTMP group if input becomes unavailable.
N
restartDelayB 8Number of seconds to wait until a restart is initiated.
:�
�
	medialiveDChannelEncoderSettingsOutputGroupOutputGroupSettingsUdpGroupSettings�aws:medialive/ChannelEncoderSettingsOutputGroupOutputGroupSettingsUdpGroupSettings:ChannelEncoderSettingsOutputGroupOutputGroupSettingsUdpGroupSettings�
�U
inputLossActionB" <Specifies behavior of last resort when input video os lost.
J
timedMetadataId3FrameB" +Indicates ID3 frame that has the timecode.

timedMetadataId3PeriodB :�
�
	medialive5ChannelEncoderSettingsOutputGroupOutputOutputSettingsyaws:medialive/ChannelEncoderSettingsOutputGroupOutputOutputSettings:ChannelEncoderSettingsOutputGroupOutputOutputSettings�
��
archiveOutputSettings�B�:�
�
	medialiveJChannelEncoderSettingsOutputGroupOutputOutputSettingsArchiveOutputSettings�aws:medialive/ChannelEncoderSettingsOutputGroupOutputOutputSettingsArchiveOutputSettings:ChannelEncoderSettingsOutputGroupOutputOutputSettingsArchiveOutputSettingsGArchive output settings. See Archive Output Settings for more details.
�
frameCaptureOutputSettings�B�:�
�
	medialiveOChannelEncoderSettingsOutputGroupOutputOutputSettingsFrameCaptureOutputSettings�aws:medialive/ChannelEncoderSettingsOutputGroupOutputOutputSettingsFrameCaptureOutputSettings:ChannelEncoderSettingsOutputGroupOutputOutputSettingsFrameCaptureOutputSettings�
hlsOutputSettings�B�:�
�
	medialiveFChannelEncoderSettingsOutputGroupOutputOutputSettingsHlsOutputSettings�aws:medialive/ChannelEncoderSettingsOutputGroupOutputOutputSettingsHlsOutputSettings:ChannelEncoderSettingsOutputGroupOutputOutputSettingsHlsOutputSettings�
mediaPackageOutputSettings�B�:�
�
	medialiveOChannelEncoderSettingsOutputGroupOutputOutputSettingsMediaPackageOutputSettings�aws:medialive/ChannelEncoderSettingsOutputGroupOutputOutputSettingsMediaPackageOutputSettings:ChannelEncoderSettingsOutputGroupOutputOutputSettingsMediaPackageOutputSettingsBMedia package output settings. This can be set as an empty block.
�
msSmoothOutputSettings�B�:�
�
	medialiveKChannelEncoderSettingsOutputGroupOutputOutputSettingsMsSmoothOutputSettings�aws:medialive/ChannelEncoderSettingsOutputGroupOutputOutputSettingsMsSmoothOutputSettings:ChannelEncoderSettingsOutputGroupOutputOutputSettingsMsSmoothOutputSettings�
multiplexOutputSettings�B�:�
�
	medialiveLChannelEncoderSettingsOutputGroupOutputOutputSettingsMultiplexOutputSettings�aws:medialive/ChannelEncoderSettingsOutputGroupOutputOutputSettingsMultiplexOutputSettings:ChannelEncoderSettingsOutputGroupOutputOutputSettingsMultiplexOutputSettingsKMultiplex output settings. See Multiplex Output Settings for more details.
�
rtmpOutputSettings�B�:�
�
	medialiveGChannelEncoderSettingsOutputGroupOutputOutputSettingsRtmpOutputSettings�aws:medialive/ChannelEncoderSettingsOutputGroupOutputOutputSettingsRtmpOutputSettings:ChannelEncoderSettingsOutputGroupOutputOutputSettingsRtmpOutputSettingsARTMP output settings. See RTMP Output Settings for more details.
�
udpOutputSettings�B�:�
�
	medialiveFChannelEncoderSettingsOutputGroupOutputOutputSettingsUdpOutputSettings�aws:medialive/ChannelEncoderSettingsOutputGroupOutputOutputSettingsUdpOutputSettings:ChannelEncoderSettingsOutputGroupOutputOutputSettingsUdpOutputSettings?UDP output settings. See UDP Output Settings for more details.
:�
�
	medialiveJChannelEncoderSettingsOutputGroupOutputOutputSettingsArchiveOutputSettings�aws:medialive/ChannelEncoderSettingsOutputGroupOutputOutputSettingsArchiveOutputSettings:ChannelEncoderSettingsOutputGroupOutputOutputSettingsArchiveOutputSettings�
��
containerSettings�B�:�
�
	medialive[ChannelEncoderSettingsOutputGroupOutputOutputSettingsArchiveOutputSettingsContainerSettings�aws:medialive/ChannelEncoderSettingsOutputGroupOutputOutputSettingsArchiveOutputSettingsContainerSettings:ChannelEncoderSettingsOutputGroupOutputOutputSettingsArchiveOutputSettingsContainerSettings^Settings specific to the container type of the file. See Container Settings for more details.
*
	extensionB" Output file extension.
�
nameModifierB" lString concatenated to the end of the destination filename. Required for multiple outputs of the same type.
:�	
�
	medialive[ChannelEncoderSettingsOutputGroupOutputOutputSettingsArchiveOutputSettingsContainerSettings�aws:medialive/ChannelEncoderSettingsOutputGroupOutputOutputSettingsArchiveOutputSettingsContainerSettings:ChannelEncoderSettingsOutputGroupOutputOutputSettingsArchiveOutputSettingsContainerSettings�
��
m2tsSettings�B�:�
�
	medialivegChannelEncoderSettingsOutputGroupOutputOutputSettingsArchiveOutputSettingsContainerSettingsM2tsSettings�aws:medialive/ChannelEncoderSettingsOutputGroupOutputOutputSettingsArchiveOutputSettingsContainerSettingsM2tsSettings:ChannelEncoderSettingsOutputGroupOutputOutputSettingsArchiveOutputSettingsContainerSettingsM2tsSettings�M2TS Settings. See [M2TS Settings](https://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-medialive-channel-m2tssettings.html) for more details.
�
rawSettings�B�:�
�
	medialivefChannelEncoderSettingsOutputGroupOutputOutputSettingsArchiveOutputSettingsContainerSettingsRawSettings�aws:medialive/ChannelEncoderSettingsOutputGroupOutputOutputSettingsArchiveOutputSettingsContainerSettingsRawSettings:ChannelEncoderSettingsOutputGroupOutputOutputSettingsArchiveOutputSettingsContainerSettingsRawSettings1Raw Settings. This can be set as an empty block.
:�
�
	medialivegChannelEncoderSettingsOutputGroupOutputOutputSettingsArchiveOutputSettingsContainerSettingsM2tsSettings�aws:medialive/ChannelEncoderSettingsOutputGroupOutputOutputSettingsArchiveOutputSettingsContainerSettingsM2tsSettings:ChannelEncoderSettingsOutputGroupOutputOutputSettingsArchiveOutputSettingsContainerSettingsM2tsSettings�
� 
absentInputAudioBehaviorB" 
aribB" 
aribCaptionsPidB" 
aribCaptionsPidControlB" 
audioBufferModelB" 
audioFramesPerPesB 
	audioPidsB" 
audioStreamTypeB" 
bitrateB 
bufferModelB" 
ccDescriptorB" �
dvbNitSettings�B�:�
�
	medialiveuChannelEncoderSettingsOutputGroupOutputOutputSettingsArchiveOutputSettingsContainerSettingsM2tsSettingsDvbNitSettings�aws:medialive/ChannelEncoderSettingsOutputGroupOutputOutputSettingsArchiveOutputSettingsContainerSettingsM2tsSettingsDvbNitSettings:ChannelEncoderSettingsOutputGroupOutputOutputSettingsArchiveOutputSettingsContainerSettingsM2tsSettingsDvbNitSettings�
dvbSdtSettings�B�:�
�
	medialiveuChannelEncoderSettingsOutputGroupOutputOutputSettingsArchiveOutputSettingsContainerSettingsM2tsSettingsDvbSdtSettings�aws:medialive/ChannelEncoderSettingsOutputGroupOutputOutputSettingsArchiveOutputSettingsContainerSettingsM2tsSettingsDvbSdtSettings:ChannelEncoderSettingsOutputGroupOutputOutputSettingsArchiveOutputSettingsContainerSettingsM2tsSettingsDvbSdtSettings

dvbSubPidsB" �
dvbTdtSettings�B�:�
�
	medialiveuChannelEncoderSettingsOutputGroupOutputOutputSettingsArchiveOutputSettingsContainerSettingsM2tsSettingsDvbTdtSettings�aws:medialive/ChannelEncoderSettingsOutputGroupOutputOutputSettingsArchiveOutputSettingsContainerSettingsM2tsSettingsDvbTdtSettings:ChannelEncoderSettingsOutputGroupOutputOutputSettingsArchiveOutputSettingsContainerSettingsM2tsSettingsDvbTdtSettings
dvbTeletextPidB" 
ebifB" 
ebpAudioIntervalB" 
ebpLookaheadMsB 
ebpPlacementB" 
ecmPidB" 
esRateInPesB" 
etvPlatformPidB" 
etvSignalPidB" 
fragmentTimeB 
klvB" 
klvDataPidsB" 
nielsenId3BehaviorB" 
nullPacketBitrateB 
patIntervalB 

pcrControlB" 
	pcrPeriodB 
pcrPidB" 
pmtIntervalB 
pmtPidB" 

programNumB 
rateModeB" 

scte27PidsB" 
scte35ControlB" <
	scte35PidB" )PID from which to read SCTE-35 messages.

segmentationMarkersB" 
segmentationStyleB" 
segmentationTimeB 
timedMetadataBehaviorB" 
timedMetadataPidB" 
transportStreamIdB 
videoPidB" :�
�
	medialiveuChannelEncoderSettingsOutputGroupOutputOutputSettingsArchiveOutputSettingsContainerSettingsM2tsSettingsDvbNitSettings�aws:medialive/ChannelEncoderSettingsOutputGroupOutputOutputSettingsArchiveOutputSettingsContainerSettingsM2tsSettingsDvbNitSettings:ChannelEncoderSettingsOutputGroupOutputOutputSettingsArchiveOutputSettingsContainerSettingsM2tsSettingsDvbNitSettings;
9
	networkId 
networkName" 
repIntervalB :�
�
	medialiveuChannelEncoderSettingsOutputGroupOutputOutputSettingsArchiveOutputSettingsContainerSettingsM2tsSettingsDvbSdtSettings�aws:medialive/ChannelEncoderSettingsOutputGroupOutputOutputSettingsArchiveOutputSettingsContainerSettingsM2tsSettingsDvbSdtSettings:ChannelEncoderSettingsOutputGroupOutputOutputSettingsArchiveOutputSettingsContainerSettingsM2tsSettingsDvbSdtSettings\
Z
	outputSdtB" 
repIntervalB 
serviceNameB" 
serviceProviderNameB" :�
�
	medialiveuChannelEncoderSettingsOutputGroupOutputOutputSettingsArchiveOutputSettingsContainerSettingsM2tsSettingsDvbTdtSettings�aws:medialive/ChannelEncoderSettingsOutputGroupOutputOutputSettingsArchiveOutputSettingsContainerSettingsM2tsSettingsDvbTdtSettings:ChannelEncoderSettingsOutputGroupOutputOutputSettingsArchiveOutputSettingsContainerSettingsM2tsSettingsDvbTdtSettings

repIntervalB :�
�
	medialivefChannelEncoderSettingsOutputGroupOutputOutputSettingsArchiveOutputSettingsContainerSettingsRawSettings�aws:medialive/ChannelEncoderSettingsOutputGroupOutputOutputSettingsArchiveOutputSettingsContainerSettingsRawSettings:ChannelEncoderSettingsOutputGroupOutputOutputSettingsArchiveOutputSettingsContainerSettingsRawSettings
 :�
�
	medialiveOChannelEncoderSettingsOutputGroupOutputOutputSettingsFrameCaptureOutputSettings�aws:medialive/ChannelEncoderSettingsOutputGroupOutputOutputSettingsFrameCaptureOutputSettings:ChannelEncoderSettingsOutputGroupOutputOutputSettingsFrameCaptureOutputSettings�
��
nameModifierB" lString concatenated to the end of the destination filename. Required for multiple outputs of the same type.
:�
�
	medialiveFChannelEncoderSettingsOutputGroupOutputOutputSettingsHlsOutputSettings�aws:medialive/ChannelEncoderSettingsOutputGroupOutputOutputSettingsHlsOutputSettings:ChannelEncoderSettingsOutputGroupOutputOutputSettingsHlsOutputSettings�
�
h265PackagingTypeB" �
hlsSettings�:�
�
	medialiveQChannelEncoderSettingsOutputGroupOutputOutputSettingsHlsOutputSettingsHlsSettings�aws:medialive/ChannelEncoderSettingsOutputGroupOutputOutputSettingsHlsOutputSettingsHlsSettings:ChannelEncoderSettingsOutputGroupOutputOutputSettingsHlsOutputSettingsHlsSettings�
nameModifierB" lString concatenated to the end of the destination filename. Required for multiple outputs of the same type.

segmentModifierB" :�
�
	medialiveQChannelEncoderSettingsOutputGroupOutputOutputSettingsHlsOutputSettingsHlsSettings�aws:medialive/ChannelEncoderSettingsOutputGroupOutputOutputSettingsHlsOutputSettingsHlsSettings:ChannelEncoderSettingsOutputGroupOutputOutputSettingsHlsOutputSettingsHlsSettings�
��
audioOnlyHlsSettings�B�:�
�
	medialiveeChannelEncoderSettingsOutputGroupOutputOutputSettingsHlsOutputSettingsHlsSettingsAudioOnlyHlsSettings�aws:medialive/ChannelEncoderSettingsOutputGroupOutputOutputSettingsHlsOutputSettingsHlsSettingsAudioOnlyHlsSettings:ChannelEncoderSettingsOutputGroupOutputOutputSettingsHlsOutputSettingsHlsSettingsAudioOnlyHlsSettings�
fmp4HlsSettings�B�:�
�
	medialive`ChannelEncoderSettingsOutputGroupOutputOutputSettingsHlsOutputSettingsHlsSettingsFmp4HlsSettings�aws:medialive/ChannelEncoderSettingsOutputGroupOutputOutputSettingsHlsOutputSettingsHlsSettingsFmp4HlsSettings:ChannelEncoderSettingsOutputGroupOutputOutputSettingsHlsOutputSettingsHlsSettingsFmp4HlsSettings�
frameCaptureHlsSettings�B�:�
�
	medialivehChannelEncoderSettingsOutputGroupOutputOutputSettingsHlsOutputSettingsHlsSettingsFrameCaptureHlsSettings�aws:medialive/ChannelEncoderSettingsOutputGroupOutputOutputSettingsHlsOutputSettingsHlsSettingsFrameCaptureHlsSettings:ChannelEncoderSettingsOutputGroupOutputOutputSettingsHlsOutputSettingsHlsSettingsFrameCaptureHlsSettings�
standardHlsSettings�B�:�
�
	medialivedChannelEncoderSettingsOutputGroupOutputOutputSettingsHlsOutputSettingsHlsSettingsStandardHlsSettings�aws:medialive/ChannelEncoderSettingsOutputGroupOutputOutputSettingsHlsOutputSettingsHlsSettingsStandardHlsSettings:ChannelEncoderSettingsOutputGroupOutputOutputSettingsHlsOutputSettingsHlsSettingsStandardHlsSettings:�
�
	medialiveeChannelEncoderSettingsOutputGroupOutputOutputSettingsHlsOutputSettingsHlsSettingsAudioOnlyHlsSettings�aws:medialive/ChannelEncoderSettingsOutputGroupOutputOutputSettingsHlsOutputSettingsHlsSettingsAudioOnlyHlsSettings:ChannelEncoderSettingsOutputGroupOutputOutputSettingsHlsOutputSettingsHlsSettingsAudioOnlyHlsSettings�
�
audioGroupIdB" �
audioOnlyImage�B�:�
�
	medialivesChannelEncoderSettingsOutputGroupOutputOutputSettingsHlsOutputSettingsHlsSettingsAudioOnlyHlsSettingsAudioOnlyImage�aws:medialive/ChannelEncoderSettingsOutputGroupOutputOutputSettingsHlsOutputSettingsHlsSettingsAudioOnlyHlsSettingsAudioOnlyImage:ChannelEncoderSettingsOutputGroupOutputOutputSettingsHlsOutputSettingsHlsSettingsAudioOnlyHlsSettingsAudioOnlyImage
audioTrackTypeB" 
segmentTypeB" :�
�
	medialivesChannelEncoderSettingsOutputGroupOutputOutputSettingsHlsOutputSettingsHlsSettingsAudioOnlyHlsSettingsAudioOnlyImage�aws:medialive/ChannelEncoderSettingsOutputGroupOutputOutputSettingsHlsOutputSettingsHlsSettingsAudioOnlyHlsSettingsAudioOnlyImage:ChannelEncoderSettingsOutputGroupOutputOutputSettingsHlsOutputSettingsHlsSettingsAudioOnlyHlsSettingsAudioOnlyImage6
4
passwordParamB" 	
uri" 
usernameB" :�
�
	medialive`ChannelEncoderSettingsOutputGroupOutputOutputSettingsHlsOutputSettingsHlsSettingsFmp4HlsSettings�aws:medialive/ChannelEncoderSettingsOutputGroupOutputOutputSettingsHlsOutputSettingsHlsSettingsFmp4HlsSettings:ChannelEncoderSettingsOutputGroupOutputOutputSettingsHlsOutputSettingsHlsSettingsFmp4HlsSettingsY
W
audioRenditionSetsB" 
nielsenId3BehaviorB" 
timedMetadataBehaviorB" :�
�
	medialivehChannelEncoderSettingsOutputGroupOutputOutputSettingsHlsOutputSettingsHlsSettingsFrameCaptureHlsSettings�aws:medialive/ChannelEncoderSettingsOutputGroupOutputOutputSettingsHlsOutputSettingsHlsSettingsFrameCaptureHlsSettings:ChannelEncoderSettingsOutputGroupOutputOutputSettingsHlsOutputSettingsHlsSettingsFrameCaptureHlsSettings
 :�
�
	medialivedChannelEncoderSettingsOutputGroupOutputOutputSettingsHlsOutputSettingsHlsSettingsStandardHlsSettings�aws:medialive/ChannelEncoderSettingsOutputGroupOutputOutputSettingsHlsOutputSettingsHlsSettingsStandardHlsSettings:ChannelEncoderSettingsOutputGroupOutputOutputSettingsHlsOutputSettingsHlsSettingsStandardHlsSettings�
�
audioRenditionSetsB" �
m3u8Settings�:�
�
	medialivepChannelEncoderSettingsOutputGroupOutputOutputSettingsHlsOutputSettingsHlsSettingsStandardHlsSettingsM3u8Settings�aws:medialive/ChannelEncoderSettingsOutputGroupOutputOutputSettingsHlsOutputSettingsHlsSettingsStandardHlsSettingsM3u8Settings:ChannelEncoderSettingsOutputGroupOutputOutputSettingsHlsOutputSettingsHlsSettingsStandardHlsSettingsM3u8Settings:�
�
	medialivepChannelEncoderSettingsOutputGroupOutputOutputSettingsHlsOutputSettingsHlsSettingsStandardHlsSettingsM3u8Settings�aws:medialive/ChannelEncoderSettingsOutputGroupOutputOutputSettingsHlsOutputSettingsHlsSettingsStandardHlsSettingsM3u8Settings:ChannelEncoderSettingsOutputGroupOutputOutputSettingsHlsOutputSettingsHlsSettingsStandardHlsSettingsM3u8Settings�
�
audioFramesPerPesB 
	audioPidsB" 
ecmPidB" 
nielsenId3BehaviorB" 
patIntervalB 

pcrControlB" 
	pcrPeriodB 
pcrPidB" 
pmtIntervalB 
pmtPidB" 

programNumB 
scte35BehaviorB" <
	scte35PidB" )PID from which to read SCTE-35 messages.

timedMetadataBehaviorB" 
timedMetadataPidB" 
transportStreamIdB 
videoPidB" :�
�
	medialiveOChannelEncoderSettingsOutputGroupOutputOutputSettingsMediaPackageOutputSettings�aws:medialive/ChannelEncoderSettingsOutputGroupOutputOutputSettingsMediaPackageOutputSettings:ChannelEncoderSettingsOutputGroupOutputOutputSettingsMediaPackageOutputSettings
 :�
�
	medialiveKChannelEncoderSettingsOutputGroupOutputOutputSettingsMsSmoothOutputSettings�aws:medialive/ChannelEncoderSettingsOutputGroupOutputOutputSettingsMsSmoothOutputSettings:ChannelEncoderSettingsOutputGroupOutputOutputSettingsMsSmoothOutputSettings�
�
h265PackagingTypeB" �
nameModifierB" lString concatenated to the end of the destination filename. Required for multiple outputs of the same type.
:�
�
	medialiveLChannelEncoderSettingsOutputGroupOutputOutputSettingsMultiplexOutputSettings�aws:medialive/ChannelEncoderSettingsOutputGroupOutputOutputSettingsMultiplexOutputSettings:ChannelEncoderSettingsOutputGroupOutputOutputSettingsMultiplexOutputSettings�
��
destination�:�
�
	medialiveWChannelEncoderSettingsOutputGroupOutputOutputSettingsMultiplexOutputSettingsDestination�aws:medialive/ChannelEncoderSettingsOutputGroupOutputOutputSettingsMultiplexOutputSettingsDestination:ChannelEncoderSettingsOutputGroupOutputOutputSettingsMultiplexOutputSettingsDestination>Destination is a multiplex. See Destination for more details.
:�
�
	medialiveWChannelEncoderSettingsOutputGroupOutputOutputSettingsMultiplexOutputSettingsDestination�aws:medialive/ChannelEncoderSettingsOutputGroupOutputOutputSettingsMultiplexOutputSettingsDestination:ChannelEncoderSettingsOutputGroupOutputOutputSettingsMultiplexOutputSettingsDestination>
<:
destinationRefId" "Reference ID for the destination.
:�
�
	medialiveGChannelEncoderSettingsOutputGroupOutputOutputSettingsRtmpOutputSettings�aws:medialive/ChannelEncoderSettingsOutputGroupOutputOutputSettingsRtmpOutputSettings:ChannelEncoderSettingsOutputGroupOutputOutputSettingsRtmpOutputSettings�
�U
certificateModeB" <Setting to allow self signed or verified RTMP certificates.
�
connectionRetryIntervalB jNumber of seconds to wait before retrying connection to the flash media server if the connection is lost.
�
destination�:�
�
	medialiveRChannelEncoderSettingsOutputGroupOutputOutputSettingsRtmpOutputSettingsDestination�aws:medialive/ChannelEncoderSettingsOutputGroupOutputOutputSettingsRtmpOutputSettingsDestination:ChannelEncoderSettingsOutputGroupOutputOutputSettingsRtmpOutputSettingsDestinationOThe RTMP endpoint excluding the stream name. See Destination for more details.
.

numRetriesB Number of retry attempts.
:�
�
	medialiveRChannelEncoderSettingsOutputGroupOutputOutputSettingsRtmpOutputSettingsDestination�aws:medialive/ChannelEncoderSettingsOutputGroupOutputOutputSettingsRtmpOutputSettingsDestination:ChannelEncoderSettingsOutputGroupOutputOutputSettingsRtmpOutputSettingsDestination>
<:
destinationRefId" "Reference ID for the destination.
:�
�
	medialiveFChannelEncoderSettingsOutputGroupOutputOutputSettingsUdpOutputSettings�aws:medialive/ChannelEncoderSettingsOutputGroupOutputOutputSettingsUdpOutputSettings:ChannelEncoderSettingsOutputGroupOutputOutputSettingsUdpOutputSettings�	
�	:

bufferMsecB &UDP output buffering in milliseconds.
�
containerSettings�:�
�
	medialiveWChannelEncoderSettingsOutputGroupOutputOutputSettingsUdpOutputSettingsContainerSettings�aws:medialive/ChannelEncoderSettingsOutputGroupOutputOutputSettingsUdpOutputSettingsContainerSettings:ChannelEncoderSettingsOutputGroupOutputOutputSettingsUdpOutputSettingsContainerSettingsAUDP container settings. See Container Settings for more details.
�
destination�:�
�
	medialiveQChannelEncoderSettingsOutputGroupOutputOutputSettingsUdpOutputSettingsDestination�aws:medialive/ChannelEncoderSettingsOutputGroupOutputOutputSettingsUdpOutputSettingsDestination:ChannelEncoderSettingsOutputGroupOutputOutputSettingsUdpOutputSettingsDestination^Destination address and port number for RTP or UDP packets. See Destination for more details.
�
fecOutputSettings�B�:�
�
	medialiveWChannelEncoderSettingsOutputGroupOutputOutputSettingsUdpOutputSettingsFecOutputSettings�aws:medialive/ChannelEncoderSettingsOutputGroupOutputOutputSettingsUdpOutputSettingsFecOutputSettings:ChannelEncoderSettingsOutputGroupOutputOutputSettingsUdpOutputSettingsFecOutputSettings:�
�
	medialiveWChannelEncoderSettingsOutputGroupOutputOutputSettingsUdpOutputSettingsContainerSettings�aws:medialive/ChannelEncoderSettingsOutputGroupOutputOutputSettingsUdpOutputSettingsContainerSettings:ChannelEncoderSettingsOutputGroupOutputOutputSettingsUdpOutputSettingsContainerSettings�
��
m2tsSettings�B�:�
�
	medialivecChannelEncoderSettingsOutputGroupOutputOutputSettingsUdpOutputSettingsContainerSettingsM2tsSettings�aws:medialive/ChannelEncoderSettingsOutputGroupOutputOutputSettingsUdpOutputSettingsContainerSettingsM2tsSettings:ChannelEncoderSettingsOutputGroupOutputOutputSettingsUdpOutputSettingsContainerSettingsM2tsSettings�M2TS Settings. See [M2TS Settings](https://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-medialive-channel-m2tssettings.html) for more details.
:�
�
	medialivecChannelEncoderSettingsOutputGroupOutputOutputSettingsUdpOutputSettingsContainerSettingsM2tsSettings�aws:medialive/ChannelEncoderSettingsOutputGroupOutputOutputSettingsUdpOutputSettingsContainerSettingsM2tsSettings:ChannelEncoderSettingsOutputGroupOutputOutputSettingsUdpOutputSettingsContainerSettingsM2tsSettings�
� 
absentInputAudioBehaviorB" 
aribB" 
aribCaptionsPidB" 
aribCaptionsPidControlB" 
audioBufferModelB" 
audioFramesPerPesB 
	audioPidsB" 
audioStreamTypeB" 
bitrateB 
bufferModelB" 
ccDescriptorB" �
dvbNitSettings�B�:�
�
	medialiveqChannelEncoderSettingsOutputGroupOutputOutputSettingsUdpOutputSettingsContainerSettingsM2tsSettingsDvbNitSettings�aws:medialive/ChannelEncoderSettingsOutputGroupOutputOutputSettingsUdpOutputSettingsContainerSettingsM2tsSettingsDvbNitSettings:ChannelEncoderSettingsOutputGroupOutputOutputSettingsUdpOutputSettingsContainerSettingsM2tsSettingsDvbNitSettings�
dvbSdtSettings�B�:�
�
	medialiveqChannelEncoderSettingsOutputGroupOutputOutputSettingsUdpOutputSettingsContainerSettingsM2tsSettingsDvbSdtSettings�aws:medialive/ChannelEncoderSettingsOutputGroupOutputOutputSettingsUdpOutputSettingsContainerSettingsM2tsSettingsDvbSdtSettings:ChannelEncoderSettingsOutputGroupOutputOutputSettingsUdpOutputSettingsContainerSettingsM2tsSettingsDvbSdtSettings

dvbSubPidsB" �
dvbTdtSettings�B�:�
�
	medialiveqChannelEncoderSettingsOutputGroupOutputOutputSettingsUdpOutputSettingsContainerSettingsM2tsSettingsDvbTdtSettings�aws:medialive/ChannelEncoderSettingsOutputGroupOutputOutputSettingsUdpOutputSettingsContainerSettingsM2tsSettingsDvbTdtSettings:ChannelEncoderSettingsOutputGroupOutputOutputSettingsUdpOutputSettingsContainerSettingsM2tsSettingsDvbTdtSettings
dvbTeletextPidB" 
ebifB" 
ebpAudioIntervalB" 
ebpLookaheadMsB 
ebpPlacementB" 
ecmPidB" 
esRateInPesB" 
etvPlatformPidB" 
etvSignalPidB" 
fragmentTimeB 
klvB" 
klvDataPidsB" 
nielsenId3BehaviorB" 
nullPacketBitrateB 
patIntervalB 

pcrControlB" 
	pcrPeriodB 
pcrPidB" 
pmtIntervalB 
pmtPidB" 

programNumB 
rateModeB" 

scte27PidsB" 
scte35ControlB" <
	scte35PidB" )PID from which to read SCTE-35 messages.

segmentationMarkersB" 
segmentationStyleB" 
segmentationTimeB 
timedMetadataBehaviorB" 
timedMetadataPidB" 
transportStreamIdB 
videoPidB" :�
�
	medialiveqChannelEncoderSettingsOutputGroupOutputOutputSettingsUdpOutputSettingsContainerSettingsM2tsSettingsDvbNitSettings�aws:medialive/ChannelEncoderSettingsOutputGroupOutputOutputSettingsUdpOutputSettingsContainerSettingsM2tsSettingsDvbNitSettings:ChannelEncoderSettingsOutputGroupOutputOutputSettingsUdpOutputSettingsContainerSettingsM2tsSettingsDvbNitSettings;
9
	networkId 
networkName" 
repIntervalB :�
�
	medialiveqChannelEncoderSettingsOutputGroupOutputOutputSettingsUdpOutputSettingsContainerSettingsM2tsSettingsDvbSdtSettings�aws:medialive/ChannelEncoderSettingsOutputGroupOutputOutputSettingsUdpOutputSettingsContainerSettingsM2tsSettingsDvbSdtSettings:ChannelEncoderSettingsOutputGroupOutputOutputSettingsUdpOutputSettingsContainerSettingsM2tsSettingsDvbSdtSettings\
Z
	outputSdtB" 
repIntervalB 
serviceNameB" 
serviceProviderNameB" :�
�
	medialiveqChannelEncoderSettingsOutputGroupOutputOutputSettingsUdpOutputSettingsContainerSettingsM2tsSettingsDvbTdtSettings�aws:medialive/ChannelEncoderSettingsOutputGroupOutputOutputSettingsUdpOutputSettingsContainerSettingsM2tsSettingsDvbTdtSettings:ChannelEncoderSettingsOutputGroupOutputOutputSettingsUdpOutputSettingsContainerSettingsM2tsSettingsDvbTdtSettings

repIntervalB :�
�
	medialiveQChannelEncoderSettingsOutputGroupOutputOutputSettingsUdpOutputSettingsDestination�aws:medialive/ChannelEncoderSettingsOutputGroupOutputOutputSettingsUdpOutputSettingsDestination:ChannelEncoderSettingsOutputGroupOutputOutputSettingsUdpOutputSettingsDestination>
<:
destinationRefId" "Reference ID for the destination.
:�
�
	medialiveWChannelEncoderSettingsOutputGroupOutputOutputSettingsUdpOutputSettingsFecOutputSettings�aws:medialive/ChannelEncoderSettingsOutputGroupOutputOutputSettingsUdpOutputSettingsFecOutputSettings:ChannelEncoderSettingsOutputGroupOutputOutputSettingsUdpOutputSettingsFecOutputSettings�
�>
columnDepthB )The height of the FEC protection matrix.
E

includeFecB" 1Enables column only or column and row based FEC.
;
	rowLengthB (The width of the FEC protection matrix.
:�
�
	medialive$ChannelEncoderSettingsTimecodeConfigWaws:medialive/ChannelEncoderSettingsTimecodeConfig:ChannelEncoderSettingsTimecodeConfig�
�[
source" MThe source for the timecode that will be associated with the events outputs.
q
syncThresholdB ZThreshold in frames beyond which output timecode is resynchronized to the input timecode.
:�
�
	medialive&ChannelEncoderSettingsVideoDescription[aws:medialive/ChannelEncoderSettingsVideoDescription:ChannelEncoderSettingsVideoDescription�
��
codecSettings�B�:�
�
	medialive3ChannelEncoderSettingsVideoDescriptionCodecSettingsuaws:medialive/ChannelEncoderSettingsVideoDescriptionCodecSettings:ChannelEncoderSettingsVideoDescriptionCodecSettingsEThe video codec settings. See Video Codec Settings for more details.
/
heightB Output video height in pixels.
/
name" #The name of the video description.
b
respondToAfdB" LIndicate how to respond to the AFD values that might be in the input video.
3
scalingBehaviorB" Behavior on how to scale.
S
	sharpnessB @Changes the strength of the anti-alias filter used for scaling.
-
widthB Output video width in pixels.
:�
�
	medialive3ChannelEncoderSettingsVideoDescriptionCodecSettingsuaws:medialive/ChannelEncoderSettingsVideoDescriptionCodecSettings:ChannelEncoderSettingsVideoDescriptionCodecSettings�
��
frameCaptureSettings�B�:�
�
	medialiveGChannelEncoderSettingsVideoDescriptionCodecSettingsFrameCaptureSettings�aws:medialive/ChannelEncoderSettingsVideoDescriptionCodecSettingsFrameCaptureSettings:ChannelEncoderSettingsVideoDescriptionCodecSettingsFrameCaptureSettingsEFrame capture settings. See Frame Capture Settings for more details.
�
h264Settings�B�:�
�
	medialive?ChannelEncoderSettingsVideoDescriptionCodecSettingsH264Settings�aws:medialive/ChannelEncoderSettingsVideoDescriptionCodecSettingsH264Settings:ChannelEncoderSettingsVideoDescriptionCodecSettingsH264Settings3H264 settings. See H264 Settings for more details.
�
h265Settings�B�:�
�
	medialive?ChannelEncoderSettingsVideoDescriptionCodecSettingsH265Settings�aws:medialive/ChannelEncoderSettingsVideoDescriptionCodecSettingsH265Settings:ChannelEncoderSettingsVideoDescriptionCodecSettingsH265Settings:�
�
	medialiveGChannelEncoderSettingsVideoDescriptionCodecSettingsFrameCaptureSettings�aws:medialive/ChannelEncoderSettingsVideoDescriptionCodecSettingsFrameCaptureSettings:ChannelEncoderSettingsVideoDescriptionCodecSettingsFrameCaptureSettings�
�_
captureIntervalB FThe frequency at which to capture frames for inclusion in the output.
C
captureIntervalUnitsB" %Unit for the frame capture interval.
:�
�
	medialive?ChannelEncoderSettingsVideoDescriptionCodecSettingsH264Settings�aws:medialive/ChannelEncoderSettingsVideoDescriptionCodecSettingsH264Settings:ChannelEncoderSettingsVideoDescriptionCodecSettingsH264Settings�
�I
adaptiveQuantizationB" +Enables or disables adaptive quantization.
X
afdSignalingB" BIndicates that AFD values will be written into the output stream.
1
bitrateB  Average bitrate in bits/second.


bufFillPctB )
bufSizeB Size of buffer in bits.
D
colorMetadataB" -Includes color space metadata in the output.
0
entropyEncodingB" Entropy encoding mode.
�
filterSettings�B�:�
�
	medialiveMChannelEncoderSettingsVideoDescriptionCodecSettingsH264SettingsFilterSettings�aws:medialive/ChannelEncoderSettingsVideoDescriptionCodecSettingsH264SettingsFilterSettings:ChannelEncoderSettingsVideoDescriptionCodecSettingsH264SettingsFilterSettingsJFilters to apply to an encode. See H264 Filter Settings for more details.
[
fixedAfdB" IFour bit AFD value to write on all frames of video in the output stream.

	flickerAqB" g
forceFieldPicturesB" KControls whether coding is performed on a field basis or on a frame basis.
R
framerateControlB" 8Indicates how the output video frame rate is specified.
5
framerateDenominatorB Framerate denominator.
1
framerateNumeratorB Framerate numerator.
(
gopBReferenceB" GOP-B reference.
4
gopClosedCadenceB Frequency of closed GOPs.
D
gopNumBFramesB -Number of B-frames between reference frames.
U
gopSizeB DGOP size in units of either frames of seconds per `gop_size_units`.
U
gopSizeUnitsB" ?Indicates if the `gop_size` is specified in frames or seconds.

levelB" H264 level.
3
lookAheadRateControlB" Amount of lookahead.
t

maxBitrateB `Set the maximum bitrate in order to accommodate expected spikes in the complexity of the video.
$
minIIntervalB Min interval.
9
numRefFramesB #Number of reference frames to use.
N

parControlB" :Indicates how the output pixel aspect ratio is specified.
8
parDenominatorB  Pixel Aspect Ratio denominator.
4
parNumeratorB Pixel Aspect Ratio numerator.

profileB" H264 profile.
%
qualityLevelB" Quality level.
L
qvbrQualityLevelB 2Controls the target quality for the video encode.
,
rateControlModeB" Rate control mode.
4
scanTypeB" "Sets the scan type of the output.
3
sceneChangeDetectB" Scene change detection.
.
slicesB Number of slices per picture.

softnessB 
Softness.
i
	spatialAqB" VMakes adjustments within each frame based on spatial variation of content complexity.
%
subgopLengthB" Subgop length.
C
syntaxB" 3Produces a bitstream compliant with SMPTE RP-2027.
k

temporalAqB" WMakes adjustments within each frame based on temporal variation of content complexity.
i
timecodeInsertionB" NDetermines how timecodes should be inserted into the video elementary stream.
:�
�
	medialiveMChannelEncoderSettingsVideoDescriptionCodecSettingsH264SettingsFilterSettings�aws:medialive/ChannelEncoderSettingsVideoDescriptionCodecSettingsH264SettingsFilterSettings:ChannelEncoderSettingsVideoDescriptionCodecSettingsH264SettingsFilterSettings�
��
temporalFilterSettings�B�:�
�
	medialivecChannelEncoderSettingsVideoDescriptionCodecSettingsH264SettingsFilterSettingsTemporalFilterSettings�aws:medialive/ChannelEncoderSettingsVideoDescriptionCodecSettingsH264SettingsFilterSettingsTemporalFilterSettings:ChannelEncoderSettingsVideoDescriptionCodecSettingsH264SettingsFilterSettingsTemporalFilterSettings:�
�
	medialivecChannelEncoderSettingsVideoDescriptionCodecSettingsH264SettingsFilterSettingsTemporalFilterSettings�aws:medialive/ChannelEncoderSettingsVideoDescriptionCodecSettingsH264SettingsFilterSettingsTemporalFilterSettings:ChannelEncoderSettingsVideoDescriptionCodecSettingsH264SettingsFilterSettingsTemporalFilterSettings_
]6
postFilterSharpeningB" Post filter sharpening.
#
strengthB" Filter strength.
:�
�
	medialive?ChannelEncoderSettingsVideoDescriptionCodecSettingsH265Settings�aws:medialive/ChannelEncoderSettingsVideoDescriptionCodecSettingsH265Settings:ChannelEncoderSettingsVideoDescriptionCodecSettingsH265Settings�
�I
adaptiveQuantizationB" +Enables or disables adaptive quantization.
X
afdSignalingB" BIndicates that AFD values will be written into the output stream.
t
alternativeTransferFunctionB" OWhether or not EML should insert an Alternative Transfer Function SEI message.
/
bitrate  Average bitrate in bits/second.
)
bufSizeB Size of buffer in bits.
D
colorMetadataB" -Includes color space metadata in the output.
�
colorSpaceSettings�B�:�
�
	medialiveQChannelEncoderSettingsVideoDescriptionCodecSettingsH265SettingsColorSpaceSettings�aws:medialive/ChannelEncoderSettingsVideoDescriptionCodecSettingsH265SettingsColorSpaceSettings:ChannelEncoderSettingsVideoDescriptionCodecSettingsH265SettingsColorSpaceSettingsVDefine the color metadata for the output. H265 Color Space Settings for more details.
�
filterSettings�B�:�
�
	medialiveMChannelEncoderSettingsVideoDescriptionCodecSettingsH265SettingsFilterSettings�aws:medialive/ChannelEncoderSettingsVideoDescriptionCodecSettingsH265SettingsFilterSettings:ChannelEncoderSettingsVideoDescriptionCodecSettingsH265SettingsFilterSettingsJFilters to apply to an encode. See H265 Filter Settings for more details.
[
fixedAfdB" IFour bit AFD value to write on all frames of video in the output stream.

	flickerAqB" 3
framerateDenominator Framerate denominator.
/
framerateNumerator Framerate numerator.
4
gopClosedCadenceB Frequency of closed GOPs.
U
gopSizeB DGOP size in units of either frames of seconds per `gop_size_units`.
U
gopSizeUnitsB" ?Indicates if the `gop_size` is specified in frames or seconds.

levelB" H265 level.
3
lookAheadRateControlB" Amount of lookahead.
t

maxBitrateB `Set the maximum bitrate in order to accommodate expected spikes in the complexity of the video.
$
minIIntervalB Min interval.
#
minQpB Set the minimum QP.
\
mvOverPictureBoundariesB" ;Enables or disables motion vector over picture boundaries.
W
mvTemporalPredictorB" :Enables or disables the motion vector temporal predictor.
8
parDenominatorB  Pixel Aspect Ratio denominator.
4
parNumeratorB Pixel Aspect Ratio numerator.

profileB" H265 profile.
L
qvbrQualityLevelB 2Controls the target quality for the video encode.
,
rateControlModeB" Rate control mode.
4
scanTypeB" "Sets the scan type of the output.
3
sceneChangeDetectB" Scene change detection.
.
slicesB Number of slices per picture.
/
tierB" !Set the H265 tier in the output.
.

tileHeightB Sets the height of tiles.
;
tilePaddingB" &Enables or disables padding of tiles.
,
	tileWidthB Sets the width of tiles.
�
timecodeBurninSettings�B�:�
�
	medialiveUChannelEncoderSettingsVideoDescriptionCodecSettingsH265SettingsTimecodeBurninSettings�aws:medialive/ChannelEncoderSettingsVideoDescriptionCodecSettingsH265SettingsTimecodeBurninSettings:ChannelEncoderSettingsVideoDescriptionCodecSettingsH265SettingsTimecodeBurninSettingsPApply a burned in timecode. See H265 Timecode Burnin Settings for more details.
i
timecodeInsertionB" NDetermines how timecodes should be inserted into the video elementary stream.
7
treeblockSizeB"  Sets the size of the treeblock.
:�
�
	medialiveQChannelEncoderSettingsVideoDescriptionCodecSettingsH265SettingsColorSpaceSettings�aws:medialive/ChannelEncoderSettingsVideoDescriptionCodecSettingsH265SettingsColorSpaceSettings:ChannelEncoderSettingsVideoDescriptionCodecSettingsH265SettingsColorSpaceSettings�
��
colorSpacePassthroughSettings�B�:�
�
	medialivenChannelEncoderSettingsVideoDescriptionCodecSettingsH265SettingsColorSpaceSettingsColorSpacePassthroughSettings�aws:medialive/ChannelEncoderSettingsVideoDescriptionCodecSettingsH265SettingsColorSpaceSettingsColorSpacePassthroughSettings:ChannelEncoderSettingsVideoDescriptionCodecSettingsH265SettingsColorSpaceSettingsColorSpacePassthroughSettings3Sets the colorspace metadata to be passed through.
�
dolbyVision81Settings�B�:�
�
	medialivefChannelEncoderSettingsVideoDescriptionCodecSettingsH265SettingsColorSpaceSettingsDolbyVision81Settings�aws:medialive/ChannelEncoderSettingsVideoDescriptionCodecSettingsH265SettingsColorSpaceSettingsDolbyVision81Settings:ChannelEncoderSettingsVideoDescriptionCodecSettingsH265SettingsColorSpaceSettingsDolbyVision81Settings&Set the colorspace to Dolby Vision81.
�
hdr10Settings�B�:�
�
	medialive^ChannelEncoderSettingsVideoDescriptionCodecSettingsH265SettingsColorSpaceSettingsHdr10Settings�aws:medialive/ChannelEncoderSettingsVideoDescriptionCodecSettingsH265SettingsColorSpaceSettingsHdr10Settings:ChannelEncoderSettingsVideoDescriptionCodecSettingsH265SettingsColorSpaceSettingsHdr10SettingsJSet the colorspace to be HDR10. See H265 HDR10 Settings for more details.
�
rec601Settings�B�:�
�
	medialive_ChannelEncoderSettingsVideoDescriptionCodecSettingsH265SettingsColorSpaceSettingsRec601Settings�aws:medialive/ChannelEncoderSettingsVideoDescriptionCodecSettingsH265SettingsColorSpaceSettingsRec601Settings:ChannelEncoderSettingsVideoDescriptionCodecSettingsH265SettingsColorSpaceSettingsRec601Settings Set the colorspace to Rec. 601.
�
rec709Settings�B�:�
�
	medialive_ChannelEncoderSettingsVideoDescriptionCodecSettingsH265SettingsColorSpaceSettingsRec709Settings�aws:medialive/ChannelEncoderSettingsVideoDescriptionCodecSettingsH265SettingsColorSpaceSettingsRec709Settings:ChannelEncoderSettingsVideoDescriptionCodecSettingsH265SettingsColorSpaceSettingsRec709Settings Set the colorspace to Rec. 709.
:�
�
	medialivenChannelEncoderSettingsVideoDescriptionCodecSettingsH265SettingsColorSpaceSettingsColorSpacePassthroughSettings�aws:medialive/ChannelEncoderSettingsVideoDescriptionCodecSettingsH265SettingsColorSpaceSettingsColorSpacePassthroughSettings:ChannelEncoderSettingsVideoDescriptionCodecSettingsH265SettingsColorSpaceSettingsColorSpacePassthroughSettings
 :�
�
	medialivefChannelEncoderSettingsVideoDescriptionCodecSettingsH265SettingsColorSpaceSettingsDolbyVision81Settings�aws:medialive/ChannelEncoderSettingsVideoDescriptionCodecSettingsH265SettingsColorSpaceSettingsDolbyVision81Settings:ChannelEncoderSettingsVideoDescriptionCodecSettingsH265SettingsColorSpaceSettingsDolbyVision81Settings
 :�
�
	medialive^ChannelEncoderSettingsVideoDescriptionCodecSettingsH265SettingsColorSpaceSettingsHdr10Settings�aws:medialive/ChannelEncoderSettingsVideoDescriptionCodecSettingsH265SettingsColorSpaceSettingsHdr10Settings:ChannelEncoderSettingsVideoDescriptionCodecSettingsH265SettingsColorSpaceSettingsHdr10Settingsj
h1
maxCllB !Sets the MaxCLL value for HDR10.
3
maxFallB "Sets the MaxFALL value for HDR10.
:�
�
	medialive_ChannelEncoderSettingsVideoDescriptionCodecSettingsH265SettingsColorSpaceSettingsRec601Settings�aws:medialive/ChannelEncoderSettingsVideoDescriptionCodecSettingsH265SettingsColorSpaceSettingsRec601Settings:ChannelEncoderSettingsVideoDescriptionCodecSettingsH265SettingsColorSpaceSettingsRec601Settings
 :�
�
	medialive_ChannelEncoderSettingsVideoDescriptionCodecSettingsH265SettingsColorSpaceSettingsRec709Settings�aws:medialive/ChannelEncoderSettingsVideoDescriptionCodecSettingsH265SettingsColorSpaceSettingsRec709Settings:ChannelEncoderSettingsVideoDescriptionCodecSettingsH265SettingsColorSpaceSettingsRec709Settings
 :�
�
	medialiveMChannelEncoderSettingsVideoDescriptionCodecSettingsH265SettingsFilterSettings�aws:medialive/ChannelEncoderSettingsVideoDescriptionCodecSettingsH265SettingsFilterSettings:ChannelEncoderSettingsVideoDescriptionCodecSettingsH265SettingsFilterSettings�
��
temporalFilterSettings�B�:�
�
	medialivecChannelEncoderSettingsVideoDescriptionCodecSettingsH265SettingsFilterSettingsTemporalFilterSettings�aws:medialive/ChannelEncoderSettingsVideoDescriptionCodecSettingsH265SettingsFilterSettingsTemporalFilterSettings:ChannelEncoderSettingsVideoDescriptionCodecSettingsH265SettingsFilterSettingsTemporalFilterSettings:�
�
	medialivecChannelEncoderSettingsVideoDescriptionCodecSettingsH265SettingsFilterSettingsTemporalFilterSettings�aws:medialive/ChannelEncoderSettingsVideoDescriptionCodecSettingsH265SettingsFilterSettingsTemporalFilterSettings:ChannelEncoderSettingsVideoDescriptionCodecSettingsH265SettingsFilterSettingsTemporalFilterSettings_
]6
postFilterSharpeningB" Post filter sharpening.
#
strengthB" Filter strength.
:�
�
	medialiveUChannelEncoderSettingsVideoDescriptionCodecSettingsH265SettingsTimecodeBurninSettings�aws:medialive/ChannelEncoderSettingsVideoDescriptionCodecSettingsH265SettingsTimecodeBurninSettings:ChannelEncoderSettingsVideoDescriptionCodecSettingsH265SettingsTimecodeBurninSettings�
�8
prefixB" (Set a prefix on the burned in timecode.
I
timecodeBurninFontSizeB" )Sets the size of the burned in timecode.
M
timecodeBurninPositionB" -Sets the position of the burned in timecode.
:�
`
	medialiveChannelInputAttachment;aws:medialive/ChannelInputAttachment:ChannelInputAttachment�
��
automaticInputFailoverSettings�B�:�
�
	medialive4ChannelInputAttachmentAutomaticInputFailoverSettingswaws:medialive/ChannelInputAttachmentAutomaticInputFailoverSettings:ChannelInputAttachmentAutomaticInputFailoverSettings�User-specified settings for defining what the conditions are for declaring the input unhealthy and failing over to a different input. See Automatic Input Failover Settings for more details.
C
inputAttachmentName" (User-specified name for the attachment.
$
inputId" The ID of the input.
�
inputSettings�B�:�
�
	medialive#ChannelInputAttachmentInputSettingsUaws:medialive/ChannelInputAttachmentInputSettings:ChannelInputAttachmentInputSettings;Settings of an input. See Input Settings for more details.
:�

�
	medialive4ChannelInputAttachmentAutomaticInputFailoverSettingswaws:medialive/ChannelInputAttachmentAutomaticInputFailoverSettings:ChannelInputAttachmentAutomaticInputFailoverSettings�
��
errorClearTimeMsecB �This clear time defines the requirement a recovered input must meet to be considered healthy. The input must have no failover conditions for this length of time. Enter a time in milliseconds. This value is particularly important if the input\_preference for the failover pair is set to PRIMARY\_INPUT\_PREFERRED, because after this time, MediaLive will switch back to the primary input.
�
failoverConditions�B�*�:�
�
	medialiveEChannelInputAttachmentAutomaticInputFailoverSettingsFailoverCondition�aws:medialive/ChannelInputAttachmentAutomaticInputFailoverSettingsFailoverCondition:ChannelInputAttachmentAutomaticInputFailoverSettingsFailoverCondition�A list of failover conditions. If any of these conditions occur, MediaLive will perform a failover to the other input. See Failover Condition Block for more details.
�
inputPreferenceB" hInput preference when deciding which input to make active when a previously failed input has recovered.
b
secondaryInputId" JThe input ID of the secondary input in the automatic input failover pair.
:�
�
	medialiveEChannelInputAttachmentAutomaticInputFailoverSettingsFailoverCondition�aws:medialive/ChannelInputAttachmentAutomaticInputFailoverSettingsFailoverCondition:ChannelInputAttachmentAutomaticInputFailoverSettingsFailoverCondition�
��
failoverConditionSettings�B�:�
�
	medialive^ChannelInputAttachmentAutomaticInputFailoverSettingsFailoverConditionFailoverConditionSettings�aws:medialive/ChannelInputAttachmentAutomaticInputFailoverSettingsFailoverConditionFailoverConditionSettings:ChannelInputAttachmentAutomaticInputFailoverSettingsFailoverConditionFailoverConditionSettings]Failover condition type-specific settings. See Failover Condition Settings for more details.
:�
�
	medialive^ChannelInputAttachmentAutomaticInputFailoverSettingsFailoverConditionFailoverConditionSettings�aws:medialive/ChannelInputAttachmentAutomaticInputFailoverSettingsFailoverConditionFailoverConditionSettings:ChannelInputAttachmentAutomaticInputFailoverSettingsFailoverConditionFailoverConditionSettings�
��
audioSilenceSettings�B�:�
�
	medialiverChannelInputAttachmentAutomaticInputFailoverSettingsFailoverConditionFailoverConditionSettingsAudioSilenceSettings�aws:medialive/ChannelInputAttachmentAutomaticInputFailoverSettingsFailoverConditionFailoverConditionSettingsAudioSilenceSettings:ChannelInputAttachmentAutomaticInputFailoverSettingsFailoverConditionFailoverConditionSettingsAudioSilenceSettings�MediaLive will perform a failover if the specified audio selector is silent for the specified period. See Audio Silence Failover Settings for more details.
�
inputLossSettings�B�:�
�
	medialiveoChannelInputAttachmentAutomaticInputFailoverSettingsFailoverConditionFailoverConditionSettingsInputLossSettings�aws:medialive/ChannelInputAttachmentAutomaticInputFailoverSettingsFailoverConditionFailoverConditionSettingsInputLossSettings:ChannelInputAttachmentAutomaticInputFailoverSettingsFailoverConditionFailoverConditionSettingsInputLossSettings�MediaLive will perform a failover if content is not detected in this input for the specified period. See Input Loss Failover Settings for more details.
�
videoBlackSettings�B�:�
�
	medialivepChannelInputAttachmentAutomaticInputFailoverSettingsFailoverConditionFailoverConditionSettingsVideoBlackSettings�aws:medialive/ChannelInputAttachmentAutomaticInputFailoverSettingsFailoverConditionFailoverConditionSettingsVideoBlackSettings:ChannelInputAttachmentAutomaticInputFailoverSettingsFailoverConditionFailoverConditionSettingsVideoBlackSettings�MediaLive will perform a failover if content is considered black for the specified period. See Video Black Failover Settings for more details.
:�
�
	medialiverChannelInputAttachmentAutomaticInputFailoverSettingsFailoverConditionFailoverConditionSettingsAudioSilenceSettings�aws:medialive/ChannelInputAttachmentAutomaticInputFailoverSettingsFailoverConditionFailoverConditionSettingsAudioSilenceSettings:ChannelInputAttachmentAutomaticInputFailoverSettingsFailoverConditionFailoverConditionSettingsAudioSilenceSettings�
�
audioSelectorName" �
audioSilenceThresholdMsecB �The amount of time (in milliseconds) that the active input must be silent before automatic input failover occurs. Silence is defined as audio loss or audio quieter than -50 dBFS.
:�
�
	medialiveoChannelInputAttachmentAutomaticInputFailoverSettingsFailoverConditionFailoverConditionSettingsInputLossSettings�aws:medialive/ChannelInputAttachmentAutomaticInputFailoverSettingsFailoverConditionFailoverConditionSettingsInputLossSettings:ChannelInputAttachmentAutomaticInputFailoverSettingsFailoverConditionFailoverConditionSettingsInputLossSettings�
��
inputLossThresholdMsecB oThe amount of time (in milliseconds) that no input is detected. After that time, an input failover will occur.
:�	
�
	medialivepChannelInputAttachmentAutomaticInputFailoverSettingsFailoverConditionFailoverConditionSettingsVideoBlackSettings�aws:medialive/ChannelInputAttachmentAutomaticInputFailoverSettingsFailoverConditionFailoverConditionSettingsVideoBlackSettings:ChannelInputAttachmentAutomaticInputFailoverSettingsFailoverConditionFailoverConditionSettingsVideoBlackSettings�
��
blackDetectThresholdB �A value used in calculating the threshold below which MediaLive considers a pixel to be 'black'. For the input to be considered black, every pixel in a frame must be below this threshold. The threshold is calculated as a percentage (expressed as a decimal) of white. Therefore .1 means 10% white (or 90% black). Note how the formula works for any color depth. For example, if you set this field to 0.1 in 10-bit color depth: (10230.1=102.3), which means a pixel value of 102 or less is 'black'. If you set this field to .1 in an 8-bit color depth: (2550.1=25.5), which means a pixel value of 25 or less is 'black'. The range is 0.0 to 1.0, with any number of decimal places.
�
videoBlackThresholdMsecB qThe amount of time (in milliseconds) that the active input must be black before automatic input failover occurs.
:�
�
	medialive#ChannelInputAttachmentInputSettingsUaws:medialive/ChannelInputAttachmentInputSettings:ChannelInputAttachmentInputSettings�
��
audioSelectors�B�*�:�
�
	medialive0ChannelInputAttachmentInputSettingsAudioSelectoroaws:medialive/ChannelInputAttachmentInputSettingsAudioSelector:ChannelInputAttachmentInputSettingsAudioSelectoroUsed to select the audio stream to decode for inputs that have multiple. See Audio Selectors for more details.
�
captionSelectors�B�*�:�
�
	medialive2ChannelInputAttachmentInputSettingsCaptionSelectorsaws:medialive/ChannelInputAttachmentInputSettingsCaptionSelector:ChannelInputAttachmentInputSettingsCaptionSelectoryUsed to select the caption input to use for inputs that have multiple available. See Caption Selectors for more details.
L
deblockFilterB" 5Enable or disable the deblock filter when filtering.
L
denoiseFilterB" 5Enable or disable the denoise filter when filtering.
^
filterStrengthB FAdjusts the magnitude of filtering from 1 (minimal) to 5 (strongest).
8
inputFilterB" #Turns on the filter for the input.
�
networkInputSettings�B�:�
�
	medialive7ChannelInputAttachmentInputSettingsNetworkInputSettings}aws:medialive/ChannelInputAttachmentInputSettingsNetworkInputSettings:ChannelInputAttachmentInputSettingsNetworkInputSettings=Input settings. See Network Input Settings for more details.
<
	scte35PidB )PID from which to read SCTE-35 messages.

smpte2038DataPreferenceB" ^Specifies whether to extract applicable ancillary data from a SMPTE-2038 source in the input.
7
sourceEndBehaviorB" Loop input if it is a file.
�
videoSelector�B�:�
�
	medialive0ChannelInputAttachmentInputSettingsVideoSelectoroaws:medialive/ChannelInputAttachmentInputSettingsVideoSelector:ChannelInputAttachmentInputSettingsVideoSelector:�
�
	medialive0ChannelInputAttachmentInputSettingsAudioSelectoroaws:medialive/ChannelInputAttachmentInputSettingsAudioSelector:ChannelInputAttachmentInputSettingsAudioSelector�
�H
name" <Name of the Channel.

The following arguments are optional:
�
selectorSettings�B�:�
�
	medialive@ChannelInputAttachmentInputSettingsAudioSelectorSelectorSettings�aws:medialive/ChannelInputAttachmentInputSettingsAudioSelectorSelectorSettings:ChannelInputAttachmentInputSettingsAudioSelectorSelectorSettings:�
�
	medialive@ChannelInputAttachmentInputSettingsAudioSelectorSelectorSettings�aws:medialive/ChannelInputAttachmentInputSettingsAudioSelectorSelectorSettings:ChannelInputAttachmentInputSettingsAudioSelectorSelectorSettings�
��
audioHlsRenditionSelection�B�:�
�
	medialiveZChannelInputAttachmentInputSettingsAudioSelectorSelectorSettingsAudioHlsRenditionSelection�aws:medialive/ChannelInputAttachmentInputSettingsAudioSelectorSelectorSettingsAudioHlsRenditionSelection:ChannelInputAttachmentInputSettingsAudioSelectorSelectorSettingsAudioHlsRenditionSelectionSAudio HLS Rendition Selection. See Audio HLS Rendition Selection for more details.
�
audioLanguageSelection�B�:�
�
	medialiveVChannelInputAttachmentInputSettingsAudioSelectorSelectorSettingsAudioLanguageSelection�aws:medialive/ChannelInputAttachmentInputSettingsAudioSelectorSelectorSettingsAudioLanguageSelection:ChannelInputAttachmentInputSettingsAudioSelectorSelectorSettingsAudioLanguageSelectionIAudio Language Selection. See Audio Language Selection for more details.
�
audioPidSelection�B�:�
�
	medialiveQChannelInputAttachmentInputSettingsAudioSelectorSelectorSettingsAudioPidSelection�aws:medialive/ChannelInputAttachmentInputSettingsAudioSelectorSelectorSettingsAudioPidSelection:ChannelInputAttachmentInputSettingsAudioSelectorSelectorSettingsAudioPidSelection?Audio Pid Selection. See Audio PID Selection for more details.
�
audioTrackSelection�B�:�
�
	medialiveSChannelInputAttachmentInputSettingsAudioSelectorSelectorSettingsAudioTrackSelection�aws:medialive/ChannelInputAttachmentInputSettingsAudioSelectorSelectorSettingsAudioTrackSelection:ChannelInputAttachmentInputSettingsAudioSelectorSelectorSettingsAudioTrackSelectionCAudio Track Selection. See Audio Track Selection for more details.
:�
�
	medialiveZChannelInputAttachmentInputSettingsAudioSelectorSelectorSettingsAudioHlsRenditionSelection�aws:medialive/ChannelInputAttachmentInputSettingsAudioSelectorSelectorSettingsAudioHlsRenditionSelection:ChannelInputAttachmentInputSettingsAudioSelectorSelectorSettingsAudioHlsRenditionSelection�
�a
groupId" RSpecifies the GROUP-ID in the #EXT-X-MEDIA tag of the target HLS audio rendition.
Z
name" NSpecifies the NAME in the #EXT-X-MEDIA tag of the target HLS audio rendition.
:�
�
	medialiveVChannelInputAttachmentInputSettingsAudioSelectorSelectorSettingsAudioLanguageSelection�aws:medialive/ChannelInputAttachmentInputSettingsAudioSelectorSelectorSettingsAudioLanguageSelection:ChannelInputAttachmentInputSettingsAudioSelectorSelectorSettingsAudioLanguageSelection�
�_
languageCode" KSelects a specific three-letter language code from within an audio source.
�
languageSelectionPolicyB" �When set to “strict”, the transport stream demux strictly identifies audio streams by their language descriptor. If a PMT update occurs such that an audio stream matching the initially selected language is no longer present then mute will be encoded until the language returns. If “loose”, then on a PMT update the demux will choose another audio stream in the program with the same stream type if it can’t find one with the same language.
:�
�
	medialiveQChannelInputAttachmentInputSettingsAudioSelectorSelectorSettingsAudioPidSelection�aws:medialive/ChannelInputAttachmentInputSettingsAudioSelectorSelectorSettingsAudioPidSelection:ChannelInputAttachmentInputSettingsAudioSelectorSelectorSettingsAudioPidSelection<
:8
pid -Selects a specific PID from within a source.
:�	
�
	medialiveSChannelInputAttachmentInputSettingsAudioSelectorSelectorSettingsAudioTrackSelection�aws:medialive/ChannelInputAttachmentInputSettingsAudioSelectorSelectorSettingsAudioTrackSelection:ChannelInputAttachmentInputSettingsAudioSelectorSelectorSettingsAudioTrackSelection�
��
dolbyEDecode�B�:�
�
	medialive_ChannelInputAttachmentInputSettingsAudioSelectorSelectorSettingsAudioTrackSelectionDolbyEDecode�aws:medialive/ChannelInputAttachmentInputSettingsAudioSelectorSelectorSettingsAudioTrackSelectionDolbyEDecode:ChannelInputAttachmentInputSettingsAudioSelectorSelectorSettingsAudioTrackSelectionDolbyEDecode�Configure decoding options for Dolby E streams - these should be Dolby E frames carried in PCM streams tagged with SMPTE-337. See Dolby E Decode for more details.
�
tracks�*�:�
�
	medialiveXChannelInputAttachmentInputSettingsAudioSelectorSelectorSettingsAudioTrackSelectionTrack�aws:medialive/ChannelInputAttachmentInputSettingsAudioSelectorSelectorSettingsAudioTrackSelectionTrack:ChannelInputAttachmentInputSettingsAudioSelectorSelectorSettingsAudioTrackSelectionTrackaSelects one or more unique audio tracks from within a source. See Audio Tracks for more details.
:�
�
	medialive_ChannelInputAttachmentInputSettingsAudioSelectorSelectorSettingsAudioTrackSelectionDolbyEDecode�aws:medialive/ChannelInputAttachmentInputSettingsAudioSelectorSelectorSettingsAudioTrackSelectionDolbyEDecode:ChannelInputAttachmentInputSettingsAudioSelectorSelectorSettingsAudioTrackSelectionDolbyEDecode�
��
programSelection" �Applies only to Dolby E. Enter the program ID (according to the metadata in the audio) of the Dolby E program to extract from the specified track. One program extracted per audio selector. To select multiple programs, create multiple selectors with the same Track and different Program numbers. “All channels” means to ignore the program IDs and include all the channels in this selector; useful if metadata is known to be incorrect.
:�
�
	medialiveXChannelInputAttachmentInputSettingsAudioSelectorSelectorSettingsAudioTrackSelectionTrack�aws:medialive/ChannelInputAttachmentInputSettingsAudioSelectorSelectorSettingsAudioTrackSelectionTrack:ChannelInputAttachmentInputSettingsAudioSelectorSelectorSettingsAudioTrackSelectionTrackL
JH
track ;1-based integer value that maps to a specific audio track.
:�
�
	medialive2ChannelInputAttachmentInputSettingsCaptionSelectorsaws:medialive/ChannelInputAttachmentInputSettingsCaptionSelector:ChannelInputAttachmentInputSettingsCaptionSelector�
�
languageCodeB" H
name" <Name of the Channel.

The following arguments are optional:
�
selectorSettings�B�:�
�
	medialiveBChannelInputAttachmentInputSettingsCaptionSelectorSelectorSettings�aws:medialive/ChannelInputAttachmentInputSettingsCaptionSelectorSelectorSettings:ChannelInputAttachmentInputSettingsCaptionSelectorSelectorSettings:�
�
	medialiveBChannelInputAttachmentInputSettingsCaptionSelectorSelectorSettings�aws:medialive/ChannelInputAttachmentInputSettingsCaptionSelectorSelectorSettings:ChannelInputAttachmentInputSettingsCaptionSelectorSelectorSettings�
��
ancillarySourceSettings�B�:�
�
	medialiveYChannelInputAttachmentInputSettingsCaptionSelectorSelectorSettingsAncillarySourceSettings�aws:medialive/ChannelInputAttachmentInputSettingsCaptionSelectorSelectorSettingsAncillarySourceSettings:ChannelInputAttachmentInputSettingsCaptionSelectorSelectorSettingsAncillarySourceSettingsKAncillary Source Settings. See Ancillary Source Settings for more details.
�
aribSourceSettings�B�:�
�
	medialiveTChannelInputAttachmentInputSettingsCaptionSelectorSelectorSettingsAribSourceSettings�aws:medialive/ChannelInputAttachmentInputSettingsCaptionSelectorSelectorSettingsAribSourceSettings:ChannelInputAttachmentInputSettingsCaptionSelectorSelectorSettingsAribSourceSettingsARIB Source Settings.
�
dvbSubSourceSettings�B�:�
�
	medialiveVChannelInputAttachmentInputSettingsCaptionSelectorSelectorSettingsDvbSubSourceSettings�aws:medialive/ChannelInputAttachmentInputSettingsCaptionSelectorSelectorSettingsDvbSubSourceSettings:ChannelInputAttachmentInputSettingsCaptionSelectorSelectorSettingsDvbSubSourceSettingsGDVB Sub Source Settings. See DVB Sub Source Settings for more details.
�
embeddedSourceSettings�B�:�
�
	medialiveXChannelInputAttachmentInputSettingsCaptionSelectorSelectorSettingsEmbeddedSourceSettings�aws:medialive/ChannelInputAttachmentInputSettingsCaptionSelectorSelectorSettingsEmbeddedSourceSettings:ChannelInputAttachmentInputSettingsCaptionSelectorSelectorSettingsEmbeddedSourceSettingsIEmbedded Source Settings. See Embedded Source Settings for more details.
�
scte20SourceSettings�B�:�
�
	medialiveVChannelInputAttachmentInputSettingsCaptionSelectorSelectorSettingsScte20SourceSettings�aws:medialive/ChannelInputAttachmentInputSettingsCaptionSelectorSelectorSettingsScte20SourceSettings:ChannelInputAttachmentInputSettingsCaptionSelectorSelectorSettingsScte20SourceSettingsFSCTE20 Source Settings. See SCTE 20 Source Settings for more details.
�
scte27SourceSettings�B�:�
�
	medialiveVChannelInputAttachmentInputSettingsCaptionSelectorSelectorSettingsScte27SourceSettings�aws:medialive/ChannelInputAttachmentInputSettingsCaptionSelectorSelectorSettingsScte27SourceSettings:ChannelInputAttachmentInputSettingsCaptionSelectorSelectorSettingsScte27SourceSettingsFSCTE27 Source Settings. See SCTE 27 Source Settings for more details.
�
teletextSourceSettings�B�:�
�
	medialiveXChannelInputAttachmentInputSettingsCaptionSelectorSelectorSettingsTeletextSourceSettings�aws:medialive/ChannelInputAttachmentInputSettingsCaptionSelectorSelectorSettingsTeletextSourceSettings:ChannelInputAttachmentInputSettingsCaptionSelectorSelectorSettingsTeletextSourceSettingsITeletext Source Settings. See Teletext Source Settings for more details.
:�
�
	medialiveYChannelInputAttachmentInputSettingsCaptionSelectorSelectorSettingsAncillarySourceSettings�aws:medialive/ChannelInputAttachmentInputSettingsCaptionSelectorSelectorSettingsAncillarySourceSettings:ChannelInputAttachmentInputSettingsCaptionSelectorSelectorSettingsAncillarySourceSettings�
��
sourceAncillaryChannelNumberB �Specifies the number (1 to 4) of the captions channel you want to extract from the ancillary captions. If you plan to convert the ancillary captions to another format, complete this field. If you plan to choose Embedded as the captions destination in the output (to pass through all the channels in the ancillary captions), leave this field blank because MediaLive ignores the field.
:�
�
	medialiveTChannelInputAttachmentInputSettingsCaptionSelectorSelectorSettingsAribSourceSettings�aws:medialive/ChannelInputAttachmentInputSettingsCaptionSelectorSelectorSettingsAribSourceSettings:ChannelInputAttachmentInputSettingsCaptionSelectorSelectorSettingsAribSourceSettings
 :�
�
	medialiveVChannelInputAttachmentInputSettingsCaptionSelectorSelectorSettingsDvbSubSourceSettings�aws:medialive/ChannelInputAttachmentInputSettingsCaptionSelectorSelectorSettingsDvbSubSourceSettings:ChannelInputAttachmentInputSettingsCaptionSelectorSelectorSettingsDvbSubSourceSettings�
��
ocrLanguageB" �If you will configure a WebVTT caption description that references this caption selector, use this field to provide the language to consider when translating the image-based source to text.
�
pidB �When using DVB-Sub with Burn-In or SMPTE-TT, use this PID for the source content. Unused for DVB-Sub passthrough. All DVB-Sub content is passed through, regardless of selectors.
:�
�
	medialiveXChannelInputAttachmentInputSettingsCaptionSelectorSelectorSettingsEmbeddedSourceSettings�aws:medialive/ChannelInputAttachmentInputSettingsCaptionSelectorSelectorSettingsEmbeddedSourceSettings:ChannelInputAttachmentInputSettingsCaptionSelectorSelectorSettingsEmbeddedSourceSettings�
��
convert608To708B" �If upconvert, 608 data is both passed through via the “608 compatibility bytes” fields of the 708 wrapper as well as translated into 708. 708 data present in the source content will be discarded.
�
scte20DetectionB" hSet to “auto” to handle streams with intermittent and/or non-aligned SCTE-20 and Embedded captions.
�
source608ChannelNumberB tSpecifies the 608/708 channel number within the video track from which to extract captions. Unused for passthrough.
:�
�
	medialiveVChannelInputAttachmentInputSettingsCaptionSelectorSelectorSettingsScte20SourceSettings�aws:medialive/ChannelInputAttachmentInputSettingsCaptionSelectorSelectorSettingsScte20SourceSettings:ChannelInputAttachmentInputSettingsCaptionSelectorSelectorSettingsScte20SourceSettings;
9
convert608To708B" 
source608ChannelNumberB :�
�
	medialiveVChannelInputAttachmentInputSettingsCaptionSelectorSelectorSettingsScte27SourceSettings�aws:medialive/ChannelInputAttachmentInputSettingsCaptionSelectorSelectorSettingsScte27SourceSettings:ChannelInputAttachmentInputSettingsCaptionSelectorSelectorSettingsScte27SourceSettings$
"
ocrLanguageB" 
pidB :�
�
	medialiveXChannelInputAttachmentInputSettingsCaptionSelectorSelectorSettingsTeletextSourceSettings�aws:medialive/ChannelInputAttachmentInputSettingsCaptionSelectorSelectorSettingsTeletextSourceSettings:ChannelInputAttachmentInputSettingsCaptionSelectorSelectorSettingsTeletextSourceSettings�
��
outputRectangle�B�:�
�
	medialivegChannelInputAttachmentInputSettingsCaptionSelectorSelectorSettingsTeletextSourceSettingsOutputRectangle�aws:medialive/ChannelInputAttachmentInputSettingsCaptionSelectorSelectorSettingsTeletextSourceSettingsOutputRectangle:ChannelInputAttachmentInputSettingsCaptionSelectorSelectorSettingsTeletextSourceSettingsOutputRectangleqOptionally defines a region where TTML style captions will be displayed. See Caption Rectangle for more details.
�

pageNumberB" �Specifies the teletext page number within the data stream from which to extract captions. Range of 0x100 (256) to 0x8FF (2303). Unused for passthrough. Should be specified as a hexadecimal string with no “0x” prefix.
:�
�
	medialivegChannelInputAttachmentInputSettingsCaptionSelectorSelectorSettingsTeletextSourceSettingsOutputRectangle�aws:medialive/ChannelInputAttachmentInputSettingsCaptionSelectorSelectorSettingsTeletextSourceSettingsOutputRectangle:ChannelInputAttachmentInputSettingsCaptionSelectorSelectorSettingsTeletextSourceSettingsOutputRectangle�
�
height �

leftOffset �Applies only if you plan to convert these source captions to EBU-TT-D or TTML in an output. (Make sure to leave the default if you don’t have either of these formats in the output.) You can define a display rectangle for the captions that is smaller than the underlying video frame. You define the rectangle by specifying the position of the left edge, top edge, bottom edge, and right edge of the rectangle, all within the underlying video frame. The units for the measurements are percentages. If you specify a value for one of these fields, you must specify a value for all of them. For leftOffset, specify the position of the left edge of the rectangle, as a percentage of the underlying frame width, and relative to the left edge of the frame. For example, "10" means the measurement is 10% of the underlying frame width. The rectangle left edge starts at that position from the left edge of the frame. This field corresponds to tts:origin - X in the TTML standard.
�
	topOffset �See the description in left\_offset. For top\_offset, specify the position of the top edge of the rectangle, as a percentage of the underlying frame height, and relative to the top edge of the frame. For example, "10" means the measurement is 10% of the underlying frame height. The rectangle top edge starts at that position from the top edge of the frame. This field corresponds to tts:origin - Y in the TTML standard.

width :�
�
	medialive7ChannelInputAttachmentInputSettingsNetworkInputSettings}aws:medialive/ChannelInputAttachmentInputSettingsNetworkInputSettings:ChannelInputAttachmentInputSettingsNetworkInputSettings�
��
hlsInputSettings�B�:�
�
	medialiveGChannelInputAttachmentInputSettingsNetworkInputSettingsHlsInputSettings�aws:medialive/ChannelInputAttachmentInputSettingsNetworkInputSettingsHlsInputSettings:ChannelInputAttachmentInputSettingsNetworkInputSettingsHlsInputSettingsjSpecifies HLS input settings when the uri is for a HLS manifest. See HLS Input Settings for more details.
;
serverValidationB" !Check HTTPS server certificates.
:�
�
	medialiveGChannelInputAttachmentInputSettingsNetworkInputSettingsHlsInputSettings�aws:medialive/ChannelInputAttachmentInputSettingsNetworkInputSettingsHlsInputSettings:ChannelInputAttachmentInputSettingsNetworkInputSettingsHlsInputSettings�
�W
	bandwidthB DThe bitrate is specified in bits per second, as in an HLS manifest.
)
bufferSegmentsB Buffer segments.
�
retriesB �The number of consecutive times that attempts to read a manifest or segment must fail before the input is considered unavailable.
r
retryIntervalB [The number of seconds between retries when an attempt to read a manifest or segment fails.

scte35SourceB" :�
�
	medialive0ChannelInputAttachmentInputSettingsVideoSelectoroaws:medialive/ChannelInputAttachmentInputSettingsVideoSelector:ChannelInputAttachmentInputSettingsVideoSelector/
-

colorSpaceB" 
colorSpaceUsageB" :�
i
	medialiveChannelInputSpecificationAaws:medialive/ChannelInputSpecification:ChannelInputSpecification<
:
codec" 
inputResolution" 
maximumBitrate" :�
T
	medialiveChannelMaintenance3aws:medialive/ChannelMaintenance:ChannelMaintenance�
�B
maintenanceDay" ,The day of the week to use for maintenance.
=
maintenanceStartTime" !The hour maintenance will start.
:�
<
	medialive
ChannelVpc#aws:medialive/ChannelVpc:ChannelVpc�
�
availabilityZonesB*" 
networkInterfaceIdsB*" �
publicAddressAllocationIds*" �List of public address allocation ids to associate with ENIs that will be created in Output VPC. Must specify one for SINGLE_PIPELINE, two for STANDARD channels.
�
securityGroupIdsB*" �A list of up to 5 EC2 VPC security group IDs to attach to the Output VPC network interfaces. If none are specified then the VPC default security group will be used.
�
	subnetIds*" �A list of VPC subnet IDs from the same VPC. If STANDARD channel, subnet IDs must be mapped to two unique availability zones (AZ).
:�
N
	medialiveInputDestination/aws:medialive/InputDestination:InputDestinationY
WU

streamName" CA unique name for the location the RTMP stream is being pushed to.
:~
N
	medialiveInputInputDevice/aws:medialive/InputInputDevice:InputInputDevice,
*(
id" The unique ID for the device.
:�
]
	medialiveInputMediaConnectFlow9aws:medialive/InputMediaConnectFlow:InputMediaConnectFlow4
20
flowArn" !The ARN of the MediaConnect Flow
:�
{
	medialiveInputSecurityGroupWhitelistRuleMaws:medialive/InputSecurityGroupWhitelistRule:InputSecurityGroupWhitelistRule2
0.
cidr" "The IPv4 CIDR that's whitelisted.
:�
?
	medialiveInputSource%aws:medialive/InputSource:InputSource�
�T
passwordParam" ?The key used to extract the password from EC2 Parameter store.
4
url" )The URL where the stream is pulled from.
3
username" #The username for the input source.
:�
6
	medialiveInputVpcaws:medialive/InputVpc:InputVpc�
�a
securityGroupIdsB*" EA list of up to 5 EC2 VPC security group IDs to attach to the Input.
A
	subnetIds*" .A list of 2 VPC subnet IDs from the same VPC.
:�
l
	medialiveMultiplexMultiplexSettingsCaws:medialive/MultiplexMultiplexSettings:MultiplexMultiplexSettings�
�I
#maximumVideoBufferDelayMillisecondsB Maximum video buffer delay.
9
transportStreamBitrate Transport stream bit rate.
7
transportStreamId Unique ID for each multiplex.
L
transportStreamReservedBitrateB $Transport stream reserved bit rate.
:�
�
	medialive(MultiplexProgramMultiplexProgramSettings_aws:medialive/MultiplexProgramMultiplexProgramSettings:MultiplexProgramMultiplexProgramSettings�
��
preferredChannelPipeline" dEnum for preferred channel pipeline. Options are `CURRENTLY_ACTIVE`, `PIPELINE_0`, or `PIPELINE_1`.
,
programNumber Unique program number.
�
serviceDescriptor�B�:�
�
	medialive9MultiplexProgramMultiplexProgramSettingsServiceDescriptor�aws:medialive/MultiplexProgramMultiplexProgramSettingsServiceDescriptor:MultiplexProgramMultiplexProgramSettingsServiceDescriptor=Service Descriptor. See Service Descriptor for more details.
�
videoSettings�B�:�
�
	medialive5MultiplexProgramMultiplexProgramSettingsVideoSettingsyaws:medialive/MultiplexProgramMultiplexProgramSettingsVideoSettings:MultiplexProgramMultiplexProgramSettingsVideoSettings5Video settings. See Video Settings for more details.
:�
�
	medialive9MultiplexProgramMultiplexProgramSettingsServiceDescriptor�aws:medialive/MultiplexProgramMultiplexProgramSettingsServiceDescriptor:MultiplexProgramMultiplexProgramSettingsServiceDescriptorX
V*
providerName" Unique provider name.
(
serviceName" Unique service name.
:�
�
	medialive5MultiplexProgramMultiplexProgramSettingsVideoSettingsyaws:medialive/MultiplexProgramMultiplexProgramSettingsVideoSettings:MultiplexProgramMultiplexProgramSettingsVideoSettings�
�1
constantBitrateB Constant bitrate value.
�
statmuxSettings�B�:�
�
	medialiveDMultiplexProgramMultiplexProgramSettingsVideoSettingsStatmuxSettings�aws:medialive/MultiplexProgramMultiplexProgramSettingsVideoSettingsStatmuxSettings:MultiplexProgramMultiplexProgramSettingsVideoSettingsStatmuxSettings9Statmux settings. See Statmux Settings for more details.
:�
�
	medialiveDMultiplexProgramMultiplexProgramSettingsVideoSettingsStatmuxSettings�aws:medialive/MultiplexProgramMultiplexProgramSettingsVideoSettingsStatmuxSettings:MultiplexProgramMultiplexProgramSettingsVideoSettingsStatmuxSettings|
z)
maximumBitrateB Maximum bitrate.
)
minimumBitrateB Minimum bitrate.
"
priorityB Priority value.
:�
W
	medialivegetInputDestination5aws:medialive/getInputDestination:getInputDestination�
�
ip" 

port" 	
url" n
vpcsf*d:b
`
	medialivegetInputDestinationVpc;aws:medialive/getInputDestinationVpc:getInputDestinationVpc:�
`
	medialivegetInputDestinationVpc;aws:medialive/getInputDestinationVpc:getInputDestinationVpc4
2
availabilityZone" 
networkInterfaceId" :~
W
	medialivegetInputInputDevice5aws:medialive/getInputInputDevice:getInputInputDevice#
!
id" The ID of the Input.
:{
f
	medialivegetInputMediaConnectFlow?aws:medialive/getInputMediaConnectFlow:getInputMediaConnectFlow

flowArn" :~
H
	medialivegetInputSource+aws:medialive/getInputSource:getInputSource2
0
passwordParam" 	
url" 
username" :�
T
mediapackageChannelHlsIngest2aws:mediapackage/ChannelHlsIngest:ChannelHlsIngest�
��
ingestEndpoints�B�*�:�
~
mediapackageChannelHlsIngestIngestEndpointNaws:mediapackage/ChannelHlsIngestIngestEndpoint:ChannelHlsIngestIngestEndpointA list of the ingest endpoints
:�
~
mediapackageChannelHlsIngestIngestEndpointNaws:mediapackage/ChannelHlsIngestIngestEndpoint:ChannelHlsIngestIngestEndpoint[
Y
passwordB" The password

urlB" The URL

usernameB" The username
:�
^
memorydbClusterClusterEndpoint:aws:memorydb/ClusterClusterEndpoint:ClusterClusterEndpoint�
�+
addressB" DNS hostname of the node.
b
portB TThe port number on which each of the nodes accepts connections. Defaults to `6379`.
:�
@
memorydbClusterShard&aws:memorydb/ClusterShard:ClusterShard�
�}
nameB" oName of the cluster. If omitted, the provider will assign a random, unique name. Conflicts with `name_prefix`.
{
nodesTBR*P:N
L
memorydbClusterShardNode.aws:memorydb/ClusterShardNode:ClusterShardNodeSet of nodes in this shard.
<
numNodesB *Number of individual nodes in this shard.
<
slotsB" -Keyspace for this shard. Example: `0-16383`.
:�
L
memorydbClusterShardNode.aws:memorydb/ClusterShardNode:ClusterShardNode�
�K
availabilityZoneB" 1The Availability Zone in which the node resides.
b

createTimeB" NThe date and time when the node was created. Example: `2022-01-01T21:00:00Z`.
y
	endpointslBj*h:f
d
memorydbClusterShardNodeEndpoint>aws:memorydb/ClusterShardNodeEndpoint:ClusterShardNodeEndpoint}
nameB" oName of the cluster. If omitted, the provider will assign a random, unique name. Conflicts with `name_prefix`.
:�
d
memorydbClusterShardNodeEndpoint>aws:memorydb/ClusterShardNodeEndpoint:ClusterShardNodeEndpoint�
�+
addressB" DNS hostname of the node.
b
portB TThe port number on which each of the nodes accepts connections. Defaults to `6379`.
:�
j
memorydbMultiRegionClusterTimeoutsBaws:memorydb/MultiRegionClusterTimeouts:MultiRegionClusterTimeouts�
��
createB" �A string that can be [parsed as a duration](https://pkg.go.dev/time#ParseDuration) consisting of numbers and unit suffixes, such as "30s" or "2h45m". Valid time units are "s" (seconds), "m" (minutes), "h" (hours).
�
deleteB" �A string that can be [parsed as a duration](https://pkg.go.dev/time#ParseDuration) consisting of numbers and unit suffixes, such as "30s" or "2h45m". Valid time units are "s" (seconds), "m" (minutes), "h" (hours). Setting a timeout for a Delete operation is only applicable if changes are saved into state before the destroy operation occurs.
�
updateB" �A string that can be [parsed as a duration](https://pkg.go.dev/time#ParseDuration) consisting of numbers and unit suffixes, such as "30s" or "2h45m". Valid time units are "s" (seconds), "m" (minutes), "h" (hours).
:�
a
memorydbParameterGroupParameter<aws:memorydb/ParameterGroupParameter:ParameterGroupParameterV
T'
name" The name of the parameter.
)
value" The value of the parameter.
:�	
p
memorydbSnapshotClusterConfigurationFaws:memorydb/SnapshotClusterConfiguration:SnapshotClusterConfiguration�	
�	2
descriptionB" Description for the cluster.
;
engineB" +The engine that will run on cluster nodes.
I
engineVersionB" 2Version number of the engine used by the cluster.
g
maintenanceWindowB" LThe weekly time range during which maintenance on the cluster is performed.
~
nameB" pName of the snapshot. If omitted, the provider will assign a random, unique name. Conflicts with `name_prefix`.
K
nodeTypeB" 9Compute and memory capacity of the nodes in the cluster.
4
	numShardsB !Number of shards in the cluster.
U
parameterGroupNameB" 9Name of the parameter group associated with the cluster.
D
portB 6Port number on which the cluster accepts connections.
t
snapshotRetentionLimitB TNumber of days for which MemoryDB retains automatic snapshots before deleting them.
y
snapshotWindowB" aThe daily time range (in UTC) during which MemoryDB begins taking a daily snapshot of the shard.
G
subnetGroupNameB" .Name of the subnet group used by the cluster.
P
topicArnB" >ARN of the SNS topic to which cluster notifications are sent.
4
vpcIdB" %The VPC in which the cluster exists.
:�
^
memorydbUserAuthenticationMode:aws:memorydb/UserAuthenticationMode:UserAuthenticationMode�
�a
passwordCountB JNumber of passwords belonging to the user if `type` is set to `password`.
�
	passwordsB*" {Set of passwords used for authentication if `type` is set to `password`. You can create up to two passwords for each user.
V
type" JSpecifies the authentication type. Valid values are: `password` or `iam`.
:�
g
memorydbgetClusterClusterEndpoint@aws:memorydb/getClusterClusterEndpoint:getClusterClusterEndpointg
e)
address" DNS hostname of the node.
8
port ,Port number that this node is listening on.
:�
I
memorydbgetClusterShard,aws:memorydb/getClusterShard:getClusterShard�
�!
name" Name of the cluster.
�
nodes[*Y:W
U
memorydbgetClusterShardNode4aws:memorydb/getClusterShardNode:getClusterShardNodeSet of nodes in this shard.
:
numNodes *Number of individual nodes in this shard.
:
slots" -Keyspace for this shard. Example: `0-16383`.
:�
U
memorydbgetClusterShardNode4aws:memorydb/getClusterShardNode:getClusterShardNode�
�I
availabilityZone" 1The Availability Zone in which the node resides.
`

createTime" NThe date and time when the node was created. Example: `2022-01-01T21:00:00Z`.
�
	endpointss*q:o
m
memorydbgetClusterShardNodeEndpointDaws:memorydb/getClusterShardNodeEndpoint:getClusterShardNodeEndpoint!
name" Name of the cluster.
:�
m
memorydbgetClusterShardNodeEndpointDaws:memorydb/getClusterShardNodeEndpoint:getClusterShardNodeEndpointg
e)
address" DNS hostname of the node.
8
port ,Port number that this node is listening on.
:�
j
memorydbgetParameterGroupParameterBaws:memorydb/getParameterGroupParameter:getParameterGroupParameterT
R)
name" Name of the parameter group.
%
value" Value of the parameter.
:�	
y
memorydbgetSnapshotClusterConfigurationLaws:memorydb/getSnapshotClusterConfiguration:getSnapshotClusterConfiguration�
�0
description" Description for the cluster.
9
engine" +The engine that will run on cluster nodes.
G
engineVersion" 2Version number of the engine used by the cluster.
e
maintenanceWindow" LThe weekly time range during which maintenance on the cluster is performed.
"
name" Name of the snapshot.
I
nodeType" 9Compute and memory capacity of the nodes in the cluster.
2
	numShards !Number of shards in the cluster.
S
parameterGroupName" 9Name of the parameter group associated with the cluster.
B
port 6Port number on which the cluster accepts connections.
r
snapshotRetentionLimit TNumber of days for which MemoryDB retains automatic snapshots before deleting them.
w
snapshotWindow" aThe daily time range (in UTC) during which MemoryDB begins taking a daily snapshot of the shard.
E
subnetGroupName" .Name of the subnet group used by the cluster.
N
topicArn" >ARN of the SNS topic to which cluster notifications are sent.
2
vpcId" %The VPC in which the cluster exists.
:�
g
memorydbgetUserAuthenticationMode@aws:memorydb/getUserAuthenticationMode:getUserAuthenticationMode�
�_
passwordCount JNumber of passwords belonging to the user if `type` is set to `password`.
/
type" #Type of authentication configured.
:�
I
mqBrokerConfiguration.aws:mq/BrokerConfiguration:BrokerConfigurationY
W"
idB" The Configuration ID.
1
revisionB Revision of the Configuration.
:�
U
mqBrokerEncryptionOptions6aws:mq/BrokerEncryptionOptions:BrokerEncryptionOptions�
��
kmsKeyIdB" �Amazon Resource Name (ARN) of Key Management Service (KMS) Customer Master Key (CMK) to use for encryption at rest. Requires setting `use_aws_owned_key` to `false`. To perform drift detection when AWS-managed CMKs or customer-managed CMKs are in use, this value must be configured.
�
useAwsOwnedKeyB
 �Whether to enable an AWS-owned KMS CMK that is not in your account. Defaults to `true`. Setting to `false` without configuring `kms_key_id` will create an AWS-managed CMK aliased to `aws/mq` in your account.
:�
:
mqBrokerInstance$aws:mq/BrokerInstance:BrokerInstance�
��

consoleUrlB" �The URL of the [ActiveMQ Web Console](http://activemq.apache.org/web-console.html) or the [RabbitMQ Management UI](https://www.rabbitmq.com/management.html#external-monitoring) depending on `engine_type`.
�
	endpointsB*" �Broker's wire-level protocol endpoints in the following order & format referenceable e.g., as `instances.0.endpoints.0` (SSL):
* For `ActiveMQ`:
* `ssl://broker-id.mq.us-west-2.amazonaws.com:61617`
* `amqp+ssl://broker-id.mq.us-west-2.amazonaws.com:5671`
* `stomp+ssl://broker-id.mq.us-west-2.amazonaws.com:61614`
* `mqtt+ssl://broker-id.mq.us-west-2.amazonaws.com:8883`
* `wss://broker-id.mq.us-west-2.amazonaws.com:61619`
* For `RabbitMQ`:
* `amqps://broker-id.mq.us-west-2.amazonaws.com:5671`
-
	ipAddressB" IP Address of the broker.
:�
X
mqBrokerLdapServerMetadata8aws:mq/BrokerLdapServerMetadata:BrokerLdapServerMetadata�
�k
hostsB*" ZList of a fully qualified domain name of the LDAP server and an optional failover server.
Y
roleBaseB" GFully qualified name of the directory to search for a user’s groups.
�
roleNameB" ~Specifies the LDAP attribute that identifies the group name attribute in the object returned from the group membership query.
8
roleSearchMatchingB" Search criteria for groups.
V
roleSearchSubtreeB
 ;Whether the directory search scope is the entire sub-tree.
:
serviceAccountPasswordB" Service account password.
:
serviceAccountUsernameB" Service account username.
\
userBaseB" JFully qualified name of the directory where you want to search for users.
^
userRoleNameB" HSpecifies the name of the LDAP attribute for the user group membership.
7
userSearchMatchingB" Search criteria for users.
V
userSearchSubtreeB
 ;Whether the directory search scope is the entire sub-tree.
:�
.
mq
BrokerLogsaws:mq/BrokerLogs:BrokerLogs�
��
auditB
 �Enables audit logging. Auditing is only possible for `engine_type` of `ActiveMQ`. User management action made using JMX or the ActiveMQ Web Console is logged. Defaults to `false`.
N
generalB
 =Enables general logging via CloudWatch. Defaults to `false`.
:�
p
mq BrokerMaintenanceWindowStartTimeHaws:mq/BrokerMaintenanceWindowStartTime:BrokerMaintenanceWindowStartTime�
�M
	dayOfWeek" <Day of the week, e.g., `MONDAY`, `TUESDAY`, or `WEDNESDAY`.
9
	timeOfDay" (Time, in 24-hour format, e.g., `02:00`.
c
timeZone" STime zone in either the Country/City format or the UTC offset format, e.g., `CET`.
:�
.
mq
BrokerUseraws:mq/BrokerUser:BrokerUser�
��
consoleAccessB
 �Whether to enable access to the [ActiveMQ Web Console](http://activemq.apache.org/web-console.html) for the user. Applies to `engine_type` of `ActiveMQ` only.

groupsB*" mList of groups (20 maximum) to which the ActiveMQ user belongs. Applies to `engine_type` of `ActiveMQ` only.
�
password" wPassword of the user. It must be 12 to 250 characters long, at least 4 unique characters, and must not contain commas.
S
replicationUserB
 :Whether to set set replication user. Defaults to `false`.
�
username" �Username of the user.

> **NOTE:** AWS currently does not support updating RabbitMQ users. Updates to users can only be in the RabbitMQ UI.
:r
R
mqgetBrokerConfiguration4aws:mq/getBrokerConfiguration:getBrokerConfiguration

id" 
revision :�
[
mqgetBrokerEncryptionOption:aws:mq/getBrokerEncryptionOption:getBrokerEncryptionOption(
&
kmsKeyId" 
useAwsOwnedKey
 :�
|
mq$getBrokerEngineTypesBrokerEngineTypePaws:mq/getBrokerEngineTypesBrokerEngineType:getBrokerEngineTypesBrokerEngineType�
�D

engineType" 2The MQ engine type to return version details for.
�
engineVersions�*�:�
�
mq1getBrokerEngineTypesBrokerEngineTypeEngineVersionjaws:mq/getBrokerEngineTypesBrokerEngineTypeEngineVersion:getBrokerEngineTypesBrokerEngineTypeEngineVersionThe list of engine versions.
:�
�
mq1getBrokerEngineTypesBrokerEngineTypeEngineVersionjaws:mq/getBrokerEngineTypesBrokerEngineTypeEngineVersion:getBrokerEngineTypesBrokerEngineTypeEngineVersion


name" :
C
mqgetBrokerInstance*aws:mq/getBrokerInstance:getBrokerInstance8
6

consoleUrl" 
	endpoints*" 
	ipAddress" :�
a
mqgetBrokerLdapServerMetadata>aws:mq/getBrokerLdapServerMetadata:getBrokerLdapServerMetadata�
�
hosts*" 
roleBase" 
roleName" 
roleSearchMatching" 
roleSearchSubtree
 
serviceAccountPassword" 
serviceAccountUsername" 
userBase" 
userRoleName" 
userSearchMatching" 
userSearchSubtree
 :Y
7
mqgetBrokerLogs"aws:mq/getBrokerLogs:getBrokerLogs

audit
 
general
 :�
y
mq#getBrokerMaintenanceWindowStartTimeNaws:mq/getBrokerMaintenanceWindowStartTime:getBrokerMaintenanceWindowStartTime4
2
	dayOfWeek" 
	timeOfDay" 
timeZone" :�
7
mqgetBrokerUser"aws:mq/getBrokerUser:getBrokerUserN
L
consoleAccess
 
groups*" 
replicationUser
 
username" :�
�
mq,getInstanceTypeOfferingsBrokerInstanceOption`aws:mq/getInstanceTypeOfferingsBrokerInstanceOption:getInstanceTypeOfferingsBrokerInstanceOption�
��
availabilityZones�*�:�
�
mq<getInstanceTypeOfferingsBrokerInstanceOptionAvailabilityZone�aws:mq/getInstanceTypeOfferingsBrokerInstanceOptionAvailabilityZone:getInstanceTypeOfferingsBrokerInstanceOptionAvailabilityZone5List of available AZs. See Availability Zones. below
2

engineType"  Filter response by engine type.
?
hostInstanceType" 'Filter response by host instance type.
4
storageType" !Filter response by storage type.
J
supportedDeploymentModes*" (The list of supported deployment modes.
H
supportedEngineVersions*" 'The list of supported engine versions.
:�
�
mq<getInstanceTypeOfferingsBrokerInstanceOptionAvailabilityZone�aws:mq/getInstanceTypeOfferingsBrokerInstanceOptionAvailabilityZone:getInstanceTypeOfferingsBrokerInstanceOptionAvailabilityZone/
-+
name" Name of the Availability Zone.
:�
`
mskClusterBrokerNodeGroupInfo=aws:msk/ClusterBrokerNodeGroupInfo:ClusterBrokerNodeGroupInfo�
��
azDistributionB" �The distribution of broker nodes across availability zones ([documentation](https://docs.aws.amazon.com/msk/1.0/apireference/clusters.html#clusters-model-brokerazdistribution)). Currently the only valid value is `DEFAULT`.
�
clientSubnets*" �A list of subnets to connect to in client VPC ([documentation](https://docs.aws.amazon.com/msk/1.0/apireference/clusters.html#clusters-prop-brokernodegroupinfo-clientsubnets)).
�
connectivityInfo�B�:�
�
msk*ClusterBrokerNodeGroupInfoConnectivityInfo]aws:msk/ClusterBrokerNodeGroupInfoConnectivityInfo:ClusterBrokerNodeGroupInfoConnectivityInfo�Information about the cluster access configuration. See below. For security reasons, you can't turn on public access while creating an MSK cluster. However, you can update an existing cluster to make it publicly accessible. You can also create a new cluster and then update it to make it publicly accessible ([documentation](https://docs.aws.amazon.com/msk/latest/developerguide/public-access.html)).
�
instanceType" �Specify the instance type to use for the kafka brokersE.g., kafka.m5.large. ([Pricing info](https://aws.amazon.com/msk/pricing/))
�
securityGroups*" �A list of the security groups to associate with the elastic network interfaces to control who can communicate with the cluster.
�
storageInfo�B�:�
�
msk%ClusterBrokerNodeGroupInfoStorageInfoSaws:msk/ClusterBrokerNodeGroupInfoStorageInfo:ClusterBrokerNodeGroupInfoStorageInfoaA block that contains information about storage volumes attached to MSK broker nodes. See below.
:�
�
msk*ClusterBrokerNodeGroupInfoConnectivityInfo]aws:msk/ClusterBrokerNodeGroupInfoConnectivityInfo:ClusterBrokerNodeGroupInfoConnectivityInfo�
��
publicAccess�B�:�
�
msk6ClusterBrokerNodeGroupInfoConnectivityInfoPublicAccessuaws:msk/ClusterBrokerNodeGroupInfoConnectivityInfoPublicAccess:ClusterBrokerNodeGroupInfoConnectivityInfoPublicAccess0Access control settings for brokers. See below.
�
vpcConnectivity�B�:�
�
msk9ClusterBrokerNodeGroupInfoConnectivityInfoVpcConnectivity{aws:msk/ClusterBrokerNodeGroupInfoConnectivityInfoVpcConnectivity:ClusterBrokerNodeGroupInfoConnectivityInfoVpcConnectivity8VPC connectivity access control for brokers. See below.
:�
�
msk6ClusterBrokerNodeGroupInfoConnectivityInfoPublicAccessuaws:msk/ClusterBrokerNodeGroupInfoConnectivityInfoPublicAccess:ClusterBrokerNodeGroupInfoConnectivityInfoPublicAccessY
WU
typeB" GPublic access type. Valid values: `DISABLED`, `SERVICE_PROVIDED_EIPS`.
:�
�
msk9ClusterBrokerNodeGroupInfoConnectivityInfoVpcConnectivity{aws:msk/ClusterBrokerNodeGroupInfoConnectivityInfoVpcConnectivity:ClusterBrokerNodeGroupInfoConnectivityInfoVpcConnectivity�
��
clientAuthentication�B�:�
�
mskMClusterBrokerNodeGroupInfoConnectivityInfoVpcConnectivityClientAuthentication�aws:msk/ClusterBrokerNodeGroupInfoConnectivityInfoVpcConnectivityClientAuthentication:ClusterBrokerNodeGroupInfoConnectivityInfoVpcConnectivityClientAuthenticationGConfiguration block for specifying a client authentication. See below.
:�
�
mskMClusterBrokerNodeGroupInfoConnectivityInfoVpcConnectivityClientAuthentication�aws:msk/ClusterBrokerNodeGroupInfoConnectivityInfoVpcConnectivityClientAuthentication:ClusterBrokerNodeGroupInfoConnectivityInfoVpcConnectivityClientAuthentication�
��
sasl�B�:�
�
mskQClusterBrokerNodeGroupInfoConnectivityInfoVpcConnectivityClientAuthenticationSasl�aws:msk/ClusterBrokerNodeGroupInfoConnectivityInfoVpcConnectivityClientAuthenticationSasl:ClusterBrokerNodeGroupInfoConnectivityInfoVpcConnectivityClientAuthenticationSaslJConfiguration block for specifying SASL client authentication. See below.
V
tlsB
 IConfiguration block for specifying TLS client authentication. See below.
:�
�
mskQClusterBrokerNodeGroupInfoConnectivityInfoVpcConnectivityClientAuthenticationSasl�aws:msk/ClusterBrokerNodeGroupInfoConnectivityInfoVpcConnectivityClientAuthenticationSasl:ClusterBrokerNodeGroupInfoConnectivityInfoVpcConnectivityClientAuthenticationSasl

iamB
 
scramB
 :�
�
msk%ClusterBrokerNodeGroupInfoStorageInfoSaws:msk/ClusterBrokerNodeGroupInfoStorageInfo:ClusterBrokerNodeGroupInfoStorageInfo�
��
ebsStorageInfo�B�:�
�
msk3ClusterBrokerNodeGroupInfoStorageInfoEbsStorageInfooaws:msk/ClusterBrokerNodeGroupInfoStorageInfoEbsStorageInfo:ClusterBrokerNodeGroupInfoStorageInfoEbsStorageInfo9A block that contains EBS volume information. See below.
:�
�
msk3ClusterBrokerNodeGroupInfoStorageInfoEbsStorageInfooaws:msk/ClusterBrokerNodeGroupInfoStorageInfoEbsStorageInfo:ClusterBrokerNodeGroupInfoStorageInfoEbsStorageInfo�
��
provisionedThroughput�B�:�
�
mskHClusterBrokerNodeGroupInfoStorageInfoEbsStorageInfoProvisionedThroughput�aws:msk/ClusterBrokerNodeGroupInfoStorageInfoEbsStorageInfoProvisionedThroughput:ClusterBrokerNodeGroupInfoStorageInfoEbsStorageInfoProvisionedThroughput�A block that contains EBS volume provisioned throughput information. To provision storage throughput, you must choose broker type kafka.m5.4xlarge or larger. See below.
�

volumeSizeB }The size in GiB of the EBS volume for the data drive on each broker node. Minimum value of `1` and maximum value of `16384`.
:�
�
mskHClusterBrokerNodeGroupInfoStorageInfoEbsStorageInfoProvisionedThroughput�aws:msk/ClusterBrokerNodeGroupInfoStorageInfoEbsStorageInfoProvisionedThroughput:ClusterBrokerNodeGroupInfoStorageInfoEbsStorageInfoProvisionedThroughput�
�
enabledB
 �
volumeThroughputB �Throughput value of the EBS volumes for the data drive on each kafka broker node in MiB per second. The minimum value is `250`. The maximum value varies between broker type. You can refer to the valid values for the maximum volume throughput at the following [documentation on throughput bottlenecks](https://docs.aws.amazon.com/msk/latest/developerguide/msk-provision-throughput.html#throughput-bottlenecks)
:�
c
mskClusterClientAuthentication?aws:msk/ClusterClientAuthentication:ClusterClientAuthentication�
��
sasluBs:q
o
mskClusterClientAuthenticationSaslGaws:msk/ClusterClientAuthenticationSasl:ClusterClientAuthenticationSaslJConfiguration block for specifying SASL client authentication. See below.
�
tlsrBp:n
l
mskClusterClientAuthenticationTlsEaws:msk/ClusterClientAuthenticationTls:ClusterClientAuthenticationTlsIConfiguration block for specifying TLS client authentication. See below.
9
unauthenticatedB
  Enables unauthenticated access.
:�
o
mskClusterClientAuthenticationSaslGaws:msk/ClusterClientAuthenticationSasl:ClusterClientAuthenticationSasl

iamB
 
scramB
 :�
l
mskClusterClientAuthenticationTlsEaws:msk/ClusterClientAuthenticationTls:ClusterClientAuthenticationTlsh
fd
certificateAuthorityArnsB*" @List of ACM Certificate Authority Amazon Resource Names (ARNs).
:�
Z
mskClusterConfigurationInfo9aws:msk/ClusterConfigurationInfo:ClusterConfigurationInfo�
�V
arn" KAmazon Resource Name (ARN) of the MSK Configuration to use in the cluster.
I
revision 9Revision of the MSK Configuration to use in the cluster.
:�
Q
mskClusterEncryptionInfo3aws:msk/ClusterEncryptionInfo:ClusterEncryptionInfo�
��
encryptionAtRestKmsKeyArnB" �You may specify a KMS key short ID or ARN (it will always output an ARN) to use for encrypting your data at rest.  If no key is specified, an AWS managed KMS ('aws/msk' managed service) key will be used for encrypting the data at rest.
�
encryptionInTransit�B�:�
�
msk(ClusterEncryptionInfoEncryptionInTransitYaws:msk/ClusterEncryptionInfoEncryptionInTransit:ClusterEncryptionInfoEncryptionInTransitAConfiguration block to specify encryption in transit. See below.
:�
�
msk(ClusterEncryptionInfoEncryptionInTransitYaws:msk/ClusterEncryptionInfoEncryptionInTransit:ClusterEncryptionInfoEncryptionInTransit�
��
clientBrokerB" �Encryption setting for data in transit between clients and brokers. Valid values: `TLS`, `TLS_PLAINTEXT`, and `PLAINTEXT`. Default value is `TLS`.
f
	inClusterB
 SWhether data communication among broker nodes is encrypted. Default value: `true`.
:�
H
mskClusterLoggingInfo-aws:msk/ClusterLoggingInfo:ClusterLoggingInfo�
��

brokerLogsj:h
f
mskClusterLoggingInfoBrokerLogsAaws:msk/ClusterLoggingInfoBrokerLogs:ClusterLoggingInfoBrokerLogsJConfiguration block for Broker Logs settings for logging info. See below.
:�
f
mskClusterLoggingInfoBrokerLogsAaws:msk/ClusterLoggingInfoBrokerLogs:ClusterLoggingInfoBrokerLogs�
��
cloudwatchLogs�B�:�
�
msk*ClusterLoggingInfoBrokerLogsCloudwatchLogs]aws:msk/ClusterLoggingInfoBrokerLogsCloudwatchLogs:ClusterLoggingInfoBrokerLogsCloudwatchLogs�
firehose�B�:�
~
msk$ClusterLoggingInfoBrokerLogsFirehoseQaws:msk/ClusterLoggingInfoBrokerLogsFirehose:ClusterLoggingInfoBrokerLogsFirehosex
s3rBp:n
l
mskClusterLoggingInfoBrokerLogsS3Eaws:msk/ClusterLoggingInfoBrokerLogsS3:ClusterLoggingInfoBrokerLogsS3:�
�
msk*ClusterLoggingInfoBrokerLogsCloudwatchLogs]aws:msk/ClusterLoggingInfoBrokerLogsCloudwatchLogs:ClusterLoggingInfoBrokerLogsCloudwatchLogsZ
X
enabled
 G
logGroupB" 5Name of the Cloudwatch Log Group to deliver logs to.
:�
~
msk$ClusterLoggingInfoBrokerLogsFirehoseQaws:msk/ClusterLoggingInfoBrokerLogsFirehose:ClusterLoggingInfoBrokerLogsFirehoseq
o^
deliveryStreamB" FName of the Kinesis Data Firehose delivery stream to deliver logs to.

enabled
 :�
l
mskClusterLoggingInfoBrokerLogsS3Eaws:msk/ClusterLoggingInfoBrokerLogsS3:ClusterLoggingInfoBrokerLogsS3�
�:
bucketB" *Name of the S3 bucket to deliver logs to.

enabled
 5
prefixB" %Prefix to append to the folder name.
:�
Q
mskClusterOpenMonitoring3aws:msk/ClusterOpenMonitoring:ClusterOpenMonitoring�
��

prometheuss:q
o
mskClusterOpenMonitoringPrometheusGaws:msk/ClusterOpenMonitoringPrometheus:ClusterOpenMonitoringPrometheusLConfiguration block for Prometheus settings for open monitoring. See below.
:�
o
mskClusterOpenMonitoringPrometheusGaws:msk/ClusterOpenMonitoringPrometheus:ClusterOpenMonitoringPrometheus�
��
jmxExporter�B�:�
�
msk*ClusterOpenMonitoringPrometheusJmxExporter]aws:msk/ClusterOpenMonitoringPrometheusJmxExporter:ClusterOpenMonitoringPrometheusJmxExporter1Configuration block for JMX Exporter. See below.
�
nodeExporter�B�:�
�
msk+ClusterOpenMonitoringPrometheusNodeExporter_aws:msk/ClusterOpenMonitoringPrometheusNodeExporter:ClusterOpenMonitoringPrometheusNodeExporter2Configuration block for Node Exporter. See below.
:�
�
msk*ClusterOpenMonitoringPrometheusJmxExporter]aws:msk/ClusterOpenMonitoringPrometheusJmxExporter:ClusterOpenMonitoringPrometheusJmxExporter^
\Z
enabledInBroker
 CIndicates whether you want to enable or disable the Node Exporter.
:�
�
msk+ClusterOpenMonitoringPrometheusNodeExporter_aws:msk/ClusterOpenMonitoringPrometheusNodeExporter:ClusterOpenMonitoringPrometheusNodeExporter^
\Z
enabledInBroker
 CIndicates whether you want to enable or disable the Node Exporter.
:�
T
mskReplicatorKafkaCluster5aws:msk/ReplicatorKafkaCluster:ReplicatorKafkaCluster�
��
amazonMskCluster�:�
�
msk&ReplicatorKafkaClusterAmazonMskClusterUaws:msk/ReplicatorKafkaClusterAmazonMskCluster:ReplicatorKafkaClusterAmazonMskCluster"Details of an Amazon MSK cluster.
�
	vpcConfigs:q
o
mskReplicatorKafkaClusterVpcConfigGaws:msk/ReplicatorKafkaClusterVpcConfig:ReplicatorKafkaClusterVpcConfigUDetails of an Amazon VPC which has network connectivity to the Apache Kafka cluster.
:�
�
msk&ReplicatorKafkaClusterAmazonMskClusterUaws:msk/ReplicatorKafkaClusterAmazonMskCluster:ReplicatorKafkaClusterAmazonMskCluster;
97
mskClusterArn" "The ARN of an Amazon MSK cluster.
:�
o
mskReplicatorKafkaClusterVpcConfigGaws:msk/ReplicatorKafkaClusterVpcConfig:ReplicatorKafkaClusterVpcConfig�
��
securityGroupsIdsB*" �The AWS security groups to associate with the ENIs used by the replicator. If a security group is not specified, the default security group associated with the VPC is used.
�
	subnetIds*" �The list of subnets to connect to in the virtual private cloud (VPC). AWS creates elastic network interfaces inside these subnets to allow communication between your Kafka Cluster and the replicator.
:�
i
mskReplicatorReplicationInfoListCaws:msk/ReplicatorReplicationInfoList:ReplicatorReplicationInfoList�
��
consumerGroupReplications�*�:�
�
msk5ReplicatorReplicationInfoListConsumerGroupReplicationsaws:msk/ReplicatorReplicationInfoListConsumerGroupReplication:ReplicatorReplicationInfoListConsumerGroupReplication6Configuration relating to consumer group replication.

sourceKafkaClusterAliasB" B
sourceKafkaClusterArn" %The ARN of the source Kafka cluster.
e
targetCompressionType" HThe type of compression to use writing records to target Kafka cluster.

targetKafkaClusterAliasB" B
targetKafkaClusterArn" %The ARN of the target Kafka cluster.
�
topicReplications�*�:�
�
msk-ReplicatorReplicationInfoListTopicReplicationcaws:msk/ReplicatorReplicationInfoListTopicReplication:ReplicatorReplicationInfoListTopicReplication-Configuration relating to topic replication.
:�
�
msk5ReplicatorReplicationInfoListConsumerGroupReplicationsaws:msk/ReplicatorReplicationInfoListConsumerGroupReplication:ReplicatorReplicationInfoListConsumerGroupReplication�
��
consumerGroupsToExcludesB*" bList of regular expression patterns indicating the consumer groups that should not be replicated.
p
consumerGroupsToReplicates*" LList of regular expression patterns indicating the consumer groups to copy.
_
detectAndCopyNewConsumerGroupsB
 7Whether to periodically check for new consumer groups.
�
synchroniseConsumerGroupOffsetsB
 dWhether to periodically write the translated offsets to __consumer_offsets topic in target cluster.
:�

�
msk-ReplicatorReplicationInfoListTopicReplicationcaws:msk/ReplicatorReplicationInfoListTopicReplication:ReplicatorReplicationInfoListTopicReplication�	
�	�
copyAccessControlListsForTopicsB
 bWhether to periodically configure remote topic ACLs to match their corresponding upstream topics.

copyTopicConfigurationsB
 ^Whether to periodically configure remote topics to match their corresponding upstream topics.
]
detectAndCopyNewTopicsB
 =Whether to periodically check for new topics and partitions.
�
startingPosition�B�:�
�
msk=ReplicatorReplicationInfoListTopicReplicationStartingPosition�aws:msk/ReplicatorReplicationInfoListTopicReplicationStartingPosition:ReplicatorReplicationInfoListTopicReplicationStartingPositionSConfiguration for specifying the position in the topics to start replicating from.
�
topicNameConfiguration�B�:�
�
mskCReplicatorReplicationInfoListTopicReplicationTopicNameConfiguration�aws:msk/ReplicatorReplicationInfoListTopicReplicationTopicNameConfiguration:ReplicatorReplicationInfoListTopicReplicationTopicNameConfigurationr
topicsToExcludesB*" VList of regular expression patterns indicating the topics that should not be replica.
_
topicsToReplicates*" CList of regular expression patterns indicating the topics to copy.
:�
�
msk=ReplicatorReplicationInfoListTopicReplicationStartingPosition�aws:msk/ReplicatorReplicationInfoListTopicReplicationStartingPosition:ReplicatorReplicationInfoListTopicReplicationStartingPosition_
][
typeB" MThe type of replication starting position. Supports `LATEST` and `EARLIEST`.
:�
�
mskCReplicatorReplicationInfoListTopicReplicationTopicNameConfiguration�aws:msk/ReplicatorReplicationInfoListTopicReplicationTopicNameConfiguration:ReplicatorReplicationInfoListTopicReplicationTopicNameConfiguration

typeB" :�
�
msk%ServerlessClusterClientAuthenticationSaws:msk/ServerlessClusterClientAuthentication:ServerlessClusterClientAuthentication�
��
sasl�:�
�
msk)ServerlessClusterClientAuthenticationSasl[aws:msk/ServerlessClusterClientAuthenticationSasl:ServerlessClusterClientAuthenticationSasl9Details for client authentication using SASL. See below.
:�
�
msk)ServerlessClusterClientAuthenticationSasl[aws:msk/ServerlessClusterClientAuthenticationSasl:ServerlessClusterClientAuthenticationSasl�
��
iam�:�
�
msk,ServerlessClusterClientAuthenticationSaslIamaaws:msk/ServerlessClusterClientAuthenticationSaslIam:ServerlessClusterClientAuthenticationSaslIam8Details for client authentication using IAM. See below.
:�
�
msk,ServerlessClusterClientAuthenticationSaslIamaaws:msk/ServerlessClusterClientAuthenticationSaslIam:ServerlessClusterClientAuthenticationSaslIamF
DB
enabled
 3Whether SASL/IAM authentication is enabled or not.
:�
`
mskServerlessClusterVpcConfig=aws:msk/ServerlessClusterVpcConfig:ServerlessClusterVpcConfig�
��
securityGroupIdsB*" kSpecifies up to five security groups that control inbound and outbound traffic for the serverless cluster.
v
	subnetIds*" cA list of subnets in at least two different Availability Zones that host your client applications.
:�
`
mskgetBrokerNodesNodeInfoList=aws:msk/getBrokerNodesNodeInfoList:getBrokerNodesNodeInfoList�
�F
attachedEniId" 1Attached elastic network interface of the broker
!
brokerId ID of the broker
D
clientSubnet" 0Client subnet to which this broker node belongs
L
clientVpcIpAddress" 2The client virtual private cloud (VPC) IP address
Z
	endpoints*" GSet of endpoints for accessing the broker. This does not include ports

nodeArn" ARN of the node
:�
i
mskgetClusterBrokerNodeGroupInfoCaws:msk/getClusterBrokerNodeGroupInfo:getClusterBrokerNodeGroupInfo�
�
azDistribution" 
clientSubnets*" �
connectivityInfos�*�:�
�
msk-getClusterBrokerNodeGroupInfoConnectivityInfocaws:msk/getClusterBrokerNodeGroupInfoConnectivityInfo:getClusterBrokerNodeGroupInfoConnectivityInfo
instanceType" 
securityGroups*" �
storageInfos�*�:�
�
msk(getClusterBrokerNodeGroupInfoStorageInfoYaws:msk/getClusterBrokerNodeGroupInfoStorageInfo:getClusterBrokerNodeGroupInfoStorageInfo:�
�
msk-getClusterBrokerNodeGroupInfoConnectivityInfocaws:msk/getClusterBrokerNodeGroupInfoConnectivityInfo:getClusterBrokerNodeGroupInfoConnectivityInfo�
��
publicAccesses�*�:�
�
msk9getClusterBrokerNodeGroupInfoConnectivityInfoPublicAccess{aws:msk/getClusterBrokerNodeGroupInfoConnectivityInfoPublicAccess:getClusterBrokerNodeGroupInfoConnectivityInfoPublicAccess�
vpcConnectivities�*�:�
�
msk<getClusterBrokerNodeGroupInfoConnectivityInfoVpcConnectivity�aws:msk/getClusterBrokerNodeGroupInfoConnectivityInfoVpcConnectivity:getClusterBrokerNodeGroupInfoConnectivityInfoVpcConnectivity:�
�
msk9getClusterBrokerNodeGroupInfoConnectivityInfoPublicAccess{aws:msk/getClusterBrokerNodeGroupInfoConnectivityInfoPublicAccess:getClusterBrokerNodeGroupInfoConnectivityInfoPublicAccess


type" :�
�
msk<getClusterBrokerNodeGroupInfoConnectivityInfoVpcConnectivity�aws:msk/getClusterBrokerNodeGroupInfoConnectivityInfoVpcConnectivity:getClusterBrokerNodeGroupInfoConnectivityInfoVpcConnectivity�
��
clientAuthentications�*�:�
�
mskPgetClusterBrokerNodeGroupInfoConnectivityInfoVpcConnectivityClientAuthentication�aws:msk/getClusterBrokerNodeGroupInfoConnectivityInfoVpcConnectivityClientAuthentication:getClusterBrokerNodeGroupInfoConnectivityInfoVpcConnectivityClientAuthentication:�
�
mskPgetClusterBrokerNodeGroupInfoConnectivityInfoVpcConnectivityClientAuthentication�aws:msk/getClusterBrokerNodeGroupInfoConnectivityInfoVpcConnectivityClientAuthentication:getClusterBrokerNodeGroupInfoConnectivityInfoVpcConnectivityClientAuthentication�
��
sasls�*�:�
�
mskTgetClusterBrokerNodeGroupInfoConnectivityInfoVpcConnectivityClientAuthenticationSasl�aws:msk/getClusterBrokerNodeGroupInfoConnectivityInfoVpcConnectivityClientAuthenticationSasl:getClusterBrokerNodeGroupInfoConnectivityInfoVpcConnectivityClientAuthenticationSasl	
tls
 :�
�
mskTgetClusterBrokerNodeGroupInfoConnectivityInfoVpcConnectivityClientAuthenticationSasl�aws:msk/getClusterBrokerNodeGroupInfoConnectivityInfoVpcConnectivityClientAuthenticationSasl:getClusterBrokerNodeGroupInfoConnectivityInfoVpcConnectivityClientAuthenticationSasl
	
iam
 
scram
 :�
�
msk(getClusterBrokerNodeGroupInfoStorageInfoYaws:msk/getClusterBrokerNodeGroupInfoStorageInfo:getClusterBrokerNodeGroupInfoStorageInfo�
��
ebsStorageInfos�*�:�
�
msk6getClusterBrokerNodeGroupInfoStorageInfoEbsStorageInfouaws:msk/getClusterBrokerNodeGroupInfoStorageInfoEbsStorageInfo:getClusterBrokerNodeGroupInfoStorageInfoEbsStorageInfo:�
�
msk6getClusterBrokerNodeGroupInfoStorageInfoEbsStorageInfouaws:msk/getClusterBrokerNodeGroupInfoStorageInfoEbsStorageInfo:getClusterBrokerNodeGroupInfoStorageInfoEbsStorageInfo�
��
provisionedThroughputs�*�:�
�
mskKgetClusterBrokerNodeGroupInfoStorageInfoEbsStorageInfoProvisionedThroughput�aws:msk/getClusterBrokerNodeGroupInfoStorageInfoEbsStorageInfoProvisionedThroughput:getClusterBrokerNodeGroupInfoStorageInfoEbsStorageInfoProvisionedThroughput

volumeSize :�
�
mskKgetClusterBrokerNodeGroupInfoStorageInfoEbsStorageInfoProvisionedThroughput�aws:msk/getClusterBrokerNodeGroupInfoStorageInfoEbsStorageInfoProvisionedThroughput:getClusterBrokerNodeGroupInfoStorageInfoEbsStorageInfoProvisionedThroughput)
'
enabled
 
volumeThroughput :�
S

mskconnectConnectorCapacity2aws:mskconnect/ConnectorCapacity:ConnectorCapacity�
��
autoscalingzBx:v
t

mskconnectConnectorCapacityAutoscalingHaws:mskconnect/ConnectorCapacityAutoscaling:ConnectorCapacityAutoscalingfInformation about the auto scaling parameters for the connector. See `autoscaling` Block for details.
�
provisionedCapacity�B�:�
�

mskconnect$ConnectorCapacityProvisionedCapacityXaws:mskconnect/ConnectorCapacityProvisionedCapacity:ConnectorCapacityProvisionedCapacitygDetails about a fixed capacity allocated to a connector. See `provisioned_capacity` Block for details.
:�
t

mskconnectConnectorCapacityAutoscalingHaws:mskconnect/ConnectorCapacityAutoscaling:ConnectorCapacityAutoscaling�
�P
maxWorkerCount :The maximum number of workers allocated to the connector.
�
mcuCountB �The number of microcontroller units (MCUs) allocated to each connector worker. Valid values: `1`, `2`, `4`, `8`. The default value is `1`.
P
minWorkerCount :The minimum number of workers allocated to the connector.
�
scaleInPolicy�B�:�
�

mskconnect)ConnectorCapacityAutoscalingScaleInPolicybaws:mskconnect/ConnectorCapacityAutoscalingScaleInPolicy:ConnectorCapacityAutoscalingScaleInPolicyPThe scale-in policy for the connector. See `scale_in_policy` Block for details.
�
scaleOutPolicy�B�:�
�

mskconnect*ConnectorCapacityAutoscalingScaleOutPolicydaws:mskconnect/ConnectorCapacityAutoscalingScaleOutPolicy:ConnectorCapacityAutoscalingScaleOutPolicyRThe scale-out policy for the connector. See `scale_out_policy` Block for details.
:�
�

mskconnect)ConnectorCapacityAutoscalingScaleInPolicybaws:mskconnect/ConnectorCapacityAutoscalingScaleInPolicy:ConnectorCapacityAutoscalingScaleInPolicy�
��
cpuUtilizationPercentageB iSpecifies the CPU utilization percentage threshold at which you want connector scale in to be triggered.
:�
�

mskconnect*ConnectorCapacityAutoscalingScaleOutPolicydaws:mskconnect/ConnectorCapacityAutoscalingScaleOutPolicy:ConnectorCapacityAutoscalingScaleOutPolicy�
��
cpuUtilizationPercentageB `The CPU utilization percentage threshold at which you want connector scale out to be triggered.
:�
�

mskconnect$ConnectorCapacityProvisionedCapacityXaws:mskconnect/ConnectorCapacityProvisionedCapacity:ConnectorCapacityProvisionedCapacity�
��
mcuCountB �The number of microcontroller units (MCUs) allocated to each connector worker. Valid values: `1`, `2`, `4`, `8`. The default value is `1`.
N
workerCount ;The number of workers that are allocated to the connector.
:�
_

mskconnectConnectorKafkaCluster:aws:mskconnect/ConnectorKafkaCluster:ConnectorKafkaCluster�
��
apacheKafkaCluster�:�
�

mskconnect'ConnectorKafkaClusterApacheKafkaCluster^aws:mskconnect/ConnectorKafkaClusterApacheKafkaCluster:ConnectorKafkaClusterApacheKafkaClusterlThe Apache Kafka cluster to which the connector is connected. See `apache_kafka_cluster` Block for details.
:�
�

mskconnect'ConnectorKafkaClusterApacheKafkaCluster^aws:mskconnect/ConnectorKafkaClusterApacheKafkaCluster:ConnectorKafkaClusterApacheKafkaCluster�
�>
bootstrapServers" &The bootstrap servers of the cluster.
�
vpc�:�
�

mskconnect*ConnectorKafkaClusterApacheKafkaClusterVpcdaws:mskconnect/ConnectorKafkaClusterApacheKafkaClusterVpc:ConnectorKafkaClusterApacheKafkaClusterVpcrDetails of an Amazon VPC which has network connectivity to the Apache Kafka cluster. See `vpc` Block for details.
:�
�

mskconnect*ConnectorKafkaClusterApacheKafkaClusterVpcdaws:mskconnect/ConnectorKafkaClusterApacheKafkaClusterVpc:ConnectorKafkaClusterApacheKafkaClusterVpcu
s?
securityGroups*" 'The security groups for the connector.
0
subnets*" The subnets for the connector.
:�
�

mskconnect)ConnectorKafkaClusterClientAuthenticationbaws:mskconnect/ConnectorKafkaClusterClientAuthentication:ConnectorKafkaClusterClientAuthentication�
��
authenticationTypeB" �The type of client authentication used to connect to the Apache Kafka cluster. Valid values: `IAM`, `NONE`. A value of `NONE` means that no client authentication is used. The default value is `NONE`.
:�
�

mskconnect(ConnectorKafkaClusterEncryptionInTransit`aws:mskconnect/ConnectorKafkaClusterEncryptionInTransit:ConnectorKafkaClusterEncryptionInTransit�
��
encryptionTypeB" �The type of encryption in transit to the Apache Kafka cluster. Valid values: `PLAINTEXT`, `TLS`. The default values is `PLAINTEXT`.
:�
\

mskconnectConnectorLogDelivery8aws:mskconnect/ConnectorLogDelivery:ConnectorLogDelivery�
��
workerLogDelivery�:�
�

mskconnect%ConnectorLogDeliveryWorkerLogDeliveryZaws:mskconnect/ConnectorLogDeliveryWorkerLogDelivery:ConnectorLogDeliveryWorkerLogDelivery�The workers can send worker logs to different destination types. This configuration specifies the details of these destinations. See `worker_log_delivery` Block for details.
:�
�

mskconnect%ConnectorLogDeliveryWorkerLogDeliveryZaws:mskconnect/ConnectorLogDeliveryWorkerLogDelivery:ConnectorLogDeliveryWorkerLogDelivery�
��
cloudwatchLogs�B�:�
�

mskconnect3ConnectorLogDeliveryWorkerLogDeliveryCloudwatchLogsvaws:mskconnect/ConnectorLogDeliveryWorkerLogDeliveryCloudwatchLogs:ConnectorLogDeliveryWorkerLogDeliveryCloudwatchLogsbDetails about delivering logs to Amazon CloudWatch Logs. See `cloudwatch_logs` Block for details.
�
firehose�B�:�
�

mskconnect-ConnectorLogDeliveryWorkerLogDeliveryFirehosejaws:mskconnect/ConnectorLogDeliveryWorkerLogDeliveryFirehose:ConnectorLogDeliveryWorkerLogDeliveryFirehoseaDetails about delivering logs to Amazon Kinesis Data Firehose. See `firehose` Block for details.
�
s3�B�:�
�

mskconnect'ConnectorLogDeliveryWorkerLogDeliveryS3^aws:mskconnect/ConnectorLogDeliveryWorkerLogDeliveryS3:ConnectorLogDeliveryWorkerLogDeliveryS3IDetails about delivering logs to Amazon S3. See `s3` Block for deetails.
:�
�

mskconnect3ConnectorLogDeliveryWorkerLogDeliveryCloudwatchLogsvaws:mskconnect/ConnectorLogDeliveryWorkerLogDeliveryCloudwatchLogs:ConnectorLogDeliveryWorkerLogDeliveryCloudwatchLogs�
�J
enabled
 ;Whether log delivery to Amazon CloudWatch Logs is enabled.
a
logGroupB" OThe name of the CloudWatch log group that is the destination for log delivery.
:�
�

mskconnect-ConnectorLogDeliveryWorkerLogDeliveryFirehosejaws:mskconnect/ConnectorLogDeliveryWorkerLogDeliveryFirehose:ConnectorLogDeliveryWorkerLogDeliveryFirehose�
�x
deliveryStreamB" `The name of the Kinesis Data Firehose delivery stream that is the destination for log delivery.
_
enabled
 PSpecifies whether connector logs get delivered to Amazon Kinesis Data Firehose.
:�
�

mskconnect'ConnectorLogDeliveryWorkerLogDeliveryS3^aws:mskconnect/ConnectorLogDeliveryWorkerLogDeliveryS3:ConnectorLogDeliveryWorkerLogDeliveryS3�
�T
bucketB" DThe name of the S3 bucket that is the destination for log delivery.
a
enabled
 RSpecifies whether connector logs get sent to the specified Amazon S3 destination.
H
prefixB" 8The S3 prefix that is the destination for log delivery.
:�
M

mskconnectConnectorPlugin.aws:mskconnect/ConnectorPlugin:ConnectorPlugin�
��
customPluginu:s
q

mskconnectConnectorPluginCustomPluginFaws:mskconnect/ConnectorPluginCustomPlugin:ConnectorPluginCustomPluginFDetails about a custom plugin. See `custom_plugin` Block for details.
:�
q

mskconnectConnectorPluginCustomPluginFaws:mskconnect/ConnectorPluginCustomPlugin:ConnectorPluginCustomPluginy
w@
arn" 5The Amazon Resource Name (ARN) of the custom plugin.
3
revision #The revision of the custom plugin.
:�
t

mskconnectConnectorWorkerConfigurationHaws:mskconnect/ConnectorWorkerConfiguration:ConnectorWorkerConfiguration�
�G
arn" <The Amazon Resource Name (ARN) of the worker configuration.
:
revision *The revision of the worker configuration.
:�
\

mskconnectCustomPluginLocation8aws:mskconnect/CustomPluginLocation:CustomPluginLocation�
��
s3f:d
b

mskconnectCustomPluginLocationS3<aws:mskconnect/CustomPluginLocationS3:CustomPluginLocationS3QInformation of the plugin file stored in Amazon S3. See `s3` Block for details..
:�
b

mskconnectCustomPluginLocationS3<aws:mskconnect/CustomPluginLocationS3:CustomPluginLocationS3�
�A
	bucketArn" 0The Amazon Resource Name (ARN) of an S3 bucket.
;
fileKey" ,The file key for an object in an S3 bucket.
A
objectVersionB" *The version of an object in an S3 bucket.
