use super::CodeBuilder;

impl CodeBuilder<'_> {
    pub fn create_label(&mut self, name: String) {
        self.func_symbols.insert(name, self.code_holder.instructions.len());
    }
}