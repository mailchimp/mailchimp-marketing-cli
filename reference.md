# Mailchimp API CLI Reference

Full command reference for `mcapi`.

## Commands

- [`mcapi account-exports`](#mcapi-account-exports)
- [`mcapi activity-feed`](#mcapi-activity-feed)
- [`mcapi audiences`](#mcapi-audiences)
- [`mcapi authorized-apps`](#mcapi-authorized-apps)
- [`mcapi automations`](#mcapi-automations)
- [`mcapi batch-webhooks`](#mcapi-batch-webhooks)
- [`mcapi batches`](#mcapi-batches)
- [`mcapi campaign-folders`](#mcapi-campaign-folders)
- [`mcapi campaigns`](#mcapi-campaigns)
- [`mcapi connected-sites`](#mcapi-connected-sites)
- [`mcapi conversations`](#mcapi-conversations)
- [`mcapi customer-journeys`](#mcapi-customer-journeys)
- [`mcapi ecommerce`](#mcapi-ecommerce)
- [`mcapi facebook-ads`](#mcapi-facebook-ads)
- [`mcapi file-manager`](#mcapi-file-manager)
- [`mcapi landing-pages`](#mcapi-landing-pages)
- [`mcapi lists`](#mcapi-lists)
- [`mcapi ping`](#mcapi-ping)
- [`mcapi reporting`](#mcapi-reporting)
- [`mcapi reports`](#mcapi-reports)
- [`mcapi root`](#mcapi-root)
- [`mcapi search-campaigns`](#mcapi-search-campaigns)
- [`mcapi search-members`](#mcapi-search-members)
- [`mcapi sms-campaigns`](#mcapi-sms-campaigns)
- [`mcapi surveys`](#mcapi-surveys)
- [`mcapi template-folders`](#mcapi-template-folders)
- [`mcapi templates`](#mcapi-templates)
- [`mcapi verified-domains`](#mcapi-verified-domains)

---

### `mcapi account-exports`

#### `mcapi account-exports create`

Create a new account export in your Mailchimp account.

`POST /3.0/account-exports`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--json` | `JSON` | Yes | Request body as JSON (or use individual body-field flags) |

#### `mcapi account-exports get`

Get information about a specific account export.

`GET /3.0/account-exports/{export_id}`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--fields` | `string[]` | No | A comma-separated list of fields to return. Reference parameters of sub-objects with dot notation. |
| `--exclude-fields` | `string[]` | No | A comma-separated list of fields to exclude. Reference parameters of sub-objects with dot notation. |
| `--export-id` | `string` | Yes | The unique id for the account export. |

#### `mcapi account-exports list`

Get a list of account exports for a given account.

`GET /3.0/account-exports`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--fields` | `string[]` | No | A comma-separated list of fields to return. Reference parameters of sub-objects with dot notation. |
| `--exclude-fields` | `string[]` | No | A comma-separated list of fields to exclude. Reference parameters of sub-objects with dot notation. |
| `--count` | `integer` | No | The number of records to return. Default value is 10. Maximum value is 1000 |
| `--offset` | `integer` | No | Used for [pagination](https://mailchimp.com/developer/marketing/docs/methods-parameters/#pagination), this is the number of records from a collection to skip. Default value is 0. |

---

### `mcapi activity-feed`

#### `mcapi activity-feed list`

Get information about the activity feed endpoint's resources.

`GET /3.0/activity-feed`

#### `mcapi activity-feed list-chimp-chatter`

Return the Chimp Chatter for this account ordered by most recent.

`GET /3.0/activity-feed/chimp-chatter`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--count` | `integer` | No | The number of records to return. Default value is 10. Maximum value is 1000 |
| `--offset` | `integer` | No | Used for [pagination](https://mailchimp.com/developer/marketing/docs/methods-parameters/#pagination), this is the number of records from a collection to skip. Default value is 0. |

---

### `mcapi audiences`

#### `mcapi audiences create-audience-contact`

Create a new omni-channel contact for an audience.

`POST /3.0/audiences/{audience_id}/contacts`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--audience-id` | `string` | Yes | The unique ID for the audience. |
| `--merge-field-validation-mode` | `ignore_required_checks | strict` | No | Defines how merge field validation is handled. When set to `ignore_required_checks`, the API does not raise an error if required merge fields are missing from the request. When set to `strict`, the API enforces validation and returns an error if any required merge field is not provided. If this setting is omitted, `strict` is applied by default. |
| `--data-mode` | `historical | live` | No | Indicates the data processing mode. In `historical` mode, contact data changes do not trigger automations or webhooks. In `live mode`, such changes do trigger them. |
| `--json` | `JSON` | Yes | Request body as JSON (or use individual body-field flags) |

#### `mcapi audiences get-audience-contact`

Retrieve a specific omni-channel contact in an audience.

`GET /3.0/audiences/{audience_id}/contacts/{contact_id}`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--fields` | `string[]` | No | A comma-separated list of fields to return. Reference parameters of sub-objects with dot notation. |
| `--exclude-fields` | `string[]` | No | A comma-separated list of fields to exclude. Reference parameters of sub-objects with dot notation. |
| `--audience-id` | `string` | Yes | The unique ID for the audience. |
| `--contact-id` | `string` | Yes | A unique identifier for the contact, which can be a Mailchimp contact ID or a channel hash. A channel hash must follow the format email:[md5_hash] (where the hash is the MD5 of the lowercased email address) or sms:[sha256_hash] (where the hash is the SHA256 of the E.164-formatted phone number). |

#### `mcapi audiences get-audience-contact-list`

Get a list of omni-channel contacts for a given audience.

`GET /3.0/audiences/{audience_id}/contacts`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--fields` | `string[]` | No | A comma-separated list of fields to return. Reference parameters of sub-objects with dot notation. |
| `--exclude-fields` | `string[]` | No | A comma-separated list of fields to exclude. Reference parameters of sub-objects with dot notation. |
| `--count` | `integer` | No | The number of records to return. Default value is 10. Maximum value is 1000 |
| `--cursor` | `string` | No | Paginate through a collection of records by setting the `cursor` parameter to a `next_cursor` attribute returned by a previous request. Default value fetches the first "page" of results. |
| `--created-before` | `string (date-time)` | No | Restricts the response to contacts created at or before the specified time (inclusive). Uses ISO 8601 format: 2025-04-23T15:41:36+00:00. |
| `--created-since` | `string (date-time)` | No | Restricts the response to contacts created after the specified time (exclusive). Uses ISO 8601 format: 2025-04-23T15:41:36+00:00. |
| `--updated-before` | `string (date-time)` | No | Restricts the response to contacts updated at or before the specified time (inclusive). Uses ISO 8601 format: 2025-04-23T15:41:36+00:00. |
| `--updated-since` | `string (date-time)` | No | Restricts the response to contacts updated after the specified time (exclusive). Uses ISO 8601 format: 2025-04-23T15:41:36+00:00. |
| `--audience-id` | `string` | Yes | The unique ID for the audience. |
| `--sort-field` | `created_at | updated_at` | No | Specifies the field to sort the returned contacts by. |
| `--sort-dir` | `ASC | DESC` | No | Determines the order direction for sorted results. |

#### `mcapi audiences patch-audience-contact`

Update an existing omni-channel contact.

`PATCH /3.0/audiences/{audience_id}/contacts/{contact_id}`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--audience-id` | `string` | Yes | The unique ID for the audience. |
| `--contact-id` | `string` | Yes | The unique id for the contact. |
| `--merge-field-validation-mode` | `ignore_required_checks | strict` | No | Defines how merge field validation is handled. When set to `ignore_required_checks`, the API does not raise an error if required merge fields are missing from the request. When set to `strict`, the API enforces validation and returns an error if any required merge field is not provided. If this setting is omitted, `strict` is applied by default. |
| `--data-mode` | `historical | live` | No | Indicates the data processing mode. In `historical` mode, contact data changes do not trigger automations or webhooks. In `live mode`, such changes do trigger them. |
| `--json` | `JSON` | Yes | Request body as JSON (or use individual body-field flags) |

#### `mcapi audiences post-audiences-contacts-actions-archive`

Archives a Contact.

`POST /3.0/audiences/{audience_id}/contacts/{contact_id}/actions/archive`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--audience-id` | `string` | Yes | The unique ID for the audience. |
| `--contact-id` | `string` | Yes | The unique id for the contact. |

#### `mcapi audiences post-audiences-contacts-actions-forget`

Forgets a Contact.

`POST /3.0/audiences/{audience_id}/contacts/{contact_id}/actions/forget`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--audience-id` | `string` | Yes | The unique ID for the audience. |
| `--contact-id` | `string` | Yes | The unique id for the contact. |

---

### `mcapi authorized-apps`

#### `mcapi authorized-apps get`

Get information about a specific authorized application.

`GET /3.0/authorized-apps/{app_id}`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--fields` | `string[]` | No | A comma-separated list of fields to return. Reference parameters of sub-objects with dot notation. |
| `--exclude-fields` | `string[]` | No | A comma-separated list of fields to exclude. Reference parameters of sub-objects with dot notation. |
| `--app-id` | `string` | Yes | The unique id for the connected authorized application. |

#### `mcapi authorized-apps list`

Get a list of an account's registered, connected applications.

`GET /3.0/authorized-apps`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--fields` | `string[]` | No | A comma-separated list of fields to return. Reference parameters of sub-objects with dot notation. |
| `--exclude-fields` | `string[]` | No | A comma-separated list of fields to exclude. Reference parameters of sub-objects with dot notation. |
| `--count` | `integer` | No | The number of records to return. Default value is 10. Maximum value is 1000 |
| `--offset` | `integer` | No | Used for [pagination](https://mailchimp.com/developer/marketing/docs/methods-parameters/#pagination), this is the number of records from a collection to skip. Default value is 0. |

---

### `mcapi automations`

#### `mcapi automations create`

Create a new classic automation in your Mailchimp account.

`POST /3.0/automations`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--json` | `JSON` | Yes | Request body as JSON (or use individual body-field flags) |

#### `mcapi automations create-action-archive`

Archiving will permanently end your automation and keep the report data. You’ll be able to replicate your archived automation, but you can’t restart it.

`POST /3.0/automations/{workflow_id}/actions/archive`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--workflow-id` | `string` | Yes | The unique id for the Automation workflow. |

#### `mcapi automations create-action-pause-all-email`

Pause all emails in a specific classic automation workflow.

`POST /3.0/automations/{workflow_id}/actions/pause-all-emails`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--workflow-id` | `string` | Yes | The unique id for the Automation workflow. |

#### `mcapi automations create-action-start-all-email`

Start all emails in a classic automation workflow.

`POST /3.0/automations/{workflow_id}/actions/start-all-emails`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--workflow-id` | `string` | Yes | The unique id for the Automation workflow. |

#### `mcapi automations create-email-action-pause`

Pause an automated email.

`POST /3.0/automations/{workflow_id}/emails/{workflow_email_id}/actions/pause`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--workflow-id` | `string` | Yes | The unique id for the Automation workflow. |
| `--workflow-email-id` | `string` | Yes | The unique id for the Automation workflow email. |

#### `mcapi automations create-email-action-start`

Start an automated email.

`POST /3.0/automations/{workflow_id}/emails/{workflow_email_id}/actions/start`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--workflow-id` | `string` | Yes | The unique id for the Automation workflow. |
| `--workflow-email-id` | `string` | Yes | The unique id for the Automation workflow email. |

#### `mcapi automations create-email-queue`

Manually add a subscriber to a workflow, bypassing the default trigger settings. You can also use this endpoint to trigger a series of automated emails in an API 3.0 workflow type.

`POST /3.0/automations/{workflow_id}/emails/{workflow_email_id}/queue`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--workflow-id` | `string` | Yes | The unique id for the Automation workflow. |
| `--workflow-email-id` | `string` | Yes | The unique id for the Automation workflow email. |
| `--json` | `JSON` | Yes | Request body as JSON (or use individual body-field flags) |

#### `mcapi automations create-removed-subscriber`

Remove a subscriber from a specific classic automation workflow. You can remove a subscriber at any point in an automation workflow, regardless of how many emails they've been sent from that workflow. Once they're removed, they can never be added back to the same workflow.

`POST /3.0/automations/{workflow_id}/removed-subscribers`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--workflow-id` | `string` | Yes | The unique id for the Automation workflow. |
| `--json` | `JSON` | Yes | Request body as JSON (or use individual body-field flags) |

#### `mcapi automations delete-email`

Removes an individual classic automation workflow email. Emails from certain workflow types, including the Abandoned Cart Email (abandonedCart) and Product Retargeting Email (abandonedBrowse) Workflows, cannot be deleted.

`DELETE /3.0/automations/{workflow_id}/emails/{workflow_email_id}`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--workflow-id` | `string` | Yes | The unique id for the Automation workflow. |
| `--workflow-email-id` | `string` | Yes | The unique id for the Automation workflow email. |

#### `mcapi automations get`

Get a summary of an individual classic automation workflow's settings and content. The `trigger_settings` object returns information for the first email in the workflow.

`GET /3.0/automations/{workflow_id}`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--fields` | `string[]` | No | A comma-separated list of fields to return. Reference parameters of sub-objects with dot notation. |
| `--exclude-fields` | `string[]` | No | A comma-separated list of fields to exclude. Reference parameters of sub-objects with dot notation. |
| `--workflow-id` | `string` | Yes | The unique id for the Automation workflow. |

#### `mcapi automations get-email`

Get information about an individual classic automation workflow email.

`GET /3.0/automations/{workflow_id}/emails/{workflow_email_id}`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--workflow-id` | `string` | Yes | The unique id for the Automation workflow. |
| `--workflow-email-id` | `string` | Yes | The unique id for the Automation workflow email. |

#### `mcapi automations get-email-queue`

Get information about a specific subscriber in a classic automation email queue.

`GET /3.0/automations/{workflow_id}/emails/{workflow_email_id}/queue/{subscriber_hash}`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--workflow-id` | `string` | Yes | The unique id for the Automation workflow. |
| `--workflow-email-id` | `string` | Yes | The unique id for the Automation workflow email. |
| `--subscriber-hash` | `string` | Yes | The MD5 hash of the lowercase version of the list member's email address. |

#### `mcapi automations get-removed-subscriber`

Get information about a specific subscriber who was removed from a classic automation workflow.

`GET /3.0/automations/{workflow_id}/removed-subscribers/{subscriber_hash}`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--workflow-id` | `string` | Yes | The unique id for the Automation workflow. |
| `--subscriber-hash` | `string` | Yes | The MD5 hash of the lowercase version of the list member's email address. |

#### `mcapi automations list`

Get a summary of an account's classic automations.

`GET /3.0/automations`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--count` | `integer` | No | The number of records to return. Default value is 10. Maximum value is 1000 |
| `--offset` | `integer` | No | Used for [pagination](https://mailchimp.com/developer/marketing/docs/methods-parameters/#pagination), this is the number of records from a collection to skip. Default value is 0. |
| `--fields` | `string[]` | No | A comma-separated list of fields to return. Reference parameters of sub-objects with dot notation. |
| `--exclude-fields` | `string[]` | No | A comma-separated list of fields to exclude. Reference parameters of sub-objects with dot notation. |
| `--before-create-time` | `string (date-time)` | No | Restrict the response to automations created before this time. Uses the ISO 8601 time format: 2015-10-21T15:41:36+00:00. |
| `--since-create-time` | `string (date-time)` | No | Restrict the response to automations created after this time. Uses the ISO 8601 time format: 2015-10-21T15:41:36+00:00. |
| `--before-start-time` | `string (date-time)` | No | Restrict the response to automations started before this time. Uses the ISO 8601 time format: 2015-10-21T15:41:36+00:00. |
| `--since-start-time` | `string (date-time)` | No | Restrict the response to automations started after this time. Uses the ISO 8601 time format: 2015-10-21T15:41:36+00:00. |
| `--status` | `save | paused | sending` | No | Restrict the results to automations with the specified status. |

#### `mcapi automations list-email-queue`

Get information about a classic automation email queue.

`GET /3.0/automations/{workflow_id}/emails/{workflow_email_id}/queue`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--workflow-id` | `string` | Yes | The unique id for the Automation workflow. |
| `--workflow-email-id` | `string` | Yes | The unique id for the Automation workflow email. |

#### `mcapi automations list-emails`

Get a summary of the emails in a classic automation workflow.

`GET /3.0/automations/{workflow_id}/emails`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--workflow-id` | `string` | Yes | The unique id for the Automation workflow. |

#### `mcapi automations list-removed-subscribers`

Get information about subscribers who were removed from a classic automation workflow.

`GET /3.0/automations/{workflow_id}/removed-subscribers`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--workflow-id` | `string` | Yes | The unique id for the Automation workflow. |

#### `mcapi automations update-email`

Update settings for a classic automation workflow email.  Only works with workflows of type: abandonedBrowse, abandonedCart, emailFollowup, or singleWelcome.

`PATCH /3.0/automations/{workflow_id}/emails/{workflow_email_id}`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--workflow-id` | `string` | Yes | The unique id for the Automation workflow. |
| `--workflow-email-id` | `string` | Yes | The unique id for the Automation workflow email. |
| `--json` | `JSON` | Yes | Request body as JSON (or use individual body-field flags) |

---

### `mcapi batch-webhooks`

#### `mcapi batch-webhooks create`

Configure a webhook that will fire whenever any batch request completes processing.  You may only have a maximum of 20 batch webhooks.

`POST /3.0/batch-webhooks`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--json` | `JSON` | Yes | Request body as JSON (or use individual body-field flags) |

#### `mcapi batch-webhooks delete`

Remove a batch webhook. Webhooks will no longer be sent to the given URL.

`DELETE /3.0/batch-webhooks/{batch_webhook_id}`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--batch-webhook-id` | `string` | Yes | The unique id for the batch webhook. |

#### `mcapi batch-webhooks get`

Get information about a specific batch webhook.

`GET /3.0/batch-webhooks/{batch_webhook_id}`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--fields` | `string[]` | No | A comma-separated list of fields to return. Reference parameters of sub-objects with dot notation. |
| `--exclude-fields` | `string[]` | No | A comma-separated list of fields to exclude. Reference parameters of sub-objects with dot notation. |
| `--batch-webhook-id` | `string` | Yes | The unique id for the batch webhook. |

#### `mcapi batch-webhooks list`

Get all webhooks that have been configured for batches.

`GET /3.0/batch-webhooks`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--fields` | `string[]` | No | A comma-separated list of fields to return. Reference parameters of sub-objects with dot notation. |
| `--exclude-fields` | `string[]` | No | A comma-separated list of fields to exclude. Reference parameters of sub-objects with dot notation. |
| `--count` | `integer` | No | The number of records to return. Default value is 10. Maximum value is 1000 |
| `--offset` | `integer` | No | Used for [pagination](https://mailchimp.com/developer/marketing/docs/methods-parameters/#pagination), this is the number of records from a collection to skip. Default value is 0. |

#### `mcapi batch-webhooks update`

Update a webhook that will fire whenever any batch request completes processing.

`PATCH /3.0/batch-webhooks/{batch_webhook_id}`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--batch-webhook-id` | `string` | Yes | The unique id for the batch webhook. |
| `--json` | `JSON` | Yes | Request body as JSON (or use individual body-field flags) |

---

### `mcapi batches`

#### `mcapi batches create`

Begin processing a batch operations request.

`POST /3.0/batches`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--json` | `JSON` | Yes | Request body as JSON (or use individual body-field flags) |

#### `mcapi batches delete`

Stops a batch request from running. Since only one batch request is run at a time, this can be used to cancel a long running request. The results of any completed operations will not be available after this call.

`DELETE /3.0/batches/{batch_id}`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--batch-id` | `string` | Yes | The unique id for the batch operation. |

#### `mcapi batches get`

Get the status of a batch request.

`GET /3.0/batches/{batch_id}`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--fields` | `string[]` | No | A comma-separated list of fields to return. Reference parameters of sub-objects with dot notation. |
| `--exclude-fields` | `string[]` | No | A comma-separated list of fields to exclude. Reference parameters of sub-objects with dot notation. |
| `--batch-id` | `string` | Yes | The unique id for the batch operation. |

#### `mcapi batches list`

Get a summary of batch requests that have been made.

`GET /3.0/batches`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--fields` | `string[]` | No | A comma-separated list of fields to return. Reference parameters of sub-objects with dot notation. |
| `--exclude-fields` | `string[]` | No | A comma-separated list of fields to exclude. Reference parameters of sub-objects with dot notation. |
| `--count` | `integer` | No | The number of records to return. Default value is 10. Maximum value is 1000 |
| `--offset` | `integer` | No | Used for [pagination](https://mailchimp.com/developer/marketing/docs/methods-parameters/#pagination), this is the number of records from a collection to skip. Default value is 0. |

---

### `mcapi campaign-folders`

#### `mcapi campaign-folders create`

Create a new campaign folder.

`POST /3.0/campaign-folders`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--json` | `JSON` | Yes | Request body as JSON (or use individual body-field flags) |

#### `mcapi campaign-folders delete`

Delete a specific campaign folder, and mark all the campaigns in the folder as 'unfiled'.

`DELETE /3.0/campaign-folders/{folder_id}`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--folder-id` | `string` | Yes | The unique id for the campaign folder. |

#### `mcapi campaign-folders get`

Get information about a specific folder used to organize campaigns.

`GET /3.0/campaign-folders/{folder_id}`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--fields` | `string[]` | No | A comma-separated list of fields to return. Reference parameters of sub-objects with dot notation. |
| `--exclude-fields` | `string[]` | No | A comma-separated list of fields to exclude. Reference parameters of sub-objects with dot notation. |
| `--folder-id` | `string` | Yes | The unique id for the campaign folder. |

#### `mcapi campaign-folders list`

Get all folders used to organize campaigns.

`GET /3.0/campaign-folders`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--fields` | `string[]` | No | A comma-separated list of fields to return. Reference parameters of sub-objects with dot notation. |
| `--exclude-fields` | `string[]` | No | A comma-separated list of fields to exclude. Reference parameters of sub-objects with dot notation. |
| `--count` | `integer` | No | The number of records to return. Default value is 10. Maximum value is 1000 |
| `--offset` | `integer` | No | Used for [pagination](https://mailchimp.com/developer/marketing/docs/methods-parameters/#pagination), this is the number of records from a collection to skip. Default value is 0. |

#### `mcapi campaign-folders update`

Update a specific folder used to organize campaigns.

`PATCH /3.0/campaign-folders/{folder_id}`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--folder-id` | `string` | Yes | The unique id for the campaign folder. |
| `--json` | `JSON` | Yes | Request body as JSON (or use individual body-field flags) |

---

### `mcapi campaigns`

#### `mcapi campaigns create`

Create a new Mailchimp campaign.

`POST /3.0/campaigns`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--json` | `JSON` | Yes | Request body as JSON (or use individual body-field flags) |

#### `mcapi campaigns create-action-cancel-send`

Cancel a Regular or Plain-Text Campaign after you send, before all of your recipients receive it. This feature is included with Mailchimp Pro.

`POST /3.0/campaigns/{campaign_id}/actions/cancel-send`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--campaign-id` | `string` | Yes | The unique id for the campaign. |

#### `mcapi campaigns create-action-create-resend`

Remove the guesswork for resending a campaign to certain segments. You can use this endpoint as a shortcut to replicate a campaign and resend it to common segments, such as those who didn't open the campaign, or any new subscribers since it was sent.

`POST /3.0/campaigns/{campaign_id}/actions/create-resend`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--campaign-id` | `string` | Yes | The unique id for the campaign. |
| `--json` | `JSON` | No | Request body as JSON (or use individual body-field flags) |

#### `mcapi campaigns create-action-pause`

Pause an RSS-Driven campaign.

`POST /3.0/campaigns/{campaign_id}/actions/pause`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--campaign-id` | `string` | Yes | The unique id for the campaign. |

#### `mcapi campaigns create-action-replicate`

Replicate a campaign in saved or send status.

`POST /3.0/campaigns/{campaign_id}/actions/replicate`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--campaign-id` | `string` | Yes | The unique id for the campaign. |

#### `mcapi campaigns create-action-resume`

Resume an RSS-Driven campaign.

`POST /3.0/campaigns/{campaign_id}/actions/resume`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--campaign-id` | `string` | Yes | The unique id for the campaign. |

#### `mcapi campaigns create-action-schedule`

Schedule a campaign for delivery. If you're using Multivariate Campaigns to test send times or sending RSS Campaigns, use the send action instead.

`POST /3.0/campaigns/{campaign_id}/actions/schedule`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--campaign-id` | `string` | Yes | The unique id for the campaign. |
| `--json` | `JSON` | Yes | Request body as JSON (or use individual body-field flags) |

#### `mcapi campaigns create-action-send`

Send a Mailchimp campaign. For RSS Campaigns, the campaign will send according to its schedule. All other campaigns will send immediately.

`POST /3.0/campaigns/{campaign_id}/actions/send`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--campaign-id` | `string` | Yes | The unique id for the campaign. |

#### `mcapi campaigns create-action-test`

Send a test email.

`POST /3.0/campaigns/{campaign_id}/actions/test`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--campaign-id` | `string` | Yes | The unique id for the campaign. |
| `--json` | `JSON` | Yes | Request body as JSON (or use individual body-field flags) |

#### `mcapi campaigns create-action-unschedule`

Unschedule a scheduled campaign that hasn't started sending.

`POST /3.0/campaigns/{campaign_id}/actions/unschedule`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--campaign-id` | `string` | Yes | The unique id for the campaign. |

#### `mcapi campaigns create-feedback`

Add feedback on a specific campaign.

`POST /3.0/campaigns/{campaign_id}/feedback`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--campaign-id` | `string` | Yes | The unique id for the campaign. |
| `--json` | `JSON` | Yes | Request body as JSON (or use individual body-field flags) |

#### `mcapi campaigns delete`

Remove a campaign from your Mailchimp account.

`DELETE /3.0/campaigns/{campaign_id}`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--campaign-id` | `string` | Yes | The unique id for the campaign. |

#### `mcapi campaigns delete-feedback`

Remove a specific feedback message for a campaign.

`DELETE /3.0/campaigns/{campaign_id}/feedback/{feedback_id}`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--campaign-id` | `string` | Yes | The unique id for the campaign. |
| `--feedback-id` | `string` | Yes | The unique id for the feedback message. |

#### `mcapi campaigns get`

Get information about a specific campaign.

`GET /3.0/campaigns/{campaign_id}`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--fields` | `string[]` | No | A comma-separated list of fields to return. Reference parameters of sub-objects with dot notation. |
| `--exclude-fields` | `string[]` | No | A comma-separated list of fields to exclude. Reference parameters of sub-objects with dot notation. |
| `--campaign-id` | `string` | Yes | The unique id for the campaign. |
| `--include-resend-shortcut-eligibility` | `boolean` | No | Return the `resend_shortcut_eligibility` field in the response, which tells you if the campaign is eligible for the various Campaign Resend Shortcuts offered. |
| `--include-resend-shortcut-usage` | `boolean` | No | Return the `resend_shortcut_usage` field in the response.  This includes information about campaigns related by a shortcut. |

#### `mcapi campaigns get-content`

Get the the HTML and plain-text content for a campaign.

`GET /3.0/campaigns/{campaign_id}/content`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--fields` | `string[]` | No | A comma-separated list of fields to return. Reference parameters of sub-objects with dot notation. |
| `--exclude-fields` | `string[]` | No | A comma-separated list of fields to exclude. Reference parameters of sub-objects with dot notation. |
| `--campaign-id` | `string` | Yes | The unique id for the campaign. |

#### `mcapi campaigns get-feedback`

Get a specific feedback message from a campaign.

`GET /3.0/campaigns/{campaign_id}/feedback/{feedback_id}`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--fields` | `string[]` | No | A comma-separated list of fields to return. Reference parameters of sub-objects with dot notation. |
| `--exclude-fields` | `string[]` | No | A comma-separated list of fields to exclude. Reference parameters of sub-objects with dot notation. |
| `--campaign-id` | `string` | Yes | The unique id for the campaign. |
| `--feedback-id` | `string` | Yes | The unique id for the feedback message. |

#### `mcapi campaigns list`

Get all campaigns in an account.

`GET /3.0/campaigns`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--fields` | `string[]` | No | A comma-separated list of fields to return. Reference parameters of sub-objects with dot notation. |
| `--exclude-fields` | `string[]` | No | A comma-separated list of fields to exclude. Reference parameters of sub-objects with dot notation. |
| `--count` | `integer` | No | The number of records to return. Default value is 10. Maximum value is 1000 |
| `--offset` | `integer` | No | Used for [pagination](https://mailchimp.com/developer/marketing/docs/methods-parameters/#pagination), this is the number of records from a collection to skip. Default value is 0. |
| `--type` | `regular | plaintext | absplit | rss | variate` | No | The campaign type. |
| `--status` | `save | paused | schedule | sending | sent` | No | The status of the campaign. |
| `--before-send-time` | `string (date-time)` | No | Restrict the response to campaigns sent before the set time. Uses ISO 8601 time format: 2015-10-21T15:41:36+00:00. |
| `--since-send-time` | `string (date-time)` | No | Restrict the response to campaigns sent after the set time. Uses ISO 8601 time format: 2015-10-21T15:41:36+00:00. |
| `--before-create-time` | `string (date-time)` | No | Restrict the response to campaigns created before the set time. Uses ISO 8601 time format: 2015-10-21T15:41:36+00:00. |
| `--since-create-time` | `string (date-time)` | No | Restrict the response to campaigns created after the set time. Uses ISO 8601 time format: 2015-10-21T15:41:36+00:00. |
| `--list-id` | `string` | No | The unique id for the list. |
| `--folder-id` | `string` | No | The unique folder id. |
| `--member-id` | `string` | No | Retrieve campaigns sent to a particular list member. Member ID is The MD5 hash of the lowercase version of the list member’s email address. |
| `--sort-field` | `create_time | send_time` | No | Returns files sorted by the specified field. |
| `--sort-dir` | `ASC | DESC` | No | Determines the order direction for sorted results. |
| `--include-resend-shortcut-eligibility` | `boolean` | No | Return the `resend_shortcut_eligibility` field in the response, which tells you if the campaign is eligible for the various Campaign Resend Shortcuts offered. |
| `--include-resend-shortcut-usage` | `boolean` | No | Return the `resend_shortcut_usage` field in the response.  This includes information about campaigns related by a shortcut. |

#### `mcapi campaigns list-feedback`

Get team feedback while you're working together on a Mailchimp campaign.

`GET /3.0/campaigns/{campaign_id}/feedback`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--fields` | `string[]` | No | A comma-separated list of fields to return. Reference parameters of sub-objects with dot notation. |
| `--exclude-fields` | `string[]` | No | A comma-separated list of fields to exclude. Reference parameters of sub-objects with dot notation. |
| `--campaign-id` | `string` | Yes | The unique id for the campaign. |

#### `mcapi campaigns list-send-checklist`

Review the send checklist for a campaign, and resolve any issues before sending.

`GET /3.0/campaigns/{campaign_id}/send-checklist`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--fields` | `string[]` | No | A comma-separated list of fields to return. Reference parameters of sub-objects with dot notation. |
| `--exclude-fields` | `string[]` | No | A comma-separated list of fields to exclude. Reference parameters of sub-objects with dot notation. |
| `--campaign-id` | `string` | Yes | The unique id for the campaign. |

#### `mcapi campaigns update`

Update some or all of the settings for a specific campaign.

`PATCH /3.0/campaigns/{campaign_id}`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--campaign-id` | `string` | Yes | The unique id for the campaign. |
| `--json` | `JSON` | Yes | Request body as JSON (or use individual body-field flags) |

#### `mcapi campaigns update-feedback`

Update a specific feedback message for a campaign.

`PATCH /3.0/campaigns/{campaign_id}/feedback/{feedback_id}`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--campaign-id` | `string` | Yes | The unique id for the campaign. |
| `--feedback-id` | `string` | Yes | The unique id for the feedback message. |
| `--json` | `JSON` | Yes | Request body as JSON (or use individual body-field flags) |

#### `mcapi campaigns upsert-content`

Set the content for a campaign.

`PUT /3.0/campaigns/{campaign_id}/content`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--campaign-id` | `string` | Yes | The unique id for the campaign. |
| `--json` | `JSON` | Yes | Request body as JSON (or use individual body-field flags) |

---

### `mcapi connected-sites`

#### `mcapi connected-sites create`

Create a new Mailchimp connected site.

`POST /3.0/connected-sites`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--json` | `JSON` | Yes | Request body as JSON (or use individual body-field flags) |

#### `mcapi connected-sites create-action-verify-script-installation`

Verify that the connected sites script has been installed, either via the script URL or fragment.

`POST /3.0/connected-sites/{connected_site_id}/actions/verify-script-installation`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--connected-site-id` | `string` | Yes | The unique identifier for the site. |

#### `mcapi connected-sites delete`

Remove a connected site from your Mailchimp account.

`DELETE /3.0/connected-sites/{connected_site_id}`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--connected-site-id` | `string` | Yes | The unique identifier for the site. |

#### `mcapi connected-sites get`

Get information about a specific connected site.

`GET /3.0/connected-sites/{connected_site_id}`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--fields` | `string[]` | No | A comma-separated list of fields to return. Reference parameters of sub-objects with dot notation. |
| `--exclude-fields` | `string[]` | No | A comma-separated list of fields to exclude. Reference parameters of sub-objects with dot notation. |
| `--connected-site-id` | `string` | Yes | The unique identifier for the site. |

#### `mcapi connected-sites list`

Get all connected sites in an account.

`GET /3.0/connected-sites`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--fields` | `string[]` | No | A comma-separated list of fields to return. Reference parameters of sub-objects with dot notation. |
| `--exclude-fields` | `string[]` | No | A comma-separated list of fields to exclude. Reference parameters of sub-objects with dot notation. |
| `--count` | `integer` | No | The number of records to return. Default value is 10. Maximum value is 1000 |
| `--offset` | `integer` | No | Used for [pagination](https://mailchimp.com/developer/marketing/docs/methods-parameters/#pagination), this is the number of records from a collection to skip. Default value is 0. |

---

### `mcapi conversations`

#### `mcapi conversations get`

Get details about an individual conversation. Conversations has been deprecated in favor of Inbox and these endpoints don't include Inbox data. Past Conversations are still available via this endpoint, but new campaign replies and other Inbox messages aren’t available using this endpoint.

`GET /3.0/conversations/{conversation_id}`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--fields` | `string[]` | No | A comma-separated list of fields to return. Reference parameters of sub-objects with dot notation. |
| `--exclude-fields` | `string[]` | No | A comma-separated list of fields to exclude. Reference parameters of sub-objects with dot notation. |
| `--conversation-id` | `string` | Yes | The unique id for the conversation. |

#### `mcapi conversations get-message`

Get an individual message in a conversation. Conversations has been deprecated in favor of Inbox and these endpoints don't include Inbox data. Past Conversations are still available via this endpoint, but new campaign replies and other Inbox messages aren’t available using this endpoint.

`GET /3.0/conversations/{conversation_id}/messages/{message_id}`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--fields` | `string[]` | No | A comma-separated list of fields to return. Reference parameters of sub-objects with dot notation. |
| `--exclude-fields` | `string[]` | No | A comma-separated list of fields to exclude. Reference parameters of sub-objects with dot notation. |
| `--conversation-id` | `string` | Yes | The unique id for the conversation. |
| `--message-id` | `string` | Yes | The unique id for the conversation message. |

#### `mcapi conversations list`

Get a list of conversations for the account. Conversations has been deprecated in favor of Inbox and these endpoints don't include Inbox data. Past Conversations are still available via this endpoint, but new campaign replies and other Inbox messages aren’t available using this endpoint.

`GET /3.0/conversations`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--fields` | `string[]` | No | A comma-separated list of fields to return. Reference parameters of sub-objects with dot notation. |
| `--exclude-fields` | `string[]` | No | A comma-separated list of fields to exclude. Reference parameters of sub-objects with dot notation. |
| `--count` | `integer` | No | The number of records to return. Default value is 10. Maximum value is 1000 |
| `--offset` | `integer` | No | Used for [pagination](https://mailchimp.com/developer/marketing/docs/methods-parameters/#pagination), this is the number of records from a collection to skip. Default value is 0. |
| `--has-unread-messages` | `true | false` | No | Whether the conversation has any unread messages. |
| `--list-id` | `string` | No | The unique id for the list. |
| `--campaign-id` | `string` | No | The unique id for the campaign. |

#### `mcapi conversations list-messages`

Get messages from a specific conversation. Conversations has been deprecated in favor of Inbox and these endpoints don't include Inbox data. Past Conversations are still available via this endpoint, but new campaign replies and other Inbox messages aren’t available using this endpoint.

`GET /3.0/conversations/{conversation_id}/messages`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--fields` | `string[]` | No | A comma-separated list of fields to return. Reference parameters of sub-objects with dot notation. |
| `--exclude-fields` | `string[]` | No | A comma-separated list of fields to exclude. Reference parameters of sub-objects with dot notation. |
| `--conversation-id` | `string` | Yes | The unique id for the conversation. |
| `--is-read` | `true | false` | No | Whether a conversation message has been marked as read. |
| `--before-timestamp` | `string (date-time)` | No | Restrict the response to messages created before the set time. Uses ISO 8601 time format: 2015-10-21T15:41:36+00:00. |
| `--since-timestamp` | `string (date-time)` | No | Restrict the response to messages created after the set time. Uses ISO 8601 time format: 2015-10-21T15:41:36+00:00. |

---

### `mcapi customer-journeys`

#### `mcapi customer-journeys create-journey-step-action-trigger`

A step trigger in an Automation flow. To use it, create a starting point or step from the Automation flow builder in the app using the Customer Journeys API condition. We’ll provide a url during the process that includes the {journey_id} and {step_id}. You’ll then be able to use this endpoint to trigger the condition for the posted contact.

`POST /3.0/customer-journeys/journeys/{journey_id}/steps/{step_id}/actions/trigger`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--journey-id` | `integer` | Yes | The id for the flow. |
| `--step-id` | `integer` | Yes | The id for the Step. |
| `--json` | `JSON` | Yes | Request body as JSON (or use individual body-field flags) |

---

### `mcapi ecommerce`

#### `mcapi ecommerce create-store`

Add a new store to your Mailchimp account.

`POST /3.0/ecommerce/stores`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--json` | `JSON` | Yes | Request body as JSON (or use individual body-field flags) |

#### `mcapi ecommerce create-store-cart`

Add a new cart to a store.

`POST /3.0/ecommerce/stores/{store_id}/carts`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--store-id` | `string` | Yes | The store id. |
| `--json` | `JSON` | Yes | Request body as JSON (or use individual body-field flags) |

#### `mcapi ecommerce create-store-cart-line`

Add a new line item to an existing cart.

`POST /3.0/ecommerce/stores/{store_id}/carts/{cart_id}/lines`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--store-id` | `string` | Yes | The store id. |
| `--cart-id` | `string` | Yes | The id for the cart. |
| `--json` | `JSON` | Yes | Request body as JSON (or use individual body-field flags) |

#### `mcapi ecommerce create-store-customer`

Add a new customer to a store.

`POST /3.0/ecommerce/stores/{store_id}/customers`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--store-id` | `string` | Yes | The store id. |
| `--json` | `JSON` | Yes | Request body as JSON (or use individual body-field flags) |

#### `mcapi ecommerce create-store-order`

Add a new order to a store.

`POST /3.0/ecommerce/stores/{store_id}/orders`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--store-id` | `string` | Yes | The store id. |
| `--json` | `JSON` | Yes | Request body as JSON (or use individual body-field flags) |

#### `mcapi ecommerce create-store-order-line`

Add a new line item to an existing order.

`POST /3.0/ecommerce/stores/{store_id}/orders/{order_id}/lines`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--store-id` | `string` | Yes | The store id. |
| `--order-id` | `string` | Yes | The id for the order in a store. |
| `--json` | `JSON` | Yes | Request body as JSON (or use individual body-field flags) |

#### `mcapi ecommerce create-store-product`

Add a new product to a store.

`POST /3.0/ecommerce/stores/{store_id}/products`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--store-id` | `string` | Yes | The store id. |
| `--json` | `JSON` | Yes | Request body as JSON (or use individual body-field flags) |

#### `mcapi ecommerce create-store-product-image`

Add a new image to the product.

`POST /3.0/ecommerce/stores/{store_id}/products/{product_id}/images`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--store-id` | `string` | Yes | The store id. |
| `--product-id` | `string` | Yes | The id for the product of a store. |
| `--json` | `JSON` | Yes | Request body as JSON (or use individual body-field flags) |

#### `mcapi ecommerce create-store-product-variant`

Add a new variant to the product.

`POST /3.0/ecommerce/stores/{store_id}/products/{product_id}/variants`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--store-id` | `string` | Yes | The store id. |
| `--product-id` | `string` | Yes | The id for the product of a store. |
| `--json` | `JSON` | Yes | Request body as JSON (or use individual body-field flags) |

#### `mcapi ecommerce create-store-promo-rule`

Add a new promo rule to a store.

`POST /3.0/ecommerce/stores/{store_id}/promo-rules`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--store-id` | `string` | Yes | The store id. |
| `--json` | `JSON` | Yes | Request body as JSON (or use individual body-field flags) |

#### `mcapi ecommerce create-store-promo-rule-promo-code`

Add a new promo code to a store.

`POST /3.0/ecommerce/stores/{store_id}/promo-rules/{promo_rule_id}/promo-codes`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--store-id` | `string` | Yes | The store id. |
| `--promo-rule-id` | `string` | Yes | The id for the promo rule of a store. |
| `--json` | `JSON` | Yes | Request body as JSON (or use individual body-field flags) |

#### `mcapi ecommerce delete-store`

Delete a store. Deleting a store will also delete any associated subresources, including Customers, Orders, Products, and Carts.

`DELETE /3.0/ecommerce/stores/{store_id}`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--store-id` | `string` | Yes | The store id. |

#### `mcapi ecommerce delete-store-cart`

Delete a cart.

`DELETE /3.0/ecommerce/stores/{store_id}/carts/{cart_id}`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--store-id` | `string` | Yes | The store id. |
| `--cart-id` | `string` | Yes | The id for the cart. |

#### `mcapi ecommerce delete-store-cart-line`

Delete a specific cart line item.

`DELETE /3.0/ecommerce/stores/{store_id}/carts/{cart_id}/lines/{line_id}`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--store-id` | `string` | Yes | The store id. |
| `--cart-id` | `string` | Yes | The id for the cart. |
| `--line-id` | `string` | Yes | The id for the line item of a cart. |

#### `mcapi ecommerce delete-store-customer`

Delete a customer from a store.

`DELETE /3.0/ecommerce/stores/{store_id}/customers/{customer_id}`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--store-id` | `string` | Yes | The store id. |
| `--customer-id` | `string` | Yes | The id for the customer of a store. |

#### `mcapi ecommerce delete-store-order`

Delete an order.

`DELETE /3.0/ecommerce/stores/{store_id}/orders/{order_id}`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--store-id` | `string` | Yes | The store id. |
| `--order-id` | `string` | Yes | The id for the order in a store. |

#### `mcapi ecommerce delete-store-order-line`

Delete a specific order line item.

`DELETE /3.0/ecommerce/stores/{store_id}/orders/{order_id}/lines/{line_id}`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--store-id` | `string` | Yes | The store id. |
| `--order-id` | `string` | Yes | The id for the order in a store. |
| `--line-id` | `string` | Yes | The id for the line item of an order. |

#### `mcapi ecommerce delete-store-product`

Delete a product.

`DELETE /3.0/ecommerce/stores/{store_id}/products/{product_id}`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--store-id` | `string` | Yes | The store id. |
| `--product-id` | `string` | Yes | The id for the product of a store. |

#### `mcapi ecommerce delete-store-product-image`

Delete a product image.

`DELETE /3.0/ecommerce/stores/{store_id}/products/{product_id}/images/{image_id}`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--store-id` | `string` | Yes | The store id. |
| `--product-id` | `string` | Yes | The id for the product of a store. |
| `--image-id` | `string` | Yes | The id for the product image. |

#### `mcapi ecommerce delete-store-product-variant`

Delete a product variant.

`DELETE /3.0/ecommerce/stores/{store_id}/products/{product_id}/variants/{variant_id}`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--store-id` | `string` | Yes | The store id. |
| `--product-id` | `string` | Yes | The id for the product of a store. |
| `--variant-id` | `string` | Yes | The id for the product variant. |

#### `mcapi ecommerce delete-store-promo-rule`

Delete a promo rule from a store.

`DELETE /3.0/ecommerce/stores/{store_id}/promo-rules/{promo_rule_id}`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--store-id` | `string` | Yes | The store id. |
| `--promo-rule-id` | `string` | Yes | The id for the promo rule of a store. |

#### `mcapi ecommerce delete-store-promo-rule-promo-code`

Delete a promo code from a store.

`DELETE /3.0/ecommerce/stores/{store_id}/promo-rules/{promo_rule_id}/promo-codes/{promo_code_id}`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--store-id` | `string` | Yes | The store id. |
| `--promo-rule-id` | `string` | Yes | The id for the promo rule of a store. |
| `--promo-code-id` | `string` | Yes | The id for the promo code of a store. |

#### `mcapi ecommerce get-store`

Get information about a specific store.

`GET /3.0/ecommerce/stores/{store_id}`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--fields` | `string[]` | No | A comma-separated list of fields to return. Reference parameters of sub-objects with dot notation. |
| `--exclude-fields` | `string[]` | No | A comma-separated list of fields to exclude. Reference parameters of sub-objects with dot notation. |
| `--store-id` | `string` | Yes | The store id. |

#### `mcapi ecommerce get-store-cart`

Get information about a specific cart.

`GET /3.0/ecommerce/stores/{store_id}/carts/{cart_id}`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--fields` | `string[]` | No | A comma-separated list of fields to return. Reference parameters of sub-objects with dot notation. |
| `--exclude-fields` | `string[]` | No | A comma-separated list of fields to exclude. Reference parameters of sub-objects with dot notation. |
| `--store-id` | `string` | Yes | The store id. |
| `--cart-id` | `string` | Yes | The id for the cart. |

#### `mcapi ecommerce get-store-cart-line`

Get information about a specific cart line item.

`GET /3.0/ecommerce/stores/{store_id}/carts/{cart_id}/lines/{line_id}`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--fields` | `string[]` | No | A comma-separated list of fields to return. Reference parameters of sub-objects with dot notation. |
| `--exclude-fields` | `string[]` | No | A comma-separated list of fields to exclude. Reference parameters of sub-objects with dot notation. |
| `--store-id` | `string` | Yes | The store id. |
| `--cart-id` | `string` | Yes | The id for the cart. |
| `--line-id` | `string` | Yes | The id for the line item of a cart. |

#### `mcapi ecommerce get-store-customer`

Get information about a specific customer.

`GET /3.0/ecommerce/stores/{store_id}/customers/{customer_id}`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--fields` | `string[]` | No | A comma-separated list of fields to return. Reference parameters of sub-objects with dot notation. |
| `--exclude-fields` | `string[]` | No | A comma-separated list of fields to exclude. Reference parameters of sub-objects with dot notation. |
| `--store-id` | `string` | Yes | The store id. |
| `--customer-id` | `string` | Yes | The id for the customer of a store. |

#### `mcapi ecommerce get-store-order`

Get information about a specific order.

`GET /3.0/ecommerce/stores/{store_id}/orders/{order_id}`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--fields` | `string[]` | No | A comma-separated list of fields to return. Reference parameters of sub-objects with dot notation. |
| `--exclude-fields` | `string[]` | No | A comma-separated list of fields to exclude. Reference parameters of sub-objects with dot notation. |
| `--store-id` | `string` | Yes | The store id. |
| `--order-id` | `string` | Yes | The id for the order in a store. |

#### `mcapi ecommerce get-store-order-line`

Get information about a specific order line item.

`GET /3.0/ecommerce/stores/{store_id}/orders/{order_id}/lines/{line_id}`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--fields` | `string[]` | No | A comma-separated list of fields to return. Reference parameters of sub-objects with dot notation. |
| `--exclude-fields` | `string[]` | No | A comma-separated list of fields to exclude. Reference parameters of sub-objects with dot notation. |
| `--store-id` | `string` | Yes | The store id. |
| `--order-id` | `string` | Yes | The id for the order in a store. |
| `--line-id` | `string` | Yes | The id for the line item of an order. |

#### `mcapi ecommerce get-store-product`

Get information about a specific product.

`GET /3.0/ecommerce/stores/{store_id}/products/{product_id}`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--fields` | `string[]` | No | A comma-separated list of fields to return. Reference parameters of sub-objects with dot notation. |
| `--exclude-fields` | `string[]` | No | A comma-separated list of fields to exclude. Reference parameters of sub-objects with dot notation. |
| `--store-id` | `string` | Yes | The store id. |
| `--product-id` | `string` | Yes | The id for the product of a store. |

#### `mcapi ecommerce get-store-product-image`

Get information about a specific product image.

`GET /3.0/ecommerce/stores/{store_id}/products/{product_id}/images/{image_id}`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--fields` | `string[]` | No | A comma-separated list of fields to return. Reference parameters of sub-objects with dot notation. |
| `--exclude-fields` | `string[]` | No | A comma-separated list of fields to exclude. Reference parameters of sub-objects with dot notation. |
| `--store-id` | `string` | Yes | The store id. |
| `--product-id` | `string` | Yes | The id for the product of a store. |
| `--image-id` | `string` | Yes | The id for the product image. |

#### `mcapi ecommerce get-store-product-variant`

Get information about a specific product variant.

`GET /3.0/ecommerce/stores/{store_id}/products/{product_id}/variants/{variant_id}`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--fields` | `string[]` | No | A comma-separated list of fields to return. Reference parameters of sub-objects with dot notation. |
| `--exclude-fields` | `string[]` | No | A comma-separated list of fields to exclude. Reference parameters of sub-objects with dot notation. |
| `--store-id` | `string` | Yes | The store id. |
| `--product-id` | `string` | Yes | The id for the product of a store. |
| `--variant-id` | `string` | Yes | The id for the product variant. |

#### `mcapi ecommerce get-store-promo-rule`

Get information about a specific promo rule.

`GET /3.0/ecommerce/stores/{store_id}/promo-rules/{promo_rule_id}`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--fields` | `string[]` | No | A comma-separated list of fields to return. Reference parameters of sub-objects with dot notation. |
| `--exclude-fields` | `string[]` | No | A comma-separated list of fields to exclude. Reference parameters of sub-objects with dot notation. |
| `--store-id` | `string` | Yes | The store id. |
| `--promo-rule-id` | `string` | Yes | The id for the promo rule of a store. |

#### `mcapi ecommerce get-store-promo-rule-promo-code`

Get information about a specific promo code.

`GET /3.0/ecommerce/stores/{store_id}/promo-rules/{promo_rule_id}/promo-codes/{promo_code_id}`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--fields` | `string[]` | No | A comma-separated list of fields to return. Reference parameters of sub-objects with dot notation. |
| `--exclude-fields` | `string[]` | No | A comma-separated list of fields to exclude. Reference parameters of sub-objects with dot notation. |
| `--store-id` | `string` | Yes | The store id. |
| `--promo-rule-id` | `string` | Yes | The id for the promo rule of a store. |
| `--promo-code-id` | `string` | Yes | The id for the promo code of a store. |

#### `mcapi ecommerce list`

Get information about the e-commerce endpoint's resources.

`GET /3.0/ecommerce`

#### `mcapi ecommerce list-orders`

Get information about an account's orders.

`GET /3.0/ecommerce/orders`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--fields` | `string[]` | No | A comma-separated list of fields to return. Reference parameters of sub-objects with dot notation. |
| `--exclude-fields` | `string[]` | No | A comma-separated list of fields to exclude. Reference parameters of sub-objects with dot notation. |
| `--count` | `integer` | No | The number of records to return. Default value is 10. Maximum value is 1000 |
| `--offset` | `integer` | No | Used for [pagination](https://mailchimp.com/developer/marketing/docs/methods-parameters/#pagination), this is the number of records from a collection to skip. Default value is 0. |
| `--campaign-id` | `string` | No | Restrict results to orders with a specific `campaign_id` value. |
| `--outreach-id` | `string` | No | Restrict results to orders with a specific `outreach_id` value. |
| `--customer-id` | `string` | No | Restrict results to orders made by a specific customer. |
| `--has-outreach` | `boolean` | No | Restrict results to orders that have an outreach attached. For example, an email campaign or Facebook ad. |

#### `mcapi ecommerce list-store-cart-lines`

Get information about a cart's line items.

`GET /3.0/ecommerce/stores/{store_id}/carts/{cart_id}/lines`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--fields` | `string[]` | No | A comma-separated list of fields to return. Reference parameters of sub-objects with dot notation. |
| `--exclude-fields` | `string[]` | No | A comma-separated list of fields to exclude. Reference parameters of sub-objects with dot notation. |
| `--count` | `integer` | No | The number of records to return. Default value is 10. Maximum value is 1000 |
| `--offset` | `integer` | No | Used for [pagination](https://mailchimp.com/developer/marketing/docs/methods-parameters/#pagination), this is the number of records from a collection to skip. Default value is 0. |
| `--store-id` | `string` | Yes | The store id. |
| `--cart-id` | `string` | Yes | The id for the cart. |

#### `mcapi ecommerce list-store-carts`

Get information about a store's carts.

`GET /3.0/ecommerce/stores/{store_id}/carts`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--fields` | `string[]` | No | A comma-separated list of fields to return. Reference parameters of sub-objects with dot notation. |
| `--exclude-fields` | `string[]` | No | A comma-separated list of fields to exclude. Reference parameters of sub-objects with dot notation. |
| `--count` | `integer` | No | The number of records to return. Default value is 10. Maximum value is 1000 |
| `--offset` | `integer` | No | Used for [pagination](https://mailchimp.com/developer/marketing/docs/methods-parameters/#pagination), this is the number of records from a collection to skip. Default value is 0. |
| `--store-id` | `string` | Yes | The store id. |

#### `mcapi ecommerce list-store-customers`

Get information about a store's customers.

`GET /3.0/ecommerce/stores/{store_id}/customers`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--fields` | `string[]` | No | A comma-separated list of fields to return. Reference parameters of sub-objects with dot notation. |
| `--exclude-fields` | `string[]` | No | A comma-separated list of fields to exclude. Reference parameters of sub-objects with dot notation. |
| `--count` | `integer` | No | The number of records to return. Default value is 10. Maximum value is 1000 |
| `--offset` | `integer` | No | Used for [pagination](https://mailchimp.com/developer/marketing/docs/methods-parameters/#pagination), this is the number of records from a collection to skip. Default value is 0. |
| `--store-id` | `string` | Yes | The store id. |
| `--email-address` | `string` | No | Restrict the response to customers with the email address. |

#### `mcapi ecommerce list-store-order-lines`

Get information about an order's line items.

`GET /3.0/ecommerce/stores/{store_id}/orders/{order_id}/lines`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--fields` | `string[]` | No | A comma-separated list of fields to return. Reference parameters of sub-objects with dot notation. |
| `--exclude-fields` | `string[]` | No | A comma-separated list of fields to exclude. Reference parameters of sub-objects with dot notation. |
| `--count` | `integer` | No | The number of records to return. Default value is 10. Maximum value is 1000 |
| `--offset` | `integer` | No | Used for [pagination](https://mailchimp.com/developer/marketing/docs/methods-parameters/#pagination), this is the number of records from a collection to skip. Default value is 0. |
| `--store-id` | `string` | Yes | The store id. |
| `--order-id` | `string` | Yes | The id for the order in a store. |

#### `mcapi ecommerce list-store-orders`

Get information about a store's orders.

`GET /3.0/ecommerce/stores/{store_id}/orders`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--fields` | `string[]` | No | A comma-separated list of fields to return. Reference parameters of sub-objects with dot notation. |
| `--exclude-fields` | `string[]` | No | A comma-separated list of fields to exclude. Reference parameters of sub-objects with dot notation. |
| `--count` | `integer` | No | The number of records to return. Default value is 10. Maximum value is 1000 |
| `--offset` | `integer` | No | Used for [pagination](https://mailchimp.com/developer/marketing/docs/methods-parameters/#pagination), this is the number of records from a collection to skip. Default value is 0. |
| `--store-id` | `string` | Yes | The store id. |
| `--customer-id` | `string` | No | Restrict results to orders made by a specific customer. |
| `--has-outreach` | `boolean` | No | Restrict results to orders that have an outreach attached. For example, an email campaign or Facebook ad. |
| `--campaign-id` | `string` | No | Restrict results to orders with a specific `campaign_id` value. |
| `--outreach-id` | `string` | No | Restrict results to orders with a specific `outreach_id` value. |

#### `mcapi ecommerce list-store-product-images`

Get information about a product's images.

`GET /3.0/ecommerce/stores/{store_id}/products/{product_id}/images`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--fields` | `string[]` | No | A comma-separated list of fields to return. Reference parameters of sub-objects with dot notation. |
| `--exclude-fields` | `string[]` | No | A comma-separated list of fields to exclude. Reference parameters of sub-objects with dot notation. |
| `--count` | `integer` | No | The number of records to return. Default value is 10. Maximum value is 1000 |
| `--offset` | `integer` | No | Used for [pagination](https://mailchimp.com/developer/marketing/docs/methods-parameters/#pagination), this is the number of records from a collection to skip. Default value is 0. |
| `--store-id` | `string` | Yes | The store id. |
| `--product-id` | `string` | Yes | The id for the product of a store. |

#### `mcapi ecommerce list-store-product-variants`

Get information about a product's variants.

`GET /3.0/ecommerce/stores/{store_id}/products/{product_id}/variants`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--fields` | `string[]` | No | A comma-separated list of fields to return. Reference parameters of sub-objects with dot notation. |
| `--exclude-fields` | `string[]` | No | A comma-separated list of fields to exclude. Reference parameters of sub-objects with dot notation. |
| `--count` | `integer` | No | The number of records to return. Default value is 10. Maximum value is 1000 |
| `--offset` | `integer` | No | Used for [pagination](https://mailchimp.com/developer/marketing/docs/methods-parameters/#pagination), this is the number of records from a collection to skip. Default value is 0. |
| `--store-id` | `string` | Yes | The store id. |
| `--product-id` | `string` | Yes | The id for the product of a store. |

#### `mcapi ecommerce list-store-products`

Get information about a store's products.

`GET /3.0/ecommerce/stores/{store_id}/products`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--fields` | `string[]` | No | A comma-separated list of fields to return. Reference parameters of sub-objects with dot notation. |
| `--exclude-fields` | `string[]` | No | A comma-separated list of fields to exclude. Reference parameters of sub-objects with dot notation. |
| `--count` | `integer` | No | The number of records to return. Default value is 10. Maximum value is 1000 |
| `--offset` | `integer` | No | Used for [pagination](https://mailchimp.com/developer/marketing/docs/methods-parameters/#pagination), this is the number of records from a collection to skip. Default value is 0. |
| `--store-id` | `string` | Yes | The store id. |

#### `mcapi ecommerce list-store-promo-rule-promo-codes`

Get information about a store's promo codes.

`GET /3.0/ecommerce/stores/{store_id}/promo-rules/{promo_rule_id}/promo-codes`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--fields` | `string[]` | No | A comma-separated list of fields to return. Reference parameters of sub-objects with dot notation. |
| `--exclude-fields` | `string[]` | No | A comma-separated list of fields to exclude. Reference parameters of sub-objects with dot notation. |
| `--count` | `integer` | No | The number of records to return. Default value is 10. Maximum value is 1000 |
| `--offset` | `integer` | No | Used for [pagination](https://mailchimp.com/developer/marketing/docs/methods-parameters/#pagination), this is the number of records from a collection to skip. Default value is 0. |
| `--promo-rule-id` | `string` | Yes | The id for the promo rule of a store. |
| `--store-id` | `string` | Yes | The store id. |

#### `mcapi ecommerce list-store-promo-rules`

Get information about a store's promo rules.

`GET /3.0/ecommerce/stores/{store_id}/promo-rules`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--fields` | `string[]` | No | A comma-separated list of fields to return. Reference parameters of sub-objects with dot notation. |
| `--exclude-fields` | `string[]` | No | A comma-separated list of fields to exclude. Reference parameters of sub-objects with dot notation. |
| `--count` | `integer` | No | The number of records to return. Default value is 10. Maximum value is 1000 |
| `--offset` | `integer` | No | Used for [pagination](https://mailchimp.com/developer/marketing/docs/methods-parameters/#pagination), this is the number of records from a collection to skip. Default value is 0. |
| `--store-id` | `string` | Yes | The store id. |

#### `mcapi ecommerce list-stores`

Get information about all stores in the account.

`GET /3.0/ecommerce/stores`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--fields` | `string[]` | No | A comma-separated list of fields to return. Reference parameters of sub-objects with dot notation. |
| `--exclude-fields` | `string[]` | No | A comma-separated list of fields to exclude. Reference parameters of sub-objects with dot notation. |
| `--count` | `integer` | No | The number of records to return. Default value is 10. Maximum value is 1000 |
| `--offset` | `integer` | No | Used for [pagination](https://mailchimp.com/developer/marketing/docs/methods-parameters/#pagination), this is the number of records from a collection to skip. Default value is 0. |

#### `mcapi ecommerce update-store`

Update a store.

`PATCH /3.0/ecommerce/stores/{store_id}`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--store-id` | `string` | Yes | The store id. |
| `--json` | `JSON` | Yes | Request body as JSON (or use individual body-field flags) |

#### `mcapi ecommerce update-store-cart`

Update a specific cart.

`PATCH /3.0/ecommerce/stores/{store_id}/carts/{cart_id}`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--store-id` | `string` | Yes | The store id. |
| `--cart-id` | `string` | Yes | The id for the cart. |
| `--json` | `JSON` | Yes | Request body as JSON (or use individual body-field flags) |

#### `mcapi ecommerce update-store-cart-line`

Update a specific cart line item.

`PATCH /3.0/ecommerce/stores/{store_id}/carts/{cart_id}/lines/{line_id}`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--store-id` | `string` | Yes | The store id. |
| `--cart-id` | `string` | Yes | The id for the cart. |
| `--line-id` | `string` | Yes | The id for the line item of a cart. |
| `--json` | `JSON` | Yes | Request body as JSON (or use individual body-field flags) |

#### `mcapi ecommerce update-store-customer`

Update a customer.

`PATCH /3.0/ecommerce/stores/{store_id}/customers/{customer_id}`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--store-id` | `string` | Yes | The store id. |
| `--customer-id` | `string` | Yes | The id for the customer of a store. |
| `--json` | `JSON` | Yes | Request body as JSON (or use individual body-field flags) |

#### `mcapi ecommerce update-store-order`

Update a specific order.

`PATCH /3.0/ecommerce/stores/{store_id}/orders/{order_id}`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--store-id` | `string` | Yes | The store id. |
| `--order-id` | `string` | Yes | The id for the order in a store. |
| `--json` | `JSON` | Yes | Request body as JSON (or use individual body-field flags) |

#### `mcapi ecommerce update-store-order-line`

Update a specific order line item.

`PATCH /3.0/ecommerce/stores/{store_id}/orders/{order_id}/lines/{line_id}`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--store-id` | `string` | Yes | The store id. |
| `--order-id` | `string` | Yes | The id for the order in a store. |
| `--line-id` | `string` | Yes | The id for the line item of an order. |
| `--json` | `JSON` | Yes | Request body as JSON (or use individual body-field flags) |

#### `mcapi ecommerce update-store-product`

Update a specific product.

`PATCH /3.0/ecommerce/stores/{store_id}/products/{product_id}`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--store-id` | `string` | Yes | The store id. |
| `--product-id` | `string` | Yes | The id for the product of a store. |
| `--json` | `JSON` | Yes | Request body as JSON (or use individual body-field flags) |

#### `mcapi ecommerce update-store-product-image`

Update a product image.

`PATCH /3.0/ecommerce/stores/{store_id}/products/{product_id}/images/{image_id}`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--store-id` | `string` | Yes | The store id. |
| `--product-id` | `string` | Yes | The id for the product of a store. |
| `--image-id` | `string` | Yes | The id for the product image. |
| `--json` | `JSON` | Yes | Request body as JSON (or use individual body-field flags) |

#### `mcapi ecommerce update-store-product-variant`

Update a product variant.

`PATCH /3.0/ecommerce/stores/{store_id}/products/{product_id}/variants/{variant_id}`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--store-id` | `string` | Yes | The store id. |
| `--product-id` | `string` | Yes | The id for the product of a store. |
| `--variant-id` | `string` | Yes | The id for the product variant. |
| `--json` | `JSON` | Yes | Request body as JSON (or use individual body-field flags) |

#### `mcapi ecommerce update-store-promo-rule`

Update a promo rule.

`PATCH /3.0/ecommerce/stores/{store_id}/promo-rules/{promo_rule_id}`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--store-id` | `string` | Yes | The store id. |
| `--promo-rule-id` | `string` | Yes | The id for the promo rule of a store. |
| `--json` | `JSON` | Yes | Request body as JSON (or use individual body-field flags) |

#### `mcapi ecommerce update-store-promo-rule-promo-code`

Update a promo code.

`PATCH /3.0/ecommerce/stores/{store_id}/promo-rules/{promo_rule_id}/promo-codes/{promo_code_id}`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--store-id` | `string` | Yes | The store id. |
| `--promo-rule-id` | `string` | Yes | The id for the promo rule of a store. |
| `--promo-code-id` | `string` | Yes | The id for the promo code of a store. |
| `--json` | `JSON` | Yes | Request body as JSON (or use individual body-field flags) |

#### `mcapi ecommerce upsert-store-customer`

Add or update a customer.

`PUT /3.0/ecommerce/stores/{store_id}/customers/{customer_id}`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--store-id` | `string` | Yes | The store id. |
| `--customer-id` | `string` | Yes | The id for the customer of a store. |
| `--json` | `JSON` | Yes | Request body as JSON (or use individual body-field flags) |

#### `mcapi ecommerce upsert-store-product`

Update a specific product.

`PUT /3.0/ecommerce/stores/{store_id}/products/{product_id}`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--store-id` | `string` | Yes | The store id. |
| `--product-id` | `string` | Yes | The id for the product of a store. |
| `--json` | `JSON` | Yes | Request body as JSON (or use individual body-field flags) |

#### `mcapi ecommerce upsert-store-product-variant`

Add or update a product variant.

`PUT /3.0/ecommerce/stores/{store_id}/products/{product_id}/variants/{variant_id}`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--store-id` | `string` | Yes | The store id. |
| `--product-id` | `string` | Yes | The id for the product of a store. |
| `--variant-id` | `string` | Yes | The id for the product variant. |
| `--json` | `JSON` | Yes | Request body as JSON (or use individual body-field flags) |

---

### `mcapi facebook-ads`

#### `mcapi facebook-ads get`

Get details of a Facebook ad.

`GET /3.0/facebook-ads/{outreach_id}`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--fields` | `string[]` | No | A comma-separated list of fields to return. Reference parameters of sub-objects with dot notation. |
| `--outreach-id` | `string` | Yes | The outreach id. |
| `--exclude-fields` | `string[]` | No | A comma-separated list of fields to exclude. Reference parameters of sub-objects with dot notation. |

#### `mcapi facebook-ads list`

Get list of Facebook ads.

`GET /3.0/facebook-ads`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--fields` | `string[]` | No | A comma-separated list of fields to return. Reference parameters of sub-objects with dot notation. |
| `--exclude-fields` | `string[]` | No | A comma-separated list of fields to exclude. Reference parameters of sub-objects with dot notation. |
| `--count` | `integer` | No | The number of records to return. Default value is 10. Maximum value is 1000 |
| `--offset` | `integer` | No | Used for [pagination](https://mailchimp.com/developer/marketing/docs/methods-parameters/#pagination), this is the number of records from a collection to skip. Default value is 0. |
| `--sort-field` | `created_at | updated_at | end_time` | No | Returns files sorted by the specified field. |
| `--sort-dir` | `ASC | DESC` | No | Determines the order direction for sorted results. |

---

### `mcapi file-manager`

#### `mcapi file-manager create-file`

Upload a new image or file to the File Manager.

`POST /3.0/file-manager/files`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--json` | `JSON` | Yes | Request body as JSON (or use individual body-field flags) |

#### `mcapi file-manager create-folder`

Create a new folder in the File Manager.

`POST /3.0/file-manager/folders`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--json` | `JSON` | Yes | Request body as JSON (or use individual body-field flags) |

#### `mcapi file-manager delete-file`

Remove a specific file from the File Manager.

`DELETE /3.0/file-manager/files/{file_id}`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--file-id` | `string` | Yes | The unique id for the File Manager file. |

#### `mcapi file-manager delete-folder`

Delete a specific folder in the File Manager.

`DELETE /3.0/file-manager/folders/{folder_id}`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--folder-id` | `string` | Yes | The unique id for the File Manager folder. |

#### `mcapi file-manager get-file`

Get information about a specific file in the File Manager.

`GET /3.0/file-manager/files/{file_id}`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--fields` | `string[]` | No | A comma-separated list of fields to return. Reference parameters of sub-objects with dot notation. |
| `--exclude-fields` | `string[]` | No | A comma-separated list of fields to exclude. Reference parameters of sub-objects with dot notation. |
| `--file-id` | `string` | Yes | The unique id for the File Manager file. |

#### `mcapi file-manager get-folder`

Get information about a specific folder in the File Manager.

`GET /3.0/file-manager/folders/{folder_id}`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--fields` | `string[]` | No | A comma-separated list of fields to return. Reference parameters of sub-objects with dot notation. |
| `--exclude-fields` | `string[]` | No | A comma-separated list of fields to exclude. Reference parameters of sub-objects with dot notation. |
| `--folder-id` | `string` | Yes | The unique id for the File Manager folder. |

#### `mcapi file-manager list`

Get information about the file-manager endpoint's resources

`GET /3.0/file-manager`

#### `mcapi file-manager list-files`

Get a list of available images and files stored in the File Manager for the account.

`GET /3.0/file-manager/files`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--fields` | `string[]` | No | A comma-separated list of fields to return. Reference parameters of sub-objects with dot notation. |
| `--exclude-fields` | `string[]` | No | A comma-separated list of fields to exclude. Reference parameters of sub-objects with dot notation. |
| `--count` | `integer` | No | The number of records to return. Default value is 10. Maximum value is 1000 |
| `--offset` | `integer` | No | Used for [pagination](https://mailchimp.com/developer/marketing/docs/methods-parameters/#pagination), this is the number of records from a collection to skip. Default value is 0. |
| `--type` | `string` | No | The file type for the File Manager file. |
| `--created-by` | `string` | No | The Mailchimp account user who created the File Manager file. |
| `--before-created-at` | `string` | No | Restrict the response to files created before the set date. Uses ISO 8601 time format: 2015-10-21T15:41:36+00:00. |
| `--since-created-at` | `string` | No | Restrict the response to files created after the set date. Uses ISO 8601 time format: 2015-10-21T15:41:36+00:00. |
| `--sort-field` | `added_date | name | size` | No | Returns files sorted by the specified field. |
| `--sort-dir` | `ASC | DESC` | No | Determines the order direction for sorted results. |

#### `mcapi file-manager list-folder-files`

Get a list of available images and files stored in this folder.

`GET /3.0/file-manager/folders/{folder_id}/files`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--folder-id` | `string` | Yes | The unique id for the File Manager folder. |
| `--fields` | `string[]` | No | A comma-separated list of fields to return. Reference parameters of sub-objects with dot notation. |
| `--exclude-fields` | `string[]` | No | A comma-separated list of fields to exclude. Reference parameters of sub-objects with dot notation. |
| `--count` | `integer` | No | The number of records to return. Default value is 10. Maximum value is 1000 |
| `--offset` | `integer` | No | Used for [pagination](https://mailchimp.com/developer/marketing/docs/methods-parameters/#pagination), this is the number of records from a collection to skip. Default value is 0. |
| `--type` | `string` | No | The file type for the File Manager file. |
| `--created-by` | `string` | No | The Mailchimp account user who created the File Manager file. |
| `--before-created-at` | `string` | No | Restrict the response to files created before the set date. Uses ISO 8601 time format: 2015-10-21T15:41:36+00:00. |
| `--since-created-at` | `string` | No | Restrict the response to files created after the set date. Uses ISO 8601 time format: 2015-10-21T15:41:36+00:00. |
| `--sort-field` | `added_date | name | size` | No | Returns files sorted by the specified field. |
| `--sort-dir` | `ASC | DESC` | No | Determines the order direction for sorted results. |

#### `mcapi file-manager list-folders`

Get a list of all folders in the File Manager.

`GET /3.0/file-manager/folders`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--fields` | `string[]` | No | A comma-separated list of fields to return. Reference parameters of sub-objects with dot notation. |
| `--exclude-fields` | `string[]` | No | A comma-separated list of fields to exclude. Reference parameters of sub-objects with dot notation. |
| `--count` | `integer` | No | The number of records to return. Default value is 10. Maximum value is 1000 |
| `--offset` | `integer` | No | Used for [pagination](https://mailchimp.com/developer/marketing/docs/methods-parameters/#pagination), this is the number of records from a collection to skip. Default value is 0. |
| `--created-by` | `string` | No | The Mailchimp account user who created the File Manager file. |
| `--before-created-at` | `string` | No | Restrict the response to files created before the set date. Uses ISO 8601 time format: 2015-10-21T15:41:36+00:00. |
| `--since-created-at` | `string` | No | Restrict the response to files created after the set date. Uses ISO 8601 time format: 2015-10-21T15:41:36+00:00. |

#### `mcapi file-manager update-file`

Update a file in the File Manager.

`PATCH /3.0/file-manager/files/{file_id}`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--file-id` | `string` | Yes | The unique id for the File Manager file. |
| `--json` | `JSON` | Yes | Request body as JSON (or use individual body-field flags) |

#### `mcapi file-manager update-folder`

Update a specific File Manager folder.

`PATCH /3.0/file-manager/folders/{folder_id}`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--folder-id` | `string` | Yes | The unique id for the File Manager folder. |
| `--json` | `JSON` | Yes | Request body as JSON (or use individual body-field flags) |

---

### `mcapi landing-pages`

#### `mcapi landing-pages create`

Create an unpublished and contentless Mailchimp landing page.

`POST /3.0/landing-pages`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--use-default-list` | `boolean` | No | Will create the Landing Page using the account's Default List instead of requiring a list_id. |
| `--json` | `JSON` | Yes | Request body as JSON (or use individual body-field flags) |

#### `mcapi landing-pages create-action-publish`

Publish a landing page that is in draft, unpublished, or has been previously published and edited.

`POST /3.0/landing-pages/{page_id}/actions/publish`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--page-id` | `string` | Yes | The unique id for the page. |

#### `mcapi landing-pages create-action-unpublish`

Unpublish a landing page that is in draft or has been published.

`POST /3.0/landing-pages/{page_id}/actions/unpublish`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--page-id` | `string` | Yes | The unique id for the page. |

#### `mcapi landing-pages delete`

Delete a landing page.

`DELETE /3.0/landing-pages/{page_id}`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--page-id` | `string` | Yes | The unique id for the page. |

#### `mcapi landing-pages get`

Get information about a specific page.

`GET /3.0/landing-pages/{page_id}`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--fields` | `string[]` | No | A comma-separated list of fields to return. Reference parameters of sub-objects with dot notation. |
| `--exclude-fields` | `string[]` | No | A comma-separated list of fields to exclude. Reference parameters of sub-objects with dot notation. |
| `--page-id` | `string` | Yes | The unique id for the page. |

#### `mcapi landing-pages list`

Get all landing pages.

`GET /3.0/landing-pages`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--sort-dir` | `ASC | DESC` | No | Determines the order direction for sorted results. |
| `--sort-field` | `created_at | updated_at` | No | Returns files sorted by the specified field. |
| `--fields` | `string[]` | No | A comma-separated list of fields to return. Reference parameters of sub-objects with dot notation. |
| `--exclude-fields` | `string[]` | No | A comma-separated list of fields to exclude. Reference parameters of sub-objects with dot notation. |
| `--count` | `integer` | No | The number of records to return. Default value is 10. Maximum value is 1000 |

#### `mcapi landing-pages list-content`

Get the the HTML for your landing page.

`GET /3.0/landing-pages/{page_id}/content`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--fields` | `string[]` | No | A comma-separated list of fields to return. Reference parameters of sub-objects with dot notation. |
| `--exclude-fields` | `string[]` | No | A comma-separated list of fields to exclude. Reference parameters of sub-objects with dot notation. |
| `--page-id` | `string` | Yes | The unique id for the page. |

#### `mcapi landing-pages update`

Update a landing page.

`PATCH /3.0/landing-pages/{page_id}`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--page-id` | `string` | Yes | The unique id for the page. |
| `--json` | `JSON` | Yes | Request body as JSON (or use individual body-field flags) |

---

### `mcapi lists`

#### `mcapi lists batch-add-or-remove-members`

Batch add/remove list members to static segment

`POST /3.0/lists/{list_id}/segments/{segment_id}`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--list-id` | `string` | Yes | The unique ID for the list. |
| `--segment-id` | `string` | Yes | The unique id for the segment. |
| `--json` | `JSON` | Yes | Request body as JSON (or use individual body-field flags) |

#### `mcapi lists batch-subscribe-or-unsubscribe`

Batch subscribe or unsubscribe list members.

`POST /3.0/lists/{list_id}`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--list-id` | `string` | Yes | The unique ID for the list. |
| `--skip-merge-validation` | `boolean` | No | If skip_merge_validation is true, member data will be accepted without merge field values, even if the merge field is usually required. This defaults to false. |
| `--skip-duplicate-check` | `boolean` | No | If skip_duplicate_check is true, we will ignore duplicates sent in the request when using the batch sub/unsub on the lists endpoint. The status of the first appearance in the request will be saved. This defaults to false. |
| `--json` | `JSON` | Yes | Request body as JSON (or use individual body-field flags) |

#### `mcapi lists create`

Create a new list in your Mailchimp account.

`POST /3.0/lists`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--json` | `JSON` | Yes | Request body as JSON (or use individual body-field flags) |

#### `mcapi lists create-interest-category`

Create a new interest category.

`POST /3.0/lists/{list_id}/interest-categories`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--list-id` | `string` | Yes | The unique ID for the list. |
| `--json` | `JSON` | Yes | Request body as JSON (or use individual body-field flags) |

#### `mcapi lists create-interest-category-interest`

Create a new interest or 'group name' for a specific category.

`POST /3.0/lists/{list_id}/interest-categories/{interest_category_id}/interests`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--list-id` | `string` | Yes | The unique ID for the list. |
| `--interest-category-id` | `string` | Yes | The unique ID for the interest category. |
| `--json` | `JSON` | Yes | Request body as JSON (or use individual body-field flags) |

#### `mcapi lists create-list-survey-action-replicate`

Replicate a survey.

`POST /3.0/lists/{list_id}/surveys/{survey_id}/actions/replicate`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--list-id` | `string` | Yes | The unique ID for the list. |
| `--survey-id` | `string` | Yes | The ID of the survey. |
| `--json` | `JSON` | No | Request body as JSON (or use individual body-field flags) |

#### `mcapi lists create-member`

Add a new member to the list.

`POST /3.0/lists/{list_id}/members`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--list-id` | `string` | Yes | The unique ID for the list. |
| `--skip-merge-validation` | `boolean` | No | If skip_merge_validation is true, member data will be accepted without merge field values, even if the merge field is usually required. This defaults to false. |
| `--json` | `JSON` | Yes | Request body as JSON (or use individual body-field flags) |

#### `mcapi lists create-member-action-delete-permanent`

Delete all personally identifiable information related to a list member, and remove them from a list. This will make it impossible to re-import the list member.

`POST /3.0/lists/{list_id}/members/{subscriber_hash}/actions/delete-permanent`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--list-id` | `string` | Yes | The unique ID for the list. |
| `--subscriber-hash` | `string` | Yes | The MD5 hash of the lowercase version of the list member's email address. |

#### `mcapi lists create-member-event`

Add an event for a list member.

`POST /3.0/lists/{list_id}/members/{subscriber_hash}/events`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--list-id` | `string` | Yes | The unique ID for the list. |
| `--subscriber-hash` | `string` | Yes | The MD5 hash of the lowercase version of the list member's email address. This endpoint also accepts a list member's email address or contact_id. |
| `--json` | `JSON` | Yes | Request body as JSON (or use individual body-field flags) |

#### `mcapi lists create-member-note`

Add a new note for a specific subscriber.

`POST /3.0/lists/{list_id}/members/{subscriber_hash}/notes`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--list-id` | `string` | Yes | The unique ID for the list. |
| `--subscriber-hash` | `string` | Yes | The MD5 hash of the lowercase version of the list member's email address. |
| `--json` | `JSON` | Yes | Request body as JSON (or use individual body-field flags) |

#### `mcapi lists create-member-tag`

Add or remove tags from a list member. If a tag that does not exist is passed in and set as 'active', a new tag will be created.

`POST /3.0/lists/{list_id}/members/{subscriber_hash}/tags`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--list-id` | `string` | Yes | The unique ID for the list. |
| `--subscriber-hash` | `string` | Yes | The MD5 hash of the lowercase version of the list member's email address. |
| `--json` | `JSON` | Yes | Request body as JSON (or use individual body-field flags) |

#### `mcapi lists create-merge-field`

Add a new merge field for a specific audience.

`POST /3.0/lists/{list_id}/merge-fields`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--list-id` | `string` | Yes | The unique ID for the list. |
| `--json` | `JSON` | Yes | Request body as JSON (or use individual body-field flags) |

#### `mcapi lists create-segment`

Create a new segment in a specific list.

`POST /3.0/lists/{list_id}/segments`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--list-id` | `string` | Yes | The unique ID for the list. |
| `--json` | `JSON` | Yes | Request body as JSON (or use individual body-field flags) |

#### `mcapi lists create-segment-member`

Add a member to a static segment.

`POST /3.0/lists/{list_id}/segments/{segment_id}/members`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--list-id` | `string` | Yes | The unique ID for the list. |
| `--segment-id` | `string` | Yes | The unique id for the segment. |
| `--json` | `JSON` | Yes | Request body as JSON (or use individual body-field flags) |

#### `mcapi lists create-signup-form`

Customize a list's default signup form.

`POST /3.0/lists/{list_id}/signup-forms`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--list-id` | `string` | Yes | The unique ID for the list. |
| `--json` | `JSON` | Yes | Request body as JSON (or use individual body-field flags) |

#### `mcapi lists create-survey`

Create a draft survey for an audience.

`POST /3.0/lists/{list_id}/surveys`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--list-id` | `string` | Yes | The unique ID for the list. |
| `--json` | `JSON` | Yes | Request body as JSON (or use individual body-field flags) |

#### `mcapi lists create-webhook`

Create a new webhook for a specific list.

`POST /3.0/lists/{list_id}/webhooks`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--list-id` | `string` | Yes | The unique ID for the list. |
| `--json` | `JSON` | Yes | Request body as JSON (or use individual body-field flags) |

#### `mcapi lists delete`

Delete a list from your Mailchimp account. If you delete a list, you'll lose the list history—including subscriber activity, unsubscribes, complaints, and bounces. You’ll also lose subscribers’ email addresses, unless you exported and backed up your list.

`DELETE /3.0/lists/{list_id}`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--list-id` | `string` | Yes | The unique ID for the list. |

#### `mcapi lists delete-interest-category`

Delete a specific interest category.

`DELETE /3.0/lists/{list_id}/interest-categories/{interest_category_id}`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--list-id` | `string` | Yes | The unique ID for the list. |
| `--interest-category-id` | `string` | Yes | The unique ID for the interest category. |

#### `mcapi lists delete-interest-category-interest`

Delete interests or group names in a specific category.

`DELETE /3.0/lists/{list_id}/interest-categories/{interest_category_id}/interests/{interest_id}`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--list-id` | `string` | Yes | The unique ID for the list. |
| `--interest-category-id` | `string` | Yes | The unique ID for the interest category. |
| `--interest-id` | `string` | Yes | The specific interest or 'group name'. |

#### `mcapi lists delete-member`

Archive a list member. To permanently delete, use the delete-permanent action.

`DELETE /3.0/lists/{list_id}/members/{subscriber_hash}`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--list-id` | `string` | Yes | The unique ID for the list. |
| `--subscriber-hash` | `string` | Yes | The MD5 hash of the lowercase version of the list member's email address. This endpoint also accepts a list member's email address or contact_id. |

#### `mcapi lists delete-member-note`

Delete a specific note for a specific list member.

`DELETE /3.0/lists/{list_id}/members/{subscriber_hash}/notes/{note_id}`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--list-id` | `string` | Yes | The unique ID for the list. |
| `--subscriber-hash` | `string` | Yes | The MD5 hash of the lowercase version of the list member's email address. This endpoint also accepts a list member's email address or contact_id. |
| `--note-id` | `string` | Yes | The id for the note. |

#### `mcapi lists delete-merge-field`

Delete a specific merge field.

`DELETE /3.0/lists/{list_id}/merge-fields/{merge_id}`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--list-id` | `string` | Yes | The unique ID for the list. |
| `--merge-id` | `string` | Yes | The id for the merge field. |

#### `mcapi lists delete-segment`

Delete a specific segment in a list.

`DELETE /3.0/lists/{list_id}/segments/{segment_id}`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--list-id` | `string` | Yes | The unique ID for the list. |
| `--segment-id` | `string` | Yes | The unique id for the segment. |

#### `mcapi lists delete-segment-member`

Remove a member from the specified static segment.

`DELETE /3.0/lists/{list_id}/segments/{segment_id}/members/{subscriber_hash}`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--list-id` | `string` | Yes | The unique ID for the list. |
| `--segment-id` | `string` | Yes | The unique id for the segment. |
| `--subscriber-hash` | `string` | Yes | The MD5 hash of the lowercase version of the list member's email address. |

#### `mcapi lists delete-survey`

Delete a survey.

`DELETE /3.0/lists/{list_id}/surveys/{survey_id}`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--list-id` | `string` | Yes | The unique ID for the list. |
| `--survey-id` | `string` | Yes | The ID of the survey. |

#### `mcapi lists delete-webhook`

Delete a specific webhook in a list.

`DELETE /3.0/lists/{list_id}/webhooks/{webhook_id}`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--list-id` | `string` | Yes | The unique ID for the list. |
| `--webhook-id` | `string` | Yes | The webhook's id. |

#### `mcapi lists get`

Get information about a specific list in your Mailchimp account. Results include list members who have signed up but haven't confirmed their subscription yet and unsubscribed or cleaned.

`GET /3.0/lists/{list_id}`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--fields` | `string[]` | No | A comma-separated list of fields to return. Reference parameters of sub-objects with dot notation. |
| `--exclude-fields` | `string[]` | No | A comma-separated list of fields to exclude. Reference parameters of sub-objects with dot notation. |
| `--list-id` | `string` | Yes | The unique ID for the list. |
| `--include-total-contacts` | `boolean` | No | Deprecated. Return the total_contacts field in the stats response, which contains an approximate count of subscribed, unsubscribed, and transactional contacts. For a complete audience contact count, use the /audiences endpoint instead. |

#### `mcapi lists get-abuse-report`

Get details about a specific abuse report.

`GET /3.0/lists/{list_id}/abuse-reports/{report_id}`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--fields` | `string[]` | No | A comma-separated list of fields to return. Reference parameters of sub-objects with dot notation. |
| `--exclude-fields` | `string[]` | No | A comma-separated list of fields to exclude. Reference parameters of sub-objects with dot notation. |
| `--count` | `integer` | No | The number of records to return. Default value is 10. Maximum value is 1000 |
| `--offset` | `integer` | No | Used for [pagination](https://mailchimp.com/developer/marketing/docs/methods-parameters/#pagination), this is the number of records from a collection to skip. Default value is 0. |
| `--list-id` | `string` | Yes | The unique ID for the list. |
| `--report-id` | `string` | Yes | The id for the abuse report. |

#### `mcapi lists get-growth-history`

Get a summary of a specific list's growth activity for a specific month and year.

`GET /3.0/lists/{list_id}/growth-history/{month}`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--fields` | `string[]` | No | A comma-separated list of fields to return. Reference parameters of sub-objects with dot notation. |
| `--exclude-fields` | `string[]` | No | A comma-separated list of fields to exclude. Reference parameters of sub-objects with dot notation. |
| `--list-id` | `string` | Yes | The unique ID for the list. |
| `--month` | `string` | Yes | A specific month of list growth history. |

#### `mcapi lists get-interest-category`

Get information about a specific interest category.

`GET /3.0/lists/{list_id}/interest-categories/{interest_category_id}`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--list-id` | `string` | Yes | The unique ID for the list. |
| `--interest-category-id` | `string` | Yes | The unique ID for the interest category. |
| `--fields` | `string[]` | No | A comma-separated list of fields to return. Reference parameters of sub-objects with dot notation. |
| `--exclude-fields` | `string[]` | No | A comma-separated list of fields to exclude. Reference parameters of sub-objects with dot notation. |

#### `mcapi lists get-interest-category-interest`

Get interests or 'group names' for a specific category.

`GET /3.0/lists/{list_id}/interest-categories/{interest_category_id}/interests/{interest_id}`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--list-id` | `string` | Yes | The unique ID for the list. |
| `--interest-category-id` | `string` | Yes | The unique ID for the interest category. |
| `--interest-id` | `string` | Yes | The specific interest or 'group name'. |
| `--fields` | `string[]` | No | A comma-separated list of fields to return. Reference parameters of sub-objects with dot notation. |
| `--exclude-fields` | `string[]` | No | A comma-separated list of fields to exclude. Reference parameters of sub-objects with dot notation. |

#### `mcapi lists get-member`

Get information about a specific list member, including a currently subscribed, unsubscribed, or bounced member.

`GET /3.0/lists/{list_id}/members/{subscriber_hash}`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--fields` | `string[]` | No | A comma-separated list of fields to return. Reference parameters of sub-objects with dot notation. |
| `--exclude-fields` | `string[]` | No | A comma-separated list of fields to exclude. Reference parameters of sub-objects with dot notation. |
| `--list-id` | `string` | Yes | The unique ID for the list. |
| `--subscriber-hash` | `string` | Yes | The MD5 hash of the lowercase version of the list member's email address. This endpoint also accepts a list member's email address or contact_id. |

#### `mcapi lists get-member-note`

Get a specific note for a specific list member.

`GET /3.0/lists/{list_id}/members/{subscriber_hash}/notes/{note_id}`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--list-id` | `string` | Yes | The unique ID for the list. |
| `--subscriber-hash` | `string` | Yes | The MD5 hash of the lowercase version of the list member's email address. This endpoint also accepts a list member's email address or contact_id. |
| `--note-id` | `string` | Yes | The id for the note. |
| `--fields` | `string[]` | No | A comma-separated list of fields to return. Reference parameters of sub-objects with dot notation. |
| `--exclude-fields` | `string[]` | No | A comma-separated list of fields to exclude. Reference parameters of sub-objects with dot notation. |

#### `mcapi lists get-merge-field`

Get information about a specific merge field.

`GET /3.0/lists/{list_id}/merge-fields/{merge_id}`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--list-id` | `string` | Yes | The unique ID for the list. |
| `--merge-id` | `string` | Yes | The id for the merge field. |
| `--exclude-fields` | `string[]` | No | A comma-separated list of fields to exclude. Reference parameters of sub-objects with dot notation. |
| `--fields` | `string[]` | No | A comma-separated list of fields to return. Reference parameters of sub-objects with dot notation. |

#### `mcapi lists get-segment`

Get information about a specific segment.

`GET /3.0/lists/{list_id}/segments/{segment_id}`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--fields` | `string[]` | No | A comma-separated list of fields to return. Reference parameters of sub-objects with dot notation. |
| `--exclude-fields` | `string[]` | No | A comma-separated list of fields to exclude. Reference parameters of sub-objects with dot notation. |
| `--list-id` | `string` | Yes | The unique ID for the list. |
| `--segment-id` | `string` | Yes | The unique id for the segment. |
| `--include-cleaned` | `boolean` | No | Include cleaned members in response |
| `--include-transactional` | `boolean` | No | Include transactional members in response |
| `--include-unsubscribed` | `boolean` | No | Include unsubscribed members in response |

#### `mcapi lists get-survey`

Get details about a specific survey.

`GET /3.0/lists/{list_id}/surveys/{survey_id}`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--list-id` | `string` | Yes | The unique ID for the list. |
| `--survey-id` | `string` | Yes | The ID of the survey. |

#### `mcapi lists get-webhook`

Get information about a specific webhook.

`GET /3.0/lists/{list_id}/webhooks/{webhook_id}`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--list-id` | `string` | Yes | The unique ID for the list. |
| `--webhook-id` | `string` | Yes | The webhook's id. |

#### `mcapi lists list`

Get information about all lists in the account.

`GET /3.0/lists`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--fields` | `string[]` | No | A comma-separated list of fields to return. Reference parameters of sub-objects with dot notation. |
| `--exclude-fields` | `string[]` | No | A comma-separated list of fields to exclude. Reference parameters of sub-objects with dot notation. |
| `--count` | `integer` | No | The number of records to return. Default value is 10. Maximum value is 1000 |
| `--offset` | `integer` | No | Used for [pagination](https://mailchimp.com/developer/marketing/docs/methods-parameters/#pagination), this is the number of records from a collection to skip. Default value is 0. |
| `--before-date-created` | `string` | No | Restrict response to lists created before the set date. Uses ISO 8601 time format: 2015-10-21T15:41:36+00:00. |
| `--since-date-created` | `string` | No | Restrict results to lists created after the set date. Uses ISO 8601 time format: 2015-10-21T15:41:36+00:00. |
| `--before-campaign-last-sent` | `string` | No | Restrict results to lists created before the last campaign send date. Uses ISO 8601 time format: 2015-10-21T15:41:36+00:00. |
| `--since-campaign-last-sent` | `string` | No | Restrict results to lists created after the last campaign send date. Uses ISO 8601 time format: 2015-10-21T15:41:36+00:00. |
| `--email` | `string` | No | Restrict results to lists that include a specific subscriber's email address. |
| `--sort-field` | `date_created` | No | Returns files sorted by the specified field. |
| `--sort-dir` | `ASC | DESC` | No | Determines the order direction for sorted results. |
| `--has-ecommerce-store` | `boolean` | No | Restrict results to lists that contain an active, connected, undeleted ecommerce store. |
| `--include-total-contacts` | `boolean` | No | Deprecated. Return the total_contacts field in the stats response, which contains an approximate count of subscribed, unsubscribed, and transactional contacts. For a complete audience contact count, use the /audiences endpoint instead. |

#### `mcapi lists list-abuse-reports`

Get all abuse reports for a specific list.

`GET /3.0/lists/{list_id}/abuse-reports`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--fields` | `string[]` | No | A comma-separated list of fields to return. Reference parameters of sub-objects with dot notation. |
| `--exclude-fields` | `string[]` | No | A comma-separated list of fields to exclude. Reference parameters of sub-objects with dot notation. |
| `--count` | `integer` | No | The number of records to return. Default value is 10. Maximum value is 1000 |
| `--offset` | `integer` | No | Used for [pagination](https://mailchimp.com/developer/marketing/docs/methods-parameters/#pagination), this is the number of records from a collection to skip. Default value is 0. |
| `--list-id` | `string` | Yes | The unique ID for the list. |

#### `mcapi lists list-activity`

Get up to the previous 180 days of daily detailed aggregated activity stats for a list, not including Automation activity.

`GET /3.0/lists/{list_id}/activity`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--fields` | `string[]` | No | A comma-separated list of fields to return. Reference parameters of sub-objects with dot notation. |
| `--exclude-fields` | `string[]` | No | A comma-separated list of fields to exclude. Reference parameters of sub-objects with dot notation. |
| `--count` | `integer` | No | The number of records to return. Default value is 10. Maximum value is 1000 |
| `--offset` | `integer` | No | Used for [pagination](https://mailchimp.com/developer/marketing/docs/methods-parameters/#pagination), this is the number of records from a collection to skip. Default value is 0. |
| `--list-id` | `string` | Yes | The unique ID for the list. |

#### `mcapi lists list-clients`

Get a list of the top email clients based on user-agent strings.

`GET /3.0/lists/{list_id}/clients`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--fields` | `string[]` | No | A comma-separated list of fields to return. Reference parameters of sub-objects with dot notation. |
| `--exclude-fields` | `string[]` | No | A comma-separated list of fields to exclude. Reference parameters of sub-objects with dot notation. |
| `--list-id` | `string` | Yes | The unique ID for the list. |

#### `mcapi lists list-growth-history`

Get a month-by-month summary of a specific list's growth activity.

`GET /3.0/lists/{list_id}/growth-history`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--fields` | `string[]` | No | A comma-separated list of fields to return. Reference parameters of sub-objects with dot notation. |
| `--exclude-fields` | `string[]` | No | A comma-separated list of fields to exclude. Reference parameters of sub-objects with dot notation. |
| `--count` | `integer` | No | The number of records to return. Default value is 10. Maximum value is 1000 |
| `--offset` | `integer` | No | Used for [pagination](https://mailchimp.com/developer/marketing/docs/methods-parameters/#pagination), this is the number of records from a collection to skip. Default value is 0. |
| `--list-id` | `string` | Yes | The unique ID for the list. |
| `--sort-field` | `month` | No | Returns files sorted by the specified field. |
| `--sort-dir` | `ASC | DESC` | No | Determines the order direction for sorted results. |

#### `mcapi lists list-interest-categories`

Get information about a list's interest categories.

`GET /3.0/lists/{list_id}/interest-categories`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--fields` | `string[]` | No | A comma-separated list of fields to return. Reference parameters of sub-objects with dot notation. |
| `--exclude-fields` | `string[]` | No | A comma-separated list of fields to exclude. Reference parameters of sub-objects with dot notation. |
| `--count` | `integer` | No | The number of records to return. Default value is 10. Maximum value is 1000 |
| `--offset` | `integer` | No | Used for [pagination](https://mailchimp.com/developer/marketing/docs/methods-parameters/#pagination), this is the number of records from a collection to skip. Default value is 0. |
| `--list-id` | `string` | Yes | The unique ID for the list. |
| `--type` | `string` | No | Restrict results a type of interest group |
| `--sort-field` | `name | display_order` | No | Returns interest categories sorted by the specified field. Defaults to display_order. |
| `--sort-dir` | `ASC | DESC` | No | Determines the order direction for sorted results. |

#### `mcapi lists list-interest-category-interests`

Get a list of this category's interests.

`GET /3.0/lists/{list_id}/interest-categories/{interest_category_id}/interests`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--list-id` | `string` | Yes | The unique ID for the list. |
| `--interest-category-id` | `string` | Yes | The unique ID for the interest category. |
| `--fields` | `string[]` | No | A comma-separated list of fields to return. Reference parameters of sub-objects with dot notation. |
| `--exclude-fields` | `string[]` | No | A comma-separated list of fields to exclude. Reference parameters of sub-objects with dot notation. |
| `--count` | `integer` | No | The number of records to return. Default value is 10. Maximum value is 1000 |
| `--offset` | `integer` | No | Used for [pagination](https://mailchimp.com/developer/marketing/docs/methods-parameters/#pagination), this is the number of records from a collection to skip. Default value is 0. |

#### `mcapi lists list-locations`

Get the locations (countries) that the list's subscribers have been tagged to based on geocoding their IP address.

`GET /3.0/lists/{list_id}/locations`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--fields` | `string[]` | No | A comma-separated list of fields to return. Reference parameters of sub-objects with dot notation. |
| `--exclude-fields` | `string[]` | No | A comma-separated list of fields to exclude. Reference parameters of sub-objects with dot notation. |
| `--list-id` | `string` | Yes | The unique ID for the list. |

#### `mcapi lists list-member-activity`

Get the last 50 events of a member's activity on a specific list, including opens, clicks, and unsubscribes.

`GET /3.0/lists/{list_id}/members/{subscriber_hash}/activity`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--list-id` | `string` | Yes | The unique ID for the list. |
| `--subscriber-hash` | `string` | Yes | The MD5 hash of the lowercase version of the list member's email address. This endpoint also accepts a list member's email address or contact_id. |
| `--fields` | `string[]` | No | A comma-separated list of fields to return. Reference parameters of sub-objects with dot notation. |
| `--exclude-fields` | `string[]` | No | A comma-separated list of fields to exclude. Reference parameters of sub-objects with dot notation. |
| `--action` | `abuse | bounce | click | open | sent | unsub | ecomm[]` | No | A comma seperated list of actions to return. |

#### `mcapi lists list-member-activity-feed`

Get a member's activity on a specific list, including opens, clicks, and unsubscribes.

`GET /3.0/lists/{list_id}/members/{subscriber_hash}/activity-feed`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--list-id` | `string` | Yes | The unique ID for the list. |
| `--subscriber-hash` | `string` | Yes | The MD5 hash of the lowercase version of the list member's email address. This endpoint also accepts a list member's email address or contact_id. |
| `--fields` | `string[]` | No | A comma-separated list of fields to return. Reference parameters of sub-objects with dot notation. |
| `--exclude-fields` | `string[]` | No | A comma-separated list of fields to exclude. Reference parameters of sub-objects with dot notation. |
| `--count` | `integer` | No | The number of records to return. Default value is 10. Maximum value is 1000 |
| `--offset` | `integer` | No | Used for [pagination](https://mailchimp.com/developer/marketing/docs/methods-parameters/#pagination), this is the number of records from a collection to skip. Default value is 0. |
| `--activity-filters` | `bounce | click | conversation | ecommerce_signup | event | web_engagement | generic_signup | landing_page_signup | marketing_permission | note | open | order | postcard_sent | sent | signup | squatter_signup | unsub | website_signup | survey_response | sms_bulk_sent | inbox_thread | qbo_payment_link | video_call_transcripts | whatsapp_bulk_sent | whatsapp_delivered[]` | No | A comma-separated list of activity filters that correspond to a set of activity types, e.g "?activity_filters=open,bounce,click". |

#### `mcapi lists list-member-events`

Get events for a contact.

`GET /3.0/lists/{list_id}/members/{subscriber_hash}/events`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--list-id` | `string` | Yes | The unique ID for the list. |
| `--subscriber-hash` | `string` | Yes | The MD5 hash of the lowercase version of the list member's email address. This endpoint also accepts a list member's email address or contact_id. |
| `--count` | `integer` | No | The number of records to return. Default value is 10. Maximum value is 1000 |
| `--offset` | `integer` | No | Used for [pagination](https://mailchimp.com/developer/marketing/docs/methods-parameters/#pagination), this is the number of records from a collection to skip. Default value is 0. |
| `--fields` | `string[]` | No | A comma-separated list of fields to return. Reference parameters of sub-objects with dot notation. |
| `--exclude-fields` | `string[]` | No | A comma-separated list of fields to exclude. Reference parameters of sub-objects with dot notation. |

#### `mcapi lists list-member-goals`

Get the last 50 Goal events for a member on a specific list.

`GET /3.0/lists/{list_id}/members/{subscriber_hash}/goals`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--list-id` | `string` | Yes | The unique ID for the list. |
| `--subscriber-hash` | `string` | Yes | The MD5 hash of the lowercase version of the list member's email address. This endpoint also accepts a list member's email address or contact_id. |
| `--fields` | `string[]` | No | A comma-separated list of fields to return. Reference parameters of sub-objects with dot notation. |
| `--exclude-fields` | `string[]` | No | A comma-separated list of fields to exclude. Reference parameters of sub-objects with dot notation. |

#### `mcapi lists list-member-notes`

Get recent notes for a specific list member.

`GET /3.0/lists/{list_id}/members/{subscriber_hash}/notes`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--list-id` | `string` | Yes | The unique ID for the list. |
| `--subscriber-hash` | `string` | Yes | The MD5 hash of the lowercase version of the list member's email address. |
| `--sort-field` | `created_at | updated_at | note_id` | No | Returns notes sorted by the specified field. |
| `--sort-dir` | `ASC | DESC` | No | Determines the order direction for sorted results. |
| `--fields` | `string[]` | No | A comma-separated list of fields to return. Reference parameters of sub-objects with dot notation. |
| `--exclude-fields` | `string[]` | No | A comma-separated list of fields to exclude. Reference parameters of sub-objects with dot notation. |
| `--count` | `integer` | No | The number of records to return. Default value is 10. Maximum value is 1000 |
| `--offset` | `integer` | No | Used for [pagination](https://mailchimp.com/developer/marketing/docs/methods-parameters/#pagination), this is the number of records from a collection to skip. Default value is 0. |

#### `mcapi lists list-member-tags`

Get the tags on a list member.

`GET /3.0/lists/{list_id}/members/{subscriber_hash}/tags`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--list-id` | `string` | Yes | The unique ID for the list. |
| `--subscriber-hash` | `string` | Yes | The MD5 hash of the lowercase version of the list member's email address. This endpoint also accepts a list member's email address or contact_id. |
| `--fields` | `string[]` | No | A comma-separated list of fields to return. Reference parameters of sub-objects with dot notation. |
| `--exclude-fields` | `string[]` | No | A comma-separated list of fields to exclude. Reference parameters of sub-objects with dot notation. |
| `--count` | `integer` | No | The number of records to return. Default value is 10. Maximum value is 1000 |
| `--offset` | `integer` | No | Used for [pagination](https://mailchimp.com/developer/marketing/docs/methods-parameters/#pagination), this is the number of records from a collection to skip. Default value is 0. |

#### `mcapi lists list-members`

Get information about members in a specific Mailchimp list.

`GET /3.0/lists/{list_id}/members`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--fields` | `string[]` | No | A comma-separated list of fields to return. Reference parameters of sub-objects with dot notation. |
| `--exclude-fields` | `string[]` | No | A comma-separated list of fields to exclude. Reference parameters of sub-objects with dot notation. |
| `--count` | `integer` | No | The number of records to return. Default value is 10. Maximum value is 1000 |
| `--offset` | `integer` | No | Used for [pagination](https://mailchimp.com/developer/marketing/docs/methods-parameters/#pagination), this is the number of records from a collection to skip. Default value is 0. |
| `--list-id` | `string` | Yes | The unique ID for the list. |
| `--email-type` | `string` | No | The email type. |
| `--status` | `subscribed | unsubscribed | cleaned | pending | transactional | archived` | No | The subscriber's status. |
| `--since-timestamp-opt` | `string` | No | Restrict results to subscribers who opted-in after the set timeframe. Uses ISO 8601 time format: 2015-10-21T15:41:36+00:00. |
| `--before-timestamp-opt` | `string` | No | Restrict results to subscribers who opted-in before the set timeframe. Uses ISO 8601 time format: 2015-10-21T15:41:36+00:00. |
| `--since-last-changed` | `string` | No | Restrict results to subscribers whose information changed after the set timeframe. Uses ISO 8601 time format: 2015-10-21T15:41:36+00:00. |
| `--before-last-changed` | `string` | No | Restrict results to subscribers whose information changed before the set timeframe. Uses ISO 8601 time format: 2015-10-21T15:41:36+00:00. |
| `--unique-email-id` | `string` | No | A unique identifier for the email address across all Mailchimp lists. |
| `--vip-only` | `boolean` | No | A filter to return only the list's VIP members. Passing `true` will restrict results to VIP list members, passing `false` will return all list members. |
| `--interest-category-id` | `string` | No | The unique id for the interest category. |
| `--interest-ids` | `string` | No | Used to filter list members by interests. Must be accompanied by interest_category_id and interest_match. The value must be a comma separated list of interest ids present for any supplied interest categories. |
| `--interest-match` | `any | all | none` | No | Used to filter list members by interests. Must be accompanied by interest_category_id and interest_ids. "any" will match a member with any of the interest supplied, "all" will only match members with every interest supplied, and "none" will match members without any of the interest supplied. |
| `--sort-field` | `timestamp_opt | timestamp_signup | last_changed` | No | Returns files sorted by the specified field. |
| `--sort-dir` | `ASC | DESC` | No | Determines the order direction for sorted results. |
| `--since-last-campaign` | `boolean` | No | Filter subscribers by those subscribed/unsubscribed/pending/cleaned since last email campaign send. Member status is required to use this filter. |
| `--unsubscribed-since` | `string` | No | Filter subscribers by those unsubscribed since a specific date. Using any status other than unsubscribed with this filter will result in an error. |

#### `mcapi lists list-merge-fields`

Get a list of all merge fields for an audience.

`GET /3.0/lists/{list_id}/merge-fields`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--list-id` | `string` | Yes | The unique ID for the list. |
| `--fields` | `string[]` | No | A comma-separated list of fields to return. Reference parameters of sub-objects with dot notation. |
| `--exclude-fields` | `string[]` | No | A comma-separated list of fields to exclude. Reference parameters of sub-objects with dot notation. |
| `--count` | `integer` | No | The number of records to return. Default value is 10. Maximum value is 1000 |
| `--offset` | `integer` | No | Used for [pagination](https://mailchimp.com/developer/marketing/docs/methods-parameters/#pagination), this is the number of records from a collection to skip. Default value is 0. |
| `--type` | `string` | No | The merge field type. |
| `--required` | `boolean` | No | Whether it's a required merge field. |

#### `mcapi lists list-segment-members`

Get information about members in a saved segment.

`GET /3.0/lists/{list_id}/segments/{segment_id}/members`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--fields` | `string[]` | No | A comma-separated list of fields to return. Reference parameters of sub-objects with dot notation. |
| `--exclude-fields` | `string[]` | No | A comma-separated list of fields to exclude. Reference parameters of sub-objects with dot notation. |
| `--count` | `integer` | No | The number of records to return. Default value is 10. Maximum value is 1000 |
| `--offset` | `integer` | No | Used for [pagination](https://mailchimp.com/developer/marketing/docs/methods-parameters/#pagination), this is the number of records from a collection to skip. Default value is 0. |
| `--list-id` | `string` | Yes | The unique ID for the list. |
| `--segment-id` | `string` | Yes | The unique id for the segment. |
| `--include-cleaned` | `boolean` | No | Include cleaned members in response |
| `--include-transactional` | `boolean` | No | Include transactional members in response |
| `--include-unsubscribed` | `boolean` | No | Include unsubscribed members in response |

#### `mcapi lists list-segments`

Get information about all available segments for a specific list.

`GET /3.0/lists/{list_id}/segments`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--fields` | `string[]` | No | A comma-separated list of fields to return. Reference parameters of sub-objects with dot notation. |
| `--exclude-fields` | `string[]` | No | A comma-separated list of fields to exclude. Reference parameters of sub-objects with dot notation. |
| `--count` | `integer` | No | The number of records to return. Default value is 10. Maximum value is 1000 |
| `--offset` | `integer` | No | Used for [pagination](https://mailchimp.com/developer/marketing/docs/methods-parameters/#pagination), this is the number of records from a collection to skip. Default value is 0. |
| `--list-id` | `string` | Yes | The unique ID for the list. |
| `--type` | `string` | No | Limit results based on segment type. |
| `--since-created-at` | `string` | No | Restrict results to segments created after the set time. Uses ISO 8601 time format: 2015-10-21T15:41:36+00:00. |
| `--before-created-at` | `string` | No | Restrict results to segments created before the set time. Uses ISO 8601 time format: 2015-10-21T15:41:36+00:00. |
| `--include-cleaned` | `boolean` | No | Include cleaned members in response |
| `--include-transactional` | `boolean` | No | Include transactional members in response |
| `--include-unsubscribed` | `boolean` | No | Include unsubscribed members in response |
| `--since-updated-at` | `string` | No | Restrict results to segments update after the set time. Uses ISO 8601 time format: 2015-10-21T15:41:36+00:00. |
| `--before-updated-at` | `string` | No | Restrict results to segments update before the set time. Uses ISO 8601 time format: 2015-10-21T15:41:36+00:00. |
| `--exclude-type` | `saved | static | fuzzy` | No | Exclude results based on segment type. For example, use `exclude_type=static` to exclude tags from the response. |

#### `mcapi lists list-signup-forms`

Get signup forms for a specific list.

`GET /3.0/lists/{list_id}/signup-forms`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--list-id` | `string` | Yes | The unique ID for the list. |

#### `mcapi lists list-surveys`

Get information about all available surveys for a specific list.

`GET /3.0/lists/{list_id}/surveys`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--list-id` | `string` | Yes | The unique ID for the list. |

#### `mcapi lists list-tag-search`

Search for tags on a list by name. If no name is provided, will return all tags on the list.

`GET /3.0/lists/{list_id}/tag-search`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--list-id` | `string` | Yes | The unique ID for the list. |
| `--name` | `string` | No | The search query used to filter tags.  The search query will be compared to each tag as a prefix, so all tags that have a name starting with this field will be returned. |

#### `mcapi lists list-webhooks`

Get information about all webhooks for a specific list.

`GET /3.0/lists/{list_id}/webhooks`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--list-id` | `string` | Yes | The unique ID for the list. |

#### `mcapi lists update`

Update the settings for a specific list.

`PATCH /3.0/lists/{list_id}`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--list-id` | `string` | Yes | The unique ID for the list. |
| `--json` | `JSON` | Yes | Request body as JSON (or use individual body-field flags) |

#### `mcapi lists update-interest-category`

Update a specific interest category.

`PATCH /3.0/lists/{list_id}/interest-categories/{interest_category_id}`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--list-id` | `string` | Yes | The unique ID for the list. |
| `--interest-category-id` | `string` | Yes | The unique ID for the interest category. |
| `--json` | `JSON` | Yes | Request body as JSON (or use individual body-field flags) |

#### `mcapi lists update-interest-category-interest`

Update interests or 'group names' for a specific category.

`PATCH /3.0/lists/{list_id}/interest-categories/{interest_category_id}/interests/{interest_id}`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--list-id` | `string` | Yes | The unique ID for the list. |
| `--interest-category-id` | `string` | Yes | The unique ID for the interest category. |
| `--interest-id` | `string` | Yes | The specific interest or 'group name'. |
| `--json` | `JSON` | Yes | Request body as JSON (or use individual body-field flags) |

#### `mcapi lists update-member`

Update information for a specific list member.

`PATCH /3.0/lists/{list_id}/members/{subscriber_hash}`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--list-id` | `string` | Yes | The unique ID for the list. |
| `--subscriber-hash` | `string` | Yes | The MD5 hash of the lowercase version of the list member's email address. This endpoint also accepts a list member's email address or contact_id. |
| `--skip-merge-validation` | `boolean` | No | If skip_merge_validation is true, member data will be accepted without merge field values, even if the merge field is usually required. This defaults to false. |
| `--json` | `JSON` | Yes | Request body as JSON (or use individual body-field flags) |

#### `mcapi lists update-member-note`

Update a specific note for a specific list member.

`PATCH /3.0/lists/{list_id}/members/{subscriber_hash}/notes/{note_id}`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--list-id` | `string` | Yes | The unique ID for the list. |
| `--subscriber-hash` | `string` | Yes | The MD5 hash of the lowercase version of the list member's email address. This endpoint also accepts a list member's email address or contact_id. |
| `--note-id` | `string` | Yes | The id for the note. |
| `--json` | `JSON` | Yes | Request body as JSON (or use individual body-field flags) |

#### `mcapi lists update-merge-field`

Update a specific merge field.

`PATCH /3.0/lists/{list_id}/merge-fields/{merge_id}`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--list-id` | `string` | Yes | The unique ID for the list. |
| `--merge-id` | `string` | Yes | The id for the merge field. |
| `--json` | `JSON` | Yes | Request body as JSON (or use individual body-field flags) |

#### `mcapi lists update-segment`

Update a specific segment in a list.

`PATCH /3.0/lists/{list_id}/segments/{segment_id}`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--list-id` | `string` | Yes | The unique ID for the list. |
| `--segment-id` | `string` | Yes | The unique id for the segment. |
| `--json` | `JSON` | Yes | Request body as JSON (or use individual body-field flags) |

#### `mcapi lists update-survey`

Update a survey. When sections is provided, send the complete section list in display order. Any existing section not included is deleted.

`PATCH /3.0/lists/{list_id}/surveys/{survey_id}`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--list-id` | `string` | Yes | The unique ID for the list. |
| `--survey-id` | `string` | Yes | The ID of the survey. |
| `--json` | `JSON` | Yes | Request body as JSON (or use individual body-field flags) |

#### `mcapi lists update-webhook`

Update the settings for an existing webhook.

`PATCH /3.0/lists/{list_id}/webhooks/{webhook_id}`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--list-id` | `string` | Yes | The unique ID for the list. |
| `--webhook-id` | `string` | Yes | The webhook's id. |
| `--json` | `JSON` | Yes | Request body as JSON (or use individual body-field flags) |

#### `mcapi lists upsert-member`

Add or update a list member.

`PUT /3.0/lists/{list_id}/members/{subscriber_hash}`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--list-id` | `string` | Yes | The unique ID for the list. |
| `--subscriber-hash` | `string` | Yes | The MD5 hash of the lowercase version of the list member's email address. This endpoint also accepts a list member's email address or contact_id. |
| `--skip-merge-validation` | `boolean` | No | If skip_merge_validation is true, member data will be accepted without merge field values, even if the merge field is usually required. This defaults to false. |
| `--json` | `JSON` | Yes | Request body as JSON (or use individual body-field flags) |

---

### `mcapi ping`

#### `mcapi ping list`

A health check for the API that won't return any account-specific information.

`GET /3.0/ping`

---

### `mcapi reporting`

#### `mcapi reporting get-facebook-ad`

Get report of a Facebook ad.

`GET /3.0/reporting/facebook-ads/{outreach_id}`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--fields` | `string[]` | No | A comma-separated list of fields to return. Reference parameters of sub-objects with dot notation. |
| `--outreach-id` | `string` | Yes | The outreach id. |
| `--exclude-fields` | `string[]` | No | A comma-separated list of fields to exclude. Reference parameters of sub-objects with dot notation. |

#### `mcapi reporting get-landing-page`

Get report of a landing page.

`GET /3.0/reporting/landing-pages/{outreach_id}`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--fields` | `string[]` | No | A comma-separated list of fields to return. Reference parameters of sub-objects with dot notation. |
| `--outreach-id` | `string` | Yes | The outreach id. |
| `--exclude-fields` | `string[]` | No | A comma-separated list of fields to exclude. Reference parameters of sub-objects with dot notation. |

#### `mcapi reporting get-survey`

Get report for a survey.

`GET /3.0/reporting/surveys/{survey_id}`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--fields` | `string[]` | No | A comma-separated list of fields to return. Reference parameters of sub-objects with dot notation. |
| `--exclude-fields` | `string[]` | No | A comma-separated list of fields to exclude. Reference parameters of sub-objects with dot notation. |
| `--survey-id` | `string` | Yes | The ID of the survey. |

#### `mcapi reporting get-survey-question`

Get report for a survey question.

`GET /3.0/reporting/surveys/{survey_id}/questions/{question_id}`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--fields` | `string[]` | No | A comma-separated list of fields to return. Reference parameters of sub-objects with dot notation. |
| `--exclude-fields` | `string[]` | No | A comma-separated list of fields to exclude. Reference parameters of sub-objects with dot notation. |
| `--survey-id` | `string` | Yes | The ID of the survey. |
| `--question-id` | `string` | Yes | The ID of the survey question. |

#### `mcapi reporting get-survey-respons`

Get a single survey response.

`GET /3.0/reporting/surveys/{survey_id}/responses/{response_id}`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--survey-id` | `string` | Yes | The ID of the survey. |
| `--response-id` | `string` | Yes | The ID of the survey response. |

#### `mcapi reporting list`

Get information about the reporting endpoint's resources.

`GET /3.0/reporting`

#### `mcapi reporting list-facebook-ad-ecommerce-product-activity`

Get breakdown of product activity for an outreach.

`GET /3.0/reporting/facebook-ads/{outreach_id}/ecommerce-product-activity`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--fields` | `string[]` | No | A comma-separated list of fields to return. Reference parameters of sub-objects with dot notation. |
| `--exclude-fields` | `string[]` | No | A comma-separated list of fields to exclude. Reference parameters of sub-objects with dot notation. |
| `--count` | `integer` | No | The number of records to return. Default value is 10. Maximum value is 1000 |
| `--offset` | `integer` | No | Used for [pagination](https://mailchimp.com/developer/marketing/docs/methods-parameters/#pagination), this is the number of records from a collection to skip. Default value is 0. |
| `--outreach-id` | `string` | Yes | The outreach id. |
| `--sort-field` | `title | total_revenue | total_purchased` | No | Returns files sorted by the specified field. |

#### `mcapi reporting list-facebook-ads`

Get reports of Facebook ads.

`GET /3.0/reporting/facebook-ads`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--fields` | `string[]` | No | A comma-separated list of fields to return. Reference parameters of sub-objects with dot notation. |
| `--exclude-fields` | `string[]` | No | A comma-separated list of fields to exclude. Reference parameters of sub-objects with dot notation. |
| `--count` | `integer` | No | The number of records to return. Default value is 10. Maximum value is 1000 |
| `--offset` | `integer` | No | Used for [pagination](https://mailchimp.com/developer/marketing/docs/methods-parameters/#pagination), this is the number of records from a collection to skip. Default value is 0. |
| `--sort-field` | `created_at | updated_at | end_time` | No | Returns files sorted by the specified field. |
| `--sort-dir` | `ASC | DESC` | No | Determines the order direction for sorted results. |

#### `mcapi reporting list-landing-pages`

Get reports of landing pages.

`GET /3.0/reporting/landing-pages`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--fields` | `string[]` | No | A comma-separated list of fields to return. Reference parameters of sub-objects with dot notation. |
| `--exclude-fields` | `string[]` | No | A comma-separated list of fields to exclude. Reference parameters of sub-objects with dot notation. |
| `--count` | `integer` | No | The number of records to return. Default value is 10. Maximum value is 1000 |
| `--offset` | `integer` | No | Used for [pagination](https://mailchimp.com/developer/marketing/docs/methods-parameters/#pagination), this is the number of records from a collection to skip. Default value is 0. |

#### `mcapi reporting list-survey-question-answers`

Get answers for a survey question.

`GET /3.0/reporting/surveys/{survey_id}/questions/{question_id}/answers`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--fields` | `string[]` | No | A comma-separated list of fields to return. Reference parameters of sub-objects with dot notation. |
| `--exclude-fields` | `string[]` | No | A comma-separated list of fields to exclude. Reference parameters of sub-objects with dot notation. |
| `--survey-id` | `string` | Yes | The ID of the survey. |
| `--question-id` | `string` | Yes | The ID of the survey question. |
| `--respondent-familiarity-is` | `new | known | unknown` | No | Filter survey responses by familiarity of the respondents. |

#### `mcapi reporting list-survey-questions`

Get reports for survey questions.

`GET /3.0/reporting/surveys/{survey_id}/questions`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--fields` | `string[]` | No | A comma-separated list of fields to return. Reference parameters of sub-objects with dot notation. |
| `--exclude-fields` | `string[]` | No | A comma-separated list of fields to exclude. Reference parameters of sub-objects with dot notation. |
| `--survey-id` | `string` | Yes | The ID of the survey. |

#### `mcapi reporting list-survey-responses`

Get responses to a survey.

`GET /3.0/reporting/surveys/{survey_id}/responses`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--fields` | `string[]` | No | A comma-separated list of fields to return. Reference parameters of sub-objects with dot notation. |
| `--exclude-fields` | `string[]` | No | A comma-separated list of fields to exclude. Reference parameters of sub-objects with dot notation. |
| `--survey-id` | `string` | Yes | The ID of the survey. |
| `--answered-question` | `integer` | No | The ID of the question that was answered. |
| `--chose-answer` | `string` | No | The ID of the option chosen to filter responses on. |
| `--respondent-familiarity-is` | `new | known | unknown` | No | Filter survey responses by familiarity of the respondents. |

#### `mcapi reporting list-surveys`

Get reports for surveys.

`GET /3.0/reporting/surveys`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--fields` | `string[]` | No | A comma-separated list of fields to return. Reference parameters of sub-objects with dot notation. |
| `--exclude-fields` | `string[]` | No | A comma-separated list of fields to exclude. Reference parameters of sub-objects with dot notation. |
| `--count` | `integer` | No | The number of records to return. Default value is 10. Maximum value is 1000 |
| `--offset` | `integer` | No | Used for [pagination](https://mailchimp.com/developer/marketing/docs/methods-parameters/#pagination), this is the number of records from a collection to skip. Default value is 0. |

---

### `mcapi reports`

#### `mcapi reports get`

Get report details for a specific sent campaign.

`GET /3.0/reports/{campaign_id}`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--fields` | `string[]` | No | A comma-separated list of fields to return. Reference parameters of sub-objects with dot notation. |
| `--exclude-fields` | `string[]` | No | A comma-separated list of fields to exclude. Reference parameters of sub-objects with dot notation. |
| `--campaign-id` | `string` | Yes | The unique id for the campaign. |

#### `mcapi reports get-abuse-report`

Get information about a specific abuse report for a campaign.

`GET /3.0/reports/{campaign_id}/abuse-reports/{report_id}`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--fields` | `string[]` | No | A comma-separated list of fields to return. Reference parameters of sub-objects with dot notation. |
| `--exclude-fields` | `string[]` | No | A comma-separated list of fields to exclude. Reference parameters of sub-objects with dot notation. |
| `--campaign-id` | `string` | Yes | The unique id for the campaign. |
| `--report-id` | `string` | Yes | The id for the abuse report. |

#### `mcapi reports get-click-detail`

Get click details for a specific link in a campaign.

`GET /3.0/reports/{campaign_id}/click-details/{link_id}`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--fields` | `string[]` | No | A comma-separated list of fields to return. Reference parameters of sub-objects with dot notation. |
| `--exclude-fields` | `string[]` | No | A comma-separated list of fields to exclude. Reference parameters of sub-objects with dot notation. |
| `--campaign-id` | `string` | Yes | The unique id for the campaign. |
| `--link-id` | `string` | Yes | The id for the link. |
| `--filter-bots` | `boolean` | No | When true, exclude automated bot clicks so the returned click counts reflect human clicks only, matching the in-app Recipient Activity view. Filtering changes a link's counts, but never removes a link from the response. Defaults to false (all clicks). |

#### `mcapi reports get-click-detail-member`

Get information about a specific subscriber who clicked a link in a specific campaign.

`GET /3.0/reports/{campaign_id}/click-details/{link_id}/members/{subscriber_hash}`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--fields` | `string[]` | No | A comma-separated list of fields to return. Reference parameters of sub-objects with dot notation. |
| `--exclude-fields` | `string[]` | No | A comma-separated list of fields to exclude. Reference parameters of sub-objects with dot notation. |
| `--campaign-id` | `string` | Yes | The unique id for the campaign. |
| `--link-id` | `string` | Yes | The id for the link. |
| `--subscriber-hash` | `string` | Yes | The MD5 hash of the lowercase version of the list member's email address. |

#### `mcapi reports get-email-activity`

Get a specific list member's activity in a campaign including opens, clicks, and bounces.

`GET /3.0/reports/{campaign_id}/email-activity/{subscriber_hash}`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--fields` | `string[]` | No | A comma-separated list of fields to return. Reference parameters of sub-objects with dot notation. |
| `--exclude-fields` | `string[]` | No | A comma-separated list of fields to exclude. Reference parameters of sub-objects with dot notation. |
| `--campaign-id` | `string` | Yes | The unique id for the campaign. |
| `--subscriber-hash` | `string` | Yes | The MD5 hash of the lowercase version of the list member's email address. |
| `--since` | `string` | No | Restrict results to email activity events that occur after a specific time. Uses ISO 8601 time format: 2015-10-21T15:41:36+00:00. |
| `--filter-bots` | `boolean` | No | When true, exclude automated bot and Apple Mail Privacy Protection (MPP) proxy activity so the returned activity reflects human-only opens and clicks, matching the in-app Recipient Activity view. Filtering removes events from a member's activity, but never removes the member from the response. Defaults to false (all activity). |

#### `mcapi reports get-open-detail`

Get information about a specific subscriber who opened a campaign.

`GET /3.0/reports/{campaign_id}/open-details/{subscriber_hash}`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--fields` | `string[]` | No | A comma-separated list of fields to return. Reference parameters of sub-objects with dot notation. |
| `--exclude-fields` | `string[]` | No | A comma-separated list of fields to exclude. Reference parameters of sub-objects with dot notation. |
| `--campaign-id` | `string` | Yes | The unique id for the campaign. |
| `--subscriber-hash` | `string` | Yes | The MD5 hash of the lowercase version of the list member's email address. |
| `--filter-bots` | `boolean` | No | When true, exclude automated (proxy/bot) opens so the returned open counts reflect human opens only, matching the in-app Recipient Activity view. A member whose opens are all automated is excluded from the human-only view. Defaults to false (all opens). |

#### `mcapi reports get-sent-to`

Get information about a specific campaign recipient.

`GET /3.0/reports/{campaign_id}/sent-to/{subscriber_hash}`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--fields` | `string[]` | No | A comma-separated list of fields to return. Reference parameters of sub-objects with dot notation. |
| `--exclude-fields` | `string[]` | No | A comma-separated list of fields to exclude. Reference parameters of sub-objects with dot notation. |
| `--campaign-id` | `string` | Yes | The unique id for the campaign. |
| `--subscriber-hash` | `string` | Yes | The MD5 hash of the lowercase version of the list member's email address. |

#### `mcapi reports get-unsubscribed`

Get information about a specific list member who unsubscribed from a campaign.

`GET /3.0/reports/{campaign_id}/unsubscribed/{subscriber_hash}`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--fields` | `string[]` | No | A comma-separated list of fields to return. Reference parameters of sub-objects with dot notation. |
| `--exclude-fields` | `string[]` | No | A comma-separated list of fields to exclude. Reference parameters of sub-objects with dot notation. |
| `--campaign-id` | `string` | Yes | The unique id for the campaign. |
| `--subscriber-hash` | `string` | Yes | The MD5 hash of the lowercase version of the list member's email address. |

#### `mcapi reports list`

Get campaign reports.

`GET /3.0/reports`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--fields` | `string[]` | No | A comma-separated list of fields to return. Reference parameters of sub-objects with dot notation. |
| `--exclude-fields` | `string[]` | No | A comma-separated list of fields to exclude. Reference parameters of sub-objects with dot notation. |
| `--count` | `integer` | No | The number of records to return. Default value is 10. Maximum value is 1000 |
| `--offset` | `integer` | No | Used for [pagination](https://mailchimp.com/developer/marketing/docs/methods-parameters/#pagination), this is the number of records from a collection to skip. Default value is 0. |
| `--type` | `regular | plaintext | absplit | rss | variate` | No | The campaign type. |
| `--before-send-time` | `string (date-time)` | No | Restrict the response to campaigns sent before the set time. Uses ISO 8601 time format: 2015-10-21T15:41:36+00:00. |
| `--since-send-time` | `string (date-time)` | No | Restrict the response to campaigns sent after the set time. Uses ISO 8601 time format: 2015-10-21T15:41:36+00:00. |

#### `mcapi reports list-abuse-reports`

Get a list of abuse complaints for a specific campaign.

`GET /3.0/reports/{campaign_id}/abuse-reports`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--fields` | `string[]` | No | A comma-separated list of fields to return. Reference parameters of sub-objects with dot notation. |
| `--exclude-fields` | `string[]` | No | A comma-separated list of fields to exclude. Reference parameters of sub-objects with dot notation. |
| `--campaign-id` | `string` | Yes | The unique id for the campaign. |

#### `mcapi reports list-advice`

Get feedback based on a campaign's statistics. Advice feedback is based on campaign stats like opens, clicks, unsubscribes, bounces, and more.

`GET /3.0/reports/{campaign_id}/advice`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--fields` | `string[]` | No | A comma-separated list of fields to return. Reference parameters of sub-objects with dot notation. |
| `--exclude-fields` | `string[]` | No | A comma-separated list of fields to exclude. Reference parameters of sub-objects with dot notation. |
| `--campaign-id` | `string` | Yes | The unique id for the campaign. |

#### `mcapi reports list-click-detail-members`

Get information about list members who clicked on a specific link in a campaign.

`GET /3.0/reports/{campaign_id}/click-details/{link_id}/members`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--fields` | `string[]` | No | A comma-separated list of fields to return. Reference parameters of sub-objects with dot notation. |
| `--exclude-fields` | `string[]` | No | A comma-separated list of fields to exclude. Reference parameters of sub-objects with dot notation. |
| `--count` | `integer` | No | The number of records to return. Default value is 10. Maximum value is 1000 |
| `--offset` | `integer` | No | Used for [pagination](https://mailchimp.com/developer/marketing/docs/methods-parameters/#pagination), this is the number of records from a collection to skip. Default value is 0. |
| `--campaign-id` | `string` | Yes | The unique id for the campaign. |
| `--link-id` | `string` | Yes | The id for the link. |

#### `mcapi reports list-click-details`

Get information about clicks on specific links in your Mailchimp campaigns.

`GET /3.0/reports/{campaign_id}/click-details`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--fields` | `string[]` | No | A comma-separated list of fields to return. Reference parameters of sub-objects with dot notation. |
| `--exclude-fields` | `string[]` | No | A comma-separated list of fields to exclude. Reference parameters of sub-objects with dot notation. |
| `--count` | `integer` | No | The number of records to return. Default value is 10. Maximum value is 1000 |
| `--offset` | `integer` | No | Used for [pagination](https://mailchimp.com/developer/marketing/docs/methods-parameters/#pagination), this is the number of records from a collection to skip. Default value is 0. |
| `--campaign-id` | `string` | Yes | The unique id for the campaign. |
| `--sort-field` | `total_clicks | unique_clicks` | No | Returns click reports sorted by the specified field. |
| `--sort-dir` | `ASC | DESC` | No | Determines the order direction for sorted results. |
| `--filter-bots` | `boolean` | No | When true, exclude automated bot clicks so the returned click counts reflect human clicks only, matching the in-app Recipient Activity view. Filtering changes a link's counts, but never removes a link from the response. Defaults to false (all clicks). |

#### `mcapi reports list-domain-performance`

Get statistics for the top-performing email domains in a campaign.

`GET /3.0/reports/{campaign_id}/domain-performance`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--fields` | `string[]` | No | A comma-separated list of fields to return. Reference parameters of sub-objects with dot notation. |
| `--exclude-fields` | `string[]` | No | A comma-separated list of fields to exclude. Reference parameters of sub-objects with dot notation. |
| `--campaign-id` | `string` | Yes | The unique id for the campaign. |

#### `mcapi reports list-ecommerce-product-activity`

Get breakdown of product activity for a campaign

`GET /3.0/reports/{campaign_id}/ecommerce-product-activity`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--fields` | `string[]` | No | A comma-separated list of fields to return. Reference parameters of sub-objects with dot notation. |
| `--exclude-fields` | `string[]` | No | A comma-separated list of fields to exclude. Reference parameters of sub-objects with dot notation. |
| `--count` | `integer` | No | The number of records to return. Default value is 10. Maximum value is 1000 |
| `--offset` | `integer` | No | Used for [pagination](https://mailchimp.com/developer/marketing/docs/methods-parameters/#pagination), this is the number of records from a collection to skip. Default value is 0. |
| `--campaign-id` | `string` | Yes | The unique id for the campaign. |
| `--sort-field` | `title | total_revenue | total_purchased` | No | Returns files sorted by the specified field. |

#### `mcapi reports list-eepurl`

Get a summary of social activity for the campaign, tracked by EepURL.

`GET /3.0/reports/{campaign_id}/eepurl`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--fields` | `string[]` | No | A comma-separated list of fields to return. Reference parameters of sub-objects with dot notation. |
| `--exclude-fields` | `string[]` | No | A comma-separated list of fields to exclude. Reference parameters of sub-objects with dot notation. |
| `--campaign-id` | `string` | Yes | The unique id for the campaign. |

#### `mcapi reports list-email-activity`

Get a list of member's subscriber activity in a specific campaign.

`GET /3.0/reports/{campaign_id}/email-activity`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--fields` | `string[]` | No | A comma-separated list of fields to return. Reference parameters of sub-objects with dot notation. |
| `--exclude-fields` | `string[]` | No | A comma-separated list of fields to exclude. Reference parameters of sub-objects with dot notation. |
| `--count` | `integer` | No | The number of records to return. Default value is 10. Maximum value is 1000 |
| `--offset` | `integer` | No | Used for [pagination](https://mailchimp.com/developer/marketing/docs/methods-parameters/#pagination), this is the number of records from a collection to skip. Default value is 0. |
| `--campaign-id` | `string` | Yes | The unique id for the campaign. |
| `--since` | `string` | No | Restrict results to email activity events that occur after a specific time. Uses ISO 8601 time format: 2015-10-21T15:41:36+00:00. |
| `--filter-bots` | `boolean` | No | When true, exclude automated bot and Apple Mail Privacy Protection (MPP) proxy activity so the returned activity reflects human-only opens and clicks, matching the in-app Recipient Activity view. Filtering removes events from a member's activity, but never removes the member from the response. Defaults to false (all activity). |

#### `mcapi reports list-locations`

Get top open locations for a specific campaign.

`GET /3.0/reports/{campaign_id}/locations`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--fields` | `string[]` | No | A comma-separated list of fields to return. Reference parameters of sub-objects with dot notation. |
| `--exclude-fields` | `string[]` | No | A comma-separated list of fields to exclude. Reference parameters of sub-objects with dot notation. |
| `--campaign-id` | `string` | Yes | The unique id for the campaign. |
| `--count` | `integer` | No | The number of records to return. Default value is 10. Maximum value is 1000 |
| `--offset` | `integer` | No | Used for [pagination](https://mailchimp.com/developer/marketing/docs/methods-parameters/#pagination), this is the number of records from a collection to skip. Default value is 0. |

#### `mcapi reports list-open-details`

Get detailed information about any campaign emails that were opened by a list member.

`GET /3.0/reports/{campaign_id}/open-details`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--fields` | `string[]` | No | A comma-separated list of fields to return. Reference parameters of sub-objects with dot notation. |
| `--exclude-fields` | `string[]` | No | A comma-separated list of fields to exclude. Reference parameters of sub-objects with dot notation. |
| `--count` | `integer` | No | The number of records to return. Default value is 10. Maximum value is 1000 |
| `--offset` | `integer` | No | Used for [pagination](https://mailchimp.com/developer/marketing/docs/methods-parameters/#pagination), this is the number of records from a collection to skip. Default value is 0. |
| `--campaign-id` | `string` | Yes | The unique id for the campaign. |
| `--since` | `string` | No | Restrict results to campaign open events that occur after a specific time. Uses ISO 8601 time format: 2015-10-21T15:41:36+00:00. |
| `--sort-field` | `opens_count` | No | Returns open reports sorted by the specified field. |
| `--sort-dir` | `ASC | DESC` | No | Determines the order direction for sorted results. |
| `--filter-bots` | `boolean` | No | When true, exclude automated (proxy/bot) opens so the returned open counts reflect human opens only, matching the in-app Recipient Activity view. A member whose opens are all automated is excluded from the human-only view. Defaults to false (all opens). |

#### `mcapi reports list-sent-to`

Get information about campaign recipients.

`GET /3.0/reports/{campaign_id}/sent-to`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--fields` | `string[]` | No | A comma-separated list of fields to return. Reference parameters of sub-objects with dot notation. |
| `--exclude-fields` | `string[]` | No | A comma-separated list of fields to exclude. Reference parameters of sub-objects with dot notation. |
| `--count` | `integer` | No | The number of records to return. Default value is 10. Maximum value is 1000 |
| `--offset` | `integer` | No | Used for [pagination](https://mailchimp.com/developer/marketing/docs/methods-parameters/#pagination), this is the number of records from a collection to skip. Default value is 0. |
| `--campaign-id` | `string` | Yes | The unique id for the campaign. |

#### `mcapi reports list-sub-reports`

Get a list of reports with child campaigns for a specific parent campaign.

`GET /3.0/reports/{campaign_id}/sub-reports`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--fields` | `string[]` | No | A comma-separated list of fields to return. Reference parameters of sub-objects with dot notation. |
| `--exclude-fields` | `string[]` | No | A comma-separated list of fields to exclude. Reference parameters of sub-objects with dot notation. |
| `--campaign-id` | `string` | Yes | The unique id for the campaign. |

#### `mcapi reports list-unsubscribed`

Get information about members who have unsubscribed from a specific campaign.

`GET /3.0/reports/{campaign_id}/unsubscribed`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--fields` | `string[]` | No | A comma-separated list of fields to return. Reference parameters of sub-objects with dot notation. |
| `--exclude-fields` | `string[]` | No | A comma-separated list of fields to exclude. Reference parameters of sub-objects with dot notation. |
| `--count` | `integer` | No | The number of records to return. Default value is 10. Maximum value is 1000 |
| `--offset` | `integer` | No | Used for [pagination](https://mailchimp.com/developer/marketing/docs/methods-parameters/#pagination), this is the number of records from a collection to skip. Default value is 0. |
| `--campaign-id` | `string` | Yes | The unique id for the campaign. |

---

### `mcapi root`

#### `mcapi root list`

Get links to all other resources available in the API.

`GET /3.0/`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--fields` | `string[]` | No | A comma-separated list of fields to return. Reference parameters of sub-objects with dot notation. |
| `--exclude-fields` | `string[]` | No | A comma-separated list of fields to exclude. Reference parameters of sub-objects with dot notation. |

---

### `mcapi search-campaigns`

#### `mcapi search-campaigns list`

Search all campaigns for the specified query terms.

`GET /3.0/search-campaigns`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--fields` | `string[]` | No | A comma-separated list of fields to return. Reference parameters of sub-objects with dot notation. |
| `--exclude-fields` | `string[]` | No | A comma-separated list of fields to exclude. Reference parameters of sub-objects with dot notation. |
| `--query` | `string` | Yes | The search query used to filter results. |

---

### `mcapi search-members`

#### `mcapi search-members list`

Search for list members. This search can be restricted to a specific list, or can be used to search across all lists in an account.

`GET /3.0/search-members`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--fields` | `string[]` | No | A comma-separated list of fields to return. Reference parameters of sub-objects with dot notation. |
| `--exclude-fields` | `string[]` | No | A comma-separated list of fields to exclude. Reference parameters of sub-objects with dot notation. |
| `--query` | `string` | Yes | The search query used to filter results. Query should be a valid email, or a string representing a contact's first or last name. |
| `--list-id` | `string` | No | The unique id for the list. |

---

### `mcapi sms-campaigns`

#### `mcapi sms-campaigns create`

Create a new SMS campaign.

`POST /3.0/sms-campaigns`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--json` | `JSON` | Yes | Request body as JSON (or use individual body-field flags) |

#### `mcapi sms-campaigns create-action-cancel-send`

Cancel a scheduled or sending SMS campaign.

`POST /3.0/sms-campaigns/{sms_campaign_id}/actions/cancel-send`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--sms-campaign-id` | `string` | Yes | The unique id for the SMS campaign. |

#### `mcapi sms-campaigns create-action-schedule`

Schedule an SMS campaign for delivery.

`POST /3.0/sms-campaigns/{sms_campaign_id}/actions/schedule`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--sms-campaign-id` | `string` | Yes | The unique id for the SMS campaign. |
| `--json` | `JSON` | Yes | Request body as JSON (or use individual body-field flags) |

#### `mcapi sms-campaigns create-action-send`

Send an SMS campaign.

`POST /3.0/sms-campaigns/{sms_campaign_id}/actions/send`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--sms-campaign-id` | `string` | Yes | The unique id for the SMS campaign. |

#### `mcapi sms-campaigns delete`

Remove a campaign from your Mailchimp account.

`DELETE /3.0/sms-campaigns/{sms_campaign_id}`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--sms-campaign-id` | `string` | Yes | The unique id for the SMS campaign. |

#### `mcapi sms-campaigns get`

Get the details for a single SMS campaign.

`GET /3.0/sms-campaigns/{sms_campaign_id}`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--sms-campaign-id` | `string` | Yes | The unique id for the SMS campaign. |
| `--fields` | `string[]` | No | A comma-separated list of fields to return. Reference parameters of sub-objects with dot notation. |
| `--exclude-fields` | `string[]` | No | A comma-separated list of fields to exclude. Reference parameters of sub-objects with dot notation. |

#### `mcapi sms-campaigns get-content`

Get the content for an SMS campaign.

`GET /3.0/sms-campaigns/{sms_campaign_id}/content`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--sms-campaign-id` | `string` | Yes | The unique id for the SMS campaign. |
| `--fields` | `string[]` | No | A comma-separated list of fields to return. Reference parameters of sub-objects with dot notation. |
| `--exclude-fields` | `string[]` | No | A comma-separated list of fields to exclude. Reference parameters of sub-objects with dot notation. |

#### `mcapi sms-campaigns list`

Get all SMS campaigns in an account.

`GET /3.0/sms-campaigns`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--fields` | `string[]` | No | A comma-separated list of fields to return. Reference parameters of sub-objects with dot notation. |
| `--exclude-fields` | `string[]` | No | A comma-separated list of fields to exclude. Reference parameters of sub-objects with dot notation. |
| `--count` | `integer` | No | The number of records to return. Default value is 10. Maximum value is 1000 |
| `--offset` | `integer` | No | Used for [pagination](https://mailchimp.com/developer/marketing/docs/methods-parameters/#pagination), this is the number of records from a collection to skip. Default value is 0. |

#### `mcapi sms-campaigns update`

Update an SMS campaign.

`PATCH /3.0/sms-campaigns/{sms_campaign_id}`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--sms-campaign-id` | `string` | Yes | The unique id for the SMS campaign. |
| `--json` | `JSON` | Yes | Request body as JSON (or use individual body-field flags) |

#### `mcapi sms-campaigns upsert-content`

Set the content for an SMS campaign.

`PUT /3.0/sms-campaigns/{sms_campaign_id}/content`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--sms-campaign-id` | `string` | Yes | The unique id for the SMS campaign. |
| `--json` | `JSON` | Yes | Request body as JSON (or use individual body-field flags) |

---

### `mcapi surveys`

#### `mcapi surveys create-list-survey-action-create-email`

Utilize the List ID and Survey ID to generate a Campaign that links to your survey.

`POST /3.0/lists/{list_id}/surveys/{survey_id}/actions/create-email`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--list-id` | `string` | Yes | The unique ID for the list. |
| `--survey-id` | `string` | Yes | The ID of the survey. |

#### `mcapi surveys create-list-survey-action-publish`

Publish a survey that is in draft, unpublished, or has been previously published and edited.

`POST /3.0/lists/{list_id}/surveys/{survey_id}/actions/publish`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--list-id` | `string` | Yes | The unique ID for the list. |
| `--survey-id` | `string` | Yes | The ID of the survey. |

#### `mcapi surveys create-list-survey-action-unpublish`

Unpublish a survey that has been published.

`POST /3.0/lists/{list_id}/surveys/{survey_id}/actions/unpublish`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--list-id` | `string` | Yes | The unique ID for the list. |
| `--survey-id` | `string` | Yes | The ID of the survey. |

---

### `mcapi template-folders`

#### `mcapi template-folders create`

Create a new template folder.

`POST /3.0/template-folders`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--json` | `JSON` | Yes | Request body as JSON (or use individual body-field flags) |

#### `mcapi template-folders delete`

Delete a specific template folder, and mark all the templates in the folder as 'unfiled'.

`DELETE /3.0/template-folders/{folder_id}`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--folder-id` | `string` | Yes | The unique id for the template folder. |

#### `mcapi template-folders get`

Get information about a specific folder used to organize templates.

`GET /3.0/template-folders/{folder_id}`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--fields` | `string[]` | No | A comma-separated list of fields to return. Reference parameters of sub-objects with dot notation. |
| `--exclude-fields` | `string[]` | No | A comma-separated list of fields to exclude. Reference parameters of sub-objects with dot notation. |
| `--folder-id` | `string` | Yes | The unique id for the template folder. |

#### `mcapi template-folders list`

Get all folders used to organize templates.

`GET /3.0/template-folders`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--fields` | `string[]` | No | A comma-separated list of fields to return. Reference parameters of sub-objects with dot notation. |
| `--exclude-fields` | `string[]` | No | A comma-separated list of fields to exclude. Reference parameters of sub-objects with dot notation. |
| `--count` | `integer` | No | The number of records to return. Default value is 10. Maximum value is 1000 |
| `--offset` | `integer` | No | Used for [pagination](https://mailchimp.com/developer/marketing/docs/methods-parameters/#pagination), this is the number of records from a collection to skip. Default value is 0. |

#### `mcapi template-folders update`

Update a specific folder used to organize templates.

`PATCH /3.0/template-folders/{folder_id}`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--folder-id` | `string` | Yes | The unique id for the template folder. |
| `--json` | `JSON` | Yes | Request body as JSON (or use individual body-field flags) |

---

### `mcapi templates`

#### `mcapi templates create`

Create a new template for the account. Only Classic templates are supported.

`POST /3.0/templates`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--json` | `JSON` | Yes | Request body as JSON (or use individual body-field flags) |

#### `mcapi templates delete`

Delete a specific template.

`DELETE /3.0/templates/{template_id}`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--template-id` | `string` | Yes | The unique id for the template. |

#### `mcapi templates get`

Get information about a specific template.

`GET /3.0/templates/{template_id}`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--fields` | `string[]` | No | A comma-separated list of fields to return. Reference parameters of sub-objects with dot notation. |
| `--exclude-fields` | `string[]` | No | A comma-separated list of fields to exclude. Reference parameters of sub-objects with dot notation. |
| `--template-id` | `string` | Yes | The unique id for the template. |

#### `mcapi templates list`

Get a list of an account's available templates.

`GET /3.0/templates`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--fields` | `string[]` | No | A comma-separated list of fields to return. Reference parameters of sub-objects with dot notation. |
| `--exclude-fields` | `string[]` | No | A comma-separated list of fields to exclude. Reference parameters of sub-objects with dot notation. |
| `--count` | `integer` | No | The number of records to return. Default value is 10. Maximum value is 1000 |
| `--offset` | `integer` | No | Used for [pagination](https://mailchimp.com/developer/marketing/docs/methods-parameters/#pagination), this is the number of records from a collection to skip. Default value is 0. |
| `--created-by` | `string` | No | The Mailchimp account user who created the template. |
| `--since-date-created` | `string` | No | Restrict the response to templates created after the set date. Uses ISO 8601 time format: 2015-10-21T15:41:36+00:00. |
| `--before-date-created` | `string` | No | Restrict the response to templates created before the set date. Uses ISO 8601 time format: 2015-10-21T15:41:36+00:00. |
| `--type` | `string` | No | Limit results based on template type. |
| `--category` | `string` | No | Limit results based on category. |
| `--folder-id` | `string` | No | The unique folder id. |
| `--sort-field` | `date_created | date_edited | name` | No | Returns user templates sorted by the specified field. |
| `--content-type` | `html | template | multichannel` | No | Limit results based on how the template's content is put together. Only templates of type `user` can be filtered by `content_type`. If you want to retrieve saved templates created with the legacy email editor, then filter `content_type` to `template`. If you'd rather pull your saved templates for the new editor, filter to `multichannel`. For code your own templates, filter to `html`. |
| `--sort-dir` | `ASC | DESC` | No | Determines the order direction for sorted results. |

#### `mcapi templates list-default-content`

Get the sections that you can edit in a template, including each section's default content.

`GET /3.0/templates/{template_id}/default-content`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--fields` | `string[]` | No | A comma-separated list of fields to return. Reference parameters of sub-objects with dot notation. |
| `--exclude-fields` | `string[]` | No | A comma-separated list of fields to exclude. Reference parameters of sub-objects with dot notation. |
| `--template-id` | `string` | Yes | The unique id for the template. |

#### `mcapi templates update`

Update the name, HTML, or `folder_id` of an existing template.

`PATCH /3.0/templates/{template_id}`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--template-id` | `string` | Yes | The unique id for the template. |
| `--json` | `JSON` | Yes | Request body as JSON (or use individual body-field flags) |

---

### `mcapi verified-domains`

#### `mcapi verified-domains create`

Add a domain to the account.

`POST /3.0/verified-domains`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--json` | `JSON` | Yes | Request body as JSON (or use individual body-field flags) |

#### `mcapi verified-domains create-action-verify`

Verify a domain for sending.

`POST /3.0/verified-domains/{domain_name}/actions/verify`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--domain-name` | `string` | Yes | The domain name. |
| `--json` | `JSON` | Yes | Request body as JSON (or use individual body-field flags) |

#### `mcapi verified-domains delete`

Delete a verified domain from the account.

`DELETE /3.0/verified-domains/{domain_name}`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--domain-name` | `string` | Yes | The domain name. |

#### `mcapi verified-domains get`

Get the details for a single domain on the account.

`GET /3.0/verified-domains/{domain_name}`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--domain-name` | `string` | Yes | The domain name. |

#### `mcapi verified-domains list`

Get all of the sending domains on the account.

`GET /3.0/verified-domains`

---

## Global flags

These flags are available on every command:

| Flag | Description |
|------|-------------|
| `--dry-run` | Print the HTTP request without sending it |
| `--json <JSON\|->` | Supply the request body as JSON (or `-` for stdin) |
| `--params <JSON>` | Merge extra parameters as JSON |
| `--format <json\|table\|yaml\|csv>` | Output format (default: `json`) |
| `--output <PATH>` | Write binary responses to a file |
| `--base-url <URL>` | Override the API base URL |
| `--page-all` | Auto-paginate and stream all results |
| `--page-limit <N>` | Max pages to fetch (default: `10`) |
| `-q, --quiet` | Suppress stdout on success |
| `-h, --help` | Print help |
| `-V, --version` | Print version |

