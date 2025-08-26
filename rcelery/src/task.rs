use crate::celery_app::CelerApp;
pub trait TaskFactory {
    fn register(self, config: &mut CelerApp);
}

impl<T: TaskFactory> TaskFactory for Vec<T> {
    fn register(self, config: &mut CelerApp) {
        self.into_iter()
            .for_each(|factory| factory.register(config));
    }
}
