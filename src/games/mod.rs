//! Currently supported games.

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

/// Alien Swarm
pub mod aliens;
/// Age of Chivalry
pub mod aoc;
/// ARMA 2: Operation Arrowhead
pub mod arma2oa;
/// ARK: Survival Evolved
pub mod ase;
/// Alien Swarm: Reactive Drop
pub mod asrd;
/// Avorion
pub mod avorion;
/// Battalion 1944
pub mod bat1944;
/// BrainBread 2
pub mod bb2;
/// Battlefield 1942
pub mod bf1942;
/// Black Mesa
pub mod bm;
/// Ballistic Overkill
pub mod bo;
/// Codename CURE
pub mod ccure;
/// Colony Survival
pub mod cosu;
/// Counter-Strike
pub mod cs;
/// Counter Strike: Condition Zero
pub mod cscz;
/// Counter-Strike: Global Offensive
pub mod csgo;
/// Counter-Strike: Source
pub mod css;
/// Crysis Wars
pub mod cw;
/// Day of Defeat
pub mod dod;
/// Day of Defeat: Source
pub mod dods;
/// Day of Infamy
pub mod doi;
/// Don't Starve Together
pub mod dst;
/// Frontlines: Fuel of War
pub mod ffow;
/// Garry's Mod
pub mod gm;
/// Halo: Combat Evolved
pub mod haloce;
/// Half-Life 2 Deathmatch
pub mod hl2dm;
/// Half-Life Deathmatch: Source
pub mod hldms;
/// Hell Let Loose
pub mod hll;
/// Insurgency
pub mod ins;
/// Insurgency: Modern Infantry Combat
pub mod insmic;
/// Insurgency: Sandstorm
pub mod inss;
/// Left 4 Dead
pub mod l4d;
/// Left 4 Dead 2
pub mod l4d2;
/// Minecraft
pub mod mc;
/// Operation: Harsh Doorstop
pub mod ohd;
/// Onset
pub mod onset;
/// Project Zomboid
pub mod pz;
/// Quake 1
pub mod quake1;
/// Quake 2
pub mod quake2;
/// Quake 3: Arena
pub mod quake3a;
/// Risk of Rain 2
pub mod ror2;
/// Rust
pub mod rust;
/// Sven Co-op
pub mod sc;
/// 7 Days To Die
pub mod sdtd;
/// Soldier of Fortune 2
pub mod sof2;
/// Serious Sam
pub mod ss;
/// The Forest
pub mod tf;
/// Team Fortress 2
pub mod tf2;
/// Team Fortress Classic
pub mod tfc;
/// The Ship
pub mod ts;
/// Unturned
pub mod unturned;
/// Unreal Tournament
pub mod ut;
/// V Rising
pub mod vr;

use crate::protocols::gamespy::GameSpyVersion;
use crate::protocols::quake::QuakeVersion;
use crate::protocols::{self, Protocol};
use crate::GDResult;
use std::net::{IpAddr, SocketAddr};

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Debug, Clone, PartialEq)]
pub struct Game {
    pub name: &'static str,
    pub default_port: u16,
    pub protocol: Protocol,
}

#[cfg(feature = "game_defs")]
mod definitions;

#[cfg(feature = "game_defs")]
pub use definitions::GAMES;

pub fn query(game: &Game, address: &IpAddr, port: Option<u16>) -> GDResult<protocols::GenericResponse> {
    let socket_addr = SocketAddr::new(*address, port.unwrap_or(game.default_port));
    Ok(match &game.protocol {
        Protocol::Valve(steam_app) => {
            protocols::valve::query(&socket_addr, steam_app.as_engine(), None, None).map(|r| r.into())?
        }
        Protocol::Minecraft(version) => {
            match version {
                Some(protocols::minecraft::Server::Java) => {
                    protocols::minecraft::query_java(&socket_addr, None).map(|r| r.into())?
                }
                Some(protocols::minecraft::Server::Bedrock) => {
                    protocols::minecraft::query_bedrock(&socket_addr, None).map(|r| r.into())?
                }
                Some(protocols::minecraft::Server::Legacy(group)) => {
                    protocols::minecraft::query_legacy_specific(*group, &socket_addr, None).map(|r| r.into())?
                }
                None => protocols::minecraft::query(&socket_addr, None).map(|r| r.into())?,
            }
        }
        Protocol::Gamespy(version) => {
            match version {
                GameSpyVersion::One => protocols::gamespy::one::query(&socket_addr, None).map(|r| r.into())?,
                GameSpyVersion::Two => protocols::gamespy::two::query(&socket_addr, None).map(|r| r.into())?,
                GameSpyVersion::Three => protocols::gamespy::three::query(&socket_addr, None).map(|r| r.into())?,
            }
        }
        Protocol::Quake(version) => {
            match version {
                QuakeVersion::One => protocols::quake::one::query(&socket_addr, None).map(|r| r.into())?,
                QuakeVersion::Two => protocols::quake::two::query(&socket_addr, None).map(|r| r.into())?,
                QuakeVersion::Three => protocols::quake::three::query(&socket_addr, None).map(|r| r.into())?,
            }
        }
        Protocol::TheShip => ts::query(address, port).map(|r| r.into())?,
        Protocol::FFOW => ffow::query(address, port).map(|r| r.into())?,
    })
}
