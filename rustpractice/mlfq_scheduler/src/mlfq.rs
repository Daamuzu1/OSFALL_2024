// src/mlfq.rs

#[derive(Clone)]
pub struct Process {
    pub id: u32,
    pub priority: usize,  // Represents the current queue index
    pub remaining_time: u32,
    pub total_executed_time: u32,
}

pub struct MLFQ {
    queues: Vec<Vec<Process>>,
    num_levels: usize,
    time_quanta: Vec<u32>,
    current_time: u32,
}

impl MLFQ {
    pub fn new(num_levels: usize, time_quanta: Vec<u32>) -> Self {
        MLFQ {
            queues: vec![Vec::new(); num_levels],
            num_levels,
            time_quanta,
            current_time: 0,
        }
    }

    // Exercise 1: Queue Management
    pub fn add_process(&mut self, process: Process) {
            // Checking if the priority is valid
            if process.priority < self.num_levels {
            // Add the process to the appropriate queue based on its priority
                self.queues[process.priority].push(process);
            } 
            else {
                // Print error message if priority is out of bounds
                println!("Error: Process priority {} is out of bounds (0 to {}).", process.priority, self.num_levels - 1);
                }
    }

    // Exercise 2: Process Execution
    pub fn execute_process(&mut self, queue_index: usize) {
   // Check if the queue index is valid
   if queue_index >= self.num_levels {
    println!("Error: Queue index {} is out of bounds (0 to {}).", queue_index, self.num_levels - 1);
    return;
}

// Check if the queue has any processes to execute
if self.queues[queue_index].is_empty() {
    println!("No processes to execute in queue {}.", queue_index);
    return;
}

// Retrieve the process to execute
let process_index = 0; // Assuming we always execute the first process in the queue
let mut process = self.queues[queue_index].remove(process_index);

// Determine the time quantum for the current queue
let time_quantum = self.time_quanta[queue_index];

// Execute the process for the minimum of remaining time or time quantum
let execution_time = std::cmp::min(process.remaining_time, time_quantum);
process.remaining_time -= execution_time;
process.total_executed_time += execution_time;
self.current_time += execution_time;

// Check if the process is completed
if process.remaining_time == 0 {
    println!("Process {} completed.", process.id);
} else {
    // Move the process to the next lower priority queue if not completed
    if queue_index + 1 < self.num_levels {
        process.priority += 1; // Move to the next lower priority queue
        self.queues[queue_index + 1].push(process);
    } else {
        // If it's the lowest priority queue, re-add it to the same queue
        self.queues[queue_index].push(process);
    }
}
}
    

    // Exercise 3: Priority Boost
    pub fn priority_boost(&mut self) {
        // Iterate over all queues
        for queue in 0..self.num_levels {
            // Iterate through the processes in the current queue
            while !self.queues[queue].is_empty() {
                // Remove the process from the current queue
                let mut process = self.queues[queue].remove(0);
                // Reset the priority of the process to 0
                process.priority = 0;
                // Add the process to the highest priority queue
                self.queues[0].push(process);
            }
        }
    }

    // Simulate time passing and trigger a boost if needed
    pub fn update_time(&mut self, elapsed_time: u32) {
        self.current_time += elapsed_time;
        let boost_interval = 100;
        if self.current_time % boost_interval == 0 {
            self.priority_boost();
        }
    }
}

// Automated Test Cases
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add_process() {
        let mut mlfq = MLFQ::new(3, vec![2, 4, 8]);
        
        let process1 = Process { id: 1, priority: 0, remaining_time: 10, total_executed_time: 0 };
        let process2 = Process { id: 2, priority: 1, remaining_time: 5, total_executed_time: 0 };
        let process3 = Process { id: 3, priority: 5, remaining_time: 8, total_executed_time: 0 };

        mlfq.add_process(process1);
        mlfq.add_process(process2);
        mlfq.add_process(process3);

        assert_eq!(mlfq.queues[0].len(), 1);
        assert_eq!(mlfq.queues[1].len(), 1);
        assert_eq!(mlfq.queues[2].len(), 1);
    }

    #[test]
    fn test_execute_process() {
        let mut mlfq = MLFQ::new(3, vec![2, 4, 8]);
        mlfq.queues[0].push(Process { id: 1, priority: 0, remaining_time: 5, total_executed_time: 0 });

        mlfq.execute_process(0);

        assert_eq!(mlfq.queues[0].len(), 0);
        assert_eq!(mlfq.queues[1].len(), 1);
        assert_eq!(mlfq.queues[1][0].remaining_time, 3);
        assert_eq!(mlfq.queues[1][0].total_executed_time, 2);
    }

    #[test]
    fn test_priority_boost() {
        let mut mlfq = MLFQ::new(3, vec![2, 4, 8]);
        mlfq.queues[1].push(Process { id: 1, priority: 1, remaining_time: 5, total_executed_time: 3 });
        mlfq.queues[2].push(Process { id: 2, priority: 2, remaining_time: 3, total_executed_time: 7 });

        mlfq.update_time(100); // Should trigger priority boost

        assert_eq!(mlfq.queues[0].len(), 2);
        assert_eq!(mlfq.queues[1].len(), 0);
        assert_eq!(mlfq.queues[2].len(), 0);
    }

    #[test]
    fn test_boost_does_not_occur_prematurely() {
        let mut mlfq = MLFQ::new(3, vec![2, 4, 8]);
        mlfq.queues[1].push(Process { id: 1, priority: 1, remaining_time: 5, total_executed_time: 3 });
        
        mlfq.update_time(50); // No boost should happen

        assert_eq!(mlfq.queues[1].len(), 1);
        assert_eq!(mlfq.queues[0].len(), 0);
    }
}