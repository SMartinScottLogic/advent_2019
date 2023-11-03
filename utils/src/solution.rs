pub trait Solution {
    type Result;

    fn analyse(&mut self, is_full: bool);

    fn answer_part1(&self, is_full: bool) -> Self::Result;
    fn answer_part2(&self, is_full: bool) -> Self::Result;
}

pub fn load<T: Solution + TryFrom<std::io::BufReader<std::fs::File>, Error = std::io::Error>>(
    filename: &str,
) -> std::io::Result<T> {
    let file = std::fs::File::open(filename)?;

    let reader = std::io::BufReader::new(file);
    T::try_from(reader)
}
