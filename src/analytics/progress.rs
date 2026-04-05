use std::fmt;

#[derive(Clone, Copy, PartialEq, PartialOrd, Eq, Ord)]
pub enum BoMBook {
    Nephi1,
    Nephi2,
    Jacob,
    Enos,
    Jarom,
    Omni,
    WoM,
    Mosiah,
    Alma,
    Helaman,
    Nephi3,
    Nephi4,
    Mormon,
    Ether,
    Moroni,
}

impl BoMBook {
    pub const fn chapters(&self) -> usize {
        match self {
            BoMBook::Nephi1 => 22,
            BoMBook::Nephi2 => 33,
            BoMBook::Jacob => 7,
            BoMBook::Enos => 1,
            BoMBook::Jarom => 1,
            BoMBook::Omni => 1,
            BoMBook::WoM => 1,
            BoMBook::Mosiah => 29,
            BoMBook::Alma => 63,
            BoMBook::Helaman => 16,
            BoMBook::Nephi3 => 30,
            BoMBook::Nephi4 => 1,
            BoMBook::Mormon => 9,
            BoMBook::Ether => 15,
            BoMBook::Moroni => 10,
        }
    }

    pub const fn next(&self) -> Option<BoMBook> {
        match self {
            BoMBook::Nephi1 => Some(BoMBook::Nephi2),
            BoMBook::Nephi2 => Some(BoMBook::Jacob),
            BoMBook::Jacob => Some(BoMBook::Enos),
            BoMBook::Enos => Some(BoMBook::Jarom),
            BoMBook::Jarom => Some(BoMBook::Omni),
            BoMBook::Omni => Some(BoMBook::WoM),
            BoMBook::WoM => Some(BoMBook::Mosiah),
            BoMBook::Mosiah => Some(BoMBook::Alma),
            BoMBook::Alma => Some(BoMBook::Helaman),
            BoMBook::Helaman => Some(BoMBook::Nephi3),
            BoMBook::Nephi3 => Some(BoMBook::Nephi4),
            BoMBook::Nephi4 => Some(BoMBook::Mormon),
            BoMBook::Mormon => Some(BoMBook::Ether),
            BoMBook::Ether => Some(BoMBook::Moroni),
            BoMBook::Moroni => None,
        }
    }

    pub fn from_url_segment(s: &str) -> Option<Self> {
        match s {
            "1-ne" => Some(BoMBook::Nephi1),
            "2-ne" => Some(BoMBook::Nephi2),
            "jacob" => Some(BoMBook::Jacob),
            "enos" => Some(BoMBook::Enos),
            "jarom" => Some(BoMBook::Jarom),
            "omni" => Some(BoMBook::Omni),
            "w-of-m" => Some(BoMBook::WoM),
            "mosiah" => Some(BoMBook::Mosiah),
            "alma" => Some(BoMBook::Alma),
            "hel" => Some(BoMBook::Helaman),
            "3-ne" => Some(BoMBook::Nephi3),
            "4-ne" => Some(BoMBook::Nephi4),
            "morm" => Some(BoMBook::Mormon),
            "ether" => Some(BoMBook::Ether),
            "moro" => Some(BoMBook::Moroni),
            _ => None,
        }
    }
}

#[derive(Clone, Copy, PartialEq, PartialOrd, Eq, Ord)]
pub struct BoMProgress {
    pub book: BoMBook,
    pub chapter: usize,
}

impl BoMProgress {
    pub fn next(self) -> Option<Self> {
        let chapter = if self.chapter < self.book.chapters() {
            self.chapter + 1
        } else {
            0
        };
        let book = if chapter == 0 {
            self.book.next()?
        } else {
            self.book
        };

        Some(Self { book, chapter })
    }

    pub const fn progress(self) -> usize {
        let mut count = 0;
        let mut book = BoMBook::Nephi1;
        while (book as usize) < self.book as usize {
            count += book.chapters();
            book = match book.next() {
                Some(b) => b,
                None => BoMBook::Moroni,
            };
        }

        count += self.chapter;

        count
    }

    const TOTAL: usize = BoMProgress {
        book: BoMBook::Moroni,
        chapter: 10,
    }
    .progress()
        + 1;

    pub fn percentage(self) -> f64 {
        (self.progress() as f64) / (Self::TOTAL as f64)
    }

    pub fn from_percentage(p: f64) -> Self {
        Self::from_progress((p * (Self::TOTAL as f64)) as usize)
    }

    pub fn from_progress(mut p: usize) -> Self {
        let mut book = BoMBook::Nephi1;
        while p > book.chapters() {
            p -= book.chapters();
            book = book.next().unwrap_or(BoMBook::Moroni);
        }

        BoMProgress { book, chapter: p }
    }

    pub fn from_url(url: &str) -> Option<Self> {
        let (_, url) = url.split_once("/study/scriptures/bofm/")?;
        let mut parts = url.split('/').flat_map(|u| u.split('?'));
        let book = BoMBook::from_url_segment(parts.next()?)?;
        let chapter = parts.next()?.parse::<usize>().ok()? - 1;

        Some(BoMProgress { book, chapter })
    }
}

impl fmt::Display for BoMBook {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let name = match self {
            BoMBook::Nephi1 => "1 Nephi",
            BoMBook::Nephi2 => "2 Nephi",
            BoMBook::Jacob => "Jacob",
            BoMBook::Enos => "Enos",
            BoMBook::Jarom => "Jarom",
            BoMBook::Omni => "Omni",
            BoMBook::WoM => "Words of Mormon",
            BoMBook::Mosiah => "Mosiah",
            BoMBook::Alma => "Alma",
            BoMBook::Helaman => "Helaman",
            BoMBook::Nephi3 => "3 Nephi",
            BoMBook::Nephi4 => "4 Nephi",
            BoMBook::Mormon => "Mormon",
            BoMBook::Ether => "Ether",
            BoMBook::Moroni => "Moroni",
        };
        write!(f, "{name}")
    }
}

impl fmt::Display for BoMProgress {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} {}", self.book, self.chapter + 1)
    }
}
