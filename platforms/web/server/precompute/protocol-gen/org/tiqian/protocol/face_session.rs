#[derive(Clone, Copy)]
pub struct FaceSession;

impl FaceSession {
    pub fn face_session_ordered_face_specs(source_orders: &Vec<Option<f64>>) -> Vec<u32> {
        let mut inputs: Vec<OrderedFaceSpecInput> = vec![];
        let mut input_order_number = 0.0f64;
        let mut i = 0u32;
        while (i32::from_ne_bytes(((i) as i32).to_ne_bytes())) < (i32::from_ne_bytes(((u32::try_from((source_orders.len()) & 0xFFFF_FFFF).unwrap_or(0)) as i32).to_ne_bytes())) {
            let raw = source_orders[usize::try_from(i).unwrap_or(0)];
            let resolved = match &(raw) { None => input_order_number, Some(__option) => *__option };
            inputs.push(OrderedFaceSpecInput { index: i, order: resolved });
            i = u32::wrapping_add(i, 1);
            input_order_number += 1.0f64;
        }
        let mut write_idx = 1u32;
        while (i32::from_ne_bytes(((write_idx) as i32).to_ne_bytes())) < (i32::from_ne_bytes(((u32::try_from((inputs.len()) & 0xFFFF_FFFF).unwrap_or(0)) as i32).to_ne_bytes())) {
            let record = inputs[usize::try_from(write_idx).unwrap_or(0)];
            let mut read_idx = write_idx;
            while (read_idx) > (0) && FaceSession::face_session_face_order_before((record).clone(), inputs[usize::try_from(u32::wrapping_sub(read_idx, 1)).unwrap_or(0)]) {
                inputs[usize::try_from(read_idx).unwrap_or(0)] = inputs[usize::try_from(u32::wrapping_sub(read_idx, 1)).unwrap_or(0)];
                read_idx = u32::wrapping_sub(read_idx, 1);
            }
            inputs[usize::try_from(read_idx).unwrap_or(0)] = record;
            write_idx = u32::wrapping_add(write_idx, 1);
        }
        let mut out: Vec<u32> = vec![];
        let mut j = 0u32;
        while (i32::from_ne_bytes(((j) as i32).to_ne_bytes())) < (i32::from_ne_bytes(((u32::try_from((inputs.len()) & 0xFFFF_FFFF).unwrap_or(0)) as i32).to_ne_bytes())) {
            out.push(inputs[usize::try_from(j).unwrap_or(0)].index);
            j = u32::wrapping_add(j, 1);
        }
        return out;
    }

    pub(crate) fn face_session_face_order_before(a: OrderedFaceSpecInput, b: OrderedFaceSpecInput) -> bool {
        if a.order != b.order {
            return (a.order) < (b.order);
        }
        return (i32::from_ne_bytes(((a.index) as i32).to_ne_bytes())) < (i32::from_ne_bytes(((b.index) as i32).to_ne_bytes()));
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct OrderedFaceSpecInput {
    pub index: u32,
    pub order: f64,
}
