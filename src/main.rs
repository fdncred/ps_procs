mod process;

use core::time::Duration;
use process::collect_proc;

fn main() {
    let show_thread = true;

    let proc = collect_proc(Duration::from_millis(1000), show_thread);
    for p in &proc {
        println!(
            "pid: {} ppid: {} path_info: {} thread_cnt: {} thread_0_cpu: {} udp_cnt: {} tcp_cnt: {} cur_usage: {:?} prev_usage: {:?} interval_ms: {}",
            &p.pid,
            &p.ppid,
            &p.curr_path.as_ref().unwrap().name,
            &p.curr_threads.len(),
            &p.curr_threads[0].pth_cpu_usage,
            &p.curr_udps.len(),
            &p.curr_tcps.len(),
            &p.curr_res,
            &p.prev_res,
            &p.interval.as_millis(),
        );
    }
}
