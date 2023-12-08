#[derive(Debug)]
pub struct Linspace {
    start: u64,
    len: u64,
}


#[derive(Debug)]
pub struct LinspaceUnion {
    linspaces: Vec<Linspace>,
}


impl Linspace {
    pub fn end(&self) -> u64 {
        self.start + self.len - 1
    }

    pub fn shift(&mut self, shift: u64) {
        self.start += shift;
    }

    pub fn negative_shift(&mut self, shift: u64) {
        self.start -= shift;
    }

    pub fn new(start: u64, len: u64) -> Linspace {
        Linspace {
            start,
            len,
        }
    }
}


impl Default for LinspaceUnion {
    fn default() -> Self {
        Self::new()
    }
}

impl LinspaceUnion {
    pub fn new() -> LinspaceUnion {
        LinspaceUnion {
            linspaces: vec![],
        }
    }

    pub fn push(&mut self, linspace: Linspace) {
        self.linspaces.push(linspace);
    }

    pub fn shift(&mut self, shift: u64) {
        for linspace in self.linspaces.iter_mut() {
            linspace.shift(shift);
        }
    }

    pub fn negative_shift(&mut self, shift: u64) {
        for linspace in self.linspaces.iter_mut() {
            linspace.negative_shift(shift);
        }
    }

    pub fn min(&self) -> u64 {
        return self.linspaces.iter().map(|linspace| linspace.start).min().unwrap();
    }

    pub fn max(&self) -> u64 {
        return self.linspaces.iter().map(|linspace| linspace.end()).max().unwrap();
    }

    pub fn extract(&mut self, start: u64, len: u64) -> Option::<LinspaceUnion> {
        let mut linspaces = vec![];
        let mut to_remove = vec![];
        let mut to_add = vec![];

        let end = start + len - 1;

        for (i, linspace) in self.linspaces.iter().enumerate() {
            if (linspace.start >= start && linspace.start <= end) ||
               (linspace.end() >= start && linspace.end() <= end)
            {
                let start = u64::max(linspace.start, start);
                let end = u64::min(linspace.end(), end);
                let len = end - start + 1;

                to_remove.push(i);
                linspaces.push(
                    Linspace {
                        start,
                        len,
                    }
                );

                if start > linspace.start {
                    to_add.push(
                        Linspace {
                            start: linspace.start,
                            len: start - linspace.start,
                        }
                    );
                }

                if end < linspace.end() {
                    to_add.push(
                        Linspace {
                            start: end + 1,
                            len: linspace.end() - end,
                        }
                    );
                }
            }
        }

        if !linspaces.is_empty() {
            for i in to_remove.iter().rev() {
                self.linspaces.remove(*i);
            }

            for linspace in to_add {
                self.linspaces.push(linspace);
            }

            return Some(
                LinspaceUnion {
                    linspaces,
                }
            );
        }

        None
    }

    pub fn extend(&mut self, other: &LinspaceUnion) {
        for linspace in other.linspaces.iter() {
            self.push(Linspace::new(linspace.start, linspace.len));
        }
    }
}
