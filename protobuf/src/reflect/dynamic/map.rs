use crate::reflect::map::ReflectMap;
use crate::reflect::map::ReflectMapIter;
use crate::reflect::value::hashable::ReflectValueBoxHashable;
use crate::reflect::ReflectValueBox;
use crate::reflect::ReflectValueRef;
use crate::reflect::RuntimeTypeBox;
use std::collections::HashMap;

#[derive(Debug, Clone)]
enum Maps {
    U32(HashMap<u32, ReflectValueBox>),
    I32(HashMap<i32, ReflectValueBox>),
    U64(HashMap<u64, ReflectValueBox>),
    I64(HashMap<i64, ReflectValueBox>),
    Bool(HashMap<bool, ReflectValueBox>),
    String(HashMap<String, ReflectValueBox>),
}

impl Maps {
    fn len(&self) -> usize {
        match self {
            Maps::U32(m) => m.len(),
            Maps::I32(m) => m.len(),
            Maps::U64(m) => m.len(),
            Maps::I64(m) => m.len(),
            Maps::Bool(m) => m.len(),
            Maps::String(m) => m.len(),
        }
    }

    fn is_empty(&self) -> bool {
        match self {
            Maps::U32(m) => m.is_empty(),
            Maps::I32(m) => m.is_empty(),
            Maps::U64(m) => m.is_empty(),
            Maps::I64(m) => m.is_empty(),
            Maps::Bool(m) => m.is_empty(),
            Maps::String(m) => m.is_empty(),
        }
    }

    fn clear(&mut self) {
        match self {
            Maps::U32(m) => m.clear(),
            Maps::I32(m) => m.clear(),
            Maps::U64(m) => m.clear(),
            Maps::I64(m) => m.clear(),
            Maps::Bool(m) => m.clear(),
            Maps::String(m) => m.clear(),
        }
    }

    fn key_type(&self) -> RuntimeTypeBox {
        match self {
            Maps::U32(..) => RuntimeTypeBox::U32,
            Maps::I32(..) => RuntimeTypeBox::I32,
            Maps::U64(..) => RuntimeTypeBox::U64,
            Maps::I64(..) => RuntimeTypeBox::I64,
            Maps::Bool(..) => RuntimeTypeBox::Bool,
            Maps::String(..) => RuntimeTypeBox::String,
        }
    }
}

#[derive(Debug, Clone)]
pub(crate) struct DynamicMap {
    value: RuntimeTypeBox,
    maps: Maps,
}

impl DynamicMap {
    pub fn new(key: RuntimeTypeBox, value: RuntimeTypeBox) -> DynamicMap {
        DynamicMap {
            value,
            maps: match key {
                RuntimeTypeBox::U32 => Maps::U32(HashMap::new()),
                RuntimeTypeBox::I32 => Maps::I32(HashMap::new()),
                RuntimeTypeBox::U64 => Maps::U64(HashMap::new()),
                RuntimeTypeBox::I64 => Maps::I64(HashMap::new()),
                RuntimeTypeBox::Bool => Maps::Bool(HashMap::new()),
                RuntimeTypeBox::String => Maps::String(HashMap::new()),
                t => panic!("type cannot be hashmap key: {}", t),
            },
        }
    }
}

impl ReflectMap for DynamicMap {
    fn reflect_iter(&self) -> ReflectMapIter {
        // TODO
        unimplemented!()
    }

    fn len(&self) -> usize {
        self.maps.len()
    }

    fn is_empty(&self) -> bool {
        self.maps.is_empty()
    }

    fn get<'a>(&'a self, key: ReflectValueRef) -> Option<ReflectValueRef<'a>> {
        match (&self.maps, key) {
            (Maps::U32(m), ReflectValueRef::U32(v)) => m.get(&v),
            (Maps::U64(m), ReflectValueRef::U64(v)) => m.get(&v),
            (Maps::I32(m), ReflectValueRef::I32(v)) => m.get(&v),
            (Maps::I64(m), ReflectValueRef::I64(v)) => m.get(&v),
            (Maps::Bool(m), ReflectValueRef::Bool(v)) => m.get(&v),
            (Maps::String(m), ReflectValueRef::String(v)) => m.get(&*v),
            _ => None,
        }
        .map(ReflectValueBox::as_value_ref)
    }

    fn insert(&mut self, key: ReflectValueBoxHashable, value: ReflectValueBox) {
        assert!(value.get_type() == self.value);
        match (&mut self.maps, &key) {
            (Maps::U32(m), ReflectValueBoxHashable::U32(k)) => m.insert(*k, value),
            (Maps::U64(m), ReflectValueBoxHashable::U64(k)) => m.insert(*k, value),
            (Maps::I32(m), ReflectValueBoxHashable::I32(k)) => m.insert(*k, value),
            (Maps::I64(m), ReflectValueBoxHashable::I64(k)) => m.insert(*k, value),
            (Maps::Bool(m), ReflectValueBoxHashable::Bool(k)) => m.insert(*k, value),
            (Maps::String(m), _) => match key {
                ReflectValueBoxHashable::String(k) => m.insert(k, value),
                _ => panic!("wrong key type"),
            },
            _ => panic!("wrong key type"),
        };
    }

    fn clear(&mut self) {
        self.maps.clear()
    }

    fn key_type(&self) -> RuntimeTypeBox {
        self.maps.key_type()
    }

    fn value_type(&self) -> RuntimeTypeBox {
        self.value.clone()
    }
}