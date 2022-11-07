
pub trait CoR <T> {
    fn execute(&mut self, handling_obj: &mut T) {
        self.handle(handling_obj);

        if let Some(next) = &mut self.next() {
            next.execute(handling_obj);
        }
    }

    fn handle(&mut self, handling_obj: &mut T);
    fn next(&mut self) -> &mut Option<Box<dyn CoR<T> >>;
}

/// Helps to wrap an object into a boxed type.
pub(self) fn into_next<T>(
    handler: impl CoR<T> + Sized + 'static,
) -> Option<Box< dyn CoR<T> >> {
    Some(Box::new(handler))
}




#[derive(Default)]
pub struct Handler<T> {
    next: Option< Box< dyn CoR<T> > >,
    handling_fn: Option<fn( &mut T)> 
}

impl<T> Handler<T> {
    pub fn new(next: impl CoR<T> + 'static,  handling: fn(&mut T)) -> Self {
        Self {
            next: into_next(next),
            handling_fn: Some(handling)
        }
    }
    pub fn default(handling: fn(&mut T)) -> Self {
        Self {
            next: None,
            handling_fn: Some(handling)
        }
    } 
}

impl<T: std::fmt::Debug> CoR<T> for Handler<T> {
    fn handle(&mut self, handling_obj: &mut T) {
        match self.handling_fn {
            Some(function) => function(handling_obj),
            None => (),
        } 
    }

    fn next(&mut self) -> &mut Option<Box<dyn CoR<T>>> {
        &mut self.next
    }
}
