use vqsort_rs::Key64Value64;

#[derive(Debug, Clone, Copy)]
pub struct Work {
    pub mem_usage: i64,
    pub start: u64,
    pub duration: u64,
}

#[derive(Clone, Copy)]
struct Event {
    mem_diff: i64,
    timestamp: u64,
}

impl From<Key64Value64> for Event {
    fn from(value: Key64Value64) -> Self {
        Self {
            mem_diff: value.value as i64,
            timestamp: value.key,
        }
    }
}

pub fn can_process(memory_limit: u64, work: impl IntoIterator<Item = Work>) -> bool {
    let work = work.into_iter();
    let mut events = Vec::with_capacity(2 * work.size_hint().0);
    for work in work {
        events.push(Key64Value64 {
            value: work.mem_usage as u64,
            key: work.start,
        });
        events.push(Key64Value64 {
            value: (-work.mem_usage) as u64,
            key: work.start + work.duration,
        });
    }

    vqsort_rs::sort(&mut events);

    let mut merged_event: Option<Event> = None;
    let mut remaining_mem = memory_limit as i64;
    for event in events {
        let event = Event::from(event);
        if let Some(buffered_event) = merged_event.as_mut() {
            if buffered_event.timestamp == event.timestamp {
                buffered_event.mem_diff += event.mem_diff;
            } else if buffered_event.mem_diff > remaining_mem {
                return false;
            } else {
                remaining_mem -= buffered_event.mem_diff;
                merged_event = Some(event);
            }
        } else {
            merged_event = Some(event);
        }
    }

    if let Some(buffered_event) = merged_event {
        if buffered_event.mem_diff > remaining_mem {
            return false;
        }
    }

    true
}

#[cfg(test)]
mod tests {
    use super::*;

    fn work(mem_usage: i64, start: u64, duration: u64) -> Work {
        Work {
            mem_usage,
            start,
            duration,
        }
    }

    #[test]
    fn test_empty_work_list() {
        let tasks: Vec<Work> = vec![];
        assert!(can_process(100, tasks));
    }

    #[test]
    fn test_single_task_within_limit() {
        let tasks = vec![work(50, 10, 20)];
        assert!(can_process(100, tasks));
    }

    #[test]
    fn test_single_task_exceeds_limit() {
        let tasks = vec![work(150, 0, 10)];
        assert!(!can_process(100, tasks));
    }

    #[test]
    fn test_non_overlapping_tasks() {
        let tasks = vec![work(50, 0, 10), work(30, 20, 5)];
        assert!(can_process(60, tasks));
    }

    #[test]
    fn test_overlapping_within_limit() {
        let tasks = vec![work(50, 0, 10), work(30, 5, 10)];
        assert!(can_process(80, tasks.iter().copied()));
        assert!(!can_process(79, tasks));
    }

    #[test]
    fn test_overlapping_exceeds_limit() {
        let tasks = vec![work(40, 0, 20), work(50, 10, 10), work(30, 5, 10)];
        assert!(!can_process(100, tasks.iter().copied()));
        assert!(can_process(120, tasks));
    }

    #[test]
    fn test_deallocation_increases_memory() {
        let tasks = vec![work(70, 0, 10), work(50, 10, 5)];
        assert!(can_process(70, tasks.iter().copied()));
        assert!(!can_process(69, tasks));
    }

    #[test]
    fn test_multiple_events_same_timestamp() {
        let tasks = vec![work(10, 5, 5), work(20, 5, 5), work(30, 5, 10)];
        assert!(can_process(60, tasks.clone()));
        assert!(!can_process(59, tasks));
    }

    #[test]
    fn test_zero_duration_task() {
        let tasks = vec![work(50, 100, 0)];
        assert!(can_process(0, tasks));
    }
}
