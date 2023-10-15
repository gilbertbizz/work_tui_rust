use std::{
    sync::mpsc, 
    time::{Instant, Duration},
    collections::{BTreeMap, VecDeque},
};

use rand::prelude::Distribution;

pub const NUM_DOWNLOADS: usize = 10;

type DownloadId = usize;
type WorkerId = usize;

pub enum Event {
    Input(crossterm::event::KeyEvent),
    Tick,
    Resize,
    DownloadUpdate(WorkerId, DownloadId, f64),
    DownloadDone(WorkerId, DownloadId)
}

pub struct Worker {
   pub id: WorkerId,
   pub tx: mpsc::Sender<Download> 
}

pub struct Download {
   pub id: DownloadId,
   pub size: usize,
}

pub struct DownloadInProgress {
 pub id: DownloadId,
 pub started_at: Instant,
 pub progress: f64 
}

pub struct Downloads {
   pub pending: VecDeque<Download>,
   pub in_progress: BTreeMap<WorkerId, DownloadInProgress>,
}

impl Downloads {
    pub fn next(&mut self, worker_id: WorkerId) -> Option<Download> {
       match self.pending.pop_front() {
            Some(d) => {
                self.in_progress.insert(
                    worker_id, 
                    DownloadInProgress { 
                        id: d.id, 
                        started_at: Instant::now(), 
                        progress: 0.0,
                    });
                Some(d)
            },
            None => None
       } 
    }
}

pub fn downloads() -> Downloads {
    let distribution = rand::distributions::Uniform::new(0, 1000);
    let mut rng = rand::thread_rng();
    let pending = (0..NUM_DOWNLOADS)
        .map(|id| {
            let size = distribution.sample(&mut rng);
            Download {
                id,
                size
            }
        }).collect();
    Downloads {
        pending,
        in_progress: BTreeMap::new(),
    }
}

pub fn workers(tx: mpsc::Sender<Event>) -> Vec<Worker> {
    (0..4)
        .map(|id| {
            let (worker_tx, worker_rx) = mpsc::channel::<Download>();
            let tx = tx.clone();
            std::thread::spawn(move || {
                while let Ok(download) = worker_rx.recv() {
                    let mut remaining = download.size;
                    while remaining > 0 {
                        let wait = (remaining as u64).min(10);
                        std::thread::sleep(Duration::from_millis(wait * 10));
                        remaining = remaining.saturating_sub(10);
                        let progress = (download.size - remaining) * 100 / download.size;
                        tx.send(Event::DownloadUpdate(id, download.id, progress as f64))
                            .unwrap();
                    }
                    tx.send(Event::DownloadDone(id, download.id)).unwrap();
                }
            });
           Worker {id, tx: worker_tx} 
        }).collect()
}
