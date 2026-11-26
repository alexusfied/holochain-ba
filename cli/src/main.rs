use anyhow::Ok;
use clap::Parser;
use std::{fs::File, io::BufReader};

use holochain_client::{
    AdminWebsocket, AppWebsocket, AuthorizeSigningCredentialsPayload, CellInfo, ClientAgentSigner, ExternIO, IssueAppAuthenticationTokenPayload, ZomeCallTarget
};

use event_integrity::Event;

#[derive(Parser)]
struct Arguments {
    path: std::path::PathBuf,
    admin_port: u16,
    app_port: u16,
    app_id: String
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let args = Arguments::parse();
    
    let file = File::open(args.path).expect("File could not be opened");
    let reader = BufReader::new(file);
    let event: Event = serde_json::from_reader(reader).expect("File could not be parsed");

    // Connect to the admin websocket
    let admin_ws = AdminWebsocket::connect(format!("127.0.0.1:{0}", args.admin_port), None).await?;

    // Get the auth token
    let issued = admin_ws
        .issue_app_auth_token(IssueAppAuthenticationTokenPayload::for_installed_app_id(args.app_id.clone())).await?;

    // Connect to the app websocket
    let signer = ClientAgentSigner::default();
    let app_ws = AppWebsocket::connect(
        format!("127.0.0.1:{0}", args.app_port),
        issued.token,
        signer.clone().into(),
        None
    ).await?;

    // Authorize signing credentials
    let app_info = app_ws.app_info().await?.expect("app not found");
    let (role_name, cells) = app_info.cell_info.iter().next().expect("no roles");
    let cell_id = match &cells[0] {
        CellInfo::Provisioned(c) => c.cell_id.clone(),
        _ => anyhow::bail!("first cell is not provisioned"),
    };

    let credentials = admin_ws
        .authorize_signing_credentials(AuthorizeSigningCredentialsPayload {
            cell_id: cell_id.clone(),
            functions: None, // None = allow all functions
        })
        .await?;
    signer.add_credentials(cell_id, credentials);

    // Create the event
    app_ws.call_zome(
        ZomeCallTarget::RoleName(role_name.clone()),
        "event".into(),
        "create_event".into(),
        ExternIO::encode(event)?
    )
    .await?;

    println!("Event imported successfully");
    Ok(()) 
}
