
pub mod webschema {

    enum ParseTech {
        HTML,
        JS
    }
    /// ## WebSchema `:: webpage schematic`
    /// Schematic constructed from a webschema file(.wsch).
    pub struct WebSchema<T> {
        name: T,
        r#type: ParseTech,
    }
}
