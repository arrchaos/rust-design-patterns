use std::{cell::RefCell, fmt::Debug};
use crate::structural::responsiblity_chain::cor::CoR;
use super::cor::Handler;
#[derive(Debug)]
struct TestState {
   int:  u32,
   vector: RefCell<Vec<u32>>
}
fn create_chain<T: Debug + 'static>(callback: fn(&mut T)) -> Handler<T> {
    let default_sum_handler = Handler::<T>::default(callback);
    let pre_latest_sum = Handler::<T>::new(default_sum_handler, callback);
    let first_sum = Handler::<T>::new(pre_latest_sum, callback);
    first_sum
}

#[test]
fn test_mutation() {
    let mut testing_state = TestState { int: 0 , vector: RefCell::new(Vec::new())};
    create_chain(| v: &mut TestState |  { 
        v.int += 1;
    }).execute(&mut testing_state);
    assert_eq!(testing_state.int, 3)
}
 #[test]
fn test_sequence() {
    let mut testing_state = TestState { int: 0 , vector: RefCell::new(Vec::new())};
   
    create_chain(| v: &mut TestState |  { 
        v.int += 1;
        v.vector.borrow_mut().push(v.int)
    }).execute(&mut testing_state);
    println!("{:?}", testing_state);
    assert_eq!(testing_state.vector.take(), vec![1, 2, 3])
}
