use codec::Encode;
use sp_keyring::AccountKeyring;
use substrate_subxt::{Call, ClientBuilder, EventsDecoder, NodeTemplateRuntime, PairSigner};

#[derive(Encode)]
pub struct AddBytesCall {
    data: Vec<u8>,
}

impl Call<NodeTemplateRuntime> for AddBytesCall {
    const MODULE: &'static str = "TemplateModule";
    const FUNCTION: &'static str = "ipfs_add_bytes";
    fn events_decoder(_decoder: &mut EventsDecoder<NodeTemplateRuntime>) {}
}

#[async_std::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let signer = PairSigner::<NodeTemplateRuntime, _>::new(AccountKeyring::Alice.pair());
    let client = ClientBuilder::<NodeTemplateRuntime>::new().build().await?;
    let metadata = client.metadata();
    let event_decoder = EventsDecoder::<NodeTemplateRuntime>::new(metadata.clone());

    let add_bytes_call = AddBytesCall {
        data: vec![1, 2, 3, 4],
    };

    let signed_extrinsic = client.create_signed(add_bytes_call, &signer).await?;
    let result = client
        .submit_and_watch_extrinsic(signed_extrinsic, event_decoder)
        .await?;

    println!("Result: {:?}", result);

    Ok(())
}
