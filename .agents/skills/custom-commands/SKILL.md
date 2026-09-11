---
name: mcapi-custom-commands
description: How to author custom commands for the mcapi CLI using the co-generated SDK.
---

# Custom Commands for `mcapi`

## Overview

The `mcapi` CLI supports user-authored custom commands that are
compiled into the binary alongside the auto-generated API commands.
Custom commands get a fully-wired SDK client that inherits the CLI's
auth, retries, TLS, base URL, and global headers — zero configuration required.

## Architecture

```
cli/mcapi/custom.rs    ← Your command handlers (protected by .fernignore)
cli/mcapi/sdk.rs       ← Generated bridge: client() + block_on()
cli/mcapi/main.rs      ← Generated entrypoint (calls custom::register)
mcapi-sdk/             ← Co-generated typed SDK crate
mcapi-types/           ← Co-generated typed model crate
```

## Adding a Custom Command

### 1. Edit `cli/mcapi/custom.rs`

This file is protected by `.fernignore` — `fern generate` will never
overwrite it. Register commands in the `register()` function:

```rust
use mcapi_sdk::api::*;

pub fn register(app: CliApp) -> CliApp {
    let app = app.command(
        clap::Command::new("get")
            .about("Get account export info")
            .arg(clap::Arg::new("export_id").required(true))
        ,
        |matches, ctx| {
            let export_id = matches.get_one::<String>("export_id").unwrap();
            let client = super::sdk::client(ctx);
            let result = super::sdk::block_on(
                client.account_exports.get(export_id),
            )?;
            println!("{}", serde_json::to_string_pretty(&result).unwrap());
            Ok(())
        },
    );
    app
}
```

Then build and test:
```bash
cargo build
mcapi get <export_id>
```

### 2. Available SDK Clients

The `super::sdk::client(ctx)` call returns a `mcapi_sdk::api::Client`
with the following sub-clients:

| Field | Type | Description |
|-------|------|-------------|
| `client.root` | `mcapi_sdk::api::RootClient` | root operations |
| `client.account_exports` | `mcapi_sdk::api::AccountExportsClient` | account_exports operations |
| `client.activity_feed` | `mcapi_sdk::api::ActivityFeedClient` | activity_feed operations |
| `client.authorized_apps` | `mcapi_sdk::api::AuthorizedAppsClient` | authorized_apps operations |
| `client.automations` | `mcapi_sdk::api::AutomationsClient` | automations operations |
| `client.batch_webhooks` | `mcapi_sdk::api::BatchWebhooksClient` | batch_webhooks operations |
| `client.batches` | `mcapi_sdk::api::BatchesClient` | batches operations |
| `client.campaign_folders` | `mcapi_sdk::api::CampaignFoldersClient` | campaign_folders operations |
| `client.campaigns` | `mcapi_sdk::api::CampaignsClient` | campaigns operations |
| `client.connected_sites` | `mcapi_sdk::api::ConnectedSitesClient` | connected_sites operations |
| `client.conversations` | `mcapi_sdk::api::ConversationsClient` | conversations operations |
| `client.customer_journeys` | `mcapi_sdk::api::CustomerJourneysClient` | customer_journeys operations |
| `client.ecommerce` | `mcapi_sdk::api::EcommerceClient` | ecommerce operations |
| `client.facebook_ads` | `mcapi_sdk::api::FacebookAdsClient` | facebook_ads operations |
| `client.file_manager` | `mcapi_sdk::api::FileManagerClient` | file_manager operations |
| `client.landing_pages` | `mcapi_sdk::api::LandingPagesClient` | landing_pages operations |
| `client.lists` | `mcapi_sdk::api::ListsClient` | lists operations |
| `client.surveys` | `mcapi_sdk::api::SurveysClient` | surveys operations |
| `client.ping` | `mcapi_sdk::api::PingClient` | ping operations |
| `client.reporting` | `mcapi_sdk::api::ReportingClient` | reporting operations |
| `client.reports` | `mcapi_sdk::api::ReportsClient` | reports operations |
| `client.search_campaigns` | `mcapi_sdk::api::SearchCampaignsClient` | search_campaigns operations |
| `client.sms_campaigns` | `mcapi_sdk::api::SmsCampaignsClient` | sms_campaigns operations |
| `client.search_members` | `mcapi_sdk::api::SearchMembersClient` | search_members operations |
| `client.template_folders` | `mcapi_sdk::api::TemplateFoldersClient` | template_folders operations |
| `client.templates` | `mcapi_sdk::api::TemplatesClient` | templates operations |
| `client.verified_domains` | `mcapi_sdk::api::VerifiedDomainsClient` | verified_domains operations |

### 3. Key Patterns

**Get the SDK client** (execution-sharing, fully authenticated):
```rust
let client = super::sdk::client(ctx);
```

**Run an async SDK call from a sync handler:**
```rust
let result = super::sdk::block_on(
    client.some_resource.some_method(args),
)?;
```

**Use typed models for request/response serialization:**
```rust
use mcapi_sdk::api::*;
```

### 4. Authentication

Custom commands automatically inherit the CLI's authentication.
The following auth schemes are configured:

- **bearerToken** (bearer): env `MCAPI_TOKEN`

No manual auth wiring is needed in custom command handlers.

## Regeneration Safety

| File | Regenerated? | Notes |
|------|-------------|-------|
| `cli/mcapi/custom.rs` | **No** | Protected by `.fernignore` |
| `cli/mcapi/sdk.rs` | Yes | Bridges AppContext → SDK client |
| `cli/mcapi/main.rs` | Yes | Calls `custom::register(app)` |
| `mcapi-sdk/` | Yes | Co-generated typed SDK crate |
| `mcapi-types/` | Yes | Co-generated typed models |

After running `fern generate`, your `custom.rs` is preserved. All
generated code (SDK, types, glue, main.rs) is updated to match the
latest API spec. If the SDK surface changes (renamed methods, new
sub-clients), update your `custom.rs` to match.

## Build & Test

```bash
# Build the CLI (includes custom commands)
cargo build

# Run your custom command
mcapi <your-command> [args]

# Run with verbose output for debugging
RUST_LOG=debug mcapi <your-command> [args]
```
