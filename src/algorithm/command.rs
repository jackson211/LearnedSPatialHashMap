pub enum Command {
    Set { ket: String, value: String },
    Remove { key: String },
}
