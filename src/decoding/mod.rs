use crate::meta::ArcMeta;
use std::path::Path;
use std::sync::Arc;
use async_std::task::block_on;
use async_std::task::spawn;
use async_std::task::Task;
use crate::scaler::Scaler;
use crate::decoding::tiles::decode_and_compare;
use crate::tiles::dest_tile::DestTile;
use crate::tiles::src_tile::SrcTile;
use std::path::PathBuf;
use crate::comparer::Comparer;
use crate::util::RefClonable;
use itertools::Itertools;

pub mod files;
pub mod tiles;
pub mod split;

#[macro_use]
use crate::util::*;

pub fn decode_compare_all<C: Comparer + Send + 'static>(p: Vec<PathBuf>, m: ArcMeta<C>) -> Vec<SrcTile> where C::DestImage: Send + Sync {
    /*let tasks = p.into_iter()
        .map(|i| {
            let a = Arc::clone(&tiles);
            spawn(async{ decode_and_compare::<C,S>(i,a).await })
        })
        .collect::<Vec<_>>();
    //spawn all tasks and then join all, wery constipating and may not very efficient
    tasks.into_iter()
        .map(|t| block_on(t) )
        .filter_map(|t| t.ok() )
        .collect::<Vec<_>>()*/

    /*let mut out = Vec::with_capacity(p.len());

    //so we always spawn up to 64 tasks concurrently and let the stupid executer in parallel
    for c in p.into_iter().chunks(64).into_iter() {
        block_on(async {
            let tasks = c
                .map(|i| {
                    let a = m.refc();
                    spawn(async{ decode_and_compare::<C>(i,a).await })
                })
                .collect::<Vec<_>>();

            for t in tasks {
                if let Ok(r) = t.await {
                    out.push(r);
                }
            }
        });
    }*/

    //out

    async_par!(p,m,64,i,a,{ decode_and_compare::<C>(i,a).await })
}