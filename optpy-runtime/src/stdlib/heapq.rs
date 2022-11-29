use crate::Value;

#[allow(non_snake_case)]
pub fn __heapq__heapify(x: &Value) {
    match x {
        Value::List(list) => heapify(&mut *list.0.borrow_mut()),
        _ => todo!(),
    }
}

fn shift_down<T: PartialOrd + Clone>(heap: &mut [T], start: usize, mut pos: usize) {
    let new_item = heap[pos].clone();
    while pos > start {
        let parent_pos = (pos - 1) >> 1;
        let parent = heap[parent_pos].clone();
        if new_item < parent {
            heap[pos] = parent;
            pos = parent_pos;
        } else {
            break;
        }
    }
    heap[pos] = new_item;
}

fn shift_up<T: PartialOrd + Clone>(heap: &mut [T], mut pos: usize) {
    let end = heap.len();
    let start = pos;
    let new_item = heap[pos].clone();

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
    heap[pos] = new_item;
    shift_down(heap, start, pos);
}

fn heapify<T: PartialOrd + Clone>(x: &mut [T]) {
    let n = x.len();
    for i in (0..(n / 2)).rev() {
        shift_up(x, i);
    }
}
