use std::{sync::Arc, time::Duration};

use crate::{error::AnythingError, types::AnythingResult, wait_strategy::WaitStrategy};

use super::{sequence::Sequence, sequence_group::SequenceGroup};

pub struct Sequencer {
    cursor: Sequence,
    gating_sequence_cache: Arc<Sequence>,
    gating_sequences: SequenceGroup,
    ring_capacity: u64,
    wait_strategy: WaitStrategy,
}

#[allow(dead_code)]
impl Sequencer {
    pub fn new(ring_capacity: u64, wait_strategy: WaitStrategy) -> Self {
        Self {
            cursor: Sequence::with_value(0),
            gating_sequence_cache: Arc::new(Sequence::with_value(0)),
            gating_sequences: SequenceGroup::new(),
            ring_capacity,
            wait_strategy,
        }
    }

    pub(crate) fn register_gating_sequence(&self, sequence: Arc<Sequence>) {
        self.gating_sequences.add(sequence);
    }

    pub fn get(&self) -> u64 {
        self.cursor.get()
    }

    pub fn next(&self) -> u64 {
        self.next_from(1)
            .expect("sequencer could not get next sequence from sequence 1")
    }

    pub fn next_from(&self, next: u64) -> AnythingResult<u64> {
        if next < 1 || next > self.ring_capacity {
            return Err(AnythingError::RingCapacityOutOfBoundsError);
        }

        loop {
            std::hint::spin_loop();

            let curr: u64 = self.cursor.get();
            let icurr: i64 = curr as i64;
            let next: i64 = (curr + next) as i64;

            let wrap = next - self.ring_capacity as i64;
            let cached_gating_sequence = self.gating_sequence_cache.get() as i64;

            if wrap >= cached_gating_sequence || cached_gating_sequence > icurr {
                let gating_sequence = self.gating_sequences.minimum_sequence(curr);

                match self.wait_strategy {
                    WaitStrategy::AllSubscribers => {
                        if wrap > gating_sequence as i64 {
                            std::thread::sleep(Duration::from_micros(200));
                            continue;
                        }
                    }
                    WaitStrategy::NoWait => {
                        if self.cursor.compare_exchange(curr, next as u64) {
                            return Ok(next as u64);
                        }
                    }
                    WaitStrategy::WaitForDuration(wait_time) => {
                        if self.cursor.compare_exchange(curr, next as u64) {
                            std::thread::sleep(wait_time);

                            return Ok(next as u64);
                        }
                    }
                }
                self.gating_sequence_cache.set(gating_sequence);
            } else if self.cursor.compare_exchange(curr, next as u64) {
                return Ok(next as u64);
            }
        }
    }
}
