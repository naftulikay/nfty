use git2::Progress;

use std::cmp::min;

/// Various Git utilities.
pub struct Git;

impl Git {
    /// Convert the progress of a Git clone into a value between 0 and 100.
    ///
    /// Right now, the implementation is very naïve, index progress counts for 50% of the work and
    /// transfer progress counts for 50% of the work. In reality, these should be biased based on
    /// one value being I/O bound and the other CPU bound. For super large repositories, it's also
    /// more complicated as calculating deltas can take a long time (c.f. `torvalds/linux`).
    pub fn clone_progress(progress: &Progress) -> u64 {
        let (received, indexed, total) = (
            progress.received_objects(),
            progress.indexed_objects(),
            progress.total_objects()
        );

        // get download progress
        let download_progress = if min(received, total) == 0 {
            0.0
        } else {
            ((received as f64) / (total as f64)) * 100.0
        };

        // get index progress
        let index_progress = if min(indexed, total) == 0 {
            0.0
        } else {
            ((indexed as f64) / (total as f64)) * 100.0
        };

        ((download_progress * 0.5) + (index_progress * 0.5)).round() as u64
    }

}
