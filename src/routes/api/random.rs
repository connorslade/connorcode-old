use std::time::{SystemTime, UNIX_EPOCH};
use std::{
    sync::{
        atomic::{AtomicU64, AtomicUsize, Ordering},
        Arc, RwLock,
    },
    time::Duration,
};

use afire::{internal::sync::ForceLockRwLock, Content, Method, Server};
use anyhow::{bail, Result};
use rand::prelude::*;
use rand_pcg::Pcg64;
use rand_seeder::Seeder;

use crate::app::App;

const PASSWORD_CHARS: &[u8] =
    b"abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789(~!@#$%^&*)";

pub fn attach(server: &mut Server<App>) {
    let config = &server.app().config;
    if config.tempest_station.is_empty() || config.tempest_token.is_empty() {
        return;
    }

    let req_count = AtomicUsize::new(0);
    let last_update = AtomicU64::new(0);
    let last_value = Arc::new(RwLock::new(String::new()));

    server.route(Method::GET, "/api/random", move |ctx| {
        let app = ctx.app();
        if get_epoch().as_secs() - last_update.load(Ordering::Relaxed) > 60 {
            update_weather(
                last_value.clone(),
                &app.config.tempest_station,
                &app.config.tempest_token,
            )?;
        }
        last_update.store(get_epoch().as_secs(), Ordering::Relaxed);

        let mut rng: Pcg64 = Seeder::from(format!(
            "{}{}{}",
            last_value.force_read(),
            get_epoch().as_millis(),
            req_count.fetch_add(1, Ordering::Relaxed)
        ))
        .make_rng();

        if let Some(i) = ctx.req.query.get("type") {
            let txt = match i {
                "password" => (0..20)
                    .map(|_| *PASSWORD_CHARS.choose(&mut rng).unwrap())
                    .map(char::from)
                    .collect::<String>(),
                "dice" => rng.gen_range(1..=6).to_string(),
                "coin" => if rng.gen_ratio(1, 2) {
                    "Heads"
                } else {
                    "Tails"
                }
                .to_owned(),
                _ => {
                    ctx.status(400).text("Invalid type (password)").send()?;
                    return Ok(());
                }
            };
            ctx.text(txt).content(Content::TXT).send()?;
            return Ok(());
        }

        ctx.bytes(rng.gen::<[u8; 32]>()).send()?;
        Ok(())
    });
}

fn update_weather(last_value: Arc<RwLock<String>>, station: &str, token: &str) -> Result<()> {
    let res = ureq::get(&format!(
        "https://swd.weatherflow.com/swd/rest/observations/station/{}?token={}",
        station, token
    ))
    .call()?;

    if res.status() != 200 {
        println!("[-] TEMPEST ERR: {}", res.into_string()?);
        bail!("Real world data error");
    }

    *last_value.force_write() = res.into_string()?;
    Ok(())
}

fn get_epoch() -> Duration {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("System time is set before the unix epoch. (1970-01-01 00:00:00 UTC)")
}
