#[derive(Debug)]
pub enum PreParserSymbols {
    Unknown,
    Comments(Vec<String>),
    // All of these are (Name, Type, Value, Mutable?)
    LiteralStringVariableDefinition(String, String, String, bool),
    LiteralIntegerVariableDefinition(String, String, usize, bool),
    LiteralFloatingPointVariableDefinition(String, String, f64, bool),
    OtherVariableDefinition(String, String, String, bool),
}
