use vqsort_rs::Key64Value64;

#[derive(Debug, Clone, Copy)]
pub struct Job {
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

pub fn can_process(memory_limit: u64, jobs: impl IntoIterator<Item = Job>) -> bool {
    let jobs = jobs.into_iter();
    let mut events = Vec::with_capacity(2 * jobs.size_hint().0);
    for job in jobs {
        events.push(Key64Value64 {
            value: job.mem_usage as u64,
            key: job.start,
        });
        events.push(Key64Value64 {
            value: (-job.mem_usage) as u64,
            key: job.start + job.duration,
        });
    }

    vqsort_rs::sort(&mut events);

    let mut merged_event: Option<Event> = None;
    let mut remaining_mem = memory_limit as i64;
    for event in events {
        let event = Event::from(event);
        if let Some(merged_event_) = merged_event.as_mut() {
            if merged_event_.timestamp == event.timestamp {
                merged_event_.mem_diff += event.mem_diff;
            } else if merged_event_.mem_diff > remaining_mem {
                return false;
            } else {
                remaining_mem -= merged_event_.mem_diff;
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

    fn job(mem_usage: i64, start: u64, duration: u64) -> Job {
        Job {
            mem_usage,
            start,
            duration,
        }
    }

    #[test]
    fn test_empty_job_list() {
        let tasks: Vec<Job> = vec![];
        assert!(can_process(100, tasks));
    }

    #[test]
    fn test_single_task_within_limit() {
        let tasks = vec![job(50, 10, 20)];
        assert!(can_process(100, tasks));
    }

    #[test]
    fn test_single_task_exceeds_limit() {
        let tasks = vec![job(150, 0, 10)];
        assert!(!can_process(100, tasks));
    }

    #[test]
    fn test_non_overlapping_tasks() {
        let tasks = vec![job(50, 0, 10), job(30, 20, 5)];
        assert!(can_process(60, tasks));
    }

    #[test]
    fn test_overlapping_within_limit() {
        let tasks = vec![job(50, 0, 10), job(30, 5, 10)];
        assert!(can_process(80, tasks.iter().copied()));
        assert!(!can_process(79, tasks));
    }

    #[test]
    fn test_overlapping_exceeds_limit() {
        let tasks = vec![job(40, 0, 20), job(50, 10, 10), job(30, 5, 10)];
        assert!(!can_process(100, tasks.iter().copied()));
        assert!(can_process(120, tasks));
    }

    #[test]
    fn test_deallocation_increases_memory() {
        let tasks = vec![job(70, 0, 10), job(50, 10, 5)];
        assert!(can_process(70, tasks.iter().copied()));
        assert!(!can_process(69, tasks));
    }

    #[test]
    fn test_multiple_events_same_timestamp() {
        let tasks = vec![job(10, 5, 5), job(20, 5, 5), job(30, 5, 10)];
        assert!(can_process(60, tasks.clone()));
        assert!(!can_process(59, tasks));
    }

    #[test]
    fn test_zero_duration_task() {
        let tasks = vec![job(50, 100, 0)];
        assert!(can_process(0, tasks));
    }
}
