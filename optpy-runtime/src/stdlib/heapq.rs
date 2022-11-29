use crate::Value;

#[allow(non_snake_case)]
pub fn __heapq__heapify(x: &Value) {
    match x {
        Value::List(list) => heapify(&mut *list.0.borrow_mut()),
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
