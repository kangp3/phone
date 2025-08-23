use std::error::Error;

use goertzel::sip::{tlssocket, Dialog};


#[tokio::main]
pub async fn main() -> Result<(), Box<dyn Error>> {
    let ip = public_ip::addr_v4().await.ok_or("no ip")?;

    let (mut rx_ch, tx_ch) = tlssocket::bind().await?;

    let mut dialog = Dialog::new(ip);
    let register_msg = dialog.new_request(rsip::Method::Register, vec![]);
    tx_ch.send(rsip::SipMessage::Request(register_msg.clone())).await?;
    println!("Wrote to stream: {:?}", register_msg);

    let response_msg = rx_ch.recv().await.ok_or("uh oh bad")?;
    println!("Response msg is: {:?}", response_msg);

    Ok(())
}
