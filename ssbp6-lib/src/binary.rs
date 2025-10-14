

#[cfg(test)]
pub mod tests {
    use std::error::Error;

    #[test]
    fn read_binary_project() -> Result<(), Box<dyn Error>> {
        let path = "E:/Metaphor/base_cpk/COMMON/ui/ss/camp_top.ssbp";
        if !std::fs::exists(path)? {
            return Ok(());
        }
        let mut binary = std::fs::read(path)?;
        Ok(())
    }
}