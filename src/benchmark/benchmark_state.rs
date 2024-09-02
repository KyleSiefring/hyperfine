use std::process::ExitStatus;
use crate::benchmark::timing_result::TimingResult;
use crate::util::exit_code::extract_exit_code;
use crate::util::units::Second;

pub struct BenchmarkState {

}

impl BenchmarkState {
    // pub fn new() -> Self {
    //     BenchmarkState {
    //         times_real: vec![],
    //         times_user: vec![],
    //         times_system: vec![],
    //         exit_codes: vec![],
    //         all_succeeded: true,
    //     }
    // }

    // pub fn push_iteration(&mut self, timing: TimingResult, status: ExitStatus) {
    //     self.times_real.push(timing.time_real);
    //     self.times_user.push(timing.time_user);
    //     self.times_system.push(timing.time_system);
    //     self.exit_codes.push(extract_exit_code(status));
    //
    //     self.all_succeeded = self.all_succeeded && status.success();
    // }
}