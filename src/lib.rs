use std::time::Duration;
use std::sync::mpsc;
use std::fmt::Write;
use std::io::Stdout;
use chrono::Timelike;
use notify_rust::Timeout;

#[derive(Clone,Copy)]
/// Internal program state containing set options
pub struct InternalState{
    pub time: Duration,
    pub no_stdout: bool,
    pub suppress_notifications: bool,
}


impl InternalState {
    /// updates and prints time
    fn update_time(&mut self) {
        //eprintln!("{}", self.time.as_secs());
        self.time -= Duration::from_secs(1);
        print!("\r{} ", //space at the end clears excess characters
                 Self::format_time(self.time)
        );
        use std::io::Write;
        Stdout::flush(&mut std::io::stdout()).unwrap();
    }

    /// returns true is `time` spans no Duration
    fn is_done(&self) -> bool {
        self.time.is_zero()
    }

    /// Creates and shows a system notification containing information that the timer has finished
    fn notify_done(&self) {

        use notify_rust::Notification;
        let mut summary_text: String = String::new();
        write!(summary_text, "Timer completed at {}",
               Self::current_time()
        ).unwrap();

        if let Err(_) = Notification::new()
            .appname("Timer")
            .summary(&summary_text)
            .timeout(Timeout::Never)
            .show(){
            self.notify_done_stdout();
        }
    }

    /// Prints a message to stdout that the timer has finished
    fn notify_done_stdout(&self) {
        println!("\rTimer completed at {}", Self::current_time());
    }

    /// Formats the currently stored time into "hh:mm:ss"
    fn format_time(t: Duration) -> String {
        let mut o = String::new();

        write!(o,
               "{hrs:0>2}:{mins:0>2}:{secs:0>2}",
               hrs = t.as_secs()/(60*60),
               mins = (t.as_secs()/60)%60,
               secs = t.as_secs()%60
        ).unwrap();

        o
    }

    /// Formats the current system time into "hh:mm:ss"
    fn current_time() -> String {

        let mut out = String::new();
        let et = chrono::Local::now(); //end time
        write!(out, "{:0>2}:{:0>2}:{:0>2}",
               et.hour(),
               et.minute(),
               et.minute()
        ).unwrap();
        out
    }
}

pub fn timer_core(mut is: InternalState){
    use scheduled_thread_pool::ScheduledThreadPool;

    let (tx,rx) = mpsc::channel();
    let trigger = move || {
        tx.clone().send(true).unwrap();
    };

    let pool = ScheduledThreadPool::new(1);
    pool.execute_at_fixed_rate(
        Duration::from_secs(0),
        Duration::from_secs(1),
        trigger);

    loop{
        rx.recv().unwrap();
        is.update_time();

        if is.is_done(){
            break
        }
    }
    if !is.suppress_notifications{ is.notify_done() }
    if !is.no_stdout{ is.notify_done_stdout() }

}

