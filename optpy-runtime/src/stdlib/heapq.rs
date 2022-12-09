use crate::{cell::UnsafeRefCell, Value};

#[allow(non_snake_case)]
pub fn __heapq__heapify(x: &Value) {
    match x {
        Value::List(list) => heapify(&mut *list.borrow_mut()),
        _ => todo!(),
    }
}
#[allow(non_snake_case)]
pub fn __heapq__heappush(heap: &Value, item: &Value) {
    match heap {
        Value::List(list) => heap_push(&mut *list.borrow_mut(), UnsafeRefCell::rc(item.clone())),
        _ => todo!(),
    }
}
#[allow(non_snake_case)]
pub fn __heapq__heappop(heap: &Value) -> Value {
    match heap {
        Value::List(list) => heap_pop(&mut *list.borrow_mut()).borrow().clone(),
        _ => todo!(),
    }
}

fn shift_down<T: PartialOrd>(heap: &mut [T], start: usize, mut pos: usize) {
    while pos > start {
        let parent_pos = (pos - 1) >> 1;
        if heap[pos] < heap[parent_pos] {
            heap.swap(pos, parent_pos);
            pos = parent_pos;
        } else {
            break;
        }
    }
}

fn shift_up<T: PartialOrd>(heap: &mut [T], mut pos: usize) {
    let end = heap.len();
    let start = pos;

    let mut child = 2 * pos + 1;
    while child < end {
        let right = child + 1;
        if right < end && heap[child] >= heap[right] {
            child = right;
        }

        heap.swap(pos, child);
        pos = child;
        child = 2 * pos + 1;
    }
    shift_down(heap, start, pos);
}

fn heapify<T: PartialOrd>(x: &mut [T]) {
    let n = x.len();
    for i in (0..(n / 2)).rev() {
        shift_up(x, i);
    }
}

fn heap_push<T: PartialOrd>(heap: &mut Vec<T>, item: T) {
    heap.push(item);
    let n = heap.len();
    shift_down(heap, 0, n - 1);
}

fn heap_pop<T: PartialOrd>(heap: &mut Vec<T>) -> T {
    if heap.len() >= 2 {
        let n = heap.len();
        heap.swap(n - 1, 0);
        let response = heap.pop().expect("empty heap");
        shift_up(heap, 0);
        response
    } else {
        heap.pop().expect("empty heap")
    }
}
