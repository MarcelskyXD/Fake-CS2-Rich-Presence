use std::{time::{UNIX_EPOCH, SystemTime, Duration}, thread};

use discord_rich_presence::{activity::{Assets, Timestamps, Activity, Secrets, Party}, DiscordIpc, DiscordIpcClient};
fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("starting...");
    let mut client = DiscordIpcClient::new("1092518298681344000")?;
    println!("connecting");
    client.connect()?;
    println!("connected");

    let assets = Assets::large_image(Assets::new(), "cs2-transformed");
    let start = SystemTime::now();
    let since_the_epoch = start
        .duration_since(UNIX_EPOCH)
        .expect("Time went backwards");
    println!("{:?}", since_the_epoch);
    let timestamps = Timestamps::start(Timestamps::new(), since_the_epoch.as_secs().try_into().unwrap());
    let payload = Activity::new().
        details("Competitive").
        state("In party: ").
        assets(assets).
        party(Party::new().size([1,10]).id("id")).
        secrets(Secrets::join(Secrets::new(), "join")).
        timestamps(timestamps);
    println!("Setting activity");
    client.set_activity(payload)?;
    println!("Activity set");
    println!("Looping to stop application from quitting (CTRL+C to quit)");
    loop {
        thread::sleep(Duration::from_secs(u64::MAX));
    }
}
