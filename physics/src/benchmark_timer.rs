pub(crate) trait BenchmarkTimer {
    fn reset_timer(&mut self);
    fn mark_timer(&mut self, label: &'static str);
    fn print_times(&self);
}

pub(crate) struct NoopTimerInstance;

impl BenchmarkTimer for NoopTimerInstance {
    #[inline(always)]
    fn mark_timer(&mut self, _: &'static str) {}

    #[inline(always)]
    fn reset_timer(&mut self) {}

    #[inline(always)]
    fn print_times(&self) {}
}
