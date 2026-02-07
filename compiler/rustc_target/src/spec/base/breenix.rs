use crate::spec::{PanicStrategy, TargetOptions, cvs};

pub(crate) fn opts() -> TargetOptions {
    TargetOptions {
        os: "breenix".into(),
        families: cvs!["unix"],
        has_thread_local: true,
        crt_static_default: true,
        crt_static_respected: true,
        panic_strategy: PanicStrategy::Abort,
        ..Default::default()
    }
}
