pub trait TableSchema {
    fn table_name() -> String;
    fn table_id() -> String;
}

pub trait AnyTable: Send + Sync {
    fn table_name(&self) -> &str;
    fn table_id(&self) -> &str;
    fn memory_used(&self) -> u32;
}