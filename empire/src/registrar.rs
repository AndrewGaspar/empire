use std::{collections::VecDeque, sync::{Arc, Weak}};

pub struct Registrar<T> {
    objects: Vec<Option<Arc<T>>>,
    free_list: VecDeque<usize>,
}

#[derive(Clone)]
struct RegistrationImpl<T> {
    object: Weak<T>,
    strong_ref_index: usize,
}

#[derive(Clone)]
pub struct Registration<T> {
    internal: Box<RegistrationImpl<T>>,
}

impl<T> Registrar<T> {
    pub fn new() -> Self {
        Self {
            objects: Vec::new(),
            free_list: VecDeque::new(),
        }
    }

    pub fn track_object(&mut self, object: T) -> Registration<T> {
        let arc = Arc::new(object);
        let weak = Arc::downgrade(&arc);

        let index = if let Some(index) = self.free_list.pop_front() {
            assert!(self.objects[index].is_none());
            self.objects[index] = Some(arc);
            index
        } else {
            let index = self.objects.len();
            self.objects.push(Some(arc));
            index
        };

        Registration {
            internal: Box::new(RegistrationImpl {
                object: weak,
                strong_ref_index: index,
            }),
        }
    }

    pub fn free_object(&mut self, object: Registration<T>) {
        let index = object.internal.strong_ref_index;

        {
            let tracked = self.objects[index]
                .as_ref()
                .expect("Tried to free object that was already freed.");

            debug_assert!(
                Arc::ptr_eq(&object.internal.object.upgrade().unwrap(), &tracked),
                "Internal error: registration is not tracking the correct object"
            );
        }

        self.objects[index] = None;
        self.free_list.push_back(index);
    }
}

impl<T> Registration<T> {
    pub fn get(&self) -> Weak<T> {
        self.internal.object.clone()
    }
}
